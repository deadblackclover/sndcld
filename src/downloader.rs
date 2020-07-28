use crate::config::Config;
use dirs;
use reqwest::blocking::Client;
use reqwest::header::{LOCATION, USER_AGENT};
use serde_json::{Error, Value};
use std::fs::File;
use std::io;
use std::io::Read;

/// Soundcloud API
const API: &str = "https://api.soundcloud.com/";

/// Download a song
pub fn song(config: &Config, url: String) -> Result<(), Error> {
    let resolve = format!(
        "{}resolve.json?url={}&client_id={}&format=json&_status_code_map[302]=200",
        API, url, config.token
    );

    let (location, _) = get(String::from(resolve));

    let (_, body) = get(location);
    let json: Value = serde_json::from_str(&body)?;

    let id = &json["id"];
    let mut name_mp3 = format!("{} - {}.mp3", &json["user"]["username"], &json["title"]);
    let url_track = format!("{}i1/tracks/{}/streams?client_id={}", API, id, config.token);

    let (_, body) = get(url_track);
    let json: Value = serde_json::from_str(&body)?;

    let mp3 = json["http_mp3_128_url"]
        .as_str()
        .expect("MP3 URL not found");

    name_mp3 = name_mp3
        .replace("/", "_")
        .replace(" ", "_")
        .replace("\"", "");

    println!("{}", name_mp3);

    match download(String::from(mp3), name_mp3) {
        Ok(_) => println!("Successful download!"),
        Err(_) => println!("File download error"),
    };

    Ok(())
}

/// Download playlist
pub fn playlist(config: &Config, url: String) -> Result<(), Error> {
    let resolve = format!(
        "{}resolve.json?url={}&client_id={}&format=json&_status_code_map[302]=200",
        API, url, config.token
    );

    let (location, _) = get(String::from(resolve));

    let (_, body) = get(location);
    let json: Value = serde_json::from_str(&body)?;

    let tracks = &json["tracks"];

    if tracks.is_array() {
        for track in tracks.as_array().expect("Data conversion error") {
            let object = track.as_object().expect("Data conversion error");
            let permalink_url = object
                .get("permalink_url")
                .expect("Data conversion error")
                .as_str()
                .expect("Data conversion error");
            song(config, String::from(permalink_url)).expect("Failed to get song data");
        }
    }

    Ok(())
}

/// Function for get requests
fn get(url: String) -> (String, String) {
    let client = Client::new();
    let mut res = client
        .get(&url)
        .header(USER_AGENT, "curl/7.54.1")
        .send()
        .unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    if res.headers().contains_key(LOCATION) {
        let location = res.headers()[LOCATION].to_str().unwrap();
        (String::from(location), body)
    } else {
        (String::from(""), body)
    }
}

/// Function for downloading files
fn download(url: String, name: String) -> Result<(), String> {
    let audio_path = match dirs::audio_dir() {
        Some(path) => format!("{}/{}", path.display(), name),
        None => return Err(String::from("Home directory not found!")),
    };

    let mut resp = reqwest::blocking::get(&url).expect("Request failed");
    let mut out = File::create(audio_path).expect("Failed to create file");
    io::copy(&mut resp, &mut out).expect("Failed to copy content");

    Ok(())
}
