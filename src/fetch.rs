use std::{fs::File, io::Cursor, path::PathBuf};

use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct MakeMp3Request<'a> {
    msg: &'a str,
    lang: &'a str,
    source: &'a str,
}

impl<'a> MakeMp3Request<'a> {
    fn new(msg: &'a str, lang: &'a str) -> Self {
        Self {
            msg,
            lang,
            source: "ttsmp3",
        }
    }
}

pub fn download_tts(speaker: &str, text: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Creating client with user agent
    const USER_AGENT: &str =
        "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0";
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    // Requesting MP3
    const REQUEST_TTS_URL: &str = "https://ttsmp3.com/makemp3_new.php";
    let tts_request = MakeMp3Request::new(text, speaker);
    let tts_response = client
        .post(REQUEST_TTS_URL)
        .header("Origin", "https://ttsmp3.com")
        .header("Referer", "https://ttsmp3.com/")
        .form(&tts_request)
        .send()?;

    // Fetching URL of MP3
    let tts_response: serde_json::Value = tts_response.json()?;
    let tts_url = tts_response.get("URL").unwrap().as_str().unwrap();

    // Downloading MP3
    let tts_audio = client.get(tts_url).send()?;

    // Writing MP3 to file
    let filename = PathBuf::from(format!("{text}__{speaker}.mp3"));
    let mut file = File::create(filename.clone())?;
    let mut content = Cursor::new(tts_audio.bytes()?);
    std::io::copy(&mut content, &mut file)?;

    Ok(filename)
}
