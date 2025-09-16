//! OMDb (Open Movie Database) provider implementation

use crate::providers::{MetadataProvider, SearchResult, ProviderConfig};
use rustflix_core::{Result, RustFlixError, MediaMetadata, MediaType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// OMDb metadata provider
#[derive(Debug, Clone)]
pub struct OmdbProvider {
    client: Client,
    config: ProviderConfig,
}

/// OMDb search response
#[derive(Debug, Deserialize)]
struct OmdbSearchResponse {
    #[serde(rename = "Search")]
    search: Option<Vec<OmdbSearchResult>>,
    #[serde(rename = "Response")]
    response: String,
}

/// OMDb search result
#[derive(Debug, Deserialize)]
struct OmdbSearchResult {
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Year")]
    year: String,
    #[serde(rename = "imdbID")]
    imdb_id: String,
    #[serde(rename = "Type")]
    media_type: String,
    #[serde(rename = "Poster")]
    poster: String,
}

impl OmdbProvider {
    /// Create a new OMDb provider
    pub fn new(api_key: String) -> Result<Self> {
        let config = ProviderConfig {
            name: "OMDb".to_string(),
            base_url: "http://www.omdbapi.com".to_string(),
            api_key: Some(api_key),
            rate_limit: Some(1000), // OMDb allows 1000 requests per day
            priority: 2,
            enabled: true,
        };

        Ok(Self {
            client: Client::new(),
            config,
        })
    }
}

#[async_trait]
impl MetadataProvider for OmdbProvider {
    fn name(&self) -> &str {
        &self.config.name
    }

    async fn search(&self, query: &str, media_type: MediaType) -> Result<Vec<SearchResult>> {
        let type_param = match media_type {
            MediaType::Movie => "movie",
            MediaType::TvShow => "series",
            MediaType::Episode => "episode",
            MediaType::Person => return Ok(vec![]), // OMDb doesn't support person search
            MediaType::Music => return Ok(vec![]), // OMDb doesn't support music
            MediaType::Photo => return Ok(vec![]), // OMDb doesn't support photos
            MediaType::Other => "movie", // Default to movie for other types
        };

        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| RustFlixError::config("OMDb API key not configured"))?;

        debug!("Searching OMDb for '{}' (type: {})", query, type_param);

        let response = self.client
            .get(&self.config.base_url)
            .query(&[
                ("apikey", api_key),
                ("s", &query.to_string()),
                ("type", &type_param.to_string()),
            ])
            .send()
            .await
            .map_err(RustFlixError::from)?;

        let search_response: OmdbSearchResponse = response
            .json()
            .await
            .map_err(RustFlixError::from)?;

        if search_response.response != "True" {
            return Ok(vec![]);
        }

        let results: Vec<_> = search_response.search
            .unwrap_or_default()
            .into_iter()
            .map(|result| SearchResult {
                external_id: result.imdb_id,
                title: result.title,
                original_title: None,
                release_date: result.year.parse::<i32>().ok()
                    .and_then(|year| chrono::NaiveDate::from_ymd_opt(year, 1, 1)),
                overview: None,
                poster_path: if result.poster != "N/A" { Some(result.poster) } else { None },
                media_type,
                popularity: None,
            })
            .collect();

        debug!("Found {} results from OMDb", results.len());
        Ok(results)
    }

    async fn get_metadata(&self, external_id: &str) -> Result<MediaMetadata> {
        // Placeholder implementation - would fetch detailed metadata
        Ok(MediaMetadata::default())
    }

    fn config(&self) -> &ProviderConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omdb_provider_creation() {
        let provider = OmdbProvider::new("test_api_key".to_string());
        assert!(provider.is_ok());
    }
}
