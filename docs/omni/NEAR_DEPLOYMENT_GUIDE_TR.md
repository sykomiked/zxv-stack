# ZXV NEAR dağıtımını kullanma: eksiksiz kılavuz

**Türkçe** · ZXV — parakonsistent mantık · durum makinesi · mutabakat


ZXV, parakonsistent mantığın NEAR ağında canlı bir uygulamasıdır. Bu kılavuz dağıtımı okumayı, doğrulamayı ve kullanmayı açıklar.

---

## Adresler
Kanonik adres zedec.near, canlı bir yönlendirme sözleşmesidir. Protokole (110 kitlik mikro-sözleşme), kayıt defterine (136 yetenek) ve kapıya (çekirdek değişmezi) işaret eder.

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Sistem nasıl okunur
Her istemci view çağrılarıyla gazsız okur. Evrensel arayüz kendini öğretir: ux_manifest tam şemayı verir, ux_help her konuyu yanıtlar.

---

## Nasıl katılınır
Yeni aktör mint_onboard çağırır: bir kezlik 5000 birimlik hibe, platformda harcanabilir ama devredilemez. Sonra pool_enter likidite havuzu üzerinden erişim verir.

---

## Nasıl doğrulanır
Değişmez zincir üzerinde doğrulanabilir: conjoin dört değerli sonucu verir; is_settleable yalnızca TRUE için doğrudur; verify_chain denetim zincirini onaylar.

---

## Referanslar
Çekirdek: github.com/sykomiked/zxv-stack · NEP ve defter: github.com/sykomiked/ZXV-NEP-paraconsistent · Geliştirici kiti: github.com/sykomiked/zxv-developer-kit.

Sistem açıktır.

**Yalnızca TRUE mutabakat sağlar. TRUE+FALSE=BOTH — çelişkiler kaydedilir, asla zorlanmaz.**

