use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct FavouriteRepositories<T: GitHubClient + Sync> {
    pub client: T,
}

impl<T: GitHubClient + Sync> FavouriteRepositories<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }

    pub async fn get_top_repos(
        &self,
        username: &str,
        top_n: usize,
    ) -> Result<Vec<Repository>, anyhow::Error> {
        let mut repos = self.client.fetch_repos(username).await?;
        repos.sort_by(|a, b| b.stars.cmp(&a.stars));
        Ok(repos.into_iter().take(top_n).collect())
    }
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
    async fn fetch_repos(&self, username: &str) -> Result<Vec<Repository>, anyhow::Error>;
}

pub struct RealGitHubClient {
    client: reqwest::Client,
}

impl RealGitHubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("GitHub Client")
                .build()
                .unwrap(),
        }
    }
}
#[async_trait]
impl GitHubClient for RealGitHubClient {
    async fn fetch_repos(&self, username: &str) -> Result<Vec<Repository>, anyhow::Error> {
        let url = format!("https://api.github.com/users/{}/repos", username);

        let response = self.client.get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                status,
                error_text
            ));
        }

        let repos: Vec<serde_json::Value> = response.json().await?;

        Ok(repos
            .into_iter()
            .map(|repo| Repository {
                name: repo["name"].as_str().unwrap_or_default().to_string(),
                url: repo["html_url"].as_str().unwrap_or_default().to_string(),
                description: repo["description"]
                    .as_str()
                    .unwrap_or("No description")
                    .to_string(),
                stars: repo["stargazers_count"].as_u64().unwrap_or(0) as u32,
                username: username.to_string(),
            })
            .collect())
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
    ) -> Result<Vec<Repository>, anyhow::Error> {
        if !bypass_cache {
            if let Some(cached_repos) = self.cache.get(username) {
                return Ok(cached_repos.clone());
            }
        }

        let repos = self.client.fetch_repos(username).await;
        if let Ok(ref repos_data) = repos {
            self.cache.insert(username.to_string(), repos_data.clone());
        }
        repos
    }
}

#[async_trait]
impl<T: GitHubClient + Sync + Send> GitHubClient for CachedGitHubClient<T> {
    async fn fetch_repos(&self, username: &str) -> Result<Vec<Repository>, anyhow::Error> {
        Ok(self.fetch_repos_with_cache(username, false).await?)
    }
}
