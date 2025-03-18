# Changelog

Semua perubahan penting pada proyek ini akan didokumentasikan di file ini.

Format ini berdasarkan [Keep a Changelog](https://keepachangelog.com/id/1.0.0/),
dan proyek ini mengikuti [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Struktur dasar aplikasi backend dengan Axum dan SQLx
- Implementasi REST API untuk autentikasi, pengguna dan pesan
- Dukungan WebSocket untuk komunikasi real-time
- Dukungan CI/CD dengan GitHub Actions
- Containerization dengan Docker

### Changed
- Konfigurasi CI/CD untuk deployment via SSH dengan password
- Penambahan systemd service setup otomatis

## [0.2.0] - 2025-12-10

### Added
- Test suite komprehensif untuk model dan autentikasi
- Implementasi unit test untuk model User
- Implementasi unit test untuk model Message
- Implementasi unit test untuk autentikasi JWT
- Implementasi test validasi untuk handler auth
- Integrasi test coverage dengan Codecov
- Pemisahan kode ke dalam lib.rs dan app.rs untuk memudahkan testing

### Changed
- Restrukturisasi workflow CI/CD untuk menjalankan linting, unit test, dan integrasi test secara terpisah
- Perbaikan dokumentasi testing di README.md

### Fixed
- Berbagai bug pada model User dan Message
- Masalah validasi pada handler registrasi
- Masalah pada proses autentikasi JWT

## [0.1.0] - 2025-11-15

### Added
- Implementasi dasar REST API dengan Axum
- Sistem autentikasi dengan JWT
- Model database dengan SQLx
- Konfigurasi dasar untuk CI/CD 