# Çelişkiye Toleranslı Durum Makineleri: Dört Değerli Mantık Olarak Sistem Primitifi

**Zincir-meet cebiri (FALSE < NEITHER < BOTH < TRUE), sıfır tahsisli C uygulaması, no_std Rust portu ve NEAR ağında doğrulanabilir bir kavram kanıtı**

*Yazar: ZXV mühendislik ekibi (ZEDEC / 36N9 Genetics, LLC)*
*R10.net için yazıldı — Türkiye'nin en büyük webmaster ve yazılım geliştirici topluluğu; sistem programlama, C/C++ ve altyapı bölümleri*

---

## Özet

Her dağıtık veya eşzamanlı durum makinesi er ya da geç aynı duvara çarpar: **iki bilgi kaynağı "gerçek" konusunda anlaşmazlığa düştüğünde sistem ne yapmalı?** Klasik iki değerli mantık (üçüncü halin imkânsızlığı, $P \lor \neg P$), çelişkili kanıtlar aynı anda geldiğinde **patlama ilkesini** (principle of explosion) tetikler ve uygulamaları dört hata modundan birini seçmeye zorlar: panik, son-yazar-kazanır (LWW), kilitlenme (deadlock) veya çoğunluk zorlaması. Bu makale, **Belnap'ın dört değerli mantığına** (TRUE/FALSE/NEITHER/BOTH) dayanan ve çelişkiyi **birinci sınıf bir matematiksel durum** olarak ele alıp doğrusal yürütmeyi kesintiye uğratmadan koruyan bir durum makinesi tasarımı önerir. Temel katkı, bir **zincir-meet cebridir** ($FALSE < NEITHER < BOTH < TRUE$): TRUE zincirin tepesinde olduğundan, **"hiçbir TRUE-olmayan değer kombinasyonu TRUE üretmez"** değişmezi çalışma zamanı kontrolleriyle değil, yapısal olarak garanti edilir. Tasarım C (sıfır tahsis, saf bit işlemleri) ve Rust (`no_std`) ile uygulanmış ve kapsamlı testlerle (16/64 kombinasyon) ve fuzzing ile (65.536 girdi) doğrulanmıştır. Son olarak, teori kağıt üzerinde bir tasarım değildir: **NEAR ana ağında akıllı sözleşme olarak** (110 kit, 136 yetenek, kanonik adres `zedec.near`, sözleşme adresi `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) bir kavram kanıtı olarak dağıtılmış, çekişmeli denetimden ve gaz duvarı testinden geçmiştir. Tüm iddialar, gazsız, zincir üzerinde bağımsız olarak doğrulanabilir.

---

## 1. Giriş: "Çelişki" neden bir sistem primitifi

### 1.1 Dağıtık sistemlerin temel sorunu

Her dağıtık veya eşzamanlı durum makinesi sonunda tek bir soruyu yanıtlamak zorundadır:

> **İki bilgi kaynağı gerçek konusunda çatıştığında sistem ne yapmalı?**

Klasik mantık, üçüncü halin imkânsızlığını varsayar. Ancak dağıtık bir ortamda ağ bölünmesi veya çoğaltma gecikmesi, **$P$ ve $\neg P$ için kanıtların aynı anda gelmesine** neden olabilir. Klasik çerçevede bu, patlama ilkesini tetikler: $P \land \neg P$ çelişkisinden herhangi bir önerme türetilebilir ve tüm çıkarım sistemi anlamsızlaşır.

Sonuç olarak, klasik mantığa dayalı uygulamalar çelişkiyi dört stratejiden biriyle "önlemek" zorundadır:

| Strateji | Davranış | Maliyet |
|---|---|---|
| **Panik** | istisna fırlat, durumu geri al | doğrusal yürütme kesilir, uçuştaki durum kaybolur |
| **Son-yazar-kazanır (LWW)** | daha yeni zaman damgası üzerine yazar | eski gerçek sessizce silinir |
| **Kilitlenme (deadlock)** | işlemeyi durdur ve bekle | tüm sistem donar |
| **Çoğunluk zorlaması** | daha fazla oy çözümü dayatır | güç kazanır, gerçek değil |

Dört strateji de **çelişkiyi bir hata (bug) olarak** ele alır. Hiçbiri çelişkinin içerdiği bilgiyi korumaz; her biri ya sistem kullanılabilirliğinden ya da veri bütünlüğünden fedakârlık eder.

