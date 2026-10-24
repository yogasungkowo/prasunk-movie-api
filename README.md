# Prasunk Movie API (Rust + Axum) 🎬🦀

Implementasi backend API film sendiri berbasis **Rust** dan **Axum**, mengacu pada dokumen panduan `panduan_membuat_api_film_sendiri (1).md`. API ini bertindak sebagai perantara cerdas (*reverse proxy*) yang mengambil metadata dari **TMDB** (The Movie Database) dan menyisipkan tautan pemutar video embed (*multi-server embed player*).

---

## ⚡ Fitur Utama

- 🦀 **Performa Tinggi:** Dibangun dengan Rust, Axum, dan Tokio asynchronous runtime.
- 🔒 **Reverse Proxy Aman:** Menyembunyikan TMDB API key dari sisi frontend.
- 🎥 **Multi-Server Video Embed:** Menghasilkan 7 server player video secara otomatis (VidLink, Videasy, VidFast, VidSrc, 2Embed, MultiEmbed, AutoEmbed) untuk film dan serial TV.
- 🔍 **Pencarian & Trending:** Endpoint trending mingguan dan pencarian judul film/serial TV berpaginasi.
- 🎞️ **Detail & Trailer YouTube:** Detail sinopsis, genre, poster, backdrop, daftar season (TV), serta link embed trailer YouTube resmi.
- 🌐 **Web Player Tester Bawaan:** Halaman web modern dengan dark mode interaktif untuk menguji pemutar dan pencarian film langsung di browser (`http://localhost:3000`).
- 🛡️ **CORS Enabled:** Dilengkapi middleware CORS agar dapat diakses dari klien frontend mana pun.

---

## 📁 Struktur Proyek

```
prasunk-movie-api/
├── Cargo.toml            # Konfigurasi dependensi Rust
├── .env.example          # Contoh variabel lingkungan
├── .env                  # Variabel lingkungan lokal (disimpan privat)
├── static/
│   └── index.html        # Antarmuka Web Player & API tester interaktif
└── src/
    ├── main.rs           # Entry point aplikasi & inisialisasi server Axum
    ├── models.rs         # Model data TMDB & respon JSON klien
    ├── tmdb.rs           # Service client TMDB & generator link embed player
    └── handlers.rs       # Handler route endpoint API
```

---

## 🚀 Panduan Memulai

