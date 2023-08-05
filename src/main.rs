use clap::{Parser, ValueEnum};
use languages::Language;

pub mod languages;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language program need to request TTS for
    language: Language,

    /// Which gender should be downloaded
    gender: Option<Gender>,
}

#[derive(Clone, Copy, Default, Debug, ValueEnum)]
enum Gender {
    /// Download both male and female voice (default)
    #[default]
    Both,

    /// Download only male voice
    Male,

    /// Download only female voice
    Female,
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);
}
