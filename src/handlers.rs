use crate::models::{
    DetailQuery, DetailResponse, ErrorResponse, SearchQuery, SearchResponse, SubtitleQuery,
    SubtitlesResponse, TrendingQuery, TrendingResponse,
};
use crate::subtitles::OpenSubtitlesService;
use crate::tmdb::TmdbService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Redirect},
};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub tmdb: Arc<TmdbService>,
    pub subtitles: Arc<OpenSubtitlesService>,
}

/// GET /health
pub async fn health_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tmdb_configured = state.tmdb.is_api_key_configured();
    let os_configured = state.subtitles.is_configured();

    Json(json!({
        "status": "ok",
        "tmdb_configured": tmdb_configured,
        "opensubtitles_configured": os_configured,
        "message": if tmdb_configured && os_configured {
            "Semua layanan (TMDB & OpenSubtitles) aktif dan siap."
        } else if !tmdb_configured {
            "PERINGATAN: TMDB_API_KEY belum diisi di file .env!"
        } else {
            "INFO: OPENSUBTITLES_API_KEY belum diisi di .env (Fitur subtitle non-aktif sementara)."
        },
        "attribution": "This product uses the TMDB API and OpenSubtitles.com API."
    }))
}

/// GET /film/trending?type=movie|tv
pub async fn trending_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TrendingQuery>,
) -> Result<Json<TrendingResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };

    match state.tmdb.get_trending(media_type).await {
        Ok(results) => Ok(Json(TrendingResponse { results })),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /film/search?q=judul&type=movie|tv&page=1
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };

    let q = query.q.unwrap_or_default();
    let page = query.page.unwrap_or(1);

    match state.tmdb.search(media_type, &q, page).await {
        Ok((page, total_pages, results)) => Ok(Json(SearchResponse {
            page,
            total_pages,
            results,
        })),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /film/:type/:id?season=1&episode=1
pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Path((media_type, id)): Path<(String, i64)>,
    Query(query): Query<DetailQuery>,
) -> Result<Json<DetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    if media_type != "movie" && media_type != "tv" {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Parameter type harus movie atau tv".to_string(),
            }),
        ));
    }

    let season = query.season.unwrap_or(1);
    let episode = query.episode.unwrap_or(1);

    match state.tmdb.get_detail(&media_type, id, season, episode).await {
        Ok(detail) => Ok(Json(detail)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /film/:type/:id/subtitles?lang=id,en&season=1&episode=1
pub async fn subtitles_handler(
    State(state): State<Arc<AppState>>,
    Path((media_type, id)): Path<(String, i64)>,
    Query(query): Query<SubtitleQuery>,
) -> Result<Json<SubtitlesResponse>, (StatusCode, Json<ErrorResponse>)> {
    if media_type != "movie" && media_type != "tv" {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Parameter type harus movie atau tv".to_string(),
            }),
        ));
    }

    match state
        .subtitles
        .search(
            &media_type,
            id,
            query.season,
            query.episode,
            query.lang.as_deref(),
        )
        .await
    {
        Ok(subs) => Ok(Json(SubtitlesResponse {
            total: subs.len(),
            subtitles: subs,
        })),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /subtitles/download/:file_id
pub async fn download_subtitle_handler(
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<i64>,
) -> Result<Redirect, (StatusCode, Json<ErrorResponse>)> {
    match state.subtitles.get_download_url(file_id).await {
        Ok(url) => Ok(Redirect::temporary(&url)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET / or /player - Web Player & Testing UI
pub async fn player_page_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}
