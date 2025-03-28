use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

pub struct CachedGitHubClient<T: GitHubClient + Sync + Send> {
    client: T,
    cache: Arc<DashMap<String, Vec<Repository>>>,
}

impl<T: GitHubClient + Sync + Send> CachedGitHubClient<T> {
    pub fn new(client: T) -> Self {
        Self {
            client,
            cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn fetch_repos_with_cache(
        &self,
        username: &str,
        bypass_cache: bool,
    ) -> Vec<Repository> {
        if !bypass_cache {
            if let Some(cached_repos) = self.cache.get(username) {
                return cached_repos.clone();
            }
        }

        let repos = self.client.fetch_repos(username).await;
        self.cache.insert(username.to_string(), repos.clone());
        repos
    }
}

#[async_trait]
impl<T: GitHubClient + Sync + Send> GitHubClient for CachedGitHubClient<T> {
    async fn fetch_repos(&self, username: &str) -> Vec<Repository> {
        self.fetch_repos_with_cache(username, false).await
    }
}
