use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

pub mod err;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_path(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(err::custom_error)
            .service(err::custom_error_json)
            .route("/hey/{name}", web::get().to(manual_path))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app =
            test::init_service(App::new().route("/hey/{name}", web::get().to(manual_path))).await;
        let req = test::TestRequest::get().uri("/hey/yson").to_request();
        let resp = test::call_service(&app, req).await;
        println!("{}", resp.status());
        assert!(resp.status().is_success());
    }
}
