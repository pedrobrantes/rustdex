mod api;
mod cli;
mod display;
mod error;
mod models;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("Failed to clear screen");

    let args = Args::parse();
    
    print!("Fetching information for: {}...", args.pokemon);

    // API logic
    let pokemon_data = api::fetch_pokemon(&args.pokemon).await?;

    if let Some(sprite_url) = pokemon_data.sprites.front_default.as_ref() {
        let image_bytes = api::fetch_image(sprite_url).await?;

        // Display logic
        display::display_pokemon(&pokemon_data, &image_bytes, args.width)?;
    } else {
        println!("\nNo sprite found for this Pokémon.");
    }

    Ok(())
}
