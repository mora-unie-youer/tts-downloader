use clap::{Parser, ValueEnum};
use fetch::download_tts;
use languages::{Language, Speaker};

use crate::languages::SPEAKERS;

pub mod fetch;
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
    /// Download both male and female voices (default)
    #[default]
    Both,

    /// Download only male voices
    Male,

    /// Download only female voices
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

    // Choosing required speakers
    let requested_speakers: Vec<&str> = match gender {
        Gender::Both => speakers.iter().map(|speaker| speaker.name()).collect(),
        Gender::Male => speakers
            .iter()
            .filter(|speaker| matches!(speaker, Speaker::Male(_)))
            .map(|speaker| speaker.name())
            .collect(),
        Gender::Female => speakers
            .iter()
            .filter(|speaker| matches!(speaker, Speaker::Female(_)))
            .map(|speaker| speaker.name())
            .collect(),
    };

    // Requesting TTS for each speaker
    for speaker in requested_speakers {
        // Downloading TTS and receiving filename to which it was saved
        let filename = match download_tts(speaker, &text) {
            Ok(filename) => filename,
            Err(e) => {
                eprintln!("Error occured while TTS downloading: {e}");
                continue;
            }
        };

        // Printing filename, so the user could move files automatically
        println!("{}", filename.display());
    }
}
