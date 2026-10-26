#![allow(dead_code)]

use crate::models::SubtitleItem;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;

const SUBDL_API_URL: &str = "https://api.subdl.com/api/v1/subtitles";
const SUBDL_DL_BASE: &str = "https://dl.subdl.com";
const APP_USER_AGENT: &str = "PrasunkMovieApi/1.0";

#[derive(Debug, Deserialize)]
struct SubdlResponse {
    pub status: bool,
    pub subtitles: Option<Vec<SubdlSubtitle>>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SubdlSubtitle {
    pub release_name: Option<String>,
    pub name: Option<String>,
    pub lang: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub language: Option<String>,
    pub unpack_files: Option<Vec<SubdlUnpackFile>>,
}

#[derive(Debug, Deserialize)]
struct SubdlUnpackFile {
    pub file_n_id: Option<String>,
    pub name: Option<String>,
    pub release_name: Option<String>,
    pub language: Option<String>,
    pub format: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone)]
pub struct SubdlService {
    client: Client,
    api_key: String,
}

impl SubdlService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(12))
                .build()
                .unwrap_or_default(),
            api_key,
        }
    }

    /// Cek apakah SubDL API Key sudah dikonfigurasi
    pub fn is_configured(&self) -> bool {
        let key = self.api_key.trim();
        !key.is_empty()
            && key != "your_subdl_api_key_here"
            && key != "isi_key_subdl_di_sini"
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
                "SUBDL_API_KEY belum dikonfigurasi di file .env. Dapatkan key gratis di https://subdl.com".to_string(),
            ));
        }

        let langs = languages.unwrap_or("id,en");
        let subdl_type = if media_type == "tv" { "tv" } else { "movie" };

        let mut query: HashMap<&str, String> = HashMap::new();
        query.insert("api_key", self.api_key.clone());
        query.insert("tmdb_id", tmdb_id.to_string());
        query.insert("type", subdl_type.to_string());
        query.insert("languages", langs.to_string());
        query.insert("unpack", "1".to_string()); // Otomatis unpack ke file .srt langsung

        if subdl_type == "tv" {
            if let Some(s) = season {
                query.insert("season", s.to_string());
            }
            if let Some(e) = episode {
                query.insert("episode", e.to_string());
            }
        }

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(APP_USER_AGENT));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let resp = self
            .client
            .get(SUBDL_API_URL)
            .headers(headers)
            .query(&query)
            .send()
            .await
            .map_err(|e| {
                (
                    StatusCode::BAD_GATEWAY,
                    format!("Gagal menghubungi SubDL API: {}", e),
                )
            })?;

        let status = resp.status();
        if !status.is_success() {
            let err_text = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown SubDL error".to_string());
            return Err((
                status,
                format!("SubDL API error (status {}): {}", status, err_text),
            ));
        }

        let result: SubdlResponse = resp.json().await.map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                format!("Gagal mem-parsing respon SubDL: {}", e),
            )
        })?;

        let mut items = Vec::new();

        if let Some(sub_list) = result.subtitles {
            for (idx, sub) in sub_list.into_iter().enumerate() {
                let default_lang = sub
                    .language
                    .clone()
                    .or_else(|| sub.lang.clone())
                    .unwrap_or_else(|| "id".to_string())
                    .to_lowercase();

                let author = sub.author.clone();

                // Jika ada file hasil unpack (.srt individual)
                if let Some(unpacked) = sub.unpack_files {
                    for (u_idx, u_file) in unpacked.into_iter().enumerate() {
                        let raw_url = u_file.url.as_deref().unwrap_or("");
                        let clean_path = raw_url.split('?').next().unwrap_or(raw_url);

                        let file_name = u_file
                            .name
                            .unwrap_or_else(|| format!("subtitle_{}_{}.srt", idx, u_idx));

                        let release = u_file
                            .release_name
                            .or_else(|| sub.release_name.clone())
                            .unwrap_or_else(|| file_name.clone());

                        let lang = u_file
                            .language
                            .map(|l| l.to_lowercase())
                            .unwrap_or_else(|| default_lang.clone());

                        let id = u_file
                            .file_n_id
                            .unwrap_or_else(|| format!("{}-{}", idx, u_idx));

                        let download_url = format!(
                            "/v1/api/subtitles/download?target={}",
                            urlencoding::encode(clean_path)
                        );

                        items.push(SubtitleItem {
                            id,
                            language: lang,
                            release,
                            file_name,
                            author: author.clone(),
                            download_url,
                        });
                    }
                } else if let Some(raw_url) = sub.url {
                    // Fallback jika tidak ter-unpack (file .zip)
                    let clean_path = raw_url.split('?').next().unwrap_or(&raw_url);
                    let file_name = sub
                        .name
                        .unwrap_or_else(|| format!("subtitle_{}.zip", idx));
                    let release = sub
                        .release_name
                        .unwrap_or_else(|| file_name.clone());

                    let download_url = format!(
                        "/v1/api/subtitles/download?target={}",
                        urlencoding::encode(clean_path)
                    );

                    items.push(SubtitleItem {
                        id: format!("zip-{}", idx),
                        language: default_lang,
                        release,
                        file_name,
                        author,
                        download_url,
                    });
                }
            }
        }

        Ok(items)
    }

    /// Ambil URL download aman ke CDN SubDL
    pub fn build_download_url(&self, target_path: &str) -> Result<String, (StatusCode, String)> {
        if !self.is_configured() {
            return Err((
                StatusCode::BAD_REQUEST,
                "SUBDL_API_KEY belum dikonfigurasi di file .env".to_string(),
            ));
        }

        // Keamanan: pastikan target_path hanya mengarah ke endpoint /subtitle/
        if !target_path.starts_with("/subtitle/") {
            return Err((
                StatusCode::BAD_REQUEST,
                "Target path tidak valid".to_string(),
            ));
        }

        let full_url = format!(
            "{}{}?api_key={}",
            SUBDL_DL_BASE,
            target_path,
            self.api_key.trim()
        );

        Ok(full_url)
    }
}
