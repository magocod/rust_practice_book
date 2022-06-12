use actix_web::{web, Responder, Result};
use commons::mongo::connect;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{thread, time};

use serde_json;

use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};

pub struct CounterState {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub struct AppState {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
    pub mongodb_client: Client,
}

impl AppState {
    pub async fn new() -> Self {
        let mongodb_client = connect().await.expect("failed to connect mongodb");
        Self {
            counter: Mutex::new(0),
            mongodb_client,
        }
    }
}

pub struct RabbitState {
    pub connection: Connection, // <- Mutex is necessary to mutate safely across threads
    pub channel: Channel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDuration {
    delay: u32,
}

impl RabbitState {
    pub async fn new() -> Self {
        let uri = "amqp://admin:admin@localhost:5672";
        let options = ConnectionProperties::default();

        let connection = Connection::connect(uri, options).await.unwrap();
        let channel = connection.create_channel().await.unwrap();

        channel
            .basic_qos(1, BasicQosOptions::default())
            .await
            .unwrap();

        let _queue = channel
            .queue_declare(
                "queue_test",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        let consumer = channel
            .basic_consume(
                "queue_test",
                "tag_foo",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        consumer.set_delegate(move |delivery: DeliveryResult| async move {
            let delivery = match delivery {
                // Carries the delivery alongside its channel
                Ok(Some(delivery)) => delivery,
                // The consumer got canceled
                Ok(None) => return,
                // Carries the error and is always followed by Ok(None)
                Err(error) => {
                    dbg!("Failed to consume queue message {}", error);
                    return;
                }
            };

            // Do something with the delivery data (The message payload)
            println!("delivery: {:?}", delivery.delivery_tag);

            let data_str = match String::from_utf8(delivery.data.clone()) {
                Ok(v) => v,
                Err(e) => {
                    println!("error data {}", e);
                    String::new()
                }
            };

            // println!("data: {}", data_str);
            let p: TaskDuration =
                serde_json::from_str(data_str.as_str()).expect("error parse data");
            println!("p: {:?}", p);

            let ten_millis = time::Duration::from_millis(p.delay as u64);

            println!("start sleep");
            thread::sleep(ten_millis);
            println!("end sleep");

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack send_webhook_event message");
        });

        Self {
            connection,
            channel,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterForm {
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    current: i32,
    form: Option<CounterForm>,
}

impl Counter {
    pub fn new(current: i32, form: Option<CounterForm>) -> Self {
        Self { current, form }
    }
}

pub async fn update_counter(data: web::Data<CounterState>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

pub async fn json_update_counter(
    data: web::Data<CounterState>,
    counter_form: web::Json<CounterForm>,
) -> Result<impl Responder> {
    println!("{:?}", counter_form);
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += counter_form.quantity; // <- access counter inside MutexGuard

    Ok(web::Json(Counter::new(
        *counter,
        Some(CounterForm {
            quantity: counter_form.quantity,
        }),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn increment_the_counter_by_one_unit() {
        let app_state = web::Data::new(CounterState {
            counter: Mutex::new(0),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/update_counter", web::get().to(update_counter)),
        )
        .await;
        let req = test::TestRequest::get().uri("/update_counter").to_request();

        let resp = test::call_service(&app, req).await;

        println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn increment_the_counter_by_quantity() {
        let app_state = web::Data::new(CounterState {
            counter: Mutex::new(1),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/json_update_counter", web::get().to(json_update_counter)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/json_update_counter")
            .set_json(CounterForm { quantity: 3 })
            .to_request();

        let resp = test::call_service(&app, req).await;

        println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn error_counter_increment_not_received() {
        let app_state = web::Data::new(CounterState {
            counter: Mutex::new(1),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/json_update_counter", web::get().to(json_update_counter)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/json_update_counter")
            .to_request();

        let resp = test::call_service(&app, req).await;

        println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_client_error());
    }
}
