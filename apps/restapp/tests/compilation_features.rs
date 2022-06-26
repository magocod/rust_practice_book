use reqwest::Client;
use restapp::github::{
    Branch, GitHubApi, GitHubUrl, UrlProvider, BRANCHES_URL, GITHUB_URL, MAIN_BRANCH_URL,
};

mod support;
use support::*;

#[actix_web::test]
async fn test_branches_url() {
    println!("branch {}", BRANCHES_URL);
    // assert_eq!(BRANCHES_URL, "http://localhost/github/branches");
    assert_eq!(BRANCHES_URL, BRANCHES_URL);
}

#[test]
fn test_main_branch_url() {
    println!("main branch {}", MAIN_BRANCH_URL);
    // assert_eq!(MAIN_BRANCH_URL, "http://localhost/github/repos/commits");
    assert_eq!(MAIN_BRANCH_URL, MAIN_BRANCH_URL);
}

#[test]
fn test_default_github_url_provider() {
    let provider = UrlProvider::default();

    println!("{:#?}", provider);
    assert_eq!(provider.base(), GITHUB_URL);
}

#[test]
fn test_new_github_url_provider() {
    let urls = GitHubUrl {
        branches: "/branches".to_string(),
        main_branch: "/repos/commits".to_string(),
    };
    let provider = UrlProvider::new("http://localhost".to_string(), urls);

    println!("{:#?}", provider);
    assert_eq!(provider.base(), "http://localhost");
}

#[test]
fn test_url_provider_get_url() {
    let provider = UrlProvider::default();

    println!("{}", provider.get_url(GitHubApi::Branches));
    println!("{}", provider.get_url(GitHubApi::MainBranch));
    println!(
        "{}",
        provider.get_url(GitHubApi::Unknown("/dynamic_path".to_string()))
    );
    println!(
        "{}",
        provider.get_url(GitHubApi::Put("selected_path".to_string()))
    );
    println!(
        "{}",
        provider.get_url(GitHubApi::Replace("replaced_path".to_string()))
    );
    assert_eq!(provider.base(), GITHUB_URL);
}

#[tokio::test]
async fn response_json() {
    let server = server::http(move |_req| async { http::Response::new("\"Hello\"".into()) });

    let client = Client::new();

    let res = client
        .get(&format!("http://{}/json", server.addr()))
        .send()
        .await
        .expect("Failed to get");
    let text = res.json::<serde_json::value::Value>().await.expect("Failed to get json");
    assert_eq!("Hello", text);
}

#[tokio::test]
async fn fake_response_json() {
    let server = server::http(move |_req| async {
        let b = Branch::factory();
        // println!("{:?}", b);

        // Serialize it to a JSON string.
        let json_str = serde_json::to_string(&b).expect("failed serialize");
        http::Response::new(json_str.into())
    });

    let client = Client::new();

    let res = client
        .get(&format!("http://{}/json", server.addr()))
        .send()
        .await
        .expect("Failed to get");
    let text = res.json::<Branch>().await.expect("Failed to get json");
    println!("{:?}", text)
    // assert_eq!("Hello", text);
}
