mod handlers;
mod models;
mod subtitles;
mod tmdb;

use handlers::{
    detail_handler, download_subtitle_handler, health_handler, player_page_handler, search_handler,
    subtitles_handler, trending_handler, AppState,
};
use std::net::SocketAddr;
use std::sync::Arc;
use subtitles::OpenSubtitlesService;
use tmdb::TmdbService;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inisialisasi file environment (.env)
    dotenvy::dotenv().ok();

    // 2. Setup logging / tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "prasunk_movie_api=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3. Baca konfigurasi TMDB & OpenSubtitles API Keys
    let tmdb_api_key = std::env::var("TMDB_API_KEY").unwrap_or_default();
    let opensubtitles_api_key = std::env::var("OPENSUBTITLES_API_KEY").unwrap_or_default();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    if tmdb_api_key.trim().is_empty() || tmdb_api_key == "isi_key_kamu_di_sini" {
        tracing::warn!("⚠️  TMDB_API_KEY belum disetel dengan benar di file .env!");
        tracing::warn!("   Dapatkan API key gratis di: https://www.themoviedb.org/settings/api");
    } else {
        tracing::info!("✅ TMDB API Key berhasil dimuat.");
    }

    let subtitles_service = OpenSubtitlesService::new(opensubtitles_api_key);
    if subtitles_service.is_configured() {
        tracing::info!("✅ OpenSubtitles API Key berhasil dimuat.");
    } else {
        tracing::warn!("ℹ️  OPENSUBTITLES_API_KEY belum diisi di .env.");
        tracing::warn!("   Dapatkan API key gratis di: https://www.opensubtitles.com/en/consumers");
    }

    // 4. Siapkan App State Terpadu (TMDB + Subtitles)
    let app_state = Arc::new(AppState {
        tmdb: Arc::new(TmdbService::new(tmdb_api_key)),
        subtitles: Arc::new(subtitles_service),
    });

    // 5. Konfigurasi CORS (mengizinkan akses dari frontend manapun)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 6. Router API (/v1/api/...) & Web Player
    let api_routes = axum::Router::new()
        .route("/health", axum::routing::get(health_handler))
        .route("/film/trending", axum::routing::get(trending_handler))
        .route("/film/search", axum::routing::get(search_handler))
        .route("/film/{type}/{id}", axum::routing::get(detail_handler))
        .route(
            "/film/{type}/{id}/subtitles",
            axum::routing::get(subtitles_handler),
        )
        .route(
            "/subtitles/download/{file_id}",
            axum::routing::get(download_subtitle_handler),
        );

    let app = axum::Router::new()
        .route("/", axum::routing::get(player_page_handler))
        .route("/player", axum::routing::get(player_page_handler))
        .nest("/v1/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // 7. Menjalankan Server
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("🚀 Server berjalan di http://{}", addr);
    tracing::info!("📺 Web Player & Subtitle Tester siap dibuka: http://localhost:{}", port);
    tracing::info!("📚 Endpoints (Prefix /v1/api):");
    tracing::info!("   - GET /v1/api/health");
    tracing::info!("   - GET /v1/api/film/trending?type=movie|tv");
    tracing::info!("   - GET /v1/api/film/search?q=judul&type=movie|tv&page=1");
    tracing::info!("   - GET /v1/api/film/:type/:id?season=1&episode=1");
    tracing::info!("   - GET /v1/api/film/:type/:id/subtitles?lang=id,en&season=1&episode=1");
    tracing::info!("   - GET /v1/api/subtitles/download/:file_id");

    axum::serve(listener, app).await?;

    Ok(())
}
