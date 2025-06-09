use crate::models::Pokemon;
use bytes::Bytes;
use image::{DynamicImage, GenericImageView};
use std::io::Cursor;

fn is_transparent(img: &DynamicImage, start: u32, is_row: bool) -> bool {
    if is_row {
        (0..img.width()).all(|x| img.get_pixel(x, start).0[3] == 0)
    } else {
        (0..img.height()).all(|y| img.get_pixel(start, y).0[3] == 0)
    }
}

pub fn display_pokemon(
    pokemon_data: &Pokemon,
    image_bytes: &Bytes,
    width: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)?;

    let (mut top, mut bottom) = (0, img.height() - 1);
    let (mut left, mut right) = (0, img.width() - 1);

    while top < bottom && is_transparent(&img, top, true) { top += 1; }
    while bottom > top && is_transparent(&img, bottom, true) { bottom -= 1; }
    while left < right && is_transparent(&img, left, false) { left += 1; }
    while right > left && is_transparent(&img, right, false) { right -= 1; }

    let trimmed_image = img.crop_imm(left, top, right - left + 1, bottom - top + 1);

    let config = viuer::Config {
        transparent: true,
        width: Some(width),
        ..Default::default()
    };
    
    viuer::print(&trimmed_image, &config)?;

    println!("\n--- Pokémon Information ---");
    println!("Name: {}", pokemon_data.name.to_uppercase());

    let types_str: String = pokemon_data
        .types
        .iter()
        .map(|t| t.type_info.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    println!("Type(s): {}", types_str);

    let abilities_str: String = pokemon_data
        .abilities
        .iter()
        .map(|a| a.ability.name.clone())
        .collect::<Vec<String>>()
        .join(", ");
    println!("Abilities: {}", abilities_str);

    Ok(())
}
