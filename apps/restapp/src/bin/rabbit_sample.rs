use std::{thread, time};

use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};

// #[tokio::main]
#[actix_web::main]
async fn main() {
    let uri = "amqp://admin:admin@localhost:5672";
    let options = ConnectionProperties::default();
    // Use tokio executor and reactor.
    // At the moment the reactor is only available for unix.
    // .with_executor(tokio_executor_trait::Tokio::current())
    // .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(uri, options).await.unwrap();
    let channel = connection.create_channel().await.unwrap();

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
        // println!("{:?}", delivery.data);

        let data_str = match String::from_utf8(delivery.data.clone()) {
            Ok(v) => v,
            Err(e) => {
                println!("error data {}", e);
                String::new()
            }
        };

        println!("data: {:?}", data_str);

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("Failed to ack send_webhook_event message");
    });

    channel
        .basic_publish(
            "",
            "queue_test",
            BasicPublishOptions::default(),
            b"Hello world!",
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();

    let ten_millis = time::Duration::from_millis(6000);
    let now = time::Instant::now();

    println!("start sleep");

    thread::sleep(ten_millis);

    println!("end sleep");

    assert!(now.elapsed() >= ten_millis);
}
