use once_cell::sync::OnceCell;
use serde::de;

pub(crate) static CLIENT: OnceCell<&reqwest::Client> = OnceCell::new();

pub fn get_client() -> &'static reqwest::Client {
    CLIENT.get().expect("flathub-rs: client not configured")
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
