# Autentikasi Pasaree

## Wajib

- Skema `Authorization: Bearer <jwt>`.
- Klaim `aud` harus tepat `pasaree`. Contoh klaim:
  `{"sub": "pengguna-01", "aud": "pasaree", "iss": "pasaree-auth", "exp": 1893456000}`.
- Token lintas divisi tanpa `aud` pasaree ditolak dengan 401.

## Aturan

- Basis data terpisah `pasaree`. Layanan ini tidak membaca basis data divisi lain.
- Proksi ke Lumbung memakai token layanan internal, bukan token pengguna.
- Bayar wajib menyertakan kepala `idempotency-key`.
