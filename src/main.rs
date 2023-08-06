use clap::{Parser, ValueEnum};
use languages::Language;

use crate::languages::SPEAKERS;

pub mod languages;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Language program need to request TTS for
    language: Language,

    /// Text to request TTS for
    text: String,

    /// Which gender should be downloaded
    #[arg(long)]
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

    // Fetching arguments and options from CLI
    let Cli {
        language,
        text,
        gender,
    } = cli;

    // If gender wasn't set, set to default (both)
    let gender = gender.unwrap_or_default();

    // Fetch speakers
    let speakers = SPEAKERS.get(language.into()).unwrap();
    let male_speakers = [speakers[0]];
    let female_speakers = [speakers[1]];

    // Choosing required speakers
    let requested_speakers = match gender {
        Gender::Both => &speakers[..],
        Gender::Male => &male_speakers,
        Gender::Female => &female_speakers,
    };

    dbg!(text);
    // Requesting TTS for each speaker
    for speaker in requested_speakers {
        dbg!(speaker);
    }
}
