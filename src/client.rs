use std::collections::HashMap;

use crate::app_hit::AppHit;
use crate::appstream::Appstream;
use crate::pagination::Pagination;

#[derive(Debug, Clone)]
pub struct Client {
    reqwest: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        Self {
            reqwest: reqwest::Client::new(),
            base_url: format!("https://flathub.org/api/v2"),
        }
    }

    pub async fn list_catagories(&self) -> Result<Vec<String>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!("{}/categories", self.base_url))
            .send()
            .await?;
        let catagories: Vec<String> = resp.json().await?;
        Ok(catagories)
    }

    pub async fn list_developers(&self) -> Result<Vec<String>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!("{}/developer", self.base_url))
            .send()
            .await?;
        let developers: Vec<String> = resp.json().await?;
        Ok(developers)
    }

    pub async fn list_project_groups(&self) -> Result<Vec<String>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!("{}/projectgroup", self.base_url))
            .send()
            .await?;
        let project_groups: Vec<String> = resp.json().await?;
        Ok(project_groups)
    }

    pub async fn list_all_app_ids(&self) -> Result<Vec<String>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!("{}/appstream", self.base_url))
            .send()
            .await?;
        let app_ids: Vec<String> = resp.json().await?;
        Ok(app_ids)
    }

    pub async fn status(&self) -> bool {
        let resp = self
            .reqwest
            .get(format!("{}/status", self.base_url))
            .send()
            .await;
        match resp {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn runtime_usage(&self) -> Result<HashMap<String, i32>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!("{}/runtimes", self.base_url))
            .send()
            .await?;
        let runtime_usage: HashMap<String, i32> = resp.json().await?;
        Ok(runtime_usage)
    }

    pub async fn category(
        &self,
        category: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/category/{category}?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn developer(
        &self,
        developer: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/developer/{developer}?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn subcategory(
        &self,
        category: &str,
        subcategory: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/category/{category}/subcategory/{subcategory}?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn project_group(
        &self,
        project_group: &str,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/projectgroup/{project_group}?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn recently_updated_apps(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/collection/recently-updated?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn recently_added_apps(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/collection/recently-added?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn verified_apps(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/collection/verified?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn popular_apps(
        &self,
        page: i32,
        per_page: i32,
    ) -> Result<Pagination<AppHit>, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!(
                "{}/popular/last-month?page={page}&per_page={per_page}",
                self.base_url
            ))
            .send()
            .await?;
        let pagination: Pagination<AppHit> = resp.json().await?;
        Ok(pagination)
    }

    pub async fn appstream(&self, app_id: &str) -> Result<Appstream, reqwest::Error> {
        let resp = self
            .reqwest
            .get(format!("{}/appstream/{}", self.base_url, app_id))
            .send()
            .await?;
        let appstream: Appstream = resp.json().await?;
        Ok(appstream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_catagories() {
        let client = Client::new();
        let catagories = client.list_catagories().await.unwrap();
        assert!(catagories.len() > 0);
    }

    #[tokio::test]
    async fn test_list_developers() {
        let client = Client::new();
        let developers = client.list_developers().await.unwrap();
        assert!(developers.len() > 0);
    }

    #[tokio::test]
    async fn test_list_project_groups() {
        let client = Client::new();
        let project_groups = client.list_project_groups().await.unwrap();
        assert!(project_groups.len() > 0);
    }

    #[tokio::test]
    async fn test_list_all_app_ids() {
        let client = Client::new();
        let app_ids = client.list_all_app_ids().await.unwrap();
        assert!(app_ids.len() > 0);
    }

    #[tokio::test]
    async fn test_status() {
        let client = Client::new();
        let status = client.status().await;
        assert!(status);
    }

    #[tokio::test]
    async fn test_runtime_usage() {
        let client = Client::new();
        let runtime_usage = client.runtime_usage().await.unwrap();
        assert!(runtime_usage.len() > 0);
    }

    #[tokio::test]
    async fn test_category() {
        let client = Client::new();
        let category = client.category("audiovideo", 1, 100).await.unwrap();
        assert!(category.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_developer() {
        let client = Client::new();
        let devs = client.list_developers().await.unwrap();
        for dev in devs.iter() {
            let developer = client.developer(dev, 1, 100).await.unwrap();
            if developer.hits.len() > 0 {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }

    #[tokio::test]
    async fn test_project_group() {
        let client = Client::new();
        let project_groups = client.list_project_groups().await.unwrap();
        for project_group in project_groups.iter() {
            let project_group = client.project_group(project_group, 1, 100).await.unwrap();
            if project_group.hits.len() > 0 {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }

    #[tokio::test]
    async fn test_recently_updated_apps() {
        let client = Client::new();
        let recently_updated_apps = client.recently_updated_apps(1, 100).await.unwrap();
        assert!(recently_updated_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_recently_added_apps() {
        let client = Client::new();
        let recently_added_apps = client.recently_added_apps(1, 100).await.unwrap();
        assert!(recently_added_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_verified_apps() {
        let client = Client::new();
        let verified_apps = client.verified_apps(1, 100).await.unwrap();
        assert!(verified_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_popular_apps() {
        let client = Client::new();
        let popular_apps = client.popular_apps(1, 100).await.unwrap();
        assert!(popular_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_appstream() {
        let client = Client::new();
        client.appstream("com.google.Chrome").await.unwrap();
        client.appstream("com.visualstudio.code").await.unwrap();
        client.appstream("com.spotify.Client").await.unwrap();
        client.appstream("com.discordapp.Discord").await.unwrap();
        client.appstream("com.valvesoftware.Steam").await.unwrap();
        client
            .appstream("com.github.tchx84.Flatseal")
            .await
            .unwrap();
        client.appstream("org.gnome.Geary").await.unwrap();
        client.appstream("org.gnome.Builder").await.unwrap();
        client.appstream("org.gnome.Weather").await.unwrap();
        client.appstream("org.gnome.Sdk").await.unwrap();
        client
            .appstream("com.valvesoftware.Steam.CompatibilityTool.Proton")
            .await
            .unwrap();
        client.appstream("org.freedesktop.Platform").await.unwrap();
    }

    // this test takes a long time to run so it is disabled by default
    // to enable it run cargo test -- --ignored --nocapture
    #[tokio::test]
    #[ignore]
    async fn  test_all() {
        let client = Client::new();
        for app in client.list_all_app_ids().await.unwrap().iter() {
            println!("app: {}", app);
            match client.appstream(app).await {
                Ok(_) => {}
                Err(e) => {
                    println!("error: {}", e);
                    continue;
                }
                
            }
        }
    }
}
