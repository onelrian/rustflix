//! TMDb (The Movie Database) provider implementation

use crate::providers::{MetadataProvider, SearchResult, ProviderConfig};
use rustflix_core::{Result, RustFlixError, MediaMetadata, MediaType};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// TMDb metadata provider
#[derive(Debug, Clone)]
pub struct TmdbProvider {
    client: Client,
    config: ProviderConfig,
}

/// TMDb search response
#[derive(Debug, Deserialize)]
struct TmdbSearchResponse {
    results: Vec<TmdbSearchResult>,
}

/// TMDb search result
#[derive(Debug, Deserialize)]
struct TmdbSearchResult {
    id: u32,
    title: Option<String>,
    name: Option<String>,
    original_title: Option<String>,
    original_name: Option<String>,
    release_date: Option<String>,
    first_air_date: Option<String>,
    overview: Option<String>,
    poster_path: Option<String>,
    popularity: Option<f32>,
}

impl TmdbProvider {
    /// Create a new TMDb provider
    pub fn new(api_key: String) -> Result<Self> {
        let config = ProviderConfig {
            name: "TMDb".to_string(),
            base_url: "https://api.themoviedb.org/3".to_string(),
            api_key: Some(api_key),
            rate_limit: Some(40), // TMDb allows 40 requests per 10 seconds
            priority: 1,
            enabled: true,
        };

        Ok(Self {
            client: Client::new(),
            config,
        })
    }
}

#[async_trait]
impl MetadataProvider for TmdbProvider {
    fn name(&self) -> &str {
        &self.config.name
    }

    async fn search(&self, query: &str, media_type: MediaType) -> Result<Vec<SearchResult>> {
        let endpoint = match media_type {
            MediaType::Movie => "search/movie",
            MediaType::TvShow => "search/tv",
            MediaType::Person => "search/person",
            MediaType::Episode => "search/tv", // Episodes are part of TV shows
            MediaType::Music => return Ok(vec![]), // TMDb doesn't support music
            MediaType::Photo => return Ok(vec![]), // TMDb doesn't support photos
            MediaType::Other => "search/multi",
        };

        let url = format!("{}/{}", self.config.base_url, endpoint);
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| RustFlixError::config("TMDb API key not configured"))?;

        debug!("Searching TMDb: {} for '{}'", endpoint, query);

        let response = self.client
            .get(&url)
            .query(&[
                ("api_key", api_key),
                ("query", &query.to_string()),
            ])
            .send()
            .await
            .map_err(RustFlixError::from)?;

        let search_response: TmdbSearchResponse = response
            .json()
            .await
            .map_err(RustFlixError::from)?;

        let results: Vec<_> = search_response.results
            .into_iter()
            .map(|result| SearchResult {
                external_id: result.id.to_string(),
                title: result.title.or(result.name).unwrap_or_default(),
                original_title: result.original_title.or(result.original_name),
                release_date: result.release_date
                    .or(result.first_air_date)
                    .and_then(|date| chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()),
                overview: result.overview,
                poster_path: result.poster_path,
                media_type,
                popularity: result.popularity,
            })
            .collect();

        debug!("Found {} results from TMDb", results.len());
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
    fn test_tmdb_provider_creation() {
        let provider = TmdbProvider::new("test_api_key".to_string());
        assert!(provider.is_ok());
    }
}
