use github::FavouriteRepositories;
mod github;

fn main() {
    let favourite_repos =
        FavouriteRepositories::new(github::CachedGitHubClient::new(github::RealGitHubClient));

    let top_repos = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(favourite_repos.get_top_repos("getcarv", 10));
    for repo in top_repos {
        println!("Repo Name: {}", repo.name);
        println!("Repo URL: {}", repo.url);
        println!("Repo Description: {}", repo.description);
        println!("Stars: {}", repo.stars);
        println!("Username: {}", repo.username);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use github::GitHubClient;
    use github::Repository;

    struct MockGitHubClient;

    #[async_trait]
    impl GitHubClient for MockGitHubClient {
        async fn fetch_repos(&self, _username: &str) -> Vec<Repository> {
            vec![
                Repository {
                    name: "medium-star-repo".to_string(),
                    url: "https://github.com/test/medium-star-repo".to_string(),
                    description: "Medium star repo".to_string(),
                    stars: 5,
                    username: "test".to_string(),
                },
                Repository {
                    name: "high-star-repo".to_string(),
                    url: "https://github.com/test/high-star-repo".to_string(),
                    description: "High star repo".to_string(),
                    stars: 10,
                    username: "test".to_string(),
                },
                Repository {
                    name: "low-star-repo".to_string(),
                    url: "https://github.com/test/low-star-repo".to_string(),
                    description: "Low star repo".to_string(),
                    stars: 1,
                    username: "test".to_string(),
                },
            ]
        }
    }

    #[tokio::test]
    async fn test_repo_processing() {
        let client = MockGitHubClient;
        let favourite_repositories = FavouriteRepositories::new(client);

        let top_repos = favourite_repositories.get_top_repos("test_user", 3).await;

        assert_eq!(top_repos.len(), 3, "Should return exactly 3 repositories");

        let high_star_repo = &top_repos[0];
        assert_eq!(high_star_repo.name, "high-star-repo");
        assert_eq!(high_star_repo.url, "https://github.com/test/high-star-repo");
        assert_eq!(high_star_repo.description, "High star repo");
        assert_eq!(high_star_repo.stars, 10);

        let medium_star_repo = &top_repos[1];
        assert_eq!(medium_star_repo.name, "medium-star-repo");
        assert_eq!(
            medium_star_repo.url,
            "https://github.com/test/medium-star-repo"
        );
        assert_eq!(medium_star_repo.description, "Medium star repo");
        assert_eq!(medium_star_repo.stars, 5);

        let low_star_repo = &top_repos[2];
        assert_eq!(low_star_repo.name, "low-star-repo");
        assert_eq!(low_star_repo.url, "https://github.com/test/low-star-repo");
        assert_eq!(low_star_repo.description, "Low star repo");
        assert_eq!(low_star_repo.stars, 1);
    }
}
