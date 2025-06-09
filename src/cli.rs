use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The name of the Pokémon to search for
    #[arg(short, long)]
    pub pokemon: String,

    /// Sets the width of the image in characters
    #[arg(long, short = 'w', default_value_t = 100)]
    pub width: u32,
}
