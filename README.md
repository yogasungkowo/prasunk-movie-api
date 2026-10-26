# Prasunk Movie & Subtitles API With Rust

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Framework-Axum%200.8-blue.svg?style=for-the-badge&logo=rust" alt="Axum" />
  <img src="https://img.shields.io/badge/Async%20Runtime-Tokio-lightgrey.svg?style=for-the-badge&logo=tokio" alt="Tokio" />
  <img src="https://img.shields.io/badge/Data%20Source-TMDB%20%2B%20OpenSubtitles-yellow.svg?style=for-the-badge" alt="Source" />
  <img src="https://img.shields.io/badge/Author-yogasungkowo-purple.svg?style=for-the-badge" alt="Author" />
  <img src="https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge" alt="License" />
</p>

High-Performance RESTful API Film & Serial TV yang dibangun menggunakan bahasa pemrograman **Rust** dan framework **Axum 0.8**. Berfungsi sebagai *reverse proxy* cerdas yang menyinkronkan metadata dari **TMDB (The Movie Database)**, menyisipkan **7 provider pemutar video embed** siap pakai, serta terintegrasi langsung dengan **OpenSubtitles REST API v2** untuk pencarian dan pengunduhan subtitle bahasa Indonesia & global.

Dibuat & dikembangkan oleh **[@yogasungkowo](https://github.com/yogasungkowo)**.

---

## ⚡ Fitur Utama

- **🚀 Ultra-Fast & Low Memory**: Memanfaatkan asynchronous runtime **Tokio** dan pipeline serialisasi **Serde zero-copy** dengan konsumsi memori sangat minimal (~10–25 MB RAM).
- **🔒 Secure Reverse Proxy**: Mengamankan `TMDB_API_KEY` dan `OPENSUBTITLES_API_KEY` di server tanpa membocorkannya ke sisi frontend / client.
- **🎬 7 Multi-Server Video Embed**: Otomatis menghasilkan tautan stream iframe dari 7 server cadangan (*VidLink, Videasy, VidFast, VidSrc, 2Embed, MultiEmbed, AutoEmbed*) untuk film maupun episode serial TV.
- **💬 Integrasi OpenSubtitles REST API v2**: Pencarian subtitle akurat berbasis `tmdb_id` dengan filter bahasa (`id`, `en`, dll.), metadata uploader, dan rating.
- **📥 Direct Subtitle Download**: Endpoint redirector instan (`HTTP 307`) yang langsung mengunduh file `.srt` resmi berkecepatan tinggi dari CDN OpenSubtitles.
- **🎞️ Trailer YouTube & Metadata Lengkap**: Menyaring trailer resmi YouTube (`youtube-nocookie.com/embed/{key}`), poster resolusi tinggi, backdrop, sinopsis, dan daftar season serial.
- **🌐 Built-in Permissive CORS & Tracing**: Dilengkapi dengan middleware CORS fleksibel dan request tracing logger untuk pemantauan server yang mudah.
- **🖥️ Built-in Interactive Web Player**: Dilengkapi antarmuka pemutar video responsif bawaan (dark mode) di `http://localhost:3000` untuk langsung mencoba menonton, mengganti server, dan mengunduh subtitle.
- **🧪 Unit Tested**: Memiliki suite pengujian unit bawaan untuk memvalidasi generator URL server player dan normalisasi data.

---

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (Edition 2024 / v1.80+)
- **Web Framework**: [Axum](https://docs.rs/axum) (v0.8)
- **Async Runtime**: [Tokio](https://tokio.rs/) (v1.x full)
- **HTTP Client**: [Reqwest](https://docs.rs/reqwest) (v0.12)
- **Serialization**: [Serde](https://serde.rs/) & [serde_json](https://docs.rs/serde_json)
- **Middleware**: [Tower](https://docs.rs/tower) & [Tower-HTTP](https://docs.rs/tower-http) (CORS, Trace)
- **Environment**: [Dotenvy](https://docs.rs/dotenvy)
- **Logging**: [Tracing](https://docs.rs/tracing) & [Tracing-Subscriber](https://docs.rs/tracing-subscriber)

---

## 📡 Daftar Endpoint API

| Method | Endpoint | Deskripsi |
|---|---|---|
| `GET` | `/` | Antarmuka Web Player interaktif & API tester bawaan |
| `GET` | `/player` | Alias antarmuka Web Player |
| `GET` | `/v1/api/health` | Status kesehatan server dan cek konfigurasi API Key (TMDB & OpenSubtitles) |
| `GET` | `/v1/api/film/trending` | Daftar film atau serial TV yang sedang trending sepekan ini |
| `GET` | `/v1/api/film/search` | Pencarian film atau serial TV berdasarkan kata kunci judul |
| `GET` | `/v1/api/film/:type/:id` | Detail metadata film/serial, trailer YouTube, dan 7 tautan server video embed |
| `GET` | `/v1/api/film/:type/:id/subtitles` | Pencarian subtitle film/serial dari OpenSubtitles REST API v2 |
| `GET` | `/v1/api/subtitles/download/:file_id` | Unduh file `.srt` subtitle langsung (HTTP 307 Redirect ke CDN) |

---

## ⚙️ Parameter & Filter Options

### 1. Trending (`/v1/api/film/trending`)
| Parameter | Tipe | Default | Deskripsi / Nilai yang Didukung |
|---|---|---|---|
| `type` | String | `movie` | Kategori media: `movie` (film) atau `tv` (serial TV) |

### 2. Pencarian Judul (`/v1/api/film/search`)
| Parameter | Tipe | Default | Deskripsi / Nilai yang Didukung |
|---|---|---|---|
| `q` | String | *wajib* | Kata kunci judul film atau serial (contoh: `avengers`) |
| `type` | String | `movie` | Kategori media: `movie` atau `tv` |
| `page` | Integer | `1` | Nomor halaman paginasi data |

### 3. Detail & Embed Player (`/v1/api/film/:type/:id`)
| Parameter | Tipe | Default | Deskripsi / Nilai yang Didukung |
|---|---|---|---|
| `:type` *(Path)* | String | *wajib* | `movie` atau `tv` |
| `:id` *(Path)* | Integer | *wajib* | ID TMDB unik (contoh: `35791` untuk *Resident Evil: Afterlife*) |
| `season` *(Query)* | Integer | `1` | Nomor Season (khusus kategori `tv`) |
| `episode` *(Query)* | Integer | `1` | Nomor Episode (khusus kategori `tv`) |

### 4. Pencarian Subtitle (`/v1/api/film/:type/:id/subtitles`)
| Parameter | Tipe | Default | Deskripsi / Nilai yang Didukung |
|---|---|---|---|
| `lang` | String | `id,en` | Kode bahasa subtitle dipisah koma (contoh: `id`, `en`, atau `id,en`) |
| `season` | Integer | `1` | Nomor Season serial TV |
| `episode` | Integer | `1` | Nomor Episode serial TV |

---

## 🚀 Cara Menjalankan (Getting Started)

### Persyaratan
- [Rust & Cargo](https://www.rust-lang.org/tools/install) (versi 1.80 atau yang lebih baru)
- **TMDB API Key (v3 auth)** dari [themoviedb.org/settings/api](https://www.themoviedb.org/settings/api) *(Gratis)*
- **OpenSubtitles API Key (Consumer)** dari [opensubtitles.com/en/consumers](https://www.opensubtitles.com/en/consumers) *(Gratis)*

### Instalasi & Konfigurasi

1. **Clone repository ini:**
   ```bash
   git clone https://github.com/yogasungkowo/prasunk-movie-api.git
   cd prasunk-movie-api
   ```

2. **Buat file lingkungan (`.env`):**
   Salin dari template `.env.example`:
   ```bash
   cp .env.example .env
   ```

3. **Isi API Key di dalam `.env`:**
   ```env
   # TMDB API Key (v3 auth)
   TMDB_API_KEY=masukkan_api_key_tmdb_anda

   # OpenSubtitles.com API Key (Consumer)
   OPENSUBTITLES_API_KEY=masukkan_api_key_opensubtitles_anda

   # Konfigurasi Server
   PORT=3000
   HOST=0.0.0.0
   ```

### Menjalankan Server

- **Mode Development:**
  ```bash
  cargo run
  ```
  *(Secara default server berjalan di `http://0.0.0.0:3000`)*

- **Menjalankan dengan custom port:**
  ```bash
  PORT=8080 cargo run
  ```

- **Build untuk performa optimal (Release):**
  ```bash
  cargo run --release
  ```

### Menjalankan Unit Test

Untuk memastikan seluruh logic generator server embed dan normalisasi kartu berjalan tanpa galat:

```bash
cargo test
```

---

## 📺 Web Player & Tester Bawaan

Buka browser Anda dan kunjungi:
👉 **`http://localhost:3000`**

Antarmuka web modern bawaan siap digunakan untuk:
- Memutar film dengan rasio layar 16:9 responsif.
- Mengganti server video (*VidLink, Videasy, VidFast, VidSrc, 2Embed, MultiEmbed, AutoEmbed*) secara instan dengan satu klik.
- Mencari dan mengunduh subtitle Bahasa Indonesia (🇮🇩) atau Inggris (🇬🇧).
- Menjelajahi katalog film trending mingguan secara langsung.

---

## 📋 Contoh Penggunaan (cURL)

```bash
# 1. Cek info status server & API keys
curl "http://localhost:3000/v1/api/health"

# 2. Mengambil daftar film trending
curl "http://localhost:3000/v1/api/film/trending?type=movie"

# 3. Mengambil serial TV trending
curl "http://localhost:3000/v1/api/film/trending?type=tv"

# 4. Pencarian judul film
curl "http://localhost:3000/v1/api/film/search?q=resident+evil&type=movie"

# 5. Detail film + 7 server pemutar embed
curl "http://localhost:3000/v1/api/film/movie/35791"

# 6. Detail serial TV Season 1 Episode 1
curl "http://localhost:3000/v1/api/film/tv/1399?season=1&episode=1"

# 7. Cari subtitle Indonesia & Inggris untuk film
curl "http://localhost:3000/v1/api/film/movie/35791/subtitles?lang=id,en"

# 8. Download file subtitle (.srt)
curl -L -O "http://localhost:3000/v1/api/subtitles/download/987654"
```

---

## 👨‍💻 Author

- **yogasungkowo**
  - GitHub: [@yogasungkowo](https://github.com/yogasungkowo)

---

## 📄 Lisensi

Didistribusikan di bawah lisensi [MIT](LICENSE). Silakan gunakan secara bebas untuk pengembangan proyek Anda.

---

## ⚠️ Disclaimer & Atribusi

- *This product uses the TMDB API but is not endorsed or certified by TMDB.*
- *This product uses the OpenSubtitles.com API for subtitle retrieval.*
- Proyek ini dikembangkan secara *open-source* untuk keperluan edukasi dan riset teknis integrasi API multimedia. Seluruh hak cipta konten metadata dan video pemutar adalah milik masing-masing penyedia layanan.
