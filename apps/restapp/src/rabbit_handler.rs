use actix_web::{web, HttpResponse};

use crate::state::{RabbitState, TaskDuration};

use lapin::{
    // message::DeliveryResult,
    options::BasicPublishOptions,
    BasicProperties,
};

use crate::errors::ServiceError;

pub async fn send_to_queue(
    rabbit: web::Data<RabbitState>,
    job_data: web::Json<TaskDuration>,
) -> Result<HttpResponse, ServiceError> {
    // Serialize it to a JSON string.
    let payload = serde_json::to_string(&job_data)?;
    // println!("{}", payload);

    let _conf = rabbit
        .channel
        .basic_publish(
            "",
            "queue_test",
            BasicPublishOptions::default(),
            // b"Hello world!",
            &payload.into_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    // println!("{:?}", conf);

    Ok(HttpResponse::Ok().json("send to rabbit"))
}
