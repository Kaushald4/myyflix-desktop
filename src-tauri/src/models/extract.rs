use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExtractRequest {
    pub id: String,

    #[serde(rename = "type")]
    pub content_type: String,

    pub season: Option<u32>,
    pub episode: Option<u32>,
}
