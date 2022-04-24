use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct AppState {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
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

pub async fn update_counter(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

pub async fn json_update_counter(
    data: web::Data<AppState>,
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
        let app_state = web::Data::new(AppState {
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
        let app_state = web::Data::new(AppState {
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
        let app_state = web::Data::new(AppState {
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
