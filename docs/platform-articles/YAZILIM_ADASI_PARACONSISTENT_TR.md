# Algoritmalar ve Mimari: Çelişkiyi Kucaklayan Dört Değerli Mantık Sistemi

**Belnap mantığından NEAR üzerinde doğrulanabilir bir uygulamaya — algoritma tasarımı perspektifiyle kapsamlı bir inceleme**

*Yazar: ZXV mühendislik ekibi (ZEDEC / 36N9 Genetics, LLC)*
*Yazılım Adası için yazıldı — bağımsız Türk geliştirici topluluğu forumu; algoritmalar, yazılım mimarisi ve sistem mühendisliği*

---

## Giriş: Algoritma Tasarımında "Bilinmeyen" Kavramı

Bir algoritma tasarımcısı olarak en zor sorulardan biri şudur: **veri iki farklı kaynaktan çelişkili geldiğinde ne yapmalısınız?** Çoğu sistem bu durumu bir hata olarak ele alır. Peki ya bu durumu bir *durum* olarak modelleyebilseydiniz?

Bu makalede, dört değerli (parakonsistent) mantığın algoritma ve mimari perspektifinden tam bir analizini sunuyoruz: teorisi, cebirsel yapısı, C ve Rust uygulamaları ve NEAR blok zinciri üzerindeki gerçek dağıtımı. Amacımız yalnızca bir kavramı anlatmak değil, **çalışan, doğrulanabilir bir sistemin** tüm bileşenlerini göstermektir.

---

## 1. Problem Tanımı: Klasik Mantığın Sınırı

### 1.1 İki Değerli Mantık ve Patlama İlkesi

Klasik Boole mantığı iki değer bilir: doğru (1) ve yanlış (0). Üçüncü halin imkânsızlığı ilkesi gereği her önerme ya doğrudur ya yanlıştır. Ancak dağıtık sistemlerde bu varsayım çöker: iki düğüm aynı anda birbiriyle çelişen veriler üretebilir.

Klasik mantıkta bu, **patlama ilkesini** (ex falso quodlibet) tetikler: bir çelişkiden herhangi bir önerme türetilebilir. Bu, algoritmik olarak "her şey mümkün" demektir — yani hiçbir şey güvenilir değildir. Sistemler bu yüzden şu dört stratejiden birini seçer:

1. **Panik**: İstisna fırlat, durumu geri al. → Sistem çöker.
2. **Son-Yazar-Kazanır**: Son gelen veri kazanır. → Eski veri sessizce kaybolur.
3. **Kilitlenme**: İşlemi durdur, bekle. → Sistem donar.
4. **Çoğunluk**: Çoğunluk ne derse o. → Güç, haklılığın önüne geçer.

Bu stratejilerin dördü de bilgi kaybına yol açar.

### 1.2 Parakonsistent Yaklaşım

Parakonsistent mantık şu öneriyi getirir: **çelişki bir hata değil, bir veridir.** Çelişkiyi bir bilgi durumu olarak saklayın, zorlamayın, ve kanıt netleştiğinde çözün.

---

## 2. Dört Değerli Mantığın Cebirsel Yapısı

### 2.1 Dört Durum

| Durum | Değer | Anlam |
|---|---|---|
| NEITHER | 0x00 | Kanıt yok (beklemede) |
| TRUE | 0x01 | Uzlaşma sağlandı (çözülebilir) |
| FALSE | 0x02 | Açıkça çürütüldü (reddedildi) |
| BOTH | 0x03 | Çelişki (her iki kanıt da mevcut) |

### 2.2 Zincir Düzeni: Derece Tablosu

Bu tasarımın asıl gücü, durumları bir **tam sıralama** üzerinde düzenlemektir:

```
FALSE (0) < NEITHER (1) < BOTH (2) < TRUE (3)
```

Bu, Belnap'ın orijinal örgüsünden bilinçli bir sapmadır. Neden? Çünkü TRUE'yu **tepeye** koyarak şu yapısal garantiye ulaşırız:

> **Hiçbir TRUE-olmayan değer kombinasyonu, meet işlemi sonucunda TRUE üretemez.**

Bu bir runtime kontrolü değil, **cebirsel bir zorunluluktur**. Meet (minimumu alan işlem) ancak iki girdi de TRUE ise TRUE döndürebilir.

### 2.3 Meet ve Join İşlemleri

```c
/* Zincir meet: küçük derece kazanır */
lpres_state_t conjoin(lpres_state_t a, lpres_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* güvenli kapatma */
    return rank[a] <= rank[b] ? a : b;
}

/* Zincir join: büyük derece kazanır */
lpres_state_t disjoin(lpres_state_t a, lpres_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}
```

