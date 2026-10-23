# Menggunakan penggunaan ZXV di NEAR: panduan lengkap

**Bahasa Melayu** · ZXV — logik parakonsisten · mesin keadaan · penyelesaian


ZXV ialah pelaksanaan hidup logik parakonsisten pada rangkaian NEAR. Panduan ini menerangkan cara membaca, mengesahkan, dan menggunakan penggunaan — tanpa gas, tanpa dompet untuk membaca.

---

## Alamat
Alamat kanonik ialah zedec.near, kontrak penghalaan hidup. Ia menunjuk kepada protokol (mikro-kontrak 110 kit), pendaftar (136 keupayaan), dan pintu (invarian kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Cara membaca sistem
Mana-mana klien membaca sistem tanpa gas melalui view calls. Antara muka universal mengajar sendiri: ux_manifest mengembalikan skema penuh, ux_help menjawab sebarang topik.

---

## Cara menyertai
Aktor baharu memanggil mint_onboard: geran sekali 5000 unit, boleh dibelanjakan di platform tetapi tidak boleh dipindahkan. Kemudian pool_enter memberi akses melalui kolam kecairan.

---

## Cara mengesahkan
Invarian boleh disahkan on-chain: conjoin mengembalikan hasil empat nilai; is_settleable benar hanya untuk TRUE; verify_chain mengesahkan rantai audit.

---

## Rujukan
Kernel: github.com/sykomiked/zxv-stack · NEP dan lejar: github.com/sykomiked/ZXV-NEP-paraconsistent · Kit pembangun: github.com/sykomiked/zxv-developer-kit.

Sistem ini terbuka.

**Hanya TRUE yang menyelesaikan. TRUE+FALSE=BOTH — percanggahan direkodkan, tidak pernah dipaksa.**

