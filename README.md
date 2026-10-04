# pasaree-backend-service

Layanan backend Marketplace Pasaree (Rust). Divisi Pasaree (Marketplace), org Coding-Skuy. Template Opsi A.

Rujukan utama: [Pasaree-TownHall](https://github.com/Coding-Skuy/Pasaree-TownHall) — baca `katalog/10-model-data.md`, `produk/10-alur-beli.md`, `produk/20-kontrak-api-KMP-mobile.md`, `produk/21-kontrak-api-web-bun.md`, `keuangan/10-komisi-transparan.md`.

## Cakupan

- Katalog: lapak, produk, varian, media.
- Lapak: pendaftaran, kurasi, status tayang.
- Transaksi: keranjang, checkout multi-lapak, bayar idempoten, escrow, retur.
- Proksi baca ke Lumbung untuk trip dan payout. Tidak menyimpan armada, gudang, atau kas sendiri.

## Basis Data

- Postgres terpisah bernama `pasaree`. Tidak berbagi basis data dengan divisi lain.
- Contoh: `postgres://pasaree:rahasia@localhost:5432/pasaree`
- Migrasi ada di `migrations/`. Jalankan dengan `sqlx migrate run`.

## Autentikasi

- JWT dengan klaim `aud` wajib `pasaree`. Token tanpa audiens ini ditolak.
- Lihat `docs/auth.md` untuk contoh klaim dan aturan validasi.
- Lihat `docs/runbook.md` untuk operasional harian.

## Mulai Cepat

1. Pasang Rust stabil dan `sqlx-cli`.
2. Salin `.env.example` menjadi `.env` dan isi `DATABASE_URL` basis data `pasaree`.
3. Jalankan `cargo run`.
4. Uji sehat: `curl http://localhost:8101/kesehatan`.
