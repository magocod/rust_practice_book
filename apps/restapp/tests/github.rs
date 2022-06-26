use actix_web::{http as ActixHttp, test, web, App};
use http;
mod common;
mod support;
use support::*;

use restapp::github::{Branch, GitHubUrl, UrlProvider, GitHubError};

#[actix_web::test]
async fn successful_provider_http_request() {
    let (scope, _) = common::setup();

    let server = server::http(move |_req| async {
        let b = vec![Branch::factory()];
        // Serialize it to a JSON string.
        let json_str = serde_json::to_string(&b).expect("failed serialize");
        http::Response::new(json_str.into())
    });

    let urls = GitHubUrl {
        branches: String::new(),
        main_branch: String::new(),
    };
    let provider = UrlProvider::new(format!("http://{}/json", server.addr()), urls);

    let app =
        test::init_service(App::new().app_data(web::Data::new(provider)).service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/provider_http_request")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), ActixHttp::StatusCode::OK);
}

#[actix_web::test]
async fn unknown_failed_provider_http_request() {
    let (scope, _) = common::setup();

    let server = server::http(move |_req| async {
        let response = http::Response::new("example exception plain text".into());
        let (mut parts, body) = response.into_parts();

        parts.status = http::StatusCode::BAD_REQUEST;
        http::Response::from_parts(parts, body)
    });

    let urls = GitHubUrl {
        branches: String::new(),
        main_branch: String::new(),
    };
    let provider = UrlProvider::new(format!("http://{}/json", server.addr()), urls);

    let app =
        test::init_service(App::new().app_data(web::Data::new(provider)).service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/provider_http_request")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), ActixHttp::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn not_found_failed_provider_http_request() {
    let (scope, _) = common::setup();

    let server = server::http(move |_req| async {
        let b = GitHubError::not_found();
        // Serialize it to a JSON string.
        let json_str = serde_json::to_string(&b).expect("failed serialize");
        let response = http::Response::new(json_str.into());
        let (mut parts, body) = response.into_parts();

        parts.status = http::StatusCode::NOT_FOUND;
        http::Response::from_parts(parts, body)
    });

    let urls = GitHubUrl {
        branches: String::new(),
        main_branch: String::new(),
    };
    let provider = UrlProvider::new(format!("http://{}/json", server.addr()), urls);

    let app =
        test::init_service(App::new().app_data(web::Data::new(provider)).service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/provider_http_request")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), ActixHttp::StatusCode::BAD_REQUEST);
}