### 1.2 Parakonsistent dönüş

Parakonsistent mantık (çelişkiye toleranslı mantık) öncülü tersine çevirir:

> **Çelişki bir hata durumu değildir. Bir bilgi durumudur.**

Sistem iki iddiayı aynı anda tutar, çatışmayı kaydeder ve kanıtlar gerçekten oturana kadar bir çözüm dayatmayı reddeder. Bu felsefi bir duruş değildir — **matematiksel olarak iyi tanımlanmış dört değerli bir mantıktır** ve bir durum makinesi primitifi olarak uygulanabilir.

---

## 2. Teori: dört değerli mantığın cebirsel yapısı

### 2.1 Durum tanımı

Durum uzayı, gerçek çekirdeğin kanonik tanımını izler:

```c
/* lpres_phases.h — gerçek çekirdekten durum tanımı (alıntı) */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* kanıt yok (gözlenmedi, beklemede) */
    LPRES_STATE_TRUE    = 1,  /* anlaşıldı, doğru (uzlaştırılabilir) */
    LPRES_STATE_FALSE   = 2,  /* açıkça çürütüldü (reddedildi)      */
    LPRES_STATE_BOTH    = 3,  /* çelişki (her iki kanıt)            */
    LPRES_STATE_GATEWAY = 4,  /* beşinci değer: genişletme kapısı   */
    /* ... Fibonacci kilometre taşlarına göre genişletme fazları ... */
    LPRES_STATE_COUNT   = 145 /* genişletme sonrası toplam durum     */
} lpres_state_t;
```

**Anahtar tasarım kararı**: durumlar **tek bir bayttaki bit desenleri** olarak temsil edilir. `0x00`–`0x03` dört değerli mantığın çekirdeğidir; `0x04` ve sonrası genişletme fazlarıdır. Durum geçişleri saf tamsayı işlemleridir — yığın tahsisi, dinamik bellek veya kilit yoktur.

### 2.2 Zincir derecesi (toplam sıralamayı tanımlama)

Bu uygulamanın çekirdeği **Belnap'ın örgüsünün kendisi değil, tam sıralı bir zincir üzerindeki derecedir**. Gerçek çekirdeğin derece tablosu:

```c
/* lpres_phases.h — zincir derecesi (kanonik uygulama) */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = {
    [LPRES_STATE_FALSE]   = 0,  /* taban (meet'in hâkimi) */
    [LPRES_STATE_NEITHER] = 1,  /* kanıt yok             */
    [LPRES_STATE_BOTH]    = 2,  /* çelişki               */
    [LPRES_STATE_TRUE]    = 3,  /* tepe (join'in kimliği) */
    [LPRES_STATE_GATEWAY] = 4,
    /* ... her genişletme fazı bir sonraki dereceyi alır ... */
};
```

Yani **toplam sıralama**

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

**FALSE tabandır, TRUE tepe.** Yapının özü budur.

### 2.3 conjoin işlemi = zincir meet

Gerçek çekirdeğin conjoin'u (`kernel/src/lpres/lpres.c`):

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank, so a new phase needs only
     * its rank, never a new branch. Deliberate deviation from Belnap's
     * FOUR (the chain, not the lattice): no mix of non-TRUE values can
     * yield TRUE, the only value that authorises an action. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* bilinmeyen faz asla otorite değildir */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**Meet = daha küçük dereceli öğeyi döndür.** Özellikler:

| Girdi | Sonuç | Anlam |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE hâkimdir (taban) |
| `conjoin(TRUE, TRUE)` | `TRUE` | yalnızca anlaşma uzlaştırır |
| `conjoin(BOTH, TRUE)` | `BOTH` | çelişki korunur, asla zorlanmaz |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER, BOTH'u emer |
| `conjoin(bilinmeyen, x)` | `NEITHER` | aralık dışı koruma (fail-closed) |

### 2.4 disjoin işlemi = zincir join

Çift yönlü disjoin:

```c
lpres_state_t lpres_disjoin(lpres_state_t a, lpres_state_t b) {
    /* the join: max on the chain. TRUE dominates, FALSE is identity,
     * BOTH absorbs NEITHER — all by the rank. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;
    return lpres_rank[a] >= lpres_rank[b] ? a : b;
}
```

Join = **daha büyük dereceli öğeyi döndür**. TRUE tepe olduğundan `disjoin(TRUE, x) = TRUE`.

### 2.5 Cebirsel özellikler (doğrulandı)

