use std::collections::HashMap;

use restapp::github::{Branch, CommitDetail, BRANCHES_URL, MAIN_BRANCH_URL};
use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    let resp_str = reqwest::Client::builder()
        .user_agent("reqwest-test-agent")
        .build()
        .expect("client builder")
        .get(BRANCHES_URL)
        .send()
        .await?
        .text()
        .await?;
    // println!("{:#?}", resp_str);

    // Parse the string of data into serde_json::Value.
    let mut v: Vec<Branch> = serde_json::from_str(resp_str.as_str()).unwrap();
    println!("{:#?}", v.pop());

    let client = reqwest::Client::new();
    let resp_str_b = client.get(BRANCHES_URL)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .text()
        .await?;
    // println!("{:#?}", resp_str_b);

    let vb: Vec<Branch> = serde_json::from_str(resp_str_b.as_str()).unwrap();
    println!("{:#?}", vb.first());

    let resp_b = client.get(MAIN_BRANCH_URL)
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<CommitDetail>()
        .await?;
    println!("{:#?}", resp_b);

    let resp_c = client.get(BRANCHES_URL)
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<Vec<Branch>>()
        .await?;
    println!("{:#?}", resp_c.first());

    Ok(())
}
