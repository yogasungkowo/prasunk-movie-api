#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Query parameters for paginated film list endpoints (trending, popular, now-playing, top-rated, discover)
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub r#type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<usize>,
    pub sort_by: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
}

pub type TrendingQuery = ListQuery;

/// Query parameters for search endpoint
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub r#type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<usize>,
}

/// Query parameters for movie/tv detail endpoint
#[derive(Debug, Deserialize)]
pub struct DetailQuery {
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

/// Item returned in TMDB list (trending, search, discover)
#[derive(Debug, Clone, Deserialize)]
pub struct TmdbMediaItem {
    pub id: i64,
    pub title: Option<String>,
    pub name: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f64>,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub overview: Option<String>,
}

/// TMDB Raw Paginated response (trending, popular, search, discover)
#[derive(Debug, Deserialize)]
pub struct TmdbPaginatedResponse {
    pub page: Option<u32>,
    pub total_pages: Option<u32>,
    pub total_results: Option<u64>,
    pub results: Vec<TmdbMediaItem>,
}

pub type TmdbSearchResponse = TmdbPaginatedResponse;
pub type TmdbTrendingResponse = TmdbPaginatedResponse;

/// TMDB Genre
#[derive(Debug, Deserialize)]
pub struct TmdbGenre {
    pub id: i64,
    pub name: String,
}

/// TMDB Video
#[derive(Debug, Deserialize)]
pub struct TmdbVideo {
    pub site: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub official: Option<bool>,
}

/// TMDB Videos container
#[derive(Debug, Deserialize)]
pub struct TmdbVideosContainer {
    pub results: Option<Vec<TmdbVideo>>,
}

/// TMDB Season (for TV shows)
#[derive(Debug, Deserialize)]
pub struct TmdbSeason {
    pub season_number: u32,
    pub episode_count: u32,
}

/// TMDB Detail response (movie or tv with append_to_response=videos)
#[derive(Debug, Deserialize)]
pub struct TmdbDetailResponse {
    pub id: i64,
    pub title: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub genres: Option<Vec<TmdbGenre>>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub videos: Option<TmdbVideosContainer>,
    pub seasons: Option<Vec<TmdbSeason>>,
}

// -------------------------------------------------------------
// Client API Response Structures
// -------------------------------------------------------------

/// Normalized movie/tv card for client UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardItem {
    pub id: i64,
    pub r#type: String,
    pub title: String,
    pub poster: Option<String>,
    pub rating: Option<f64>,
    pub year: String,
}

/// Unified Paginated Response for Client UI (Next.js, Vue, Mobile, Web Player)
#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse {
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: usize,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "totalResults")]
    pub total_results: u64,
    pub results: Vec<CardItem>,
}

pub type TrendingResponse = PaginatedResponse;
pub type SearchResponse = PaginatedResponse;

/// TV Show season summary for client
#[derive(Debug, Clone, Serialize)]
pub struct SeasonSummary {
    pub season: u32,
    pub episodes: u32,
}

/// Embed player server entry
#[derive(Debug, Clone, Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub url: String,
}

/// Response for GET /film/:type/:id
#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub id: i64,
    pub r#type: String,
    pub title: String,
    pub overview: Option<String>,
    pub genres: Vec<String>,
    pub poster: Option<String>,
    pub backdrop: Option<String>,
    pub trailer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<SeasonSummary>>,
    pub servers: Vec<ServerInfo>,
}

/// Query parameters for subtitles endpoint
#[derive(Debug, Deserialize)]
pub struct SubtitleQuery {
    pub lang: Option<String>,
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

/// Query parameters for subtitle download
#[derive(Debug, Deserialize)]
pub struct SubtitleDownloadQuery {
    pub target: Option<String>,
}

/// Normalized client subtitle item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleItem {
    pub id: String,
    pub language: String,
    pub release: String,
    pub file_name: String,
    pub author: Option<String>,
    pub download_url: String,
}

/// Response for GET /v1/api/film/:type/:id/subtitles
#[derive(Debug, Serialize)]
pub struct SubtitlesResponse {
    pub total: usize,
    pub subtitles: Vec<SubtitleItem>,
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