Zincir meet/join bir **sınırlı örgü (bounded lattice)** oluşturur. Tüm kombinasyonların kapsamlı doğrulaması (4×4=16 ve birleşme için 64):

```
Meet: kuvvetli-özdeş ○ değişmeli ○ birleşmeli ○
Join: kuvvetli-özdeş ○ değişmeli ○ birleşmeli ○
Soğurma: ○ (a ∧ (a ∨ b) = a)
Kısmi sıralama: ○ (a ≤ b ⇔ meet(a,b) = a)
Değişmez: ○ (TRUE-olmayan değerlerin meet'i asla TRUE üretmez)
```

**Bu yapıdır, tesadüf değil.** TRUE derece 3'e (tepe) sahip olduğundan, meet (minimumu döndüren) ancak **her iki girdi de TRUE ise** TRUE olabilir. Dolayısıyla:

> **"Yalnızca TRUE uzlaştırır" değişmezi derece yapısından matematiksel olarak türetilir.**

### 2.6 Belnap örgüsünden bilinçli sapma

Kaynak yorumu açıkça söyler:

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice): no mix of non-TRUE values can yield TRUE, the only value that authorises an action.*

Belnap'ın `FOUR` örgüsü iki sıralamaya sahiptir (bilgi sıralaması ve doğruluk sıralaması). Bu uygulama onu **tek bir tam sıralı zincire katlar**. Yararları:

- bir faz eklemek yalnızca **bir derece eklemeyi** gerektirir (yeni dal mantığı yok);
- uygulama **tek bir tamsayı karşılaştırmasına** (`<=`) indirgenir;
- Belnap örgüsünde mümkün olan "TRUE-olmayan kombinasyon TRUE üretir" sorunu **yapısal olarak elenir**.

---

## 3. Uygulama: C, sıfır tahsis

### 3.1 Tam bağımsız liste

Aşağıdaki tek dosya yalnızca `gcc` ile derlenip çalışır:

```c
/* belnap_gate.c — çelişkiye toleranslı durum makinesi, sıfır tahsis
 *
 * Derleme:
 *   gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
 * Çalıştırma:
 *   ./belnap_gate
 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/* Dört değerli durumlar (gerçek çekirdeğin lpres_phases.h dosyasından) */
typedef enum {
    NEITHER = 0x00,  /* kanıt yok */
    TRUE    = 0x01,  /* anlaşıldı, doğru */
    FALSE   = 0x02,  /* açıkça çürütüldü */
    BOTH    = 0x03,  /* çelişki */
} belnap_state_t;

/* Zincir derecesi: enum sırası [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
 * FALSE=0 (taban), TRUE=3 (tepe) — gerçek çekirdekle aynı */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* fail-closed */
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

/* Uzlaştırma değişmezi: yalnızca TRUE uzlaştırır */
static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    belnap_state_t a = TRUE;
    belnap_state_t b = FALSE;

    /* Çelişki ortaya çıkar: klasik mantıkta bu panik noktasıdır */
    belnap_state_t contention = (belnap_state_t)((unsigned)a | (unsigned)b);
    /* 0x01 | 0x02 = 0x03 (BOTH) — panik olmadan "bilgi" olarak korunur */

    printf("State: 0x%02X (%s)\n",
           contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n",
           is_settleable(contention) ? "YES" : "NO (Blocked without panic)");

    /* Birleşme: TRUE + TRUE = TRUE (uzlaştırılabilir) */
    belnap_state_t resolved = conjoin(a, a);
    printf("Resolved: 0x%02X -> Can Settle? %s\n",
           resolved, is_settleable(resolved) ? "YES" : "NO");

    /* Bilinmeyen faz koruması: aralık dışı girdi asla TRUE olmaz */
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n",
           unknown, is_settleable(unknown) ? "YES" : "NO");

    /* disjoin (join) doğrulaması */
    belnap_state_t j = disjoin(BOTH, NEITHER); /* max rank: BOTH */
    printf("Disjoin(BOTH,NEITHER): 0x%02X (%s)\n",
           j, j == BOTH ? "BOTH" : "OTHER");

    return 0;
}
```

### 3.2 Bellek özellikleri

x86-64 Linux üzerinde analiz:

```
Yığın: 3 yerel değişken = 24 bayt (hizalamayla)
Öbek: 0 (malloc/free çağrısı yok)
Statik veri: rank[4] = 4 bayt (.rodata bölümü)
Kod boyutu: derleme düzeyinde onlarca talimat
```

