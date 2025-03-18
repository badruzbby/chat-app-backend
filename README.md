# 🚀 Backend Aplikasi Chat Real-time dengan Rust

![Rust](https://img.shields.io/badge/Rust-1.60+-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8.1-blue.svg)
![SQLx](https://img.shields.io/badge/SQLx-0.8.3-green.svg)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)

Backend aplikasi chat real-time yang dibangun dengan teknologi modern dan performa tinggi menggunakan **Rust**, **Axum**, **WebSockets**, dan **SQLx**.

## 📋 Daftar Isi

- [Fitur](#-fitur)
- [Teknologi](#-teknologi)  
- [Persyaratan](#-persyaratan)
- [Cara Menjalankan](#-cara-menjalankan)
- [Struktur Project](#-struktur-project)
- [API Endpoints](#-api-endpoints)
- [Lingkungan Pengembangan](#-lingkungan-pengembangan)
- [Lisensi](#-lisensi)

## ✨ Fitur

- ✅ **Autentikasi** dengan JWT (JSON Web Token)
- ✅ **REST API** untuk operasi CRUD
- ✅ **WebSocket** untuk komunikasi real-time
- ✅ Mendukung **pesan pribadi** dan **publik**
- ✅ Melacak **status online pengguna** secara real-time
- ✅ Mendukung multiple database (**SQLite** dan **PostgreSQL**)
- ✅ **CORS middleware** untuk integrasi dengan frontend

## 🔧 Teknologi

- **Rust** - Bahasa pemrograman dengan performa tinggi dan keamanan memori
- **Axum** - Framework web yang cepat dan ergonomis
- **SQLx** - Library database async yang type-safe dengan validasi compile-time
- **PostgreSQL/SQLite** - Database relasional untuk penyimpanan persisten
- **JWT** - JSON Web Token untuk autentikasi aman
- **WebSockets** - Protokol komunikasi real-time dua arah

## 📦 Persyaratan

- **Rust** 1.60 atau lebih baru
- **SQLite** atau **PostgreSQL** (opsional)
- **Cargo** (package manager Rust)

## 🚀 Cara Menjalankan

1. **Clone repository ini**

   ```bash
   git clone https://github.com/badruzbby/chat-app-backend.git
   ```

2. **Masuk ke direktori project**

   ```bash
   cd chat-app-backend
   ```

3. **Salin `.env.example` ke `.env` dan sesuaikan pengaturan** (opsional)

   ```bash
   cp .env.example .env
   ```

   Konfigurasi database dan pengaturan lainnya dalam file `.env`

4. **Bangun dan jalankan aplikasi**

   ```bash
   cargo run
   ```

Server akan berjalan di `http://127.0.0.1:8080` (atau sesuai konfigurasi PORT di `.env`).

## 📁 Struktur Project

```
src/
├── config/          # Konfigurasi aplikasi dan environment
├── handlers/        # Handler HTTP dan WebSocket
├── middleware/      # Middleware (auth, dll)
├── models/          # Model data dan logika bisnis
├── routes/          # Definisi routes API
└── utils/           # Utility functions
migrations/          # Migrasi database SQLx
```

## 🔌 API Endpoints

### Auth

| Endpoint | Metode | Deskripsi |
|----------|--------|-----------|
| `/auth/register` | POST | Registrasi pengguna baru |
| `/auth/login` | POST | Login pengguna |

### Users

| Endpoint | Metode | Deskripsi |
|----------|--------|-----------|
| `/users/me` | GET | Mendapatkan profil pengguna saat ini |
| `/users/online` | GET | Mendapatkan daftar pengguna online |
| `/users/status` | POST | Memperbarui status online |

### Messages

| Endpoint | Metode | Deskripsi |
|----------|--------|-----------|
| `/messages` | POST | Mengirim pesan |
| `/messages/public` | GET | Mendapatkan pesan publik |
| `/messages/{receiver_id}` | GET | Mendapatkan pesan antara dua pengguna |

### WebSocket

| Endpoint | Deskripsi |
|----------|-----------|
| `/ws?token={jwt_token}` | Koneksi WebSocket untuk komunikasi real-time |

## ⚙️ Lingkungan Pengembangan

Aplikasi ini menggunakan file `.env` untuk konfigurasi. Variabel-variabel lingkungan yang tersedia:

```
PORT=8080                                               # Port server
HOST=127.0.0.1                                          # Host server
DATABASE_URL=postgres://postgres:postgres@localhost/db  # URL database
JWT_SECRET=super_secret_key                             # Secret untuk JWT
JWT_EXPIRATION=86400                                    # Waktu kadaluarsa token (detik)
```

## 📝 Lisensi

[MIT](LICENSE) © [Muhammad Badruz Zaman] 