### 2.4 Cebirsel Özellikler

Bu işlemler aşağıdaki özellikleri sağlar (tüm kombinasyonlarda doğrulandı):

- **Değişme**: `meet(a,b) == meet(b,a)`
- **Birleşme**: `meet(meet(a,b),c) == meet(a,meet(b,c))`
- **Kuvvetli özdeşlik**: `meet(a,a) == a`
- **Soğurma**: `meet(a, join(a,b)) == a`

Bu özellikler, algoritmaların **deterministik ve sıra-bağımsız** olmasını sağlar — dağıtık sistemlerde kritik bir gereksinim.

---

## 3. C Uygulaması: Sıfır Tahsisli Çekirdek

### 3.1 Tam Kod

```c
/* belnap_gate.c — çelişkiye toleranslı durum makinesi */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef enum {
    NEITHER = 0x00, TRUE = 0x01, FALSE = 0x02, BOTH = 0x03
} belnap_state_t;

/* [NEITHER,TRUE,FALSE,BOTH] sırasına göre dereceler: {1,3,0,2} */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    /* Çelişki üret: TRUE | FALSE = BOTH */
    belnap_state_t c = (belnap_state_t)((unsigned)TRUE | (unsigned)FALSE);
    printf("Çelişki durumu: 0x%02X, Çözülebilir mi? %s\n",
           c, is_settleable(c) ? "EVET" : "HAYIR (paniksiz engellendi)");

    /* Uzlaşma: TRUE ∧ TRUE = TRUE */
    belnap_state_t r = conjoin(TRUE, TRUE);
    printf("Uzlaşma: 0x%02X, Çözülebilir mi? %s\n",
           r, is_settleable(r) ? "EVET" : "HAYIR");

    /* Güvenli kapatma: bilinmeyen değer asla TRUE olamaz */
    belnap_state_t u = conjoin((belnap_state_t)7, TRUE);
    printf("Bilinmeyen koruma: 0x%02X, Çözülebilir mi? %s\n",
           u, is_settleable(u) ? "EVET" : "HAYIR");
    return 0;
}
```

### 3.2 Neden Sıfır Tahsis?

Bu çekirdek hiçbir heap tahsisi yapmaz: `malloc`/`free` çağrısı yoktur, dinamik veri yapısı yoktur. Tüm işlemler sabit boyutlu tamsayılar üzerindedir. Bu, **bellek bütçesi garantili** sistemler için kritik bir özelliktir.

### 3.3 Derleme ve Çalıştırma

```bash
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate
```

Çıktı:
```
Çelişki durumu: 0x03, Çözülebilir mi? HAYIR (paniksiz engellendi)
Uzlaşma: 0x01, Çözülebilir mi? EVET
Bilinmeyen koruma: 0x00, Çözülebilir mi? HAYIR
```

---

## 4. Rust Uygulaması: Tip Güvenliği

### 4.1 no_std Kütüphane

```rust
//! lib.rs — no_std ortamında çalışan dört değerli mantık
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool { matches!(self, Belnap::True) }

    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        if RANK[self as usize] <= RANK[other as usize] { self } else { other }
    }

    #[inline(always)]
    pub fn combine(self, other: Self) -> Self {
        if RANK[self as usize] >= RANK[other as usize] { self } else { other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contradiction_does_not_panic() {
        assert!(!Belnap::Both.is_settleable());
    }

    #[test]
    fn only_true_settles() {
        assert!(Belnap::True.is_settleable());
        assert!(!Belnap::False.is_settleable());
        assert!(!Belnap::Neither.is_settleable());
    }

    #[test]
    fn false_dominates_meet() {
        assert_eq!(Belnap::False.merge(Belnap::True), Belnap::False);
    }

    #[test]
    fn meet_associative() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals { for b in vals { for c in vals {
            assert_eq!(a.merge(b).merge(c), a.merge(b.merge(c)));
        }}}
    }

    #[test]
    fn no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals { for b in vals {
            if a != Belnap::True && b != Belnap::True {
                assert_ne!(a.merge(b), Belnap::True);
            }
        }}
    }
}
```

```bash
cargo test --release   # tüm testler geçer
```

### 4.2 WASM Hedefi

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Sonuç: yalnızca birkaç KB'lık, heap kullanmayan, saf fonksiyonlar olarak dışa aktarılan WASM.

---

## 5. Mimari Desen: Dört Değerli Durum Makinesi

