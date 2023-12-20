#[derive(serde::Deserialize, Debug)]
pub struct Pagination<T> {
    pub query: String,
    #[serde(rename = "processingTimeMs")]
    pub processing_time_ms: i32,
    #[serde(rename = "hitsPerPage")]
    pub hits_per_page: i32,
    pub page: i32,
    #[serde(rename = "totalPages")]
    pub total_pages: i32,
    #[serde(rename = "totalHits")]
    pub total_hits: i32,
    pub hits: Vec<T>,
}