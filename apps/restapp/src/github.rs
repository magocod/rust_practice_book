use crate::errors::ServiceError;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use reqwest::header::{USER_AGENT};

pub const BRANCHES_URL: &str = "https://api.github.com/repos/magocod/ts_express/branches";
pub const MAIN_BRANCH_URL: &str = "https://api.github.com/repos/magocod/ts_express/commits/7dd64cdd398c1ad5fbbdfbb58ef217898a81b9ae";

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
    pub protected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetail {
    pub sha: String,
    pub node_id: String,
    pub url: String,
    pub commit: CommitMetaData,
    pub parents: Vec<Parent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitMetaData {
    pub message: String,
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    sha: String,
    url: String,
    html_url: String,
}

pub async fn http_request() -> Result<HttpResponse, ServiceError> {
    let client = reqwest::Client::new();
    let resp = client.get(BRANCHES_URL)
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<Vec<Branch>>()
        .await?;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn reuse_http_request(http_client: web::Data<reqwest::Client>,) -> Result<HttpResponse, ServiceError> {
    let resp = http_client.get(BRANCHES_URL)
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<Vec<Branch>>()
        .await?;
    Ok(HttpResponse::Ok().json(resp))
}
