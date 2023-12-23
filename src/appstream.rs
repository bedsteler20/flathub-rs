#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Appstream {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: String,
    pub description: Option<String>,
    pub screenshots: Option<Vec<Screenshot>>,
    pub releases: Option<Vec<Release>>,
    pub urls: Option<Urls>,
    pub icon: Option<String>,
    pub name: String,
    pub summary: Option<String>,
    pub developer_name: Option<String>,
    pub categories: Option<Vec<String>>,
    pub kudos: Option<Vec<String>>,
    #[serde(rename = "mimetypes")]
    pub mime_types: Option<Vec<String>>,
    pub project_license: Option<String>,
    pub project_group: Option<String>,
    pub launchable: Option<Launchable>,
    pub bundle: Bundle,
    pub metadata: Option<Metadata>,
    pub keywords: Option<Vec<String>>,
    pub is_free_license: Option<bool>,
    pub extends: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScreenshotSizes {
    #[serde(rename = "624x351")]
    pub _624x351: Option<String>,
    #[serde(rename = "1248x702")]
    pub _1284x702: Option<String>,
    #[serde(rename = "112x63")]
    pub _112x63: Option<String>,
    #[serde(rename = "224x126")]
    pub _224x126: Option<String>,
    #[serde(rename = "752x423")]
    pub _752x423: Option<String>,
    #[serde(rename = "1504x846")]
    pub _1504x846: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Screenshot {
    pub default: Option<bool>,
    pub sizes: ScreenshotSizes,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Release {
    pub description: Option<String>,

    pub timestamp: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub urgency: Option<String>,
    pub version: String,
    pub url: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Launchable {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: String,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bundle {
    pub value: String,
    pub version: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub runtime: Option<String>,
    pub sdk: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    #[serde(rename = "flathub::manifest")]
    pub manifest: Option<String>,
    pub verified: Option<String>,
    #[serde(rename = "flathub::verification::method")]
    pub verification_method: Option<String>,
    #[serde(rename = "flathub::verification::login_provider")]
    pub verification_login_provider: Option<String>,
    #[serde(rename = "flathub::verification::login_name")]
    pub verification_login_name: Option<String>,
    #[serde(rename = "flathub::verification::website")]
    pub verification_website: Option<String>,
    #[serde(rename = "flathub::verification::timestamp")]
    pub verification_timestamp: Option<String>,
    #[serde(rename = "flathub::verification::login_is_organization")]
    pub verification_login_is_organization: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Urls {
    #[serde(rename = "bugtracker")]
    pub bug_tracker: Option<String>,
    pub contact: Option<String>,
    pub contribute: Option<String>,
    pub donation: Option<String>,
    pub faq: Option<String>,
    pub help: Option<String>,
    pub homepage: Option<String>,
    pub translate: Option<String>,
    pub vcs_browser: Option<String>,
}