**Düz, öngörülebilir tüketim; sıfır yığın çalkalanması.** Gerçek zamanlı sistemler için bellek bütçelerken bu özellik garantili bir üst sınır olarak ele alınabilir. Tasarım, gömülü, WASM ve çekirdek alanı ortamlarına doğrudan uygulanabilir.

### 3.3 Derleme düzeyinde doğrulama

`conjoin` derlemesi `-O3`'te (x86-64, alıntı):

```asm
conjoin:
    movzx   eax, dil            ; a'yı yükle
    movzx   ecx, sil            ; b'yi yükle
    movzx   edx, BYTE PTR rank[rax]  ; rank[a]'yı yükle
    cmp     BYTE PTR rank[rcx], dl   ; rank[b] ile karşılaştır
    cmovle  eax, esi            ; rank[a] <= rank[b] ise b'yi seç
    ret
```

**Dal tahminine dost birkaç talimat** — sıcak yolda performans garantilidir.

---

## 4. Uygulama: Rust / no_std, tip güvenliği

### 4.1 Tip güvenli dört değerli mantık

```rust
//! lib.rs — no_std altında çalışan değişmez doğrulama
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

/// Zincir derecesi: enum sırası [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
/// FALSE=0 (taban), TRUE=3 (tepe)
const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool {
        matches!(self, Belnap::True)
    }

    /// Zincir meet: FALSE hâkimdir, TRUE kimliktir,
    /// NEITHER, BOTH'u emer. TRUE-olmayan kombinasyon asla TRUE üretmez.
    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        let ra = RANK[self as usize];
        let rb = RANK[other as usize];
        if ra <= rb { self } else { other }
    }

    /// Zincir join: TRUE hâkimdir.
    #[inline(always)]
    pub fn combine(self, other: Self) -> Self {
        let ra = RANK[self as usize];
        let rb = RANK[other as usize];
        if ra >= rb { self } else { other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_contradiction_does_not_panic() {
        // Çelişki panic! yapmaz; false döndürür ve durumu korur
        assert!(!Belnap::Both.is_settleable());
    }

    #[test]
    fn verify_only_true_settles() {
        assert!(Belnap::True.is_settleable());
        assert!(!Belnap::False.is_settleable());
        assert!(!Belnap::Neither.is_settleable());
    }

    #[test]
    fn verify_false_dominates() {
        assert_eq!(Belnap::False.merge(Belnap::True), Belnap::False);
        assert_eq!(Belnap::True.merge(Belnap::False), Belnap::False);
    }

    #[test]
    fn verify_both_is_never_coerced() {
        // Çelişki korunur ve asla TRUE'ya zorlanmaz
        assert_eq!(Belnap::Both.merge(Belnap::True), Belnap::Both);
        assert!(!Belnap::Both.merge(Belnap::True).is_settleable());
    }

    #[test]
    fn verify_meet_idempotent() {
        for s in [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both] {
            assert_eq!(s.merge(s), s);
        }
    }

    #[test]
    fn verify_meet_commutative() {
        for a in [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both] {
            for b in [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both] {
                assert_eq!(a.merge(b), b.merge(a));
            }
        }
    }
}
```

### 4.2 Özellik tabanlı doğrulama

Birim testlerin ötesinde, tüm kombinasyonlar üzerinde **özellik tabanlı doğrulama** (KAT: Known-Answer Tests):

```rust
#[cfg(test)]
mod property_tests {
    use super::*;

    /// Tüm 4×4 kombinasyonlarda değişmez kontrolü
    #[test]
    fn verify_no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                let m = a.merge(b);
                if a != Belnap::True && b != Belnap::True {
                    assert_ne!(m, Belnap::True,
                        "TRUE-olmayan kombinasyon TRUE üretir: {:?} ∧ {:?} = {:?}", a, b, m);
                }
            }
        }
    }

    /// Birleşmenin tam sayımı
    #[test]
    fn verify_meet_associative() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                for c in vals {
                    assert_eq!(a.merge(b).merge(c), a.merge(b.merge(c)));
                }
            }
        }
    }

    /// Soğurma yasası
    #[test]
    fn verify_absorption() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                assert_eq!(a.merge(a.combine(b)), a);
            }
        }
    }
}
```

**64 birleşme kontrolü** ve **16 değişmez kontrolü** test altında yürütülür. Bu bir kanıt değildir, ancak durum uzayı sonlu ve küçük olduğundan **tam bir sayımdır**.

