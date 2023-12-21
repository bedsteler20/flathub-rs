use std::collections::HashMap;

use crate::app_hit::AppHit;
use crate::appstream::Appstream;
use crate::home_page::HomePage;
use crate::pagination::Pagination;

fn get_base_url() -> String {
    format!("https://flathub.org/api/v2")
}

pub async fn list_catagories() -> Result<Vec<String>, reqwest::Error> {
    reqwest::get(format!("{}/categories", get_base_url()))
        .await?
        .json()
        .await
}

pub async fn list_developers() -> Result<Vec<String>, reqwest::Error> {
    reqwest::get(format!("{}/developer", get_base_url()))
        .await?
        .json()
        .await
}

pub async fn list_project_groups() -> Result<Vec<String>, reqwest::Error> {
    reqwest::get(format!("{}/projectgroup", get_base_url()))
        .await?
        .json()
        .await
}

pub async fn list_all_app_ids() -> Result<Vec<String>, reqwest::Error> {
    reqwest::get(format!("{}/appstream", get_base_url()))
        .await?
        .json()
        .await
}

pub async fn status() -> bool {
    let resp = reqwest::get(format!("{}/status", get_base_url())).await;
    match resp {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn runtime_usage() -> Result<HashMap<String, i32>, reqwest::Error> {
    reqwest::get(format!("{}/runtimes", get_base_url()))
        .await?
        .json()
        .await
}

pub async fn category(
    category: &str,
    page: i32,
    per_page: i32,
) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/category/{category}?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn developer(
    developer: &str,
    page: i32,
    per_page: i32,
) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/developer/{developer}?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn subcategory(
    category: &str,
    subcategory: &str,
    page: i32,
    per_page: i32,
) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/category/{category}/subcategory/{subcategory}?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn project_group(
    project_group: &str,
    page: i32,
    per_page: i32,
) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/projectgroup/{project_group}?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn recently_updated_apps(
    page: i32,
    per_page: i32,
) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/collection/recently-updated?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn recently_added_apps(
    page: i32,
    per_page: i32,
) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/collection/recently-added?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn verified_apps(page: i32, per_page: i32) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/collection/verified?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn popular_apps(page: i32, per_page: i32) -> Result<Pagination<AppHit>, reqwest::Error> {
    reqwest::get(format!(
        "{}/popular/last-month?page={page}&per_page={per_page}",
        get_base_url()
    ))
    .await?
    .json()
    .await
}

pub async fn appstream(app_id: &str) -> Result<Appstream, reqwest::Error> {
    reqwest::get(format!("{}/appstream/{}", get_base_url(), app_id))
        .await?
        .json()
        .await
}

pub async fn home_page(items: i32) -> Result<HomePage, reqwest::Error> {
    let (popular_apps, new_apps, updated_apps, verified_apps) = tokio::join!(
        popular_apps(1, items),
        recently_added_apps(1, items),
        recently_updated_apps(1, items),
        verified_apps(1, items)
    );
    Ok(HomePage {
        popular_apps: popular_apps?.hits,
        new_apps: new_apps?.hits,
        updated_apps: updated_apps?.hits,
        verified_apps: verified_apps?.hits,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_catagories() {
        
        let catagories = list_catagories().await.unwrap();
        assert!(catagories.len() > 0);
    }

    #[tokio::test]
    async fn test_list_developers() {
        
        let developers = list_developers().await.unwrap();
        assert!(developers.len() > 0);
    }

    #[tokio::test]
    async fn test_list_project_groups() {
        
        let project_groups = list_project_groups().await.unwrap();
        assert!(project_groups.len() > 0);
    }

    #[tokio::test]
    async fn test_list_all_app_ids() {
        
        let app_ids = list_all_app_ids().await.unwrap();
        assert!(app_ids.len() > 0);
    }

    #[tokio::test]
    async fn test_status() {
        
        let status = status().await;
        assert!(status);
    }

    #[tokio::test]
    async fn test_runtime_usage() {
        
        let runtime_usage = runtime_usage().await.unwrap();
        assert!(runtime_usage.len() > 0);
    }

    #[tokio::test]
    async fn test_category() {
        
        let category = category("audiovideo", 1, 100).await.unwrap();
        assert!(category.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_developer() {
        
        let devs = list_developers().await.unwrap();
        for dev in devs.iter() {
            let developer = developer(dev, 1, 100).await.unwrap();
            if developer.hits.len() > 0 {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }

    #[tokio::test]
    async fn test_project_group() {
        
        let project_groups = list_project_groups().await.unwrap();
        for pj in project_groups.iter() {
            let project_group = project_group(pj, 1, 100).await.unwrap();
            if project_group.hits.len() > 0 {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }

    #[tokio::test]
    async fn test_recently_updated_apps() {
        
        let recently_updated_apps = recently_updated_apps(1, 100).await.unwrap();
        assert!(recently_updated_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_recently_added_apps() {
        
        let recently_added_apps = recently_added_apps(1, 100).await.unwrap();
        assert!(recently_added_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_verified_apps() {
        
        let verified_apps = verified_apps(1, 100).await.unwrap();
        assert!(verified_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_popular_apps() {
        
        let popular_apps = popular_apps(1, 100).await.unwrap();
        assert!(popular_apps.hits.len() > 0);
    }

    #[tokio::test]
    async fn test_appstream() {
        
        appstream("com.google.Chrome").await.unwrap();
        appstream("com.visualstudio.code").await.unwrap();
        appstream("com.spotify.Client").await.unwrap();
        appstream("com.discordapp.Discord").await.unwrap();
        appstream("com.valvesoftware.Steam").await.unwrap();

        appstream("com.github.tchx84.Flatseal").await.unwrap();
        appstream("org.gnome.Geary").await.unwrap();
        appstream("org.gnome.Builder").await.unwrap();
        appstream("org.gnome.Weather").await.unwrap();
        appstream("org.gnome.Sdk").await.unwrap();

        appstream("com.valvesoftware.Steam.CompatibilityTool.Proton")
            .await
            .unwrap();
        appstream("org.freedesktop.Platform").await.unwrap();
    }

    // this test takes a long time to run so it is disabled by default
    // to enable it run cargo test -- --ignored --nocapture
    #[tokio::test]
    #[ignore]
    async fn test_all() {
        
        for app in list_all_app_ids().await.unwrap().iter() {
            println!("app: {}", app);
            match appstream(app).await {
                Ok(_) => {}
                Err(e) => {
                    println!("error: {}", e);
                    continue;
                }
            }
        }
    }
}