### 1. Prasyarat
- **Rust & Cargo** (disarankan v1.80 atau lebih baru)
- **TMDB API Key (v3 auth)** dari [themoviedb.org/settings/api](https://www.themoviedb.org/settings/api)
- **OpenSubtitles API Key (Consumer)** *(Gratis)* dari [opensubtitles.com/en/consumers](https://www.opensubtitles.com/en/consumers)

### 2. Konfigurasi Lingkungan (`.env`)
Salin file `.env.example` ke `.env` jika belum ada:

```bash
cp .env.example .env
```

Buka file `.env` dan masukkan API Key TMDB & OpenSubtitles Anda:
```env
TMDB_API_KEY=masukkan_api_key_tmdb_anda_di_sini
OPENSUBTITLES_API_KEY=masukkan_api_key_opensubtitles_di_sini
PORT=3000
HOST=0.0.0.0
```

### 3. Menjalankan Server
Jalankan aplikasi dengan perintah:

```bash
cargo run
```

Atau kompilasi mode rilis dengan optimasi penuh:
```bash
cargo run --release
```

Server akan aktif di `http://localhost:3000`.

---

## 📡 Daftar Endpoint API

Semua endpoint mengembalikan respon JSON dan mendukung CORS.

### 1. Cek Status Server & Konfigurasi TMDB
- **Endpoint:** `GET /health`
- **Respon:**
  ```json
  {
    "status": "ok",
    "tmdb_configured": true,
    "message": "API aktif dan TMDB API Key sudah terpasang.",
    "attribution": "This product uses the TMDB API but is not endorsed or certified by TMDB."
  }
  ```

### 2. Film / Serial Trending
- **Endpoint:** `GET /film/trending?type=movie` atau `GET /film/trending?type=tv`
- **Query Parameter:**
  - `type` *(opsional)*: `movie` (default) atau `tv`
- **Contoh Respon:**
  ```json
  {
    "results": [
      {
        "id": 35791,
        "type": "movie",
        "title": "Resident Evil: Afterlife",
        "poster": "https://image.tmdb.org/t/p/w342/abc.jpg",
        "rating": 6.1,
        "year": "2010"
      }
    ]
  }
  ```

### 3. Pencarian Film / Serial
- **Endpoint:** `GET /film/search?q=avengers&type=movie&page=1`
- **Query Parameter:**
  - `q`: Kata kunci pencarian (judul)
  - `type` *(opsional)*: `movie` (default) atau `tv`
  - `page` *(opsional)*: Nomor halaman (default: `1`)
- **Contoh Respon:**
  ```json
  {
    "page": 1,
    "totalPages": 5,
    "results": [...]
  }
  ```

### 4. Detail Film / Serial TV & Daftar Link Pemutar Video Embed
- **Endpoint:**
  - Film: `GET /film/movie/:id`
  - Serial TV: `GET /film/tv/:id?season=1&episode=1`
- **Contoh:** `http://localhost:3000/film/movie/35791`

### 5. Pencarian Subtitle (OpenSubtitles REST API v2)
- **Endpoint:**
  - Film: `GET /film/movie/:id/subtitles?lang=id,en`
  - Serial TV: `GET /film/tv/:id/subtitles?lang=id,en&season=1&episode=1`
- **Query Parameter:**
  - `lang` *(opsional)*: Kode bahasa dipisah koma (contoh: `id`, `en`, atau `id,en`. Default: `id,en`)
  - `season` *(opsional, untuk TV)*: Nomor musim
  - `episode` *(opsional, untuk TV)*: Nomor episode
- **Contoh Respon:**
  ```json
  {
    "total": 3,
    "subtitles": [
      {
        "id": "12345",
        "file_id": 987654,
        "language": "id",
        "release": "Resident.Evil.Afterlife.2010.1080p.BluRay",
        "file_name": "Resident.Evil.Afterlife.2010.id.srt",
        "rating": 8.5,
        "download_count": 420,
        "download_url": "/subtitles/download/987654"
      }
    ]
  }
  ```

### 6. Unduh Langsung File Subtitle (.SRT)
- **Endpoint:** `GET /subtitles/download/:file_id`
- Server akan mengarahkan otomatis (*HTTP 307 Redirect*) ke link unduhan file `.srt` resmi berkecepatan tinggi dari OpenSubtitles CDN.
- **Contoh Respon:**
  ```json
  {
    "id": 35791,
    "type": "movie",
    "title": "Resident Evil: Afterlife",
    "overview": "Dalam dunia yang dirusak oleh infeksi virus...",
    "genres": ["Action", "Horror", "Science Fiction"],
    "poster": "https://image.tmdb.org/t/p/w500/abc.jpg",
    "backdrop": "https://image.tmdb.org/t/p/w1280/backdrop.jpg",
    "trailer": "https://www.youtube-nocookie.com/embed/XXXXX",
    "servers": [
      { "name": "VidLink", "url": "https://vidlink.pro/movie/35791" },
      { "name": "Videasy", "url": "https://player.videasy.net/movie/35791" },
      { "name": "VidFast", "url": "https://vidfast.pro/movie/35791" },
      { "name": "VidSrc", "url": "https://vidsrc.to/embed/movie/35791" },
      { "name": "2Embed", "url": "https://www.2embed.cc/embed/35791" },
      { "name": "MultiEmbed", "url": "https://multiembed.mov/?video_id=35791&tmdb=1" },
      { "name": "AutoEmbed", "url": "https://autoembed.co/movie/tmdb/35791" }
    ]
  }
  ```

---

## 📺 Antarmuka Pengujian (Web Player)

Buka browser dan kunjungi:
👉 `http://localhost:3000` atau `http://localhost:3000/player`

Antarmuka ini sudah terintegrasi langsung dengan API Anda untuk:
- Memilih dan menonton film langsung dengan pemutar video responsif 16:9.
- Tombol switch server (*VidLink, Videasy, VidFast, VidSrc, 2Embed, MultiEmbed, AutoEmbed*) secara langsung jika ada server yang diblokir atau buffering.
- Pencarian judul film secara dinamis.
- Menjelajahi film trending pekan ini.

---

## 🧪 Menjalankan Pengujian (Unit Tests)

Jalankan test bawaan untuk memastikan mapping server URL dan serialisasi data berjalan sempurna:

```bash
cargo test
```

---

## ⚠️ Atribusi & Lisensi TMDB

*This product uses the TMDB API but is not endorsed or certified by TMDB.*
Proyek ini dibuat untuk tujuan pembelajaran arsitektur API dan integrasi multimedia.
