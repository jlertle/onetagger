use reqwest::blocking::Client;
use reqwest::StatusCode;
use scraper::{Html, Selector, ElementRef};
use chrono::NaiveDate;
use regex::Regex;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

use crate::tagger::{Track, MusicPlatform, TrackMatcher, AudioFileInfo, TaggerConfig, MatchingUtils, parse_duration};

pub struct JunoDownload {
    client: Client
}

impl JunoDownload {
    // New instance
    pub fn new() -> JunoDownload {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:85.0) Gecko/20100101 Firefox/85.0")
            .build()
            .unwrap();

        JunoDownload {
            client
        }
    }

    // Search releases, generate tracks
    pub fn search(&self, query: &str) -> Result<Vec<Track>, Box<dyn Error>> {
        let response = self.client
            .get("https://www.junodownload.com/search/")
            .query(&[("q[all][]", query), ("solrorder", "relevancy"), ("items_per_page", "50")])
            .send()?;
        // Rate limitting
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            warn!("JunoDownload rate limit! Sleeping for 2s!");
            sleep(Duration::from_secs(2));
            return self.search(query);
        }

        // Minify and parse
        let mut data = response.text()?;
        minify_html::in_place_str(&mut data, &minify_html::Cfg {minify_js: false, minify_css: false})
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{:?}", e)))?;
        let document = Html::parse_document(&data);

        let mut out = vec![];
        let release_selector = Selector::parse("div.jd-listing-item").unwrap();
        for (index, release_element) in document.select(&release_selector).enumerate() {
            // Release
            if let Some(tracks) = self.parse_release(&release_element) {
                out.extend(tracks);
            } else {
                // Garbage elements at end of page
                if index < 50 {
                    warn!("Error parsing JunoDownload release! Index: {}, Query: {}", index, query);
                }
            }
        }

        Ok(out)
    }

    // Parse data from release element
    fn parse_release(&self, elem: &ElementRef) -> Option<Vec<Track>> {
        let mut out = vec![];
        // Artists
        let mut selector = Selector::parse("div.juno-artist").unwrap();
        let artist_element = elem.select(&selector).next()?;
        let artists = artist_element.text().filter(|a| a != &"/").collect::<Vec<_>>();
        // Release title
        selector = Selector::parse("a.juno-title").unwrap();
        let title_elem = elem.select(&selector).next()?;
        let title = title_elem.text().collect::<Vec<_>>().join(" ");
        let url = title_elem.value().attr("href")?;
        let mut release_id = url.split("/").collect::<Vec<&str>>();
        release_id.pop();
        let release_id = release_id.last().unwrap().to_string();
        // Label
        selector = Selector::parse("a.juno-label").unwrap();
        let label = elem.select(&selector).next()?.text().collect::<Vec<_>>().join(" ");
        // Info text
        selector = Selector::parse("div.col.text-right div.text-sm").unwrap();
        let mut info_text = elem.select(&selector).next()?.text().collect::<Vec<_>>();
        // Date, genres, catalog number
        let mut catalog_number = None;
        if info_text.len() == 3 {
            catalog_number = Some(info_text[0].to_string());
            info_text = info_text[1..].to_vec();
        }
        let release_date = NaiveDate::parse_from_str(info_text[0], "%d %b %y").ok()?;
        let genres: Vec<String> = info_text[1].split("/").map(|g| g.to_string()).collect();
        // Album art
        selector = Selector::parse("div.col img").unwrap();
        let image_elem = elem.select(&selector).next()?;
        let mut album_art_small = image_elem.value().attr("src")?;
        // Placeholder image
        if album_art_small.starts_with("data:image/") {
            album_art_small = image_elem.value().attr("data-src")?;
        }
        // Full resolution img
        let album_art = format!("https://imagescdn.junodownload.com/full/{}-BIG.jpg", 
            album_art_small.split("/").last().unwrap().replace(".jpg", ""));

        // Tracks
        let track_selector = Selector::parse("div.jd-listing-tracklist div.col").unwrap();
        for track_elem in elem.select(&track_selector) {
            let text = track_elem.text().collect::<Vec<_>>();
            let full = text[0].replace("\u{a0}", " ");
            // Duration
            let re = Regex::new(r" - \((\d+:\d\d)\) ?$").unwrap();
            let duration = if let Some(captures) = re.captures(&full) {
                if let Some(m) = captures.get(1) {
                    parse_duration(m.as_str()).unwrap_or(Duration::ZERO)
                } else { Duration::ZERO }
            } else { Duration::ZERO };
            //  Remove duration
            let no_duration = re.replace(&full, "");
            // Check if title or artist - title
            let split: Vec<&str> = no_duration.split(" - \"").collect();
            let mut track_artists = vec![];
            // Only title
            let track_title = if split.len() == 1 {
                split[0].to_string()
            } else {
                // Artists - "Title"
                track_artists = split[0].split(" & ").collect();
                split[1].replace("\"", "")
            };
            // BPM
            let bpm: Option<i64> = if text.len() >= 2 && text[1].contains("BPM") {
                Some(text[1].replace("\u{a0}BPM", "").parse::<i64>().ok()?)
            } else {
                None
            };
            // Get artists for track
            if track_artists.len() == 0 {
                track_artists = artists.clone();
            }
            // Generate track
            out.push(Track {
                platform: MusicPlatform::JunoDownload,
                title: track_title,
                version: None,
                artists: track_artists.into_iter().map(|a| a.to_string()).collect(),
                album: Some(title.to_owned()),
                bpm,
                genres: genres.to_owned(),
                key: None,
                label: Some(label.to_string()),
                styles: vec![],
                publish_date: None,
                publish_year: None,
                release_year: None,
                release_date: Some(release_date),
                art: Some(album_art.to_string()),
                url: format!("https://www.junodownload.com{}", url),
                catalog_number: catalog_number.clone(),
                other: vec![],
                // Only release id
                track_id: None,
                release_id: release_id.clone(),
                duration
            });
        }

        Some(out)
    }
}

impl TrackMatcher for JunoDownload {
    fn match_track(&self, info: &AudioFileInfo, config: &TaggerConfig) -> Result<Option<(f64, Track)>, Box<dyn Error>> {
        // Search
        let query = format!("{} {}", info.artist()?, MatchingUtils::clean_title(info.title()?));
        let tracks = self.search(&query)?;
        // Match
        if let Some((acc, track)) = MatchingUtils::match_track(&info, &tracks, &config) {
            return Ok(Some((acc, track)));
        }
        Ok(None)
    }
}