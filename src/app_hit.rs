use crate::utils::deserialize_bool;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct AppHit {
    pub name: String,
    pub summary: String,
    pub installs_last_month: Option<i32>,
    pub icon: String,
    pub keywords: Option<Vec<String>>,
    pub app_id: String,
    pub description: String,
    pub catagories: Option<Vec<String>>,
    pub developer_name: Option<String>,
    pub project_group: Option<String>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub verification_verified: bool,
    pub verification_method: Option<String>,
    pub verification_login_provider: Option<String>,
    pub verification_website: Option<String>,
    pub verification_timestamp: Option<String>,
    pub verification_login_is_organization: Option<String>,
}
