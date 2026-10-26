#![allow(dead_code)]

use crate::models::{OpenSubtitlesDownloadResponse, SubtitleItem};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

const OS_API_URL: &str = "https://api.opensubtitles.com/api/v1";
const APP_USER_AGENT: &str = "PrasunkMovieApi v0.1.0";

#[derive(Debug, Deserialize)]
struct OsSubtitlesResponse {
    pub total_count: Option<usize>,
    pub data: Option<Vec<OsSubtitleData>>,
}

#[derive(Debug, Deserialize)]
struct OsSubtitleData {
    pub id: Option<String>,
    pub attributes: Option<OsSubtitleAttributes>,
}

#[derive(Debug, Deserialize)]
struct OsSubtitleAttributes {
    pub subtitle_id: Option<String>,
    pub language: Option<String>,
    pub download_count: Option<u64>,
    pub ratings: Option<f64>,
    pub release: Option<String>,
    pub files: Option<Vec<OsSubtitleFile>>,
}

#[derive(Debug, Deserialize)]
struct OsSubtitleFile {
    pub file_id: Option<i64>,
    pub file_name: Option<String>,
}

#[derive(Clone)]
pub struct OpenSubtitlesService {
    client: Client,
    api_key: String,
}

impl OpenSubtitlesService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(12))
                .build()
                .unwrap_or_default(),
            api_key,
        }
    }

    /// Cek apakah OpenSubtitles API Key sudah diatur
    pub fn is_configured(&self) -> bool {
        let key = self.api_key.trim();
        !key.is_empty()
            && key != "your_opensubtitles_api_key_here"
            && key != "isi_key_opensubtitles_di_sini"
    }

    /// Siapkan headers standar untuk OpenSubtitles API v2
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Ok(val) = HeaderValue::from_str(self.api_key.trim()) {
            headers.insert("Api-Key", val);
        }
        headers
    }

    /// Cari subtitle berdasarkan ID TMDB
    pub async fn search(
        &self,
        media_type: &str,
        tmdb_id: i64,
        season: Option<u32>,
        episode: Option<u32>,
        languages: Option<&str>,
    ) -> Result<Vec<SubtitleItem>, (StatusCode, String)> {
        if !self.is_configured() {
            return Err((
                StatusCode::BAD_REQUEST,
                "OPENSUBTITLES_API_KEY belum dikonfigurasi di file .env. Dapatkan key gratis di https://www.opensubtitles.com/en/consumers".to_string(),
            ));
        }

        let url = format!("{}/subtitles", OS_API_URL);
        let langs = languages.unwrap_or("id,en");

        let mut query: HashMap<&str, String> = HashMap::new();
        query.insert("tmdb_id", tmdb_id.to_string());
        query.insert("languages", langs.to_string());

        if media_type == "tv" {
            query.insert("type", "episode".to_string());
            if let Some(s) = season {
                query.insert("season_number", s.to_string());
            }
            if let Some(e) = episode {
                query.insert("episode_number", e.to_string());
            }
        } else {
            query.insert("type", "movie".to_string());
        }

        let resp = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .query(&query)
            .send()
            .await
            .map_err(|e| {
                (
                    StatusCode::BAD_GATEWAY,
                    format!("Gagal menghubungi OpenSubtitles API: {}", e),
                )
            })?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown OpenSubtitles error".to_string());
            return Err((
                status,
                format!("OpenSubtitles API error (status {}): {}", status, err_text),
            ));
        }

        let result: OsSubtitlesResponse = resp.json().await.map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                format!("Gagal mem-parsing respon OpenSubtitles: {}", e),
            )
        })?;

        let mut subtitles = Vec::new();
        if let Some(data_items) = result.data {
            for item in data_items {
                if let Some(attrs) = item.attributes {
                    let first_file = attrs.files.as_ref().and_then(|f| f.first());
                    if let Some(file) = first_file {
                        let file_id = file.file_id.unwrap_or_default();
                        let file_name = file
                            .file_name
                            .clone()
                            .unwrap_or_else(|| "subtitle.srt".to_string());

                        let lang = attrs
                            .language
                            .clone()
                            .unwrap_or_else(|| "unknown".to_string());

                        let release = attrs
                            .release
                            .clone()
                            .unwrap_or_else(|| file_name.clone());

                        let sub_id = item
                            .id
                            .clone()
                            .or(attrs.subtitle_id.clone())
                            .unwrap_or_else(|| file_id.to_string());

                        subtitles.push(SubtitleItem {
                            id: sub_id,
                            file_id,
                            language: lang,
                            release,
                            file_name,
                            rating: attrs.ratings,
                            download_count: attrs.download_count,
                            download_url: format!("/v1/api/subtitles/download/{}", file_id),
                        });
                    }
                }
            }
        }

        Ok(subtitles)
    }

    /// Ambil link unduh langsung dari OpenSubtitles Download API
    pub async fn get_download_url(&self, file_id: i64) -> Result<String, (StatusCode, String)> {
        if !self.is_configured() {
            return Err((
                StatusCode::BAD_REQUEST,
                "OPENSUBTITLES_API_KEY belum dikonfigurasi di file .env".to_string(),
            ));
        }

        let url = format!("{}/download", OS_API_URL);
        let payload = json!({
            "file_id": file_id
        });

        let resp = self
            .client
            .post(&url)
            .headers(self.build_headers())
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                (
                    StatusCode::BAD_GATEWAY,
                    format!("Gagal menghubungi OpenSubtitles Download API: {}", e),
                )
            })?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown OpenSubtitles error".to_string());
            return Err((
                status,
                format!("Gagal membuat tautan unduhan (status {}): {}", status, err_text),
            ));
        }

        let dl_resp: OpenSubtitlesDownloadResponse = resp.json().await.map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                format!("Gagal membaca respon unduhan OpenSubtitles: {}", e),
            )
        })?;

        if let Some(link) = dl_resp.link {
            Ok(link)
        } else {
            Err((
                StatusCode::NOT_FOUND,
                dl_resp
                    .message
                    .unwrap_or_else(|| "Link unduhan tidak tersedia".to_string()),
            ))
        }
    }
}
