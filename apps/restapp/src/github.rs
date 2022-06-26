use crate::errors::ServiceError;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use reqwest::header::USER_AGENT;

use fake::faker::boolean::en::Boolean;
use fake::faker::internet::raw::{DomainSuffix, Username};
use fake::faker::lorem::raw::Word;
use fake::locales::EN;
use fake::Fake;

pub const GITHUB_URL: &str = "https://api.github.com";

#[cfg(not(test))]
pub const BRANCHES_URL: &str = "https://api.github.com/repos/magocod/ts_express/branches";
#[cfg(test)]
pub const BRANCHES_URL: &str = "http://localhost/github/branches";

#[cfg(not(test))]
pub const MAIN_BRANCH_URL: &str = "https://api.github.com/repos/magocod/ts_express/commits/7dd64cdd398c1ad5fbbdfbb58ef217898a81b9ae";
#[cfg(test)]
pub const MAIN_BRANCH_URL: &str = "http://localhost/github/repos/commits";

pub enum GitHubApi {
    Branches,
    MainBranch,
    Unknown(String),
    Put(String),     // practice
    Replace(String), // practice
}

// deprecated
#[derive(Debug)]
pub struct UrlProvider {
    base: String,
    urls: GitHubUrl,
}

// #[deprecated(since="0.1.1")]
#[derive(Debug)]
pub struct GitHubUrl {
    pub branches: String,
    pub main_branch: String,
}

impl Default for UrlProvider {
    fn default() -> UrlProvider {
        UrlProvider {
            base: GITHUB_URL.to_string(),
            urls: GitHubUrl {
                branches: "/repos/magocod/ts_express/branches".to_string(),
                main_branch:
                    "/repos/magocod/ts_express/commits/7dd64cdd398c1ad5fbbdfbb58ef217898a81b9ae"
                        .to_string(),
            },
        }
    }
}

impl UrlProvider {
    pub fn new(base: String, urls: GitHubUrl) -> Self {
        Self { base, urls }
    }

    pub fn base(&self) -> String {
        self.base.clone()
    }

    pub fn get_url(&self, path: GitHubApi) -> String {
        let mut b = self.base.clone();
        match path {
            GitHubApi::Branches => {
                b.push_str(self.urls.branches.as_str());
                b
            }
            GitHubApi::MainBranch => {
                b.push_str(self.urls.main_branch.as_str());
                b
            }
            GitHubApi::Unknown(value) => {
                b.push_str(value.as_str());
                b
            }
            GitHubApi::Put(value) => {
                b.push_str(format!("/base/{}/other_path", value).as_str());
                b
            }
            GitHubApi::Replace(value) => {
                b.push_str(
                    format!("/base/other_path/REPLACE")
                        .replace("REPLACE", value.as_str())
                        .as_str(),
                );
                b
            }
        }
    }

    pub fn summary(self) {
        println!("{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
    pub protected: bool,
}

impl Branch {
    /// generate random data
    pub fn factory() -> Self {
        Self {
            name: Username(EN).fake(),
            commit: Commit {
                sha: Word(EN).fake(),
                url: DomainSuffix(EN).fake(),
            },
            protected: Boolean(2).fake(),
        }
    }
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

pub struct GitHubError;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubError404 {
    message: String,
    documentation_url: String,
}

impl GitHubError {
    pub fn not_found() -> GitHubError404 {
        GitHubError404 {
            message: "Not Found".to_string(),
            documentation_url: "https://docs.github.com/rest".to_string(),
        }
    }
}

pub async fn http_request() -> Result<HttpResponse, ServiceError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(BRANCHES_URL)
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<Vec<Branch>>()
        .await?;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn reuse_http_request(
    http_client: web::Data<reqwest::Client>,
) -> Result<HttpResponse, ServiceError> {
    let resp = http_client
        .get(BRANCHES_URL)
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<Vec<Branch>>()
        .await?;
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn provider_http_request(
    http_client: web::Data<reqwest::Client>,
    github_url: web::Data<UrlProvider>,
) -> Result<HttpResponse, ServiceError> {
    // println!("{:#?}", github_url);
    let resp_str = http_client
        .get(github_url.get_url(GitHubApi::Branches))
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?;

    // println!("{:#?}", resp_str.status());
    // println!("{:#?}", resp_str.text().await?);

    let resp = http_client
        .get(github_url.get_url(GitHubApi::Branches))
        .header(USER_AGENT, "reqwest-test-agent")
        .send()
        .await?
        .json::<Vec<Branch>>()
        .await?;
    // println!("{:#?}", resp);
    Ok(HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branches_url() {
        // println!("branch {}", BRANCHES_URL);
        assert_eq!(BRANCHES_URL, "http://localhost/github/branches");
    }

    #[test]
    fn test_main_branch_url() {
        // println!("main branch {}", MAIN_BRANCH_URL);
        assert_eq!(MAIN_BRANCH_URL, "http://localhost/github/repos/commits");
    }
}
