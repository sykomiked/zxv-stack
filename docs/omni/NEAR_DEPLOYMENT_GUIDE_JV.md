# Nggunakake panyebaran ZXV ing NEAR: pandhuan lengkap

**Basa Jawa** · ZXV — logika parakonsisten · mesin negara · rampung


ZXV minangka implementasi urip logika parakonsisten ing jaringan NEAR. Pandhuan iki nerangake carane maca, verifikasi, lan nggunakake panyebaran — tanpa gas, tanpa dompet kanggo maca.

---

## Alamat
Alamat kanonik yaiku zedec.near, kontrak rute urip. Iki nuduhake protokol (mikro-kontrak 110 kit), registri (136 kapabilitas), lan gapura (invarian kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Cara maca sistem
Klien apa wae maca sistem tanpa gas liwat view calls. Antarmuka universal mulang dhewe: ux_manifest menehi skema lengkap, ux_help mangsuli topik apa wae.

---

## Cara gabung
Aktor anyar nelpon mint_onboard: hibah sepisan 5000 unit, bisa dibuwang ing platform nanging ora bisa ditransfer. Banjur pool_enter menehi akses liwat kolam likuiditas.

---

## Cara verifikasi
Invarian bisa diverifikasi on-chain: conjoin menehi asil papat nilai; is_settleable bener mung kanggo TRUE; verify_chain ngonfirmasi rantai audit.

---

## Referensi
Kernel: github.com/sykomiked/zxv-stack · NEP lan buku: github.com/sykomiked/ZXV-NEP-paraconsistent · Kit pangembang: github.com/sykomiked/zxv-developer-kit.

Sistem iki mbukak.

**Mung TRUE sing ngrampungake. TRUE+FALSE=BOTH — kontradiksi dicathet, ora nate dipeksa.**

