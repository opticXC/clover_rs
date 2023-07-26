#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Fact {
    pub id: String,
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub language: String,
    pub permalink: String,
}
