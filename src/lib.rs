use askama::Template;
use regex_lite::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use storage::retrieve_string;
use url::Url;
use wasm_bindgen::prelude::*;

mod omdb_api;
use omdb_api::OmdbApi;
mod storage;
mod templates;
use templates::structs::{GridItem, MovieDetails, Search};

// Function to log to the browser console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

async fn home(_params: RouteParams) -> String {
    let template = Search {
        query: String::new(),
        results: String::new(),
    };
    template.render().unwrap()
}

async fn search(params: RouteParams) -> String {
    log(&format!("{:?}", &params));
    let mut query = String::new();
    let query_get = params.query_params.get("query");
    if query_get.is_none() {
        let last_query = storage::retrieve_string("query");
        if last_query.is_ok() {
            let last_query = last_query.unwrap();
            if last_query.is_some() {
                let last_query = last_query.unwrap();
                query = last_query;
            }
        } else {
            return home(params).await;
        }
    } else {
        query = query_get.unwrap().to_string();
    }
    let store = storage::store_string("query", &query);
    if store.is_err() {
        log(&format!("{:?}", store.err().unwrap()));
    }
    let cached_results =
        retrieve_string(format!("movie_query_{}", query.replace(" ", "_")).as_str());
    if cached_results.is_ok() {
        let cached_results = cached_results.unwrap();
        if cached_results.is_some() {
            let cached_results = cached_results.unwrap();
            let template = Search {
                query: String::from(query),
                results: cached_results,
            };
            return template.render().unwrap();
        }
    }
    let api = OmdbApi::new();
    let results = api.search(query.as_str()).await;
    if results.is_err() {
        log(&format!("{:?}, {:?}", &query, results));
        todo!()
    }
    let results = results.unwrap();

    let mut response = String::new();

    for result in results {
        let template = GridItem {
            id: result.imdb_id,
            poster: result.poster,
        };
        response.push_str(template.render().unwrap().as_str())
    }

    let store = storage::store_string(
        format!("movie_query_{}", query.replace(" ", "_")).as_str(),
        &response,
    );
    if store.is_err() {
        log(&format!("{:?}", store.err().unwrap()));
    }

    let template = Search {
        query: String::from(query),
        results: response,
    };
    template.render().unwrap()
}

async fn movie(params: RouteParams) -> String {
    let id = params.path_params.get("id");
    if id.is_none() {
        todo!()
    }
    let id = id.unwrap();
    if id.len() < 1 {
        todo!()
    }
    let cached_results = retrieve_string(format!("movie_id_{}", id.as_str()).as_str());
    if cached_results.is_ok() {
        let cached_results = cached_results.unwrap();
        if cached_results.is_some() {
            let cached_results = cached_results.unwrap();
            return cached_results;
        }
    }
    let api = OmdbApi::new();
    let result = api.by_id(id.as_str()).await;
    if result.is_err() {
        todo!()
    }
    let result = result.unwrap();

    let template = MovieDetails {
        id: result.imdb_id,
        title: result.title,
        plot: result.plot,
        poster: result.poster,
    };
    let response = template.render().unwrap();

    let store = storage::store_string(format!("movie_id_{}", id.as_str()).as_str(), &response);
    if store.is_err() {
        log(&format!("{:?}", store.err().unwrap()));
    }
    response
}

fn not_found() -> String {
    String::from("<h2>Womp Womp</h2>")
}

// Function to handle routing
#[wasm_bindgen]
pub async fn handle_request(path: &str, method: &str, query_params: &str) -> String {
    let mut router = Router::new();
    // Define a route with a parameter ":id"
    router.add_route("GET", "/", |params| home(params));
    router.add_route("GET", "/search", move |params| search(params));
    router.add_route("POST", "/search", move |params| search(params));
    router.add_route("GET", "/movie/:id", |params| movie(params));
    router
        .handle(path, method.to_uppercase().as_str(), query_params)
        .await
        .unwrap_or(not_found())
}
pub struct Router {
    routes: Vec<(
        String,
        Regex,
        Box<dyn Fn(RouteParams) -> Pin<Box<dyn Future<Output = String>>> + Sync>,
    )>,
}

#[derive(Debug)]
struct RouteParams {
    path_params: HashMap<String, String>,
    query_params: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route<F, Fut>(&mut self, method: &str, pattern: &str, handler: F)
    where
        F: Fn(RouteParams) -> Fut + Sync + 'static,
        Fut: Future<Output = String> + 'static,
    {
        // Convert dynamic segments (:param) into named regex groups (?P<param>[^/]+)
        let regex_pattern = format!(
            "^{}$",
            Regex::new(r":(\w+)")
                .unwrap()
                .replace_all(pattern, r"(?P<$1>[^/]+)")
        );
        let re = Regex::new(&regex_pattern).expect("Invalid regex pattern");
        self.routes.push((
            method.to_uppercase(),
            re,
            Box::new(move |params: RouteParams| Box::pin(handler(params))),
        ));
    }

    pub async fn handle(&self, path: &str, method: &str, query_params: &str) -> Option<String> {
        for (route_method, regex, handler) in &self.routes {
            if route_method == method && regex.is_match(path) {
                if let Some(captures) = regex.captures(path) {
                    let mut params = RouteParams {
                        path_params: HashMap::new(),
                        query_params: HashMap::new(),
                    };
                    for name in regex.capture_names().flatten() {
                        if let Some(value) = captures.name(name) {
                            params
                                .path_params
                                .insert(name.to_string(), value.as_str().to_string());
                        }
                    }

                    // Parse the URL
                    let full_url = format!("https://rustmx.com{}", query_params);
                    let url = Url::parse(full_url.as_str());
                    if url.is_err() {
                        log(&format!("{}", url.err().unwrap()));
                        panic!("NO LOL")
                    }

                    // Extract and iterate over query pairs
                    for (key, value) in url.unwrap().query_pairs() {
                        params
                            .query_params
                            .insert(key.to_string(), value.to_string());
                    }
                    log(&format!("{:?}", params));
                    return Some(handler(params).await);
                }
            }
        }
        None // No matching route
    }
}
