use serde::Deserialize;
use reqwest::blocking::Client;
use std::time::Duration;
use urlencoding::encode;

const CLIENT_ID: &str = "EnTrn2ZjaZXfOU7iRsFicZvTOi1Pl3rK";

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub id: u64,
    pub title: String,
    #[serde(rename = "permalink_url")]
    pub link: String,
    pub duration: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    collection: Vec<Track>,
}

pub struct SoundCloudClient {
    client: Client,
}

impl SoundCloudClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                AppleWebKit/537.36 (KHTML, like Gecko) \
                Chrome/120.0.0.0 Safari/537.36",
            )
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to create client");

        Self { client }
    }

    pub fn search_tracks(&self, query: &str, limit: u32) -> Result<Vec<Track>, String> {
        let eq = encode(query);

        let surl = format!(
            "https://api-v2.soundcloud.com/search/tracks?q={}&client_id={}&limit={}",
            eq, CLIENT_ID, limit
        );

        let res = self
            .client
            .get(&surl)
            .send()
            .map_err(|e| format!("err: {}", e))?;

        let search_result: SearchResponse = res.json()
            .map_err(|e| format!("JSON parse error: {}", e))?;

        //println!("Parsed: {:?}", search_result);

        Ok(search_result.collection)
    }
}