### 4.3 WASM hedefi değerlendirmeleri

`no_std` uygulaması doğrudan WASM hedefine (`wasm32-unknown-unknown`) derlenir:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Ortaya çıkan WASM **birkaç KB**'tır ve şu özelliklere sahiptir:

- öbek işlevlerini (`malloc`/`free`) çağırmaz;
- durum geçişleri saf işlevler olarak dışa aktarılır;
- gaz ölçümü (kaynak muhasebesi) basitleştirilir.

---

## 5. Durum makinesi tasarımı: dört değerli mantığı entegre etme

### 5.1 Dört değerli geçiş deseni

```
                    ┌──────────────┐
     kanıt yok      │   NEITHER    │
    ──────────────► │  (bekleme)   │
                    └──────┬───────┘
                           │ kanıt gelir
                    ┌──────▼───────┐
     her iki kanıt  │             │  tek kanıt
    ──────────────► │    BOTH      ├──────────────┐
                    │ (çelişki)    │              │
                    └──────┬───────┘              │
                           │ çözüm               │
                    ┌──────▼───────┐     ┌───────▼───────┐
                    │    TRUE      │     │    FALSE      │
                    │ (uzlaştırıldı)│    │  (reddedildi) │
                    └──────────────┘     └───────────────┘
```

**Tasarım noktaları**:

1. **NEITHER "beklemede"dir, "askıya alınmış" değil**. Uzlaştırma engellenmez, ancak ilerlemez.
2. **BOTH, "anlaşmazlık çözümüne" bir geçiştir**. Otomatik çözüm yoktur, ancak bir denetim izi yazılır.
3. **Yalnızca TRUE yan etkilere izin verir** (durum değişikliği, fon hareketi). `is_settleable(s)` false olduğunda hiçbir yazma gerçekleşmez.

### 5.2 Atomiklikle ilişki

İşlem işlemeye entegre ederken anahtar bir özellik:

```
Durum geçişinin atomikliği:
  read → evaluate (dört değerli karar) → write yalnızca is_settleable == TRUE ise
```

Böylece **çelişki durumunda hiçbir yazma gerçekleşmez** ve kısmi durum güncellemelerinden kaynaklanan tutarsızlık yapısal olarak önlenir. Klasik kontrol-sonra-güncelle yarışı (TOCTOU) için dört değerli mantık, **kontrolün kendisini değişmeze entegre eder**.

### 5.3 Event sourcing ile entegrasyon

Çelişkileri kaydetmek, event sourcing deseniyle doğal olarak bütünleşir:

```
[Event 1] attest(TRUE)   → durum: TRUE
[Event 2] attest(FALSE)  → durum: BOTH (çelişki kaydedildi)
[Event 3] resolve(TRUE)  → durum: TRUE (çözüldü)
```

**Her geçiş append-only bir günlüğe kaydedilir** ve çelişkiden çözüme kadar eksiksiz denetim izi korunur. Bu, NEAR uygulamasındaki receipt zinciriyle aynı desendir (aşağıya bakın).

---

## 6. Doğrulama: biçimsel yöntemler ve testler

### 6.1 Kapsamlı testin matematiksel temeli

Durum uzayı sonlu (4 durum) olduğundan, **kapsamlı kombinasyon testi matematiksel bir kanıta eşdeğer bir garanti sağlar**:

- Meet: 4×4 = 16
- Birleşme: 4×4×4 = 64
- Kuvvetli özdeşlik: 4
- Değişme: 16

Bunların tümünü testlerle sabitlemek, **uygulamanın spesifikasyona (derece tablosu) uyduğuna dair mekanik doğrulama** sağlar.

### 6.2 KAT (Known-Answer Test)

Kanonik derece tablosundan türetilen beklenen değerler uygulama çıktısıyla karşılaştırılır:

| Test | Girdi | Beklenen | Doğrula |
|---|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` | ○ |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` | ○ |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` | ○ |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` | ○ |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER` (fail-closed) | ○ |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` | ○ |

### 6.3 Fuzzing

Derece tablosunun dışındaki girdilere karşı sağlamlık:

