# ADR-0001 — Fondasi teknis pasaree-backend-service

Status: Diterima | Tanggal: 2026-10-14

## Konteks

pasaree-backend-service adalah repo `backend-service` divisi Pasaree. Spesifikasi v1.0.0 dibekukan 2026-10-07 di Pasaree-TownHall (`platform/versions/v1.0.0/`). Dokumen TECHSTACK dan BLAST-RADIUS v1.0.0 di repo ini masih ringkas, sehingga fondasi teknis dikunci di ADR ini sebelum coding kickoff.

## Keputusan

1. Stack terkunci: Rust + Axum 0.8 + Tokio + SQLx Postgres; satu `AppError: IntoResponse`; `#[sqlx::test]` untuk test data.
2. DB logis `pasaree`, terpisah dari divisi lain; layanan ini tidak membaca atau menulis DB lain. Titeny membaca secara read-only lewat jalur yang disepakati.
3. JWT akses 15 menit dengan audiens `pasaree` (token lintas divisi ditolak 401) + refresh 7 hari dengan reuse-detection; proksi ke Lumbung memakai service key least-privilege, bukan token pengguna.
4. Pembayaran idempoten: `idempotency_key` per upaya bayar; kunci sama menghasilkan respons identik tanpa tagihan ganda.
5. Rekonsiliasi harian ke Lumbung dan payout T+1 dieksekusi Lumbung; Pasaree tidak memegang kas payout.
6. Uang selalu integer IDR; komisi dibulatkan ke bawah (`floor()`) per baris item. Komisi 8% (paket-hampers 10%, lapak baru 5% selama 30 hari) dipotong dari payout lapak, BUKAN dari pembeli; biaya layanan Rp2.000 per checkout masuk operasional Pasaree; ongkir 100% ke kas trip Lumbung; escrow via kasir, payout dieksekusi Lumbung T+1 hari kerja; uang integer IDR, komisi `floor()`.
7. Katalog jujur: harga tampil = harga bayar sebelum ongkir; komisi tertera terpisah dan tidak dibebankan ke pembeli; dilarang mark-up tersembunyi.
8. Fulfillment fisik wajib via trip Lumbung; Pasaree tidak membangun armada, gudang, atau kasir kas sendiri.
9. Rekonsiliasi Pasaree-Lumbung harian; selisih >Rp1.000 atau status menggantung >2x24 jam menjadi tiket prioritas tinggi.
10. Perubahan tarif umum diumumkan 14 hari sebelum berlaku.

## Konsekuensi

- Semua pekerjaan repo ini mengikuti `AGENTS.md`; menyimpang dari keputusan di atas butuh ADR baru bernomor berikutnya.
- Dokumen TownHall tetap sumber requirement; repo ini tidak menyalin requirement, hanya menautkannya.
- Pelanggaran batas (DB lintas divisi, komisi ke pembeli, fulfillment di luar trip Lumbung) dianggap cacat dan diblokir di review.
