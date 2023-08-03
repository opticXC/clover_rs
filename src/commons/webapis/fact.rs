use serde::{Serialize, Deserialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct Fact {
    pub id: String,
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub language: String,
    pub permalink: String,
}





pub async fn get_fact() -> Fact {
    let res = reqwest::get("https://uselessfacts.jsph.pl/api/v2/facts/random?language=en")
        .await
        .unwrap();

    res.json::<Fact>().await.unwrap()
}



