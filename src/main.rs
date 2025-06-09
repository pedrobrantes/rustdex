use clap::Parser;
use serde::Deserialize;
use std::io::{Cursor, Write};
use image::{DynamicImage, GenericImageView};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of Pokemon will be searched
    #[arg(short, long)]
    pokemon: String,
    
    /// Define the width of pokemon image
    #[arg(long, short = 'w', default_value_t = 100)]
    width: u32,
}

#[derive(Deserialize, Debug)]
struct Sprites {
    front_default: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Pokemon {
    name: String,
    types: Vec<TypeEntry>,
    abilities: Vec<AbilityEntry>,
    sprites: Sprites,
}

#[derive(Deserialize, Debug)]
struct TypeEntry {
    #[serde(rename = "type")]
    type_info: Type,
}

#[derive(Deserialize, Debug)]
struct Type {
    name: String,
}

#[derive(Deserialize, Debug)]
struct AbilityEntry {
    ability: Ability,
}

#[derive(Deserialize, Debug)]
struct Ability {
    name: String,
}

fn is_transparent(img: &DynamicImage, start: u32, is_row: bool) -> bool {
    if is_row {
        (0..img.width()).all(|x| img.get_pixel(x, start).0[3] == 0)
    } else {
        (0..img.height()).all(|y| img.get_pixel(start, y).0[3] == 0)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("Failed to clear terminal");
    let args = Args::parse();
    let pokemon_name = args.pokemon.to_lowercase();

    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_name);
    std::io::stdout().flush()?;

    print!("Searching info for: {}...", pokemon_name);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let pokemon_data: Pokemon = response.json().await?;

        if let Some(sprite_url) = pokemon_data.sprites.front_default {
            let image_bytes = reqwest::get(&sprite_url).await?.bytes().await?;
            let img = image::load(Cursor::new(&image_bytes), image::ImageFormat::Png)?;
            
            let (mut top, mut bottom) = (0, img.height() - 1);
            let (mut left, mut right) = (0, img.width() - 1);

            while top < bottom && is_transparent(&img, top, true) {
                top += 1;
            }
            while bottom > top && is_transparent(&img, bottom, true) {
                bottom -= 1;
            }
            while left < right && is_transparent(&img, left, false) {
                left += 1;
            }
            while right > left && is_transparent(&img, right, false) {
                right -= 1;
            }

            let trimmed_image = img.crop_imm(left, top, right - left + 1, bottom - top + 1);

            let config = viuer::Config {
                transparent: true,
                width: Some(args.width),
                ..Default::default()
            };
            
            viuer::print(&trimmed_image, &config).expect("The image cannot be displayed in the terminal.");
        }

        println!("\n--- Pokémon information ---");
        println!("Name: {}", pokemon_data.name.to_uppercase());

        let types_str: String = pokemon_data.types
            .iter()
            .map(|t| t.type_info.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Type(s): {}", types_str);

        let abilities_str: String = pokemon_data.abilities
            .iter()
            .map(|a| a.ability.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Ability: {}", abilities_str);

    } else {
        eprintln!("Err: Pokémon '{}' not found. verify the name and try again.", pokemon_name);
    }

    Ok(())
}
