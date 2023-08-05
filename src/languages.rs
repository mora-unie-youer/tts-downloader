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
