use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

pub mod err;
pub mod errors;
pub mod mongo_crud;
pub mod state;

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello_json")]
pub async fn hello_json() -> impl Responder {
    HttpResponse::Ok().json(MyObj {
        name: String::from("Hello world!"),
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn base_path_returns_plain_text() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();

        let resp = test::call_service(&app, req).await;

        println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn return_json_content() {
        let app = test::init_service(App::new().service(hello_json)).await;
        let req = test::TestRequest::get().uri("/hello_json").to_request();

        let resp = test::call_service(&app, req).await;

        // println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn returns_the_data_received_in_the_request() {
        let app = test::init_service(App::new().service(echo)).await;
        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(MyObj {
                name: String::from("echo"),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        // println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app =
            test::init_service(App::new().route("/hey/{name}", web::get().to(manual_path))).await;
        let req = test::TestRequest::get().uri("/hey/yson").to_request();

        let resp = test::call_service(&app, req).await;

        // println!("{}, {:?}", resp.status(), resp.response().body());
        assert!(resp.status().is_success());

        // let result: MyObj = test::read_body_json(resp).await;
        // println!("{:?}", result);
    }

    #[actix_web::test]
    async fn test_index_get_json() {
        let param = "yson";

        let app =
            test::init_service(App::new().route("/hey/{name}", web::get().to(manual_path))).await;
        let req = test::TestRequest::get()
            .uri(format!("/hey/{}", param).as_str())
            .to_request();

        let resp: MyObj = test::call_and_read_body_json(&app, req).await;

        // println!("{:?}", resp);
        assert_eq!(resp.name, param);
    }
}
