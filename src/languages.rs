use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Language {
    /// Download TTS for Chinese language (alias: zh)
    #[value(alias("zh"))]
    Chinese,

    /// Download TTS for English language (alias: en)
    #[value(alias("en"))]
    English,

    /// Download TTS for Japanese language (alias: ja)
    #[value(alias("ja"))]
    Japanese,

    /// Download TTS for Korean language (alias: ko)
    #[value(alias("ko"))]
    Korean,
}

impl From<Language> for &'static str {
    fn from(value: Language) -> Self {
        match value {
            Language::Chinese => "chinese",
            Language::English => "english",
            Language::Japanese => "japanese",
            Language::Korean => "korean",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Speaker {
    /// Defines male speaker
    Male(&'static str),

    /// Defines female speaker
    Female(&'static str),
}

impl Speaker {
    // Returns name of speaker
    pub fn name(&self) -> &'static str {
        match self {
            Self::Male(name) => name,
            Self::Female(name) => name,
        }
    }
}

/// HashMap of "language -> speakers" on tts.mp3
pub const SPEAKERS: phf::Map<&str, &[Speaker]> = phf::phf_map! {
    "chinese" => &[Speaker::Female("Zhiyu")],
    "english" => &[Speaker::Female("Ivy"), Speaker::Female("Justin"), Speaker::Female("Joanna"), Speaker::Female("Kendra"),
                Speaker::Female("Kimberly"), Speaker::Female("Salli"), Speaker::Male("Joey"), Speaker::Male("Mattew")],
    "japanese" => &[Speaker::Male("Takumi"), Speaker::Female("Mizuki")],
    "korean" => &[Speaker::Female("Seoyeon")],
};
