//! Metadata types and structures

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for metadata entries
pub type MetadataId = Uuid;

/// External service identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalId {
    pub provider: String,
    pub id: String,
}

/// Rich metadata for media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub id: MetadataId,
    pub title: String,
    pub original_title: Option<String>,
    pub description: Option<String>,
    pub tagline: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub runtime: Option<u32>, // minutes
    pub genres: Vec<String>,
    pub languages: Vec<String>,
    pub countries: Vec<String>,
    pub rating: Option<f32>, // 0.0 - 10.0
    pub vote_count: Option<u32>,
    pub popularity: Option<f32>,
    pub budget: Option<u64>,
    pub revenue: Option<u64>,
    pub cast: Vec<Person>,
    pub crew: Vec<CrewMember>,
    pub external_ids: HashMap<String, String>,
    pub images: MediaImages,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Person information (actor, director, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub character: Option<String>, // For cast members
    pub profile_path: Option<String>,
    pub external_ids: HashMap<String, String>,
}

/// Crew member with role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub person: Person,
    pub job: String,
    pub department: String,
}

/// Image collection for media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaImages {
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub logo: Option<String>,
    pub thumbnails: Vec<String>,
    pub fanart: Vec<String>,
}

/// TV show specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TvShowMetadata {
    pub base: MediaMetadata,
    pub status: TvShowStatus,
    pub episode_count: u32,
    pub season_count: u32,
    pub first_air_date: Option<NaiveDate>,
    pub last_air_date: Option<NaiveDate>,
    pub networks: Vec<String>,
    pub seasons: Vec<SeasonMetadata>,
}

/// TV season metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonMetadata {
    pub id: MetadataId,
    pub season_number: u32,
    pub name: String,
    pub description: Option<String>,
    pub air_date: Option<NaiveDate>,
    pub episode_count: u32,
    pub poster_path: Option<String>,
    pub episodes: Vec<EpisodeMetadata>,
}

/// TV episode metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMetadata {
    pub id: MetadataId,
    pub episode_number: u32,
    pub season_number: u32,
    pub name: String,
    pub description: Option<String>,
    pub air_date: Option<NaiveDate>,
    pub runtime: Option<u32>,
    pub rating: Option<f32>,
    pub vote_count: Option<u32>,
    pub still_path: Option<String>,
    pub guest_stars: Vec<Person>,
}

/// TV show status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TvShowStatus {
    Returning,
    Planned,
    InProduction,
    Ended,
    Cancelled,
    Pilot,
}

/// Movie collection metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMetadata {
    pub id: MetadataId,
    pub name: String,
    pub description: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub movies: Vec<MetadataId>,
}

/// Music album metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumMetadata {
    pub base: MediaMetadata,
    pub artist: String,
    pub album_type: AlbumType,
    pub track_count: u32,
    pub label: Option<String>,
    pub tracks: Vec<TrackMetadata>,
}

/// Music track metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    pub id: MetadataId,
    pub title: String,
    pub track_number: u32,
    pub disc_number: Option<u32>,
    pub duration: Option<u32>, // seconds
    pub artist: String,
    pub album: String,
    pub genre: Option<String>,
    pub year: Option<u32>,
}

/// Album type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AlbumType {
    Album,
    Single,
    Compilation,
    Soundtrack,
    Live,
    Remix,
    Other,
}

/// Metadata provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataProvider {
    pub name: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub rate_limit: Option<u32>, // requests per minute
    pub supported_types: Vec<String>,
    pub priority: u8, // 0-255, higher = more priority
}

impl MediaMetadata {
    /// Create new metadata entry
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            original_title: None,
            description: None,
            tagline: None,
            release_date: None,
            runtime: None,
            genres: Vec::new(),
            languages: Vec::new(),
            countries: Vec::new(),
            rating: None,
            vote_count: None,
            popularity: None,
            budget: None,
            revenue: None,
            cast: Vec::new(),
            crew: Vec::new(),
            external_ids: HashMap::new(),
            images: MediaImages::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add external ID for a provider
    pub fn add_external_id(&mut self, provider: &str, id: &str) {
        self.external_ids.insert(provider.to_string(), id.to_string());
        self.updated_at = Utc::now();
    }

    /// Get external ID for a provider
    pub fn get_external_id(&self, provider: &str) -> Option<&String> {
        self.external_ids.get(provider)
    }

    /// Check if metadata is complete enough for display
    pub fn is_complete(&self) -> bool {
        !self.title.is_empty() 
            && self.description.is_some() 
            && !self.genres.is_empty()
            && self.images.poster.is_some()
    }
}

impl Default for MediaImages {
    fn default() -> Self {
        Self {
            poster: None,
            backdrop: None,
            logo: None,
            thumbnails: Vec::new(),
            fanart: Vec::new(),
        }
    }
}

impl Person {
    /// Create new person
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            character: None,
            profile_path: None,
            external_ids: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_creation() {
        let mut metadata = MediaMetadata::new("Test Movie".to_string());
        assert_eq!(metadata.title, "Test Movie");
        assert!(!metadata.is_complete());

        metadata.description = Some("A test movie".to_string());
        metadata.genres.push("Action".to_string());
        metadata.images.poster = Some("/poster.jpg".to_string());
        
        assert!(metadata.is_complete());
    }

    #[test]
    fn test_external_ids() {
        let mut metadata = MediaMetadata::new("Test".to_string());
        metadata.add_external_id("tmdb", "12345");
        
        assert_eq!(metadata.get_external_id("tmdb"), Some(&"12345".to_string()));
        assert_eq!(metadata.get_external_id("imdb"), None);
    }
}
