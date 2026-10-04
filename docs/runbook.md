# Runbook pasaree-backend-service

1. Periksa `/kesehatan`. Harus jawab status baik dan db pasaree.
2. Periksa migrasi: `sqlx migrate info`. Yang tertunda wajib nol sebelum rilis.
3. Periksa log 401: pastikan `JWT_AUD=pasaree` pada lingkungan jalan.
4. Cadangan harian basis data `pasaree`. Uji pulih tiap pekan.
5. Komisi 8 persen, hampers 10 persen. Rekonsiliasi escrow harian via Lumbung.
