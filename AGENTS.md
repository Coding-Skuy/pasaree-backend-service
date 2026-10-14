# AGENTS.md — pasaree-backend-service

Layanan API Pasaree (Rust): katalog, lapak, kurasi, transaksi, komisi, escrow, dan rekonsiliasi dengan Lumbung. Repo ini bagian dari divisi Pasaree (Epic E2 Stabil di Hilir).

## Sumber kebenaran

| Dokumen | Tautan |
|---|---|
| BRD Katalog & Produk (BR-201..BR-206) | [BRD/20-katalog-produk.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/platform/versions/v1.0.0/BRD/20-katalog-produk.md) |
| BRD Keuangan & Komisi (BR-301..BR-306) | [BRD/30-keuangan-komisi.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/platform/versions/v1.0.0/BRD/30-keuangan-komisi.md) |
| FRD Fungsional | [FRD/10-fungsional.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/platform/versions/v1.0.0/FRD/10-fungsional.md) |
| FSD (alur, model data, kontrak) | [FSD/](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/platform/versions/v1.0.0/FSD/) |
| SRS (fungsional, non-fungsional, acceptance criteria QA) | [SRS/](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/platform/versions/v1.0.0/SRS/) |
| Alur beli (living) | [produk/10-alur-beli.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/produk/10-alur-beli.md) |
| Kontrak API mobile KMP (living) | [produk/20-kontrak-api-KMP-mobile.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/produk/20-kontrak-api-KMP-mobile.md) |
| Kontrak API web Bun (living) | [produk/21-kontrak-api-web-bun.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/produk/21-kontrak-api-web-bun.md) |
| Model data katalog (living) | [katalog/10-model-data.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/katalog/10-model-data.md) |
| Komisi transparan (living) | [keuangan/10-komisi-transparan.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/keuangan/10-komisi-transparan.md) |
| Piagam kurasi (living) | [lapak/00-piagam-kurasi.md](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/lapak/00-piagam-kurasi.md) |
| Konteks lintas divisi | [KONTEKS-LINTAS-DIVISI.md](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/KONTEKS-LINTAS-DIVISI.md) |
| Roadmap master | [townhall-roadmap.md](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/townhall-roadmap.md) |

Aturan: dilarang menduplikasi requirement di repo kode ini. Requirement berubah = PR di `Pasaree-TownHall` terlebih dahulu (dokumen `platform/versions/v1.0.0/` beku; perubahan masuk versi baru). Dokumen BRD/PRD/FRD/FSD/SRS v1.0.0 sebagian masih templat; sumber isi sah ada di folder living TownHall (`produk/`, `keuangan/`, `katalog/`, `lapak/`, `metrik/`) yang dirujuk di atas.

## Tech stack terkunci

| Aspek | Terkunci | Catatan |
|---|---|---|
| Bahasa/runtime | Rust (edition 2021) + Tokio | |
| Web | Axum 0.8 (path `{id}`) + tower-http (Trace + Timeout) | |
| Data | SQLx (Postgres) + `migrate!` | DB logis `pasaree`; migrasi di `migrations/` |
| State | `AppState { db: PgPool }` | |
| Error | satu `AppError: IntoResponse` | 400/401/404/409/422/500 |
| ID | uuid v7 untuk internal; ULID string di kontrak publik | sesuai `katalog/10-model-data.md` |
| Validasi | `validator` | |
| Auth | `jsonwebtoken` + `argon2` | akses 15 menit `aud=pasaree`; refresh 7 hari + reuse-detection; service key least-privilege ke Lumbung |
| Observabilitas | `tracing` | |
| Test | `#[sqlx::test]` | TDD merah-hijau |
| Port | 8101 | `GET /kesehatan` |

Acuan auth: `docs/auth.md` repo ini (pola yang sama dengan `lumbung-backend-service`). Larangan: berbagi DB dengan divisi lain; framework web selain Axum.

Perubahan versi crate besar (axum, sqlx) hanya lewat ADR baru.

## Aturan kerja AI

1. Baca `AGENTS.md` ini dan semua ADR di `docs/decisions/` sebelum menulis kode.
2. TDD merah-hijau: tulis test dari acceptance criteria SRS dulu; rilis diblokir bila QA gate gagal.
3. Kontrak API/event hanya diubah lewat bump versi + ADR baru.
4. Commit memakai prefiks `docs:`/`feat:`/`fix:` dan wajib memperbarui `CHANGELOG.md` repo.
5. Keputusan arsitektur baru = tambah ADR bernomor di `docs/decisions/`; jangan edit ADR lama (immutable, sama seperti aturan versi TownHall).
6. Dilarang menyentuh DB divisi lain (DB per service; Titeny read-only). DB milik repo ini: `pasaree`; JWT audiens `pasaree`.
7. Aturan uang Pasaree: Komisi 8% (paket-hampers 10%, lapak baru 5% selama 30 hari) dipotong dari payout lapak, BUKAN dari pembeli; biaya layanan Rp2.000 per checkout masuk operasional Pasaree; ongkir 100% ke kas trip Lumbung; escrow via kasir, payout dieksekusi Lumbung T+1 hari kerja; uang integer IDR, komisi `floor()`.
8. Dokumen v1.0.0 di `versions/v1.0.0/docs/` beku dan tidak ditimpa; tambahan arsitektur ada di `docs/ARSITEKTUR-AI-DEV.md`.

## Definisi selesai

Dari [townhall-roadmap.md](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/townhall-roadmap.md) §6:

- Dokumen versi lengkap (BRD/FRD/FSD/PRD/SRS) + `SNAPSHOT-ROADMAP.md` di TownHall.
- QA gate berbasis SRS lulus (TDD merah-hijau); rilis diblokir bila gagal.
- Kontrak API terbit; audit keamanan bersih (JWT audiens `pasaree` + DB per service).
- Bukti metrik lintas-divisi (KONTEKS §7) tercatat.
- CHANGELOG versi diperbarui; dokumen beku tidak diedit.
