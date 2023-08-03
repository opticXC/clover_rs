pub mod fact;
pub mod dictionary;




pub use fact::get_fact;





const BASE_SHIBE_API_URL: &str = "http://shibe.online/api/";

pub async fn get_animal_image(image_type: &String, count: i8) -> AnimalImageResult {
    let fetch_url = format!("{}{}?count={}", BASE_SHIBE_API_URL, image_type, count);
    log::debug!("Fetching url - {} ", fetch_url);

    let res = reqwest::get(fetch_url).await.unwrap();
    res.json::<AnimalImageResult>().await.unwrap()
}

type AnimalImageResult = Vec<String>;
