use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct FavouriteRepositories<T: GitHubClient + Sync> {
    pub username: String,
    pub repos: Vec<Repository>,
    pub client: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub stars: u32,
    pub username: String,
}

#[async_trait]
pub trait GitHubClient {
    async fn fetch_repos(&self, username: &str) -> Vec<Repository>;
}

pub struct RealGitHubClient;

#[async_trait]
impl GitHubClient for RealGitHubClient {
    async fn fetch_repos(&self, _username: &str) -> Vec<Repository> {
        vec![
            Repository {
                name: "repo1".to_string(),
                url: "https://github.com/user/repo1".to_string(),
                description: "A sample repository".to_string(),
                stars: 42,
                username: _username.to_string(),
            },
            Repository {
                name: "repo2".to_string(),
                url: "https://github.com/user/repo2".to_string(),
                description: "Another sample repository".to_string(),
                stars: 100,
                username: _username.to_string(),
            },
        ]
    }
}
