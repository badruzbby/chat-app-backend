# 🚀 Backend Aplikasi Chat Real-time dengan Rust

![Rust](https://img.shields.io/badge/Rust-1.60+-orange.svg)
![Axum](https://img.shields.io/badge/Axum-0.8.1-blue.svg)
![SQLx](https://img.shields.io/badge/SQLx-0.8.3-green.svg)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)
![CI/CD](https://github.com/badruzbby/chat-app-backend/actions/workflows/rust-ci-cd.yml/badge.svg)
![Test Coverage](https://img.shields.io/codecov/c/github/badruzbby/chat-app-backend/main.svg)

Backend aplikasi chat real-time yang dibangun dengan teknologi modern dan performa tinggi menggunakan **Rust**, **Axum**, **WebSockets**, dan **SQLx**.

## 📋 Daftar Isi

- [Fitur](#-fitur)
- [Teknologi](#-teknologi)  
- [Persyaratan](#-persyaratan)
- [Cara Menjalankan](#-cara-menjalankan)
- [Struktur Project](#-struktur-project)
- [API Endpoints](#-api-endpoints)
- [Testing](#-testing)
- [CI/CD](#-cicd)
- [Docker](#-docker)
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

- **Rust** 1.80 atau lebih baru
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

## 🧪 Testing

Proyek ini dilengkapi dengan test suite komprehensif yang mencakup unit test untuk model dan autentikasi.

### Struktur Test

```
tests/
├── test_user_model.rs       # Test untuk model User
├── test_message_model.rs    # Test untuk model Message
├── test_auth.rs             # Test untuk autentikasi JWT
└── test_auth_handlers.rs    # Test untuk handler registrasi dan login
```

### Menjalankan Test

Untuk menjalankan semua test:

```bash
cargo test
```

Untuk menjalankan test spesifik:

```bash
# Menjalankan semua test dalam file tertentu
cargo test --test test_user_model

# Menjalankan test spesifik
cargo test test_user_creation
```

Untuk menampilkan output dari test:

```bash
cargo test -- --nocapture
```

### Jenis Test yang Diimplementasikan

1. **Test Model User**
   - Pembuatan user baru
   - Verifikasi password
   - Konversi user ke format response

2. **Test Autentikasi**
   - Pembuatan token JWT
   - Validasi token kedaluwarsa
   - Penanganan token dengan signature tidak valid

3. **Test Model Pesan**
   - Pembuatan pesan
   - Penanganan pesan publik
   - Konversi pesan ke format response

4. **Test Handler Auth**
   - Validasi input registrasi (username dan password)
   - Pembuatan user dari data registrasi
   - Verifikasi password untuk login

### Mocking Database

Untuk test yang memerlukan database, proyek ini menggunakan pendekatan berikut:

1. **Mode Debug**: Menggunakan implementasi model yang mengembalikan data dummy pada fungsi-fungsi tertentu
2. **Struktur Modular**: Pemisahan kode ke dalam lib.rs dan app.rs untuk memudahkan testing
3. **CI Pipeline**: Konfigurasi database test terpisah untuk continuous integration

## 🔄 CI/CD

Proyek ini menggunakan GitHub Actions untuk Continuous Integration dan Continuous Deployment dengan alur kerja berikut:

### Continuous Integration

1. **Linting & Format Check**
   - Memeriksa format kode dengan `cargo fmt`
   - Menjalankan analisis statis dengan `cargo clippy`

2. **Unit Testing**
   - Menjalankan semua unit test yang independen dari database
   - Memeriksa fungsionalitas model dan autentikasi

3. **Integration Testing**
   - Menjalankan database PostgreSQL di container
   - Menerapkan migrasi database
   - Menjalankan test yang membutuhkan akses database

4. **Code Coverage**
   - Menghasilkan laporan coverage menggunakan cargo-llvm-cov
   - Mengunggah hasil ke Codecov untuk visualisasi dan monitoring

5. **Build**
   - Mengkompilasi aplikasi dalam mode release
   - Menyimpan artifact untuk deployment

### SQLx Offline Mode

Untuk memungkinkan kompilasi di lingkungan CI/CD tanpa akses ke database, proyek ini menggunakan **Offline Mode** dari SQLx. Ini memungkinkan validasi query pada waktu kompilasi tanpa koneksi database aktif.

#### Cara Menggunakan SQLx Offline Mode:

1. **Membuat file sqlx-data.json**:

   ```bash
   # Pastikan database lokal Anda berjalan
   cargo install sqlx-cli
   cargo sqlx prepare -- --lib
   ```

2. **Commit file `sqlx-data.json` ke repositori**:

   File ini berisi metadata tentang query SQL Anda dan memungkinkan SQLx untuk memeriksa query tanpa koneksi database.

3. **Mengaktifkan offline mode di CI**:

   ```bash
   # Mengaktifkan mode offline menggunakan environment variable
   SQLX_OFFLINE=true cargo build
   ```

   Perhatikan bahwa kita tidak menggunakan feature `offline` karena versi SQLx yang kita gunakan (0.8.3) tidak 
   mendukung feature ini. Sebagai gantinya, kita menggunakan environment variable `SQLX_OFFLINE=true`.

Pastikan untuk memperbarui file `sqlx-data.json` setiap kali Anda mengubah query SQL dengan menjalankan `cargo sqlx prepare` kembali.

### Continuous Deployment

Deployment otomatis ke server melalui SSH saat perubahan di-push ke branch `main` atau `master`:

1. **SSH Deployment**
   - Terhubung ke server menggunakan SSH dengan password
   - Menyalin aplikasi yang sudah dikompilasi ke server
   - Mengkonfigurasi dan memulai layanan systemd

2. **Docker Build & Push (opsional)**
   - Membuat Docker image
   - Mengunggah ke Docker Hub dengan tag `latest`

Untuk mengaktifkan deployment, tambahkan secrets berikut di repositori GitHub Anda:
- `SSH_HOST` - Alamat host server untuk deployment
- `SSH_USERNAME` - Username untuk login SSH
- `SSH_PASSWORD` - Password untuk login SSH
- `SSH_PORT` - Port SSH (opsional, default 22)
- `DATABASE_URL` - URL database untuk aplikasi di server
- `JWT_SECRET` - Secret key untuk JWT di server
- `DOCKERHUB_USERNAME` - Username Docker Hub (opsional)
- `DOCKERHUB_TOKEN` - Token akses Docker Hub (opsional)
- `CODECOV_TOKEN` - Token untuk upload hasil coverage ke Codecov

## 🐳 Docker

Anda dapat menjalankan aplikasi ini menggunakan Docker:

```bash
docker pull badruzbby/chat-app-backend:latest
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:password@host:5432/db \
  -e JWT_SECRET=your_secret_key \
  badruzbby/chat-app-backend:latest
```

Atau build image secara lokal:

```bash
docker build -t chat-app-backend .
docker run -p 8080:8080 chat-app-backend
```

### Docker Compose (Direkomendasikan)

Cara termudah untuk menjalankan aplikasi beserta database PostgreSQL adalah menggunakan Docker Compose:

```bash
# Jalankan aplikasi dan database
docker-compose up -d

# Lihat logs
docker-compose logs -f

# Hentikan semua container
docker-compose down

# Hentikan dan hapus volume data (akan menghapus semua data database)
docker-compose down -v
```

Docker Compose akan secara otomatis:
1. Membangun image backend dari Dockerfile
2. Menjalankan database PostgreSQL
3. Mengkonfigurasi jaringan antar kontainer
4. Memetakan port yang diperlukan
5. Menyediakan volume persisten untuk data PostgreSQL

Variabel lingkungan yang diperlukan sudah dikonfigurasi dalam file `docker-compose.yml`.

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
