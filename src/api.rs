use crate::models::Pokemon;
use bytes::Bytes;

const API_BASE_URL: &str = "https://pokeapi.co/api/v2";

pub async fn fetch_pokemon(name: &str) -> Result<Pokemon, reqwest::Error> {
    let url = format!("{}/pokemon/{}", API_BASE_URL, name);
    let response = reqwest::get(&url).await?;
    let pokemon_data = response.json::<Pokemon>().await?;
    Ok(pokemon_data)
}

pub async fn fetch_image(url: &str) -> Result<Bytes, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let image_bytes = response.bytes().await?;
    Ok(image_bytes)
}
