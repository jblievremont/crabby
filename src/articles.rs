use serde::Deserialize;
use reqwest::Error;
use reqwest::blocking::Response;

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub nb_hits: u64,
    pub hits: Vec<Article>,
}

pub fn search_articles(keyword: &String) -> Result<SearchResult, reqwest::Error> {
    // format valid URL
    let query = format!("https://hn.algolia.com/api/v1/search?query={}&tagFilters=story", keyword);
    println!("{}", query);
    // execute HTTP call
    let http_response: Result<Response, Error> = reqwest::blocking::get(query);
    // Try to deserialize if response success
    match http_response {
        Ok(response) => response.json::<SearchResult>(),
        Err(e) => Err(e),
    }
}