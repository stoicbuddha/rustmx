use crate::storage::{retrieve_string, store_string};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use wasm_bindgen::prelude::*;

// Function to log to the browser console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    #[serde(rename = "Search")]
    pub search: Vec<MovieSearchResult>,
    #[serde(rename = "totalResults")]
    pub total_results: String,
    #[serde(rename = "Response")]
    pub response: String,
}

#[derive(Deserialize, Debug)]
pub struct MovieSearchResult {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub movie_type: String,
    #[serde(rename = "Poster")]
    pub poster: String,
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Rated")]
    pub rated: String,
    #[serde(rename = "Released")]
    pub released: String,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "Genre")]
    pub genre: String,
    #[serde(rename = "Director")]
    pub director: String,
    #[serde(rename = "Writer")]
    pub writer: String,
    #[serde(rename = "Actors")]
    pub actors: String,
    #[serde(rename = "Plot")]
    pub plot: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Awards")]
    pub awards: String,
    #[serde(rename = "Poster")]
    pub poster: String,
    #[serde(rename = "Ratings")]
    pub ratings: Vec<Rating>,
    #[serde(rename = "Metascore")]
    pub metascore: String,
    #[serde(rename = "imdbRating")]
    pub imdb_rating: String,
    #[serde(rename = "imdbVotes")]
    pub imdb_votes: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub movie_type: String,
    #[serde(rename = "DVD")]
    pub dvd: Option<String>,
    #[serde(rename = "BoxOffice")]
    pub box_office: Option<String>,
    #[serde(rename = "Production")]
    pub production: Option<String>,
    #[serde(rename = "Website")]
    pub website: Option<String>,
    #[serde(rename = "Response")]
    pub response: String,
}

#[derive(Deserialize, Debug)]
pub struct Rating {
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Value")]
    pub value: String,
}

pub struct OmdbApi {
    api_key: &'static str,
    base_url: &'static str,
}

impl OmdbApi {
    pub fn new() -> OmdbApi {
        log("Creating new OmdbApi");
        Self {
            api_key: "YOUR_KEY_HERE",
            base_url: "https://www.omdbapi.com/",
        }
    }

    fn full_path(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn search(&self, s: &str) -> Result<Vec<MovieSearchResult>, anyhow::Error> {
        let client = reqwest::Client::new();
        let query_params = format!("?s={}&apikey={}", s, self.api_key);
        let fp = &self.full_path(&query_params);
        log(fp);
        let response_builder = client.get(fp);
        let mut headers = HeaderMap::new();
        headers.append(ACCEPT, HeaderValue::from_str("application/json").unwrap());
        let response = response_builder.headers(headers).send().await;
        if response.is_err() {
            todo!();
        }
        let result = response.unwrap();
        if result.status() != 200 {
            todo!();
        }
        let result_text = result.text().await;
        if result_text.is_err() {
            todo!();
        }
        let result_text = result_text.unwrap();
        log(&format!("{:?}", &result_text));
        let parsed: SearchResponse = serde_json::from_str(&result_text)?;
        log(&format!("{:?}", &parsed));
        Ok(parsed.search)
    }

    pub async fn by_id(&self, id: &str) -> Result<Movie, anyhow::Error> {
        let client = reqwest::Client::new();
        let query_params = format!("?i={}&apikey={}", id, self.api_key);
        let fp = &self.full_path(&query_params);
        let response_builder = client.get(fp);
        let mut headers = HeaderMap::new();
        headers.append(ACCEPT, HeaderValue::from_str("application/json").unwrap());
        let response = response_builder.headers(headers).send().await;
        if response.is_err() {
            todo!();
        }
        let result = response.unwrap();
        if result.status() != 200 {
            todo!();
        }
        let result_text = result.text().await;
        if result_text.is_err() {
            todo!();
        }
        let result_text = result_text.unwrap();
        log(&result_text);
        let parsed: Result<Movie, serde_json::Error> = serde_json::from_str(&result_text);
        if parsed.is_err() {
            log(&format!("{:?}", parsed.err().unwrap()));
            panic!("LOL")
        }
        Ok(parsed.unwrap())
    }
}
