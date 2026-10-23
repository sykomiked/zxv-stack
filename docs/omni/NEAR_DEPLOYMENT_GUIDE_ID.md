# Menggunakan penerapan ZXV di NEAR: panduan lengkap

**Bahasa Indonesia** · ZXV — logika parakonsisten · mesin keadaan · penyelesaian


ZXV adalah implementasi hidup logika parakonsisten di jaringan NEAR. Panduan ini menjelaskan cara membaca, memverifikasi, dan menggunakan penerapan — tanpa gas, tanpa dompet untuk membaca.

---

## Alamat
Alamat kanonik adalah zedec.near, kontrak perutean hidup. Ia menunjuk ke protokol (mikro-kontrak 110 kit), registri (136 kapabilitas), dan gerbang (invariant kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Cara membaca sistem
Klien mana pun membaca sistem tanpa gas melalui view calls. Antarmuka universal mengajar sendiri: ux_manifest mengembalikan skema lengkap, ux_help menjawab topik apa pun.

---

## Cara bergabung
Aktor baru memanggil mint_onboard: hibah sekali 5000 unit, dapat dibelanjakan di platform tetapi tidak dapat ditransfer. Lalu pool_enter memberi akses melalui kolam likuiditas.

---

## Cara memverifikasi
Invariant dapat diverifikasi on-chain: conjoin mengembalikan hasil empat nilai; is_settleable benar hanya untuk TRUE; verify_chain mengonfirmasi rantai audit.

---

## Referensi
Kernel: github.com/sykomiked/zxv-stack · NEP dan buku besar: github.com/sykomiked/ZXV-NEP-paraconsistent · Kit pengembang: github.com/sykomiked/zxv-developer-kit.

Sistem ini terbuka.

**Hanya TRUE yang menyelesaikan. TRUE+FALSE=BOTH — kontradiksi dicatat, tidak pernah dipaksa.**

