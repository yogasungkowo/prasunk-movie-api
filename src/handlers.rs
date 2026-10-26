use crate::models::{
    DetailQuery, DetailResponse, ErrorResponse, ListQuery, PaginatedResponse, SearchQuery,
    SubtitleDownloadQuery, SubtitleQuery, SubtitlesResponse,
};
use crate::subtitles::SubdlService;
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
    pub subtitles: Arc<SubdlService>,
}

/// GET /v1/api/health
pub async fn health_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tmdb_configured = state.tmdb.is_api_key_configured();
    let subdl_configured = state.subtitles.is_configured();

    Json(json!({
        "status": "ok",
        "tmdb_configured": tmdb_configured,
        "subdl_configured": subdl_configured,
        "message": if tmdb_configured && subdl_configured {
            "Semua layanan (TMDB & SubDL) aktif dan siap."
        } else if !tmdb_configured {
            "PERINGATAN: TMDB_API_KEY belum diisi di file .env!"
        } else {
            "INFO: SUBDL_API_KEY belum diisi di .env (Fitur subtitle non-aktif sementara)."
        },
        "attribution": "This product uses the TMDB API and SubDL API."
    }))
}

/// GET /v1/api/film/trending?type=movie|tv&page=1&limit=20
pub async fn trending_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match state.tmdb.get_trending(media_type, page, limit).await {
        Ok(res) => Ok(Json(res)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /v1/api/film/popular?type=movie|tv&page=1&limit=20
pub async fn popular_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match state.tmdb.get_popular(media_type, page, limit).await {
        Ok(res) => Ok(Json(res)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /v1/api/film/now-playing?page=1&limit=20
pub async fn now_playing_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match state.tmdb.get_now_playing(page, limit).await {
        Ok(res) => Ok(Json(res)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /v1/api/film/top-rated?type=movie|tv&page=1&limit=20
pub async fn top_rated_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match state.tmdb.get_top_rated(media_type, page, limit).await {
        Ok(res) => Ok(Json(res)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /v1/api/film/discover?type=movie|tv&page=1&limit=20&sort_by=...&genre=...&year=...
pub async fn discover_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match state
        .tmdb
        .get_discover(
            media_type,
            page,
            limit,
            query.sort_by.as_deref(),
            query.genre.as_deref(),
            query.year,
        )
        .await
    {
        Ok(res) => Ok(Json(res)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /v1/api/film/search?q=judul&type=movie|tv&page=1&limit=20
pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<PaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let media_type = if query.r#type.as_deref() == Some("tv") {
        "tv"
    } else {
        "movie"
    };

    let q = query.q.unwrap_or_default();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    match state.tmdb.search(media_type, &q, page, limit).await {
        Ok(res) => Ok(Json(res)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET /v1/api/film/:type/:id?season=1&episode=1
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

/// GET /v1/api/film/:type/:id/subtitles?lang=id,en&season=1&episode=1
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

/// GET /v1/api/subtitles/download?target=/subtitle/...
pub async fn download_subtitle_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SubtitleDownloadQuery>,
) -> Result<Redirect, (StatusCode, Json<ErrorResponse>)> {
    let target = query.target.unwrap_or_default();
    match state.subtitles.build_download_url(&target) {
        Ok(url) => Ok(Redirect::temporary(&url)),
        Err((status, msg)) => Err((status, Json(ErrorResponse { error: msg }))),
    }
}

/// GET / or /player - Web Player & Testing UI
pub async fn player_page_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}