### 5.1 Geçiş Diyagramı

```
            kanıt yok                 her iki kanıt
                 │                         │
                 ▼                         ▼
            NEITHER ──────────────► BOTH ──────────► (çözüm süreci)
            (bekleme)               (saklanır)           │
                 │                                       │ çözüm
                 │ tek kanıt                             ▼
                 ▼                                   TRUE / FALSE
            (yalnızca TRUE ilerler)          (çöz / reddet)
```

### 5.2 Mimari İlkeler

1. **NEITHER = bekleme, askıya alma değil**: İşlem ilerlemez ama engellenmez.
2. **BOTH = çözüm sürecine geçiş**: Otomatik çözüm yok; denetim izi yazılır.
3. **Sadece TRUE yan etki üretir**: `is_settleable` false ise hiçbir yazma gerçekleşmez — bu, kısmi durum tutarsızlıklarını yapısal olarak önler.

### 5.3 Event Sourcing ile Bütünleşme

```
[Olay 1] attest(TRUE)   → durum: TRUE
[Olay 2] attest(FALSE)  → durum: BOTH (çelişki kaydedildi)
[Olay 3] resolve(TRUE)  → durum: TRUE (çözüldü)
```

Append-only günlük, çelişkiden çözüme tam denetim izi sağlar.

---

## 6. Doğrulama Stratejisi

### 6.1 Kapsamlı Testler

Durum uzayı 4 elemanlı olduğu için tüm kombinasyonları test etmek mümkündür:

- Meet: 16 kombinasyon
- Birleşme: 64 kombinasyon
- Tüm cebirsel özellikler: eksiksiz doğrulama

### 6.2 Fuzzing

```c
/* Tüm u8 alanı (65.536 girdi) için değişmez kontrolü */
for (unsigned a = 0; a < 256; a++)
    for (unsigned b = 0; b < 256; b++) {
        belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
        if (m == TRUE && (a != TRUE || b != TRUE)) {
            printf("İHLAL: conjoin(%u, %u) = TRUE\n", a, b);
            return 1;
        }
    }
printf("FUZZ OK: 65536 kombinasyon, ihlal yok\n");
```

---

## 7. NEAR Üzerinde Gerçek Dağıtım

Bu tasarım kağıt üzerinde kalmadı. **NEAR ana ağında akıllı sözleşme olarak çalışıyor.**

### 7.1 Adresler

| Rol | Adres |
|---|---|
| **Kanonik (yönlendirme)** | `zedec.near` |
| **Protokol (110 kit)** | `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf` |
| **Kayıt (136 yetenek)** | `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a` |
| **Kapı (çekirdek değişmezi)** | `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8` |

### 7.2 Zincir Üzerinde Doğrulama (gazsız)

```bash
# conjoin {"inputs":[1,2]}        → "FALSE"
# is_settleable {"state":"BOTH"}  → false (çelişki çözülmez)
# is_settleable {"state":"TRUE"}  → true (uzlaşma çözülür)
# verify_chain {"from":0}         → true (denetim zinciri sağlam)
```

### 7.3 Mimari Bileşenler

- **Venedik Gaz İstasyonu**: TRUE çözümü 2× gaz eşleşmesi alır; BOTH eşleşmeyi kaybeder (drenaj kalkanı) — çelişki maliyetli hale gelir.
- **Egemen Kararnameler**: "çatallanmadan çatallanma" — çelişkili kararnameler bir arada var olur.
- **Receipt zinciri**: her çözüm hash zincirine bağlanır (verify_chain).

### 7.4 Kalite Kaydı

| Doğrulama | Sonuç |
|---|---|
| QC donanımı | 55/55, sıfır hata |
| Zincir üstü denetim | 24/24 |
| Dayanıklılık testi | 70/70 |
| Kayıt denetimi | 136/136 |
| Gaz duvarı | 125/125 + 10/10 |

---

## 8. Sonuç

Dört değerli mantık, klasik sistemlerin çözemediği bir problemi çözer: **veri çeliştiğinde ne yapmalı?** Cevap: çelişkiyi bilgi olarak sakla, zorlama, panik yapma. Ve en önemlisi — "yalnızca TRUE çözer" ilkesi, derece yapısından türetilen **cebirsel bir zorunluluktur**, çalışma zamanı kontrolü değil.

Sistem NEAR üzerinde çalışıyor, doğrulama ücretsiz. **Sistem açık.**

---

## Depolar

- **Çekirdek uygulaması**: https://github.com/sykomiked/zxv-stack
- **Akıllı sözleşme**: https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*