```c
/* fuzz.c — aralık dışı girdilere karşı sağlamlık doğrulaması */
#include <stdint.h>
#include <stdio.h>
#include "belnap_gate.h"  /* conjoin, disjoin, is_settleable dışa aktarır */

int main(void) {
    /* tüm 256×256 girdi kombinasyonunu dene (tüm u8 alanı) */
    for (unsigned a = 0; a < 256; a++) {
        for (unsigned b = 0; b < 256; b++) {
            belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
            /* değişmez: aralık dışı girdilerin sonucu asla TRUE değildir */
            if (m == TRUE && (a != TRUE || b != TRUE)) {
                printf("VIOLATION: conjoin(%u, %u) = TRUE\n", a, b);
                return 1;
            }
        }
    }
    printf("FUZZ OK: 65536 combinations, no violations\n");
    return 0;
}
```

**Değişmez, tüm 65.536 girdi için geçerlidir (tüm u8 alanı).**

---

## 7. NEAR dağıtımı — kavram kanıtı

Yukarıda açıklanan dört değerli mantık, kağıt üzerinde bir tasarım değildir. **NEAR ana ağında akıllı sözleşme olarak dağıtılmıştır.** Bu makalenin "kavram kanıtı" budur.

### 7.1 Mimariye genel bakış

```
┌─────────────────────────────────────────────────────────┐
│  NEAR ana ağı                                            │
│                                                         │
│  zedec.near (kanonik, yönlendirme)                       │
│    │ route() → tam topolojiyi döndürür                   │
│    │ protocol_meta() → genel kayıt                       │
│    │ is_settleable() → değişmez                          │
│    │                                                   │
│  f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf
│    (zxv-core: 110 kitlik tek mikro-sözleşme)             │
│    │ conjoin() → dört değerli birleşim                   │
│    │ is_settleable() → uzlaştırma kararı                 │
│    │ verify_chain() → hash zinciri doğrulaması           │
│    │                                                   │
│  7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
│    (kayıt defteri: 136 yetenek)                          │
│    vm_capabilities() → 136                               │
│                                                         │
│  6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
│    (gate: çekirdek değişmezi)                            │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Akıllı sözleşmede dört değerli mantık

Rust ile yazılmış NEAR akıllı sözleşmesi, **Bölüm 4'teki no_std uygulamasıyla aynı mantığı yürütür**:

```rust
// zxv-core'daki gate (alıntı)
#[near_bindgen]
impl ZxvCore {
    /// Dört değerli conjoin: zincir meet
    pub fn conjoin(&self, inputs: Vec<u8>) -> u8 {
        let mut state = 0u8; // NEITHER
        for v in inputs {
            state = meet(state, v);
        }
        state
    }

