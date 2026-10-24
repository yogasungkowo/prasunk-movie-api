use crate::models::{
    CardItem, DetailResponse, SeasonSummary, ServerInfo, TmdbDetailResponse, TmdbMediaItem,
    TmdbSearchResponse, TmdbTrendingResponse,
};
use reqwest::Client;
use std::collections::HashMap;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";
const IMG_BASE_URL: &str = "https://image.tmdb.org/t/p/";

#[derive(Clone)]
pub struct TmdbService {
    client: Client,
    api_key: String,
}

impl TmdbService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            api_key,
        }
    }

    /// Check if API key is configured
    pub fn is_api_key_configured(&self) -> bool {
        !self.api_key.trim().is_empty() && self.api_key != "isi_key_kamu_di_sini" && self.api_key != "your_tmdb_api_key_here"
    }

    /// Koleksi generator player embed pihak ketiga
    pub fn servers(&self, media_type: &str, id: i64, s: u32, e: u32) -> Vec<ServerInfo> {
        let is_tv = media_type == "tv";
        vec![
            ServerInfo {
                name: "VidLink".to_string(),
                url: if is_tv {
                    format!("https://vidlink.pro/tv/{}/{}/{}", id, s, e)
                } else {
                    format!("https://vidlink.pro/movie/{}", id)
                },
            },
            ServerInfo {
                name: "Videasy".to_string(),
                url: if is_tv {
                    format!("https://player.videasy.net/tv/{}/{}/{}", id, s, e)
                } else {
                    format!("https://player.videasy.net/movie/{}", id)
                },
            },
            ServerInfo {
                name: "VidFast".to_string(),
                url: if is_tv {
                    format!("https://vidfast.pro/tv/{}/{}/{}", id, s, e)
                } else {
                    format!("https://vidfast.pro/movie/{}", id)
                },
            },
            ServerInfo {
                name: "VidSrc".to_string(),
                url: if is_tv {
                    format!("https://vidsrc.to/embed/tv/{}/{}/{}", id, s, e)
                } else {
                    format!("https://vidsrc.to/embed/movie/{}", id)
                },
            },
            ServerInfo {
                name: "2Embed".to_string(),
                url: if is_tv {
                    format!("https://www.2embed.cc/embedtv/{}&s={}&e={}", id, s, e)
                } else {
                    format!("https://www.2embed.cc/embed/{}", id)
                },
            },
            ServerInfo {
                name: "MultiEmbed".to_string(),
                url: if is_tv {
                    format!("https://multiembed.mov/?video_id={}&tmdb=1&s={}&e={}", id, s, e)
                } else {
                    format!("https://multiembed.mov/?video_id={}&tmdb=1", id)
                },
            },
            ServerInfo {
                name: "AutoEmbed".to_string(),
                url: if is_tv {
                    format!("https://autoembed.co/tv/tmdb/{}-{}-{}", id, s, e)
                } else {
                    format!("https://autoembed.co/movie/tmdb/{}", id)
                },
            },
        ]
    }

    /// Normalisasi item film untuk card UI
    pub fn to_card(&self, item: &TmdbMediaItem, media_type: &str) -> CardItem {
        let title = item
            .title
            .as_ref()
            .or(item.name.as_ref())
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let poster = item
            .poster_path
            .as_ref()
            .map(|p| format!("{}w342{}", IMG_BASE_URL, p));

        let date_str = item
            .release_date
            .as_ref()
            .or(item.first_air_date.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("");

        let year = if date_str.len() >= 4 {
            date_str[0..4].to_string()
        } else {
            String::new()
        };

        CardItem {
            id: item.id,
            r#type: media_type.to_string(),
            title,
            poster,
            rating: item.vote_average,
            year,
        }
    }

    /// Generic helper to call TMDB API with query parameters
    async fn fetch_tmdb<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        extra_params: &[(&str, &str)],
    ) -> Result<T, (reqwest::StatusCode, String)> {
        let url = format!("{}{}", TMDB_BASE_URL, path);

        let mut query: HashMap<&str, &str> = HashMap::new();
        query.insert("api_key", &self.api_key);
        query.insert("language", "id-ID");

        for (k, v) in extra_params {
            query.insert(k, v);
        }

        let resp = self
            .client
            .get(&url)
            .query(&query)
            .send()
            .await
            .map_err(|e| {
                (
                    reqwest::StatusCode::BAD_GATEWAY,
                    format!("Gagal menghubungi TMDB: {}", e),
                )
            })?;

        let status = resp.status();
        if !status.is_success() {
            let error_text = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown TMDB error".to_string());
            return Err((
                status,
                format!("TMDB API Error (Status {}): {}", status, error_text),
            ));
        }

        resp.json::<T>().await.map_err(|e| {
            (
                reqwest::StatusCode::BAD_GATEWAY,
                format!("Gagal mem-parsing respon TMDB: {}", e),
            )
        })
    }

    /// GET /trending/{type}/week
    pub async fn get_trending(
        &self,
        media_type: &str,
    ) -> Result<Vec<CardItem>, (reqwest::StatusCode, String)> {
        let path = format!("/trending/{}/week", media_type);
        let data: TmdbTrendingResponse = self.fetch_tmdb(&path, &[]).await?;
        let cards = data
            .results
            .into_iter()
            .map(|item| self.to_card(&item, media_type))
            .collect();
        Ok(cards)
    }

    /// GET /search/{type}?query=...&page=...
    pub async fn search(
        &self,
        media_type: &str,
        query: &str,
        page: u32,
    ) -> Result<(u32, u32, Vec<CardItem>), (reqwest::StatusCode, String)> {
        let path = format!("/search/{}", media_type);
        let page_str = page.to_string();
        let params = [("query", query), ("page", &page_str)];
        let data: TmdbSearchResponse = self.fetch_tmdb(&path, &params).await?;
        let cards = data
            .results
            .into_iter()
            .map(|item| self.to_card(&item, media_type))
            .collect();
        Ok((data.page, data.total_pages, cards))
    }

    /// GET /{type}/{id}?append_to_response=videos
    pub async fn get_detail(
        &self,
        media_type: &str,
        id: i64,
        season: u32,
        episode: u32,
    ) -> Result<DetailResponse, (reqwest::StatusCode, String)> {
        let path = format!("/{}/{}", media_type, id);
        let params = [("append_to_response", "videos")];
        let d: TmdbDetailResponse = self.fetch_tmdb(&path, &params).await?;

        // Extract trailer URL from videos array
        let trailer = if let Some(videos) = d.videos.and_then(|v| v.results) {
            let yt_videos: Vec<_> = videos
                .into_iter()
                .filter(|v| v.site.as_deref() == Some("YouTube"))
                .collect();

            let matched = yt_videos
                .iter()
                .find(|v| v.r#type.as_deref() == Some("Trailer") && v.official == Some(true))
                .or_else(|| {
                    yt_videos
                        .iter()
                        .find(|v| v.r#type.as_deref() == Some("Trailer"))
                })
                .or_else(|| yt_videos.first());

            matched.and_then(|v| {
                v.key
                    .as_ref()
                    .map(|k| format!("https://www.youtube-nocookie.com/embed/{}", k))
            })
        } else {
            None
        };

        let title = d
            .title
            .or(d.name)
            .unwrap_or_else(|| "Unknown".to_string());

        let genres = d
            .genres
            .unwrap_or_default()
            .into_iter()
            .map(|g| g.name)
            .collect();

        let poster = d
            .poster_path
            .map(|p| format!("{}w500{}", IMG_BASE_URL, p));

        let backdrop = d
            .backdrop_path
            .map(|b| format!("{}w1280{}", IMG_BASE_URL, b));

        let seasons = if media_type == "tv" {
            d.seasons.map(|s_list| {
                s_list
                    .into_iter()
                    .map(|s| SeasonSummary {
                        season: s.season_number,
                        episodes: s.episode_count,
                    })
                    .collect()
            })
        } else {
            None
        };

        let servers = self.servers(media_type, id, season, episode);

        Ok(DetailResponse {
            id: d.id,
            r#type: media_type.to_string(),
            title,
            overview: d.overview,
            genres,
            poster,
            backdrop,
            trailer,
            seasons,
            servers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_server_urls() {
        let service = TmdbService::new("test_key".to_string());
        let servers = service.servers("movie", 35791, 1, 1);

        assert_eq!(servers.len(), 7);
        assert_eq!(servers[0].name, "VidLink");
        assert_eq!(servers[0].url, "https://vidlink.pro/movie/35791");
        assert_eq!(servers[1].name, "Videasy");
        assert_eq!(servers[1].url, "https://player.videasy.net/movie/35791");
        assert_eq!(servers[2].name, "VidFast");
        assert_eq!(servers[2].url, "https://vidfast.pro/movie/35791");
        assert_eq!(servers[3].name, "VidSrc");
        assert_eq!(servers[3].url, "https://vidsrc.to/embed/movie/35791");
        assert_eq!(servers[4].name, "2Embed");
        assert_eq!(servers[4].url, "https://www.2embed.cc/embed/35791");
        assert_eq!(servers[5].name, "MultiEmbed");
        assert_eq!(servers[5].url, "https://multiembed.mov/?video_id=35791&tmdb=1");
        assert_eq!(servers[6].name, "AutoEmbed");
        assert_eq!(servers[6].url, "https://autoembed.co/movie/tmdb/35791");
    }

    #[test]
    fn test_tv_server_urls() {
        let service = TmdbService::new("test_key".to_string());
        let servers = service.servers("tv", 1399, 2, 5);

        assert_eq!(servers.len(), 7);
        assert_eq!(servers[0].name, "VidLink");
        assert_eq!(servers[0].url, "https://vidlink.pro/tv/1399/2/5");
        assert_eq!(servers[1].name, "Videasy");
        assert_eq!(servers[1].url, "https://player.videasy.net/tv/1399/2/5");
        assert_eq!(servers[4].name, "2Embed");
        assert_eq!(servers[4].url, "https://www.2embed.cc/embedtv/1399&s=2&e=5");
        assert_eq!(servers[5].name, "MultiEmbed");
        assert_eq!(servers[5].url, "https://multiembed.mov/?video_id=1399&tmdb=1&s=2&e=5");
        assert_eq!(servers[6].name, "AutoEmbed");
        assert_eq!(servers[6].url, "https://autoembed.co/tv/tmdb/1399-2-5");
    }

    #[test]
    fn test_to_card_normalization() {
        let service = TmdbService::new("test_key".to_string());
        let item = TmdbMediaItem {
            id: 35791,
            title: Some("Resident Evil: Afterlife".to_string()),
            name: None,
            poster_path: Some("/abc.jpg".to_string()),
            backdrop_path: Some("/backdrop.jpg".to_string()),
            vote_average: Some(6.1),
            release_date: Some("2010-09-09".to_string()),
            first_air_date: None,
            overview: Some("Synopsis".to_string()),
        };

        let card = service.to_card(&item, "movie");
        assert_eq!(card.id, 35791);
        assert_eq!(card.r#type, "movie");
        assert_eq!(card.title, "Resident Evil: Afterlife");
        assert_eq!(card.poster, Some("https://image.tmdb.org/t/p/w342/abc.jpg".to_string()));
        assert_eq!(card.year, "2010");
        assert_eq!(card.rating, Some(6.1));
    }
}

