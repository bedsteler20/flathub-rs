use serde::de;


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
