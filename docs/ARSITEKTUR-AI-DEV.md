# Arsitektur AI-dev pasaree-backend-service

Dokumen tambahan untuk `versions/v1.0.0/docs/40-TECHSTACK.md` dan `60-BLAST-RADIUS.md` (kedua file itu beku v1.0.0 dan hanya berisi ringkasan; jangan ditimpa). Rujukan versi: [Pasaree-TownHall v1.0.0](https://github.com/Coding-Skuy/Pasaree-TownHall/blob/main/platform/versions/v1.0.0/CHANGELOG.md).

## Matriks stack dan fitur

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

| Modul | Tanggung jawab | Rujukan |
|---|---|---|
| auth | login, refresh, peran produsen/operator/kurator/admin/pembeli | `docs/auth.md` |
| katalog | produk, varian, media, riwayat harga, status tayang | BR-201..BR-206 |
| lapak | pendaftaran, kurasi, status AKTIF/BEKU/TUTUP | piagam kurasi |
| transaksi | keranjang, checkout, pesanan, bayar, retur | `produk/10-alur-beli.md` |
| keuangan | komisi, escrow, payout, rekonsiliasi, riwayat tarif | BR-301..BR-306 |
| proksi Lumbung | trip (cache 60 dtk), payout | BR-205, BR-303 |

## Blast radius

Batas context repo (townhall-roadmap §2: komisi 8% transparan, escrow T+1; katalog jujur; fulfillment wajib via trip Lumbung).

Boleh disentuh:

- DB `pasaree`, modul katalog/lapak/transaksi/keuangan, endpoint `/v1/*`, proksi baca ke Lumbung.
- Menerbitkan event katalog/transaksi ke `pasaree-data-pipeline`.

Dilarang:

- DB, kas, armada, atau gudang divisi lain.
- Membuat trip sendiri atau menentukan ongkir (milik Lumbung).
- Membebankan komisi ke pembeli; mengubah kontrak tanpa bump versi + ADR.
- Menyimpan data kartu/rekening mentah (rekening payout tersensor, dikelola via skema Lumbung).

Kontrak dikonsumsi:

- API Lumbung: trip (`trip_id, jadwal_berangkat, rute, titik_serah, kapasitas, ongkir`), eksekusi payout, data rekonsiliasi.

Kontrak diproduksi:

- REST Pasaree v1 untuk KMP dan web.
- Event katalog/transaksi ke data-pipeline (`transaksi.dibuat`, `lapak.status_berubah`, `logistik.biaya_diperbarui`, `vendor.status_berubah` pada registry Titeny).
- Data rekonsiliasi harian ke Lumbung.

Catatan: Pasaree tidak mengonsumsi modul `:shared:pantry-resep` (milik Pawonee). Failure utama: kontrak berubah tanpa update SRS, test tidak mengikuti acceptance criteria, rollback tidak siap. Rollback: revert commit repo ini dan kembali ke kontrak Pasaree-TownHall v1.0.0.
