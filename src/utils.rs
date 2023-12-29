use once_cell::sync::OnceCell;
use serde::de;

pub(crate) static CLIENT: OnceCell<&reqwest::Client> = OnceCell::new();

pub fn get_client() -> &'static reqwest::Client {
    if let Some(client) = CLIENT.get() {
        client
    } else  {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::USER_AGENT,
                    reqwest::header::HeaderValue::from_static("flathub-rs"),
                );
                headers.insert(
                    reqwest::header::ACCEPT, 
                    reqwest::header::HeaderValue::from_static("application/json")
                );
                headers
            })
            .build()
            .unwrap();
        configure_client(Box::leak(Box::new(client)));
        CLIENT.get().unwrap()
    }
}

pub fn configure_client(client: &'static reqwest::Client) {
    if let Err(e) = CLIENT.set(client) {
        println!("flathub-rs: failed to configure client");
    }
}

pub(crate) fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: Result<&str, D::Error> = de::Deserialize::deserialize(deserializer);
    if s.is_err() {
        return Ok(false);
    }
    let s = s.unwrap();
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["true", "false"])),
    }
}