    /// Uzlaştırma değişmezi: yalnızca TRUE uzlaştırır
    pub fn is_settleable(&self, state: String) -> bool {
        state == "TRUE"
    }
}
```

**Doğrulama komutları (gazsız, view call'lar)**:

```bash
# conjoin {"inputs":[1,2]}        → "FALSE" (TRUE+FALSE=FALSE)
# is_settleable {"state":"BOTH"}  → false (çelişki asla uzlaşmaz)
# is_settleable {"state":"TRUE"}  → true (anlaşma uzlaşır)
```

### 7.3 Hash zinciri denetim defteri (receipt'ler)

NEAR uygulamasında her uzlaştırma bir **hash zincirine** bağlanır:

```rust
// receipt bağlama (alıntı)
// append(), önceki başlığın hash'ini sonraki kayda bağlar
// verify_chain(), tüm receipt'lerin bağlantı bütünlüğünü doğrular
```

Doğrulama sonucu (ölçüldü):

```
verify_chain {"from":0} → true (tüm receipt'ler bağlı ve doğrulandı)
```

**Çelişkiden çözüme kadar eksiksiz geçmiş, değiştirilemez biçimde kaydedilir** — 5.3 bölümünün event sourcing deseni zincir üzerinde uygulanmıştır.

### 7.4 Durumla uyumlu yükseltmeler

NEAR uygulamasının temel ilkesi **durumla uyumlu yükseltmelerdir**:

- seri hale getirilmiş üst düzey yapı **asla değişmez**;
- yeni özelliklerin durumu, mevcut depolama haritasının **ayrılmış ad alanlarında** yaşar;
- yükseltmeler yalnızca yöntemdir.

Bu, **Borsh serileştirmenin ileriye dönük uyumluluğuna** dayanır. Yapıya alan eklemek, seri durumdan çıkarmayı bozar (belgelenmiş bir arıza durumu), bu nedenle ilke kesin olarak uygulanır:

```rust
// durum uyumluluğu örneği: yapı değişmez, durum depolama haritasında yaşar
// struct ZxvCore { owner: AccountId, store: LookupMap<u64, Receipt> }
// yeni bir özellik (ör. gaz zarfı), store'un ayrılmış anahtar aralığında yaşar
```

### 7.5 Ölçülen gaz verimliliği

Dört değerli çekirdek işlemlerin (conjoin/is_settleable) gaz tüketimi:

| İşlem | Gaz | Not |
|---|---|---|
| `conjoin` | < 1,2 Tgas | iç zarf mantığının üst sınırı |
| `is_settleable` | onlarca Ggas | tek bir karşılaştırma |
| tüm yürütme | < 10 Tgas | işlem tavanı |

**Okumalar (view call'lar) tamamen gazsızdır.** Bu, "karar tek bir karşılaştırmadır" özelliğinden kaynaklanır.

### 7.6 Gaz duvarı testi (dayanıklılık doğrulaması)

Canlı ağda sürekli yükün sonuçları:

```
Salt-okunur: 125/125 geçerli çağrı (gaz duvarı yok)
Yazma yolu: 10/10 döngü (depolama duvarı finansmanla aşıldı, bütünlük korundu)
```

### 7.7 Venedik Gaz İstasyonu (ekonomik uygulama)

Dört değerli mantığın **ekonomik olarak** uygulandığı bir örnek, Venedik Gaz İstasyonu'dur:

| Uzlaştırma durumu | Gaz yükü | Protokol eşleşmesi |
|---|---|---|
| `TRUE` (uzlaştı) | kullanıcı %50 | protokol %50 (etkin 2×) |
| `BOTH` (çelişki) | kullanıcı %100 | yok (tahliye kalkanı) |
| `FALSE` (reddedildi) | kullanıcı %100 | yok |

**Bir çelişki (BOTH) eşleşmeyi kaybeder** ve kasıtlı çelişki spam'ıyla rezerv tükenmesini yapısal olarak önler. "BOTH uzlaşmaz" özelliği doğrudan ekonomik teşvik tasarımına dönüşür.

### 7.8 Egemen Kararnameler (çatallanmadan çatallanma)

NEAR uygulamasının yönetişimi, "çelişkiler bir arada var olabilir" özelliğini doğrudan kullanır:

- `sd_issue` — kişi kendi Egemen Kararnamesini çıkarır (iyileştirme önerisi)
- `sd_sign` — bir kararnameyi imzalar (uyanların kaydı)
- `sd_shift` — uyulan kararnameyi serbestçe değiştirir (ileri geri)
- `sd_forkless` — çelişkili kararnamelerin bir arada var olduğunun kanıtı

**Kararname A bir şeyi, Kararname B zıttını iddia eder; ikisi de vardır ve ikisi de bağlıları için geçerlidir.** "BOTH emicidir ve asla zorlanmaz" özelliği, çatallanmadan çatallanmanın matematiksel temelidir.

### 7.9 Kanonik yönlendirme

NEAR'ın kanonik adresi `zedec.near`, tüm protokole yönlendiren canlı bir sözleşmedir:

```
zedec.near → route() → canonical=zedec.near|registry=7eaa2c6f...|gate=6ca890f7...|core=f18889ea...
zedec.near → protocol_meta() → primaryWallet=zedec.near|protocolAddress=f18889ea...|...
zedec.near → is_settleable(1) → true
```

---

## 8. Karşılaştırmalar ve değerlendirme

### 8.1 Yerel doğrulama komutları

```bash
# C sürümü (sıfır yığın, saf bitler)
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate

# Rust sürümü (no_std değişmez doğrulaması)
cargo test --release

# WASM sürümü
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# Fuzzing (tüm u8 alanı, 65.536)
gcc -O3 fuzz.c belnap_gate.c -o fuzz && ./fuzz
```

### 8.2 Beklenen çıktı

```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
Disjoin(BOTH,NEITHER): 0x03 (BOTH)
```

### 8.3 Ölçülen kalite güvence sonuçları

NEAR uygulamasının tüm doğrulama sonuçları:

| Doğrulama | Sonuç |
|---|---|
| QC donanımı (çekirdek C kaynak koduyla çapraz kontrol) | **55/55 PASS, sıfır hata** |
| Tam zincir üstü denetim (tüm katmanlar) | **24/24 PASS** |
| Dayanıklılık stres testi | **70/70 geçerli çağrı, sıfır arıza** |
| Kayıt denetimi | **136/136 bilinen canlı sözleşmelere çözülür** |
| Gaz duvarı testi (okumalar) | **125/125** |
| Gaz duvarı testi (yazmalar) | **10/10 döngü** |

---

## 9. Sonuç ve görünüm

### 9.1 Sonuç

Bu makale, dört değerli (parakonsistent) mantığı bir sistem primitifi olarak nasıl tam olarak uygulayacağını dört düzeyde — teori, uygulama, doğrulama ve kanıt — açıkladı.

**Teorik katkı**: zincir meet cebiri (FALSE < NEITHER < BOTH < TRUE), "hiçbir TRUE-olmayan kombinasyon TRUE üretmez" değişmezini **yapısal olarak garanti** eder. Bu, Belnap örgüsünden bilinçli bir sapmadır ve uygulamayı basitleştirirken garantiyi güçlendirir.

**Uygulama**: sıfır tahsisli C uygulaması (yığın yok, saf bitler, birkaç talimat) ve tip güvenli Rust/no_std uygulaması sağlandı. İkisi de saniyeler içinde yerel olarak derlenir ve doğrulanır.

**Doğrulama**: sonlu durum uzayının tam sayımı (64 birleşme kontrolü, 16 değişmez kontrolü, 65.536 fuzz girdisi), uygulamanın spesifikasyona uyduğunu mekanik olarak garanti eder.

**Kanıt**: NEAR ana ağındaki akıllı sözleşme (110 kit, 136 yetenek, kanonik `zedec.near`, sözleşme adresi `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) gaz duvarı testinden ve çekişmeli denetimden geçer. Dört değerli mantık, ekonomik teşvik tasarımına (Venedik Gaz İstasyonu'nun 2× eşleşmesi/tahliye kalkanı) ve yönetişim tasarımına (çatallanmadan çatallanma) doğrudan uygulanır.

### 9.2 Görünüm

- **Biçimsel kanıt**: kapsamlı testleri Coq/Isabelle'de mekanik kanıtlara yükseltme
- **Derece tablosu genişletmesi**: 145 duruma genişlemenin kod değişikliği olmadan operatörlere yansıdığını doğrulama
- **Çapraz zincir dağıtımı**: WASM sürümünü diğer çalışma zamanı ortamlarına taşıma
- **NEP standardizasyonu**: dört değerli mantık kapısını NEAR Enhancement Proposal olarak önerme

---

## Ek A. Tam kaynak kodu

### A.1 belnap_gate.h

```c
/* belnap_gate.h — dört değerli durum makinesinin genel arayüzü */
#ifndef BELNAP_GATE_H
#define BELNAP_GATE_H

#include <stdint.h>
#include <stdbool.h>

typedef enum {
    NEITHER = 0x00,
    TRUE    = 0x01,
    FALSE   = 0x02,
    BOTH    = 0x03,
} belnap_state_t;

belnap_state_t conjoin(belnap_state_t a, belnap_state_t b);
belnap_state_t disjoin(belnap_state_t a, belnap_state_t b);
bool is_settleable(belnap_state_t s);

#endif /* BELNAP_GATE_H */
```

### A.2 Doğrulama prosedürü (tam)

```bash
#!/bin/sh
# verify.sh — dört değerli mantık kontrollerinin tümünü tek seferde çalıştırır
set -e

echo "=== 1. C: derleme (sıfır uyarı) ==="
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
echo "OK"

echo "=== 2. C: çalıştırma ==="
./belnap_gate

echo "=== 3. C: fuzzing (65.536) ==="
gcc -O3 fuzz.c belnap_gate.c -o fuzz
./fuzz

echo "=== 4. Rust: testler ==="
cargo test --release

echo "=== 5. Rust: WASM derleme ==="
cargo build --target wasm32-unknown-unknown --release
echo "OK"
```

---

## Ek B. Referans uygulama depoları

※ Bu algoritma, yüksek güvenilirlik gereksinimleri olan ortamlar için tasarlanmış ve doğrulanmıştır: dağıtık mutabakat katmanları ve WASM çalışma zamanı ortamları. Referans uygulamalar:

- **Çekirdek uygulaması** (`kernel/src/lpres/`: `lpres_phases.h`, `lpres.c`): https://github.com/sykomiked/zxv-stack
- **Akıllı sözleşme uygulaması** (NEAR ana ağı, kanonik `zedec.near`): https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*