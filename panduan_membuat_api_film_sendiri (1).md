# Panduan Membuat API Film Sendiri (TMDB + Player Embed)

> **Penulis:** @Someone  
> **Tanggal Rilis:** 4 Oktober 2026  

---

## 📌 Gambaran Singkat

Membangun API film sendiri pada dasarnya hanya memerlukan integrasi dari dua komponen utama:
1. **TMDB (The Movie Database):** Berfungsi sebagai penyedia metadata film (judul, poster, sinopsis, genre, rating, musim/episode, serta trailer YouTube). TMDB sama sekali tidak menyediakan file video.
2. **Player Embed Pihak Ketiga:** Berfungsi sebagai pemutar video film/serial langsung di web.

Kunci integrasinya terletak pada **ID TMDB**. Setiap judul memiliki ID unik berbasis angka (contoh: *Resident Evil: Afterlife* = `35791`). ID ini digunakan untuk mengambil data dari TMDB sekaligus diarahkan ke URL player embed sehingga datanya sinkron secara otomatis.

| Komponen | Sumber | Butuh API Key? |
| :--- | :--- | :--- |
| **Metadata Film/Serial** (Judul, poster, sinopsis, genre, rating, season/episode) | [TMDB API](https://developer.themoviedb.org) | **Ya** (Gratis) |
| **Trailer** | YouTube (ID diperoleh dari TMDB) | **Tidak** |
| **Pemutar Video Film** | Player Embed Pihak Ketiga (VidLink, Videasy, dll.) | **Tidak** |
| **Subtitle Tambahan** *(Opsional, untuk custom player/stream langsung)* | [OpenSubtitles API](https://www.opensubtitles.com) | **Ya** (Gratis) |

---

## 🚀 Langkah 1 — Ambil API Key TMDB

Setiap pengembang wajib menggunakan API key miliknya sendiri demi keamanan dan pembatasan kuota (*rate limit*).

1. Buka [themoviedb.org](https://www.themoviedb.org/) dan daftarkan akun baru, lalu verifikasi email Anda.
2. Masuk ke **Settings → API** ([themoviedb.org/settings/api](https://www.themoviedb.org/settings/api)).
3. Klik **Create** / **Request an API key**, lalu pilih kategori **Developer**.
4. Isi formulir pendaftaran singkat (nama aplikasi, URL, dan deskripsi penggunaan personal/non-komersial).
5. Salin **API Key (v3 auth)** yang terdiri dari 32 karakter alfanumerik.
6. Simpan key di dalam file lingkungan `.env` server:

```env
TMDB_API_KEY=isi_key_kamu_di_sini
```

### Uji Coba Cepat via Terminal
Jalankan perintah berikut di terminal untuk memastikan API key aktif:

```bash
curl "https://api.themoviedb.org/3/movie/35791?api_key=ISI_KEY&language=id-ID"
```
*Jika respon menghasilkan objek JSON berisi data film, key Anda telah berhasil diaktifkan.*

---

## 📡 Langkah 2 — Endpoint TMDB yang Digunakan

Semua pemanggilan dasar menggunakan basis URL:  
`https://api.themoviedb.org/3` ditambah query parameter `?api_key=KEY&language=id-ID` (fallback ke `en-US` jika terjemahan Indonesia kosong).  
Ganti segmen `/movie` dengan `/tv` untuk konten serial.

| Kebutuhan | Endpoint | Keterangan Data |
| :--- | :--- | :--- |
| **Film Trending** | `/trending/movie/week` | Daftar `results[]`: `id`, `title`, `poster_path`, `vote_average` |
| **Film Populer / Terbaru** | `/movie/popular?page=1`<br>`/movie/now_playing?page=1` | Data serupa, dilengkapi `total_pages` untuk paginasi |
| **Pencarian Judul** | `/search/movie?query=resident%20evil` | Hasil pencarian berdasarkan kata kunci |
| **Detail Film** | `/movie/35791?append_to_response=credits,videos` | `title`, `overview`, `genres`, `runtime`, `backdrop_path`, cast, dan trailer |
| **Detail Serial TV** | `/tv/1399` | `name`, daftar `seasons[]` (nomor musim & jumlah episode) |
| **Episode per Season** | `/tv/1399/season/1` | Daftar `episodes[]`: `episode_number`, `name` |

### Penanganan Aset Gambar & Trailer
* **Gambar:** TMDB hanya menyediakan path berkas (contoh: `/abc123.jpg`). Gabungkan dengan CDN URL:
  * Poster: `https://image.tmdb.org/t/p/w500` + `poster_path`
  * Backdrop (Latar Lebar): `https://image.tmdb.org/t/p/w1280` + `backdrop_path`
* **Trailer:** Filter dari array `videos.results[]` di mana `site === "YouTube"` dan `type === "Trailer"`. Prioritaskan item dengan `official: true`. Putar trailer menggunakan URL iframe aman:  
  `https://www.youtube-nocookie.com/embed/{key}`

> Dokumentasi referensi lengkap: [developer.themoviedb.org](https://developer.themoviedb.org/)

---

## 🎬 Langkah 3 — Daftar Alamat Player Embed

Penyedia stream video berikut langsung menerima query berbasis ID TMDB tanpa memerlukan registrasi atau token otentikasi.

* **Format Variabel:**
  * `{id}` : ID TMDB film/serial
  * `{s}`  : Nomor Season (Musim)
  * `{e}`  : Nomor Episode

| Provider | Pola URL Film | Pola URL Serial TV |
| :--- | :--- | :--- |
| **VidLink** | `https://vidlink.pro/movie/{id}` | `https://vidlink.pro/tv/{id}/{s}/{e}` |
| **Videasy** | `https://player.videasy.net/movie/{id}` | `https://player.videasy.net/tv/{id}/{s}/{e}` |
| **VidFast** | `https://vidfast.pro/movie/{id}` | `https://vidfast.pro/tv/{id}/{s}/{e}` |
| **VidSrc** | `https://vidsrc.to/embed/movie/{id}` | `https://vidsrc.to/embed/tv/{id}/{s}/{e}` |
| **2Embed** | `https://www.2embed.cc/embed/{id}` | `https://www.2embed.cc/embedtv/{id}&s={s}&e={e}` |
| **MultiEmbed** | `https://multiembed.mov/?video_id={id}&tmdb=1` | `https://multiembed.mov/?video_id={id}&tmdb=1&s={s}&e={e}` |
| **AutoEmbed** | `https://autoembed.co/movie/tmdb/{id}` | `https://autoembed.co/tv/tmdb/{id}-{s}-{e}` |

> ⚠️ **Catatan Status Akses:**  
> Daftar di atas terverifikasi dapat diakses dari jaringan server Indonesia per **Oktober 2026**. Domain alternatif seperti `vidsrc.cc`, `vidsrc.xyz`, `embed.su`, `moviesapi.club`, dan `111movies.com` sering diblokir ISP/Trust Positif. Sediakan beberapa server cadangan pada UI pemutar Anda.

---

## 🛠️ Langkah 4 — Membuat Backend API Sendiri (Node.js + Express)

Backend ini bertindak sebagai reverse proxy: mengamankan TMDB API Key di server, menyaring payload JSON agar ringkas, dan menyisipkan daftar link player embed langsung ke respons client.

### Persyaratan
* Node.js v18.0.0 atau yang lebih baru.

### 1. Inisialisasi Proyek
```bash
mkdir film-api && cd film-api
npm init -y
npm install express dotenv
```

### 2. Konfigurasi Lingkungan (`.env`)
```env
TMDB_API_KEY=isi_key_kamu_di_sini
PORT=3000
```

### 3. Kode Server (`index.js`)
```javascript
require('dotenv').config();
const express = require('express');
const app = express();

// Izinkan CORS agar API dapat diakses frontend web
app.use((req, res, next) => {
  res.set('Access-Control-Allow-Origin', '*');
  next();
});

const TMDB = 'https://api.themoviedb.org/3';
const IMG = 'https://image.tmdb.org/t/p/';

// Helper request TMDB
async function tmdb(path, params = {}) {
  const query = new URLSearchParams({
    api_key: process.env.TMDB_API_KEY,
    language: 'id-ID',
    ...params,
  });
  const res = await fetch(`${TMDB}${path}?${query}`);
  if (!res.ok) throw new Error(`TMDB Error: ${res.status}`);
  return res.json();
}

// Koleksi generator player embed
function servers(type, id, s = 1, e = 1) {
  const tv = type === 'tv';
  return [
    { name: 'VidLink',    url: tv ? `https://vidlink.pro/tv/${id}/${s}/${e}` : `https://vidlink.pro/movie/${id}` },
    { name: 'Videasy',    url: tv ? `https://player.videasy.net/tv/${id}/${s}/${e}` : `https://player.videasy.net/movie/${id}` },
    { name: 'VidFast',    url: tv ? `https://vidfast.pro/tv/${id}/${s}/${e}` : `https://vidfast.pro/movie/${id}` },
    { name: 'VidSrc',     url: tv ? `https://vidsrc.to/embed/tv/${id}/${s}/${e}` : `https://vidsrc.to/embed/movie/${id}` },
    { name: '2Embed',     url: tv ? `https://www.2embed.cc/embedtv/${id}&s=${s}&e=${e}` : `https://www.2embed.cc/embed/${id}` },
    { name: 'MultiEmbed', url: `https://multiembed.mov/?video_id=${id}&tmdb=1` + (tv ? `&s=${s}&e=${e}` : '') },
    { name: 'AutoEmbed',  url: tv ? `https://autoembed.co/tv/tmdb/${id}-${s}-${e}` : `https://autoembed.co/movie/tmdb/${id}` },
  ];
}

// Normalisasi item film untuk card UI
const card = (m, type) => ({
  id: m.id,
  type,
  title: m.title || m.name,
  poster: m.poster_path ? `${IMG}w342${m.poster_path}` : null,
  rating: m.vote_average,
  year: (m.release_date || m.first_air_date || '').slice(0, 4),
});

// GET /film/trending?type=movie|tv
app.get('/film/trending', async (req, res) => {
  const type = req.query.type === 'tv' ? 'tv' : 'movie';
  const data = await tmdb(`/trending/${type}/week`);
  res.json({ results: data.results.map((m) => card(m, type)) });
});

// GET /film/search?q=judul&type=movie|tv&page=1
app.get('/film/search', async (req, res) => {
  const type = req.query.type === 'tv' ? 'tv' : 'movie';
  const data = await tmdb(`/search/${type}`, {
    query: req.query.q || '',
    page: req.query.page || 1,
  });
  res.json({
    page: data.page,
    totalPages: data.total_pages,
    results: data.results.map((m) => card(m, type)),
  });
});

// GET /film/movie/:id atau /film/tv/:id?season=1&episode=1
app.get('/film/:type/:id', async (req, res) => {
  const { type, id } = req.params;
  if (!['movie', 'tv'].includes(type)) {
    return res.status(404).json({ error: 'Parameter type harus movie atau tv' });
  }

  const s = Number(req.query.season || 1);
  const e = Number(req.query.episode || 1);

  const d = await tmdb(`/${type}/${id}`, { append_to_response: 'videos' });
  const yt = (d.videos?.results || []).filter((v) => v.site === 'YouTube');
  const trailer = yt.find((v) => v.type === 'Trailer' && v.official) || yt.find((v) => v.type === 'Trailer') || yt[0];

  res.json({
    id: d.id,
    type,
    title: d.title || d.name,
    overview: d.overview,
    genres: (d.genres || []).map((g) => g.name),
    poster: d.poster_path ? `${IMG}w500${d.poster_path}` : null,
    backdrop: d.backdrop_path ? `${IMG}w1280${d.backdrop_path}` : null,
    trailer: trailer ? `https://www.youtube-nocookie.com/embed/${trailer.key}` : null,
    seasons: type === 'tv' ? d.seasons.map((x) => ({ season: x.season_number, episodes: x.episode_count })) : undefined,
    servers: servers(type, id, s, e),
  });
});

// Error handling & server start
app.use((err, req, res, next) => res.status(502).json({ error: err.message }));

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`film-api running on port ${PORT}`));
```

### 4. Eksekusi dan Pengujian
```bash
node index.js
```
Uji endpoint di tab terminal lain:
```bash
# Cek data detail film + server player
curl http://localhost:3000/film/movie/35791

# Cek fungsi pencarian
curl "http://localhost:3000/film/search?q=avengers"
```

---

## 🖥️ Langkah 5 — Menampilkan Pemutar di Halaman Frontend

Contoh implementasi HTML dan JavaScript murni untuk menampilkan pemutar video responsif beserta panel tombol ganti server:

```html
<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <title>Movie Player</title>
  <style>
    .player-container {
      width: 100%;
      max-width: 960px;
      margin: 0 auto;
      aspect-ratio: 16 / 9;
      background-color: #000;
    }
    iframe {
      width: 100%;
      height: 100%;
      border: 0;
    }
    .server-list {
      margin-top: 12px;
      display: flex;
      gap: 8px;
      justify-content: center;
      flex-wrap: wrap;
    }
    .server-btn {
      padding: 6px 14px;
      cursor: pointer;
      background: #222;
      color: #fff;
      border: 1px solid #444;
      border-radius: 4px;
    }
    .server-btn:hover {
      background: #444;
    }
  </style>
</head>
<body>

  <div class="player-container">
    <iframe id="player" 
            allowfullscreen 
            allow="autoplay; encrypted-media; fullscreen; picture-in-picture">
    </iframe>
  </div>

  <div class="server-list" id="servers"></div>

  <script>
    fetch('http://localhost:3000/film/movie/35791')
      .then(res => res.json())
      .then(film => {
        const player = document.getElementById('player');
        const serverContainer = document.getElementById('servers');

        // Pasang server default (server pertama)
        if (film.servers && film.servers.length > 0) {
          player.src = film.servers[0].url;

          // Render tombol switch server
          film.servers.forEach(sv => {
            const btn = document.createElement('button');
            btn.className = 'server-btn';
            btn.textContent = sv.name;
            btn.onclick = () => {
              player.src = sv.url;
            };
            serverContainer.appendChild(btn);
          });
        }
      })
      .catch(err => console.error('Gagal memuat film:', err));
  </script>
</body>
</html>
```

---

## ⚙️ Alternatif Lanjutan: Stream Langsung (.m3u8) & Subtitle Kustom

Player embed bawaan pihak ketiga biasanya menyisipkan iklan banner atau pop-up. Jika Anda menginginkan antarmuka bersih tanpa iklan menggunakan player sendiri (seperti **Plyr** atau **Video.js** + **hls.js**):

1. **Scraper Stream Direct (`.m3u8` / `.mp4`):**
   * Gunakan proyek seperti [tmdb-embed-api (GitHub)](https://github.com/Inside4ndroid/TMDB-Embed-API).
   * Host proyek ini di VPS pribadi (default port `8787`).
   * Endpoint ekstraksi: `/api/streams/movie/{id}` atau `/api/streams/series/{id}?season=1&episode=1`.
2. **Subtitle Kustom:**
   * Integrasikan dengan **OpenSubtitles API** ([opensubtitles.com](https://www.opensubtitles.com)).
   * Ambil subtitle format `.srt` berdasarkan `tmdb_id`, konversikan ke `.vtt`, lalu pasang via tag `<track kind="subtitles">` di elemen `<video>`.

---

## ⚠️ Hal Penting & Kebijakan

* **Pop-up & Iklan Pihak Ketiga:** Konten di dalam iframe dikendalikan oleh penyedia embed. Berikan petunjuk kepada pengguna untuk berpindah server bila tautan rusak atau iklan mengganggu.
* **Resiliensi Domain & ISP:** Domain player ilegal sering terkena pemblokiran ISP. Pastikan backend Anda fleksibel untuk mengganti domain tanpa perlu merombak struktur frontend.
* **Keamanan Kunci TMDB:** Jangan pernah membagikan atau mem-push `TMDB_API_KEY` ke repository publik (seperti GitHub). Gunakan variabel environment (`.env`).
* **Batas Akses TMDB (Rate Limit):** Batas standar sekitar 50 request/detik per IP. Terapkan in-memory cache (misal Redis atau cache internal Express) selama ~10 menit untuk list dan ~24 jam untuk detail metadata.
* **Atribusi TMDB:** Wajib menyertakan logo atau pernyataan atribusi resmi TMDB di aplikasi Anda:  
  *"This product uses the TMDB API but is not endorsed or certified by TMDB."*
* **Kepatuhan Hukum & Hak Cipta:** Penyedia embed pihak ketiga meng-host konten tanpa izin resmi. Proyek arsitektur ini ditujukan khusus untuk keperluan pembelajaran dan riset teknis API.