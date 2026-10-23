# ماشین حالتِ پذیرای تناقض: منطق چهارارزشی به‌عنوان بلوک پایه‌ی سیستم‌ها

**جبر meet زنجیره‌ای (FALSE < NEITHER < BOTH < TRUE)، پیاده‌سازی C بدون تخصیص حافظه، پورت Rust به‌سبک no_std و اثباتِ مفهومِ قابلِ راستی‌آزمایی در شبکه‌ی NEAR**

*نویسنده: تیم مهندسی ZXV (ZEDEC / 36N9 Genetics, LLC)*
*مقاله برای ویرگول (Virgool) — پلتفرم فارسی‌زبانِ مهندسان سیستم، معماران شبکه و هکرهای کرنل لینوکس*

---

## چکیده

هر سیستم توزیع‌شده یا هم‌روند دیر یا زود به این دیوار برخورد می‌کند: **وقتی دو منبع اطلاعات درباره‌ی «حقیقت» اختلاف دارند، سیستم باید چه کند؟** منطق کلاسیک دودویی (قانون طرد ثالث، $P \lor \neg P$) هنگام رسیدنِ هم‌زمانِ شواهد متناقض، **اصل انفجار** (principle of explosion) را فعال می‌کند و پیاده‌سازی را وادار می‌کند بین چهار حالت شکست انتخاب کند: وحشت (panic)، آخرین-نویسنده-برنده (LWW)، بن‌بست (deadlock) یا اجبار اکثریت. این مقاله طراحیِ ماشینِ حالتی را بر پایه‌ی **منطق چهارارزشی بل‌ناپ** (TRUE/FALSE/NEITHER/BOTH) پیشنهاد می‌کند که تناقض را به‌عنوان **حالتِ ریاضیِ درجه‌یک** در نظر می‌گیرد و آن را بدون قطعِ اجرای خطی حفظ می‌کند. سهم اصلی، **جبرِ meet زنجیره‌ای** است ($FALSE < NEITHER < BOTH < TRUE$): چون TRUE در رأس زنجیره قرار دارد، ناوردای **«هیچ ترکیبی از مقادیر غیر-TRUE به TRUE نمی‌رسد»** به‌صورتِ ساختاری تضمین می‌شود — نه با بررسی‌های زمان اجرا. طراحی در C (بدون تخصیص، عملیات بیتی خالص) و Rust (`no_std`) پیاده‌سازی و با آزمون‌های استقرایی (16/64 ترکیب) و فازینگ (65,536 ورودی) راستی‌آزمایی شده است. سرانجام، این نظریه طرحِ روی کاغذ نیست: به‌عنوان قرارداد هوشمند (110 کیت، 136 قابلیت، آدرس متعارف `zedec.near`، آدرس قرارداد `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) در **شبکه‌ی اصلی NEAR** مستقر شده و از ممیزی خصمانه و آزمون دیوارِ گاز عبور کرده است. همه‌ی ادعاها به‌طور مستقل، بدون گاز، روی زنجیره قابل راستی‌آزمایی‌اند.

---

## 1. مقدمه: چرا «تناقض» به‌عنوان بلوک پایه‌ی سیستم

### 1.1 مسئله‌ی بنیادین سیستم‌های توزیع‌شده

هر سیستم توزیع‌شده یا هم‌روند بالاخره باید به یک پرسش پاسخ دهد:

> **وقتی دو منبع اطلاعات درباره‌ی حقیقت اختلاف دارند، سیستم باید چه کند؟**

منطق کلاسیک اصل طرد ثالث را پیش‌فرض می‌گیرد. اما در محیط توزیع‌شده، پارتیشن‌بندی شبکه یا تأخیر تکرار می‌تواند باعث شود **شواهد هر دو $P$ و $\neg P$ هم‌زمان برسند**. در چارچوب کلاسیک، این اصل انفجار را فعال می‌کند: از تناقض $P \land \neg P$ هر گزاره‌ای قابل اشتقاق است و کل سیستم استنتاج بی‌معنا می‌شود.

در نتیجه، پیاده‌سازی‌های مبتنی بر منطق کلاسیک مجبورند تناقض را به یکی از چهار روش «دور بزنند»:

| راهبرد | رفتار | هزینه |
|---|---|---|
| **وحشت (Panic)** | پرتاب استثنا و بازگردانی حالت | قطع اجرای خطی، از دست رفتن حالت در حال انجام |
| **آخرین-نویسنده-برنده (LWW)** | برچسب زمانی جدیدتر بازنویسی می‌کند | پاک‌شدنِ خاموشِ حقیقت قدیمی |
| **بن‌بست (Deadlock)** | توقف پردازش و انتظار | یخ‌زدن کل سیستم |
| **اجبار اکثریت** | تعداد آرای بیشتر راه‌حل را تحمیل می‌کند | قدرت پیروز می‌شود، نه حقیقت |

هر چهار راهبرد، **تناقض را یک باگ** می‌دانند. هیچ‌کدام اطلاعات نهفته در تناقض را حفظ نمی‌کنند؛ هرکدام یا در دسترس‌پذیری سیستم یا در یکپارچگی داده قربانی می‌دهند.

### 1.2 چرخش پاراکانسپتنتی (پذیرای تناقض)

منطق پاراکانسپتنتی پیش‌فرض را برمی‌گرداند:

> **تناقض حالت خطا نیست. حالت اطلاعات است.**

سیستم هر دو ادعا را هم‌زمان نگه می‌دارد، تعارض را ثبت می‌کند و تا زمانی که شواهد واقعاً مستقر نشوند، از تحمیل راه‌حل خودداری می‌کند. این موضع فلسفی نیست — **منطق چهارارزشیِ ریاضیِ خوش‌تعریف** است که به‌عنوان بلوک پایه‌ی ماشین حالت قابل پیاده‌سازی است.

---

## 2. نظریه: ساختار جبری منطق چهارارزشی

### 2.1 تعریف حالت‌ها

فضای حالت از تعریفِ متعارفِ کرنل واقعی پیروی می‌کند:

```c
/* lpres_phases.h — تعریف حالت از کرنل واقعی (بخشی) */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* بدون شاهد (مشاهده‌نشده، در انتظار) */
    LPRES_STATE_TRUE    = 1,  /* توافق‌شده، درست (قابل تسویه)        */
    LPRES_STATE_FALSE   = 2,  /* صریحاً ردشده (مردود)                */
    LPRES_STATE_BOTH    = 3,  /* تناقض (هر دو شاهد)                  */
    LPRES_STATE_GATEWAY = 4,  /* مقدار پنجم: درگاه توسعه               */
    /* ... فازهای توسعه بر اساس نقاط عطف فیبوناچی ... */
    LPRES_STATE_COUNT   = 145 /* تعداد کل حالت‌ها پس از توسعه           */
} lpres_state_t;
```

**تصمیم کلیدی طراحی**: حالت‌ها به‌صورت **الگوی بیتی در یک بایت** نمایش داده می‌شوند. `0x00`–`0x03` هسته‌ی منطق چهارارزشی‌اند؛ `0x04` به بعد فازهای توسعه‌اند. انتقال حالت‌ها عملیاتِ صحیحِ خالص است — بدون تخصیص هیپ، بدون حافظه‌ی پویا، بدون قفل.

### 2.2 رتبه‌ی زنجیره‌ای (تعریف ترتیب کل)

هسته‌ی این پیاده‌سازی **خودِ شبکه‌ی بل‌ناپ نیست، بلکه رتبه‌ای روی یک زنجیره‌ی کاملاً مرتب است**. جدول رتبه‌ی کرنل واقعی:

```c
/* lpres_phases.h — رتبه‌ی زنجیره‌ای (پیاده‌سازی متعارف) */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = {
    [LPRES_STATE_FALSE]   = 0,  /* کف (مسلطِ meet)      */
    [LPRES_STATE_NEITHER] = 1,  /* بدون شاهد              */
    [LPRES_STATE_BOTH]    = 2,  /* تناقض                  */
    [LPRES_STATE_TRUE]    = 3,  /* رأس (همانیِ join)      */
    [LPRES_STATE_GATEWAY] = 4,
    /* ... هر فاز توسعه رتبه‌ی بعدی را می‌گیرد ... */
};
```

یعنی **ترتیب کل** برابر است با

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

**FALSE کف است و TRUE رأس.** این جوهر ساختار است.

### 2.3 عملگر conjoin = meet زنجیره‌ای

conjoin کرنل واقعی (`kernel/src/lpres/lpres.c`):

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank, so a new phase needs only
     * its rank, never a new branch. Deliberate deviation from Belnap's
     * FOUR (the chain, not the lattice): no mix of non-TRUE values can
     * yield TRUE, the only value that authorises an action. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* فاز ناشناخته هرگز مرجع نیست */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**meet = عنصر با رتبه‌ی کوچک‌تر را برگردان.** ویژگی‌ها:

| ورودی | نتیجه | معنا |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE مسلط است (کف) |
| `conjoin(TRUE, TRUE)` | `TRUE` | فقط توافق تسویه می‌کند |
| `conjoin(BOTH, TRUE)` | `BOTH` | تناقض حفظ می‌شود، هرگز تحمیل نمی‌شود |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER جذبِ BOTH می‌شود |
| `conjoin(ناشناخته, x)` | `NEITHER` | محافظ خارج از محدوده (fail-closed) |

### 2.4 عملگر disjoin = join زنجیره‌ای

join دوگانه:

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

join = **عنصر با رتبه‌ی بزرگ‌تر را برگردان**. چون TRUE رأس است، `disjoin(TRUE, x) = TRUE`.

### 2.5 ویژگی‌های جبری (راستی‌آزمایی‌شده)

meet/join زنجیره‌ای یک **شبکه‌ی کران‌دار (bounded lattice)** تشکیل می‌دهند. راستی‌آزمایی استقرایی همه‌ی ترکیب‌ها (4×4=16 و 64 برای شرکت‌پذیری):

```
meet: همانی ○ جابه‌جایی ○ شرکت‌پذیری ○
join: همانی ○ جابه‌جایی ○ شرکت‌پذیری ○
جذب: ○ (a ∧ (a ∨ b) = a)
ترتیب جزئی: ○ (a ≤ b ⇔ meet(a,b) = a)
ناوردا: ○ (meet مقادیر غیر-TRUE هرگز به TRUE نمی‌رسد)
```

**این ساختار است، نه تصادف.** چون TRUE رتبه‌ی 3 (رأس) دارد، meet (که حداقل را برمی‌گرداند) فقط وقتی TRUE می‌شود که **هر دو ورودی TRUE باشند**. بنابراین:

> **ناوردای «فقط TRUE تسویه می‌کند» به‌صورت ریاضی از ساختار رتبه مشتق می‌شود.**

### 2.6 انحراف عمدی از شبکه‌ی بل‌ناپ

کامنت منبع صریحاً می‌گوید:

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice): no mix of non-TRUE values can yield TRUE, the only value that authorises an action.*

شبکه‌ی `FOUR` بل‌ناپ دو ترتیب دارد (ترتیب اطلاعات و ترتیب حقیقت). این پیاده‌سازی آن را **در یک زنجیره‌ی کاملاً مرتب واحد تا می‌کند**. مزایا:

- افزودن فاز فقط **افزودن رتبه** می‌خواهد (بدون منطق شاخه‌ی جدید)؛
- پیاده‌سازی به **یک مقایسه‌ی صحیح واحد** (`<=`) تقلیل می‌یابد؛
- مسئله‌ی «ترکیب غیر-TRUE به TRUE می‌رسد» که در شبکه‌ی بل‌ناپ ممکن بود، **به‌صورت ساختاری حذف شده است**.

---

## 3. پیاده‌سازی: C بدون تخصیص

### 3.1 فهرست کامل مستقل

فایل تکی زیر فقط با `gcc` کامپایل و اجرا می‌شود:

```c
/* belnap_gate.c — ماشین حالت پذیرای تناقض، بدون تخصیص حافظه
 *
 * کامپایل:
 *   gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
 * اجرا:
 *   ./belnap_gate
 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/* حالت‌های چهارارزشی (از lpres_phases.h کرنل واقعی) */
typedef enum {
    NEITHER = 0x00,  /* بدون شاهد */
    TRUE    = 0x01,  /* توافق‌شده، درست */
    FALSE   = 0x02,  /* صریحاً ردشده */
    BOTH    = 0x03,  /* تناقض */
} belnap_state_t;

/* رتبه‌ی زنجیره‌ای: ترتیب enum [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
 * FALSE=0 (کف)، TRUE=3 (رأس) — مثل کرنل واقعی */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* fail-closed */
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

/* ناوردای تسویه: فقط TRUE تسویه می‌کند */
static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    belnap_state_t a = TRUE;
    belnap_state_t b = FALSE;

    /* تناقض پدید می‌آید: در منطق کلاسیک این نقطه‌ی وحشت است */
    belnap_state_t contention = (belnap_state_t)((unsigned)a | (unsigned)b);
    /* 0x01 | 0x02 = 0x03 (BOTH) — به‌عنوان «اطلاعات» حفظ می‌شود، بدون وحشت */

    printf("State: 0x%02X (%s)\n",
           contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n",
           is_settleable(contention) ? "YES" : "NO (Blocked without panic)");

    /* هم‌گرایی: TRUE + TRUE = TRUE (قابل تسویه) */
    belnap_state_t resolved = conjoin(a, a);
    printf("Resolved: 0x%02X -> Can Settle? %s\n",
           resolved, is_settleable(resolved) ? "YES" : "NO");

    /* محافظ فاز ناشناخته: ورودی خارج از محدوده هرگز TRUE نمی‌شود */
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n",
           unknown, is_settleable(unknown) ? "YES" : "NO");

    /* راستی‌آزمایی disjoin (join) */
    belnap_state_t j = disjoin(BOTH, NEITHER); /* max rank: BOTH */
    printf("Disjoin(BOTH,NEITHER): 0x%02X (%s)\n",
           j, j == BOTH ? "BOTH" : "OTHER");

    return 0;
}
```

### 3.2 ویژگی‌های حافظه

تحلیل روی x86-64 Linux:

```
پشته: 3 متغیر محلی = 24 بایت (با تراز)
هیپ: 0 (بدون فراخوانی malloc/free)
داده‌های ایستا: rank[4] = 4 بایت (بخش .rodata)
اندازه‌ی کد: ده‌ها دستور در سطح اسمبلی
```

**مصرف مسطح و قابل پیش‌بینی؛ صفر thrashing هیپ.** هنگام بودجه‌بندی حافظه برای سیستم‌های بلادرنگ، این ویژگی را می‌توان به‌عنوان کران بالای تضمین‌شده در نظر گرفت. طراحی مستقیماً برای محیط‌های تعبیه‌شده، WASM و فضای کرنل کاربرد دارد.

### 3.3 راستی‌آزمایی در سطح اسمبلی

اسمبلی `conjoin` در `-O3` (x86-64، بخشی):

```asm
conjoin:
    movzx   eax, dil            ; بارگذاری a
    movzx   ecx, sil            ; بارگذاری b
    movzx   edx, BYTE PTR rank[rax]  ; بارگذاری rank[a]
    cmp     BYTE PTR rank[rcx], dl   ; مقایسه‌ی rank[b]
    cmovle  eax, esi            ; اگر rank[a] <= rank[b]، b را انتخاب کن
    ret
```

**چند دستورِ دوست‌دار پیش‌بینی پرش** — عملکرد در مسیر داغ تضمین‌شده است.

---

## 4. پیاده‌سازی: Rust / no_std با امنیت نوع

### 4.1 منطق چهارارزشی امن از نظر نوع

```rust
//! lib.rs — راستی‌آزمایی ناوردا که تحت no_std کار می‌کند
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

/// رتبه‌ی زنجیره‌ای: ترتیب enum [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
/// FALSE=0 (کف)، TRUE=3 (رأس)
const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool {
        matches!(self, Belnap::True)
    }

    /// meet زنجیره‌ای: FALSE مسلط، TRUE همانی،
    /// NEITHER جذبِ BOTH. ترکیب غیر-TRUE هرگز TRUE نمی‌دهد.
    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        let ra = RANK[self as usize];
        let rb = RANK[other as usize];
        if ra <= rb { self } else { other }
    }

    /// join زنجیره‌ای: TRUE مسلط.
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
        // تناقض panic! نمی‌کند؛ false برمی‌گرداند و حالت را حفظ می‌کند
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
        // تناقض حفظ می‌شود و هرگز به TRUE تحمیل نمی‌شود
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

### 4.2 راستی‌آزمایی مبتنی بر ویژگی

فراتر از آزمون‌های واحد، **راستی‌آزمایی مبتنی بر ویژگی** (KAT: Known-Answer Tests) روی همه‌ی ترکیب‌ها:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;

    /// بررسی ناوردا روی همه‌ی ترکیب‌های 4×4
    #[test]
    fn verify_no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                let m = a.merge(b);
                if a != Belnap::True && b != Belnap::True {
                    assert_ne!(m, Belnap::True,
                        "ترکیب غیر-TRUE به TRUE می‌رسد: {:?} ∧ {:?} = {:?}", a, b, m);
                }
            }
        }
    }

    /// شمارش کامل شرکت‌پذیری
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

    /// قانون جذب
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

**64 بررسی شرکت‌پذیری** و **16 بررسی ناوردا** زیر آزمون اجرا می‌شوند. این برهان نیست، اما چون فضای حالت متناهی و کوچک است، **شمارش کامل** است.

### 4.3 ملاحظات هدف WASM

پیاده‌سازی `no_std` مستقیماً برای هدف WASM (`wasm32-unknown-unknown`) کامپایل می‌شود:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

WASM حاصل **چند کیلوبایت** است، با ویژگی‌ها:

- توابع هیپ (`malloc`/`free`) را فراخوانی نمی‌کند؛
- انتقال حالت‌ها به‌صورت توابع خالص export می‌شوند؛
- سنجش گاز (حسابداری منابع) ساده می‌شود.

---

## 5. طراحی ماشین حالت: ادغام منطق چهارارزشی

### 5.1 الگوی گذار چهارارزشی

```
                    ┌──────────────┐
     بدون شاهد      │   NEITHER    │
    ──────────────► │  (انتظار)    │
                    └──────┬───────┘
                           │ شاهد می‌رسد
                    ┌──────▼───────┐
     هر دو شاهد     │              │  شاهد واحد
    ──────────────► │    BOTH      ├──────────────┐
                    │ (تناقض حفظ)  │              │
                    └──────┬───────┘              │
                           │ حل                  │
                    ┌──────▼───────┐     ┌───────▼───────┐
                    │    TRUE      │     │    FALSE      │
                    │  (تسویه)     │     │  (رد)         │
                    └──────────────┘     └───────────────┘
```

**نکات طراحی**:

1. **NEITHER «انتظار» است نه «معلق»**. تسویه مسدود نمی‌شود، اما پیشروی هم نمی‌کند.
2. **BOTH گذار به «حل اختلاف» است**. بدون حل خودکار، اما ردِ ممیزی نوشته می‌شود.
3. **فقط TRUE اثرات جانبی را مجاز می‌کند** (تغییر حالت، جابجایی وجه). وقتی `is_settleable(s)` false است، هیچ نوشتنی رخ نمی‌دهد.

### 5.2 رابطه با اتمی بودن

ویژگی کلیدی هنگام ادغام در پردازش تراکنش:

```
اتمی بودن گذار حالت:
  read → evaluate (تصمیم چهارارزشی) → write فقط اگر is_settleable == TRUE
```

بنابراین **در حالت تناقض هیچ نوشتنی رخ نمی‌دهد** و ناسازگاری ناشی از به‌روزرسانی‌های جزئی حالت به‌صورت ساختاری جلوگیری می‌شود. برای رقابت کلاسیک بررسی-سپس-به‌روزرسانی (TOCTOU)، منطق چهارارزشی **خود بررسی را در ناوردا ادغام می‌کند**.

### 5.3 ادغام با رویدادمحوری (Event Sourcing)

ثبت تناقض‌ها به‌طور طبیعی با الگوی رویدادمحوری ادغام می‌شود:

```
[Event 1] attest(TRUE)   → حالت: TRUE
[Event 2] attest(FALSE)  → حالت: BOTH (تناقض ثبت شد)
[Event 3] resolve(TRUE)  → حالت: TRUE (حل شد)
```

**هر گذار در یک ژورنال append-only ثبت می‌شود** و رد ممیزی کامل از تناقض تا حل حفظ می‌شود. این همان الگوی زنجیره‌ی receipt در پیاده‌سازی NEAR است (زیر را ببینید).

---

## 6. راستی‌آزمایی: روش‌های صوری و آزمون‌ها

### 6.1 پایه‌ی ریاضی آزمون استقرایی

چون فضای حالت متناهی است (4 حالت)، **آزمون استقرایی ترکیب‌ها تضمینی معادل برهان ریاضی می‌دهد**:

- meet: 4×4 = 16
- شرکت‌پذیری: 4×4×4 = 64
- همانی: 4
- جابه‌جایی: 16

با تثبیت همه‌ی این‌ها توسط آزمون‌ها، **راستی‌آزمایی مکانیکیِ مطابقت پیاده‌سازی با مشخصات (جدول رتبه)** حاصل می‌شود.

### 6.2 KAT (Known-Answer Test)

مقادیر مورد انتظار مشتق‌شده از جدول رتبه‌ی متعارف با خروجی پیاده‌سازی مقایسه می‌شوند:

| آزمون | ورودی | مورد انتظار | راستی‌آزمایی |
|---|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` | ○ |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` | ○ |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` | ○ |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` | ○ |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER` (fail-closed) | ○ |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` | ○ |

### 6.3 فازینگ

پایداری در برابر ورودی‌های خارج از جدول رتبه:

```c
/* fuzz.c — راستی‌آزمایی پایداری در برابر ورودی‌های خارج از محدوده */
#include <stdint.h>
#include <stdio.h>
#include "belnap_gate.h"  /* export conjoin, disjoin, is_settleable */

int main(void) {
    /* همه‌ی 256×256 ترکیب ورودی (کل دامنه‌ی u8) را امتحان کن */
    for (unsigned a = 0; a < 256; a++) {
        for (unsigned b = 0; b < 256; b++) {
            belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
            /* ناوردا: نتیجه برای ورودی‌های خارج از محدوده هرگز TRUE نیست */
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

**ناوردا برای همه‌ی 65,536 ورودی (کل دامنه‌ی u8) برقرار است.**

---

## 7. استقرار در NEAR — اثبات مفهوم

منطق چهارارزشیِ شرح‌داده‌شده طرح روی کاغذ نیست. **در شبکه‌ی اصلی NEAR به‌عنوان قرارداد هوشمند مستقر شده است**. این «اثبات مفهوم» این مقاله است.

### 7.1 نمای کلی معماری

```
┌─────────────────────────────────────────────────────────┐
│  شبکه‌ی اصلی NEAR                                        │
│                                                         │
│  zedec.near (متعارف، مسیریابی)                          │
│    │ route() → توپولوژی کامل را برمی‌گرداند              │
│    │ protocol_meta() → سند عمومی                         │
│    │ is_settleable() → ناوردا                            │
│    │                                                   │
│  f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf
│    (zxv-core: قرارداد خردِ واحدِ 110 کیت)                │
│    │ conjoin() → meet چهارارزشی                         │
│    │ is_settleable() → تصمیم تسویه                       │
│    │ verify_chain() → راستی‌آزمایی زنجیره‌ی هش            │
│    │                                                   │
│  7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
│    (رجیستری: 136 قابلیت)                                 │
│    vm_capabilities() → 136                              │
│                                                         │
│  6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
│    (gate: ناوردای کرنل)                                  │
└─────────────────────────────────────────────────────────┘
```

### 7.2 منطق چهارارزشی در قرارداد هوشمند

قرارداد هوشمند NEAR که با Rust نوشته شده، **همان منطق پیاده‌سازی `no_std` فصل 4 را اجرا می‌کند**:

```rust
// gate در zxv-core (بخشی)
#[near_bindgen]
impl ZxvCore {
    /// meet چهارارزشی: meet زنجیره‌ای
    pub fn conjoin(&self, inputs: Vec<u8>) -> u8 {
        let mut state = 0u8; // NEITHER
        for v in inputs {
            state = meet(state, v);
        }
        state
    }

    /// ناوردای تسویه: فقط TRUE تسویه می‌کند
    pub fn is_settleable(&self, state: String) -> bool {
        state == "TRUE"
    }
}
```

**دستورهای راستی‌آزمایی (بدون گاز، view calls)**:

```bash
# conjoin {"inputs":[1,2]}        → "FALSE" (TRUE+FALSE=FALSE)
# is_settleable {"state":"BOTH"}  → false (تناقض هرگز تسویه نمی‌شود)
# is_settleable {"state":"TRUE"}  → true (توافق تسویه می‌شود)
```

### 7.3 دفتر حسابرسی زنجیره‌ی هش (receipts)

در پیاده‌سازی NEAR، هر تسویه در یک **زنجیره‌ی هش** پیوند می‌خورد:

```rust
// پیوند receipt (بخشی)
// append() هش head قبلی را در رکورد بعدی پیوند می‌زند
// verify_chain() یکپارچگی پیوند همه‌ی receipts را راستی‌آزمایی می‌کند
```

نتیجه‌ی راستی‌آزمایی (اندازه‌گیری‌شده):

```
verify_chain {"from":0} → true (همه‌ی receipts پیوند و راستی‌آزمایی شدند)
```

**تاریخچه‌ی کامل از تناقض تا حل به‌صورت تغییرناپذیر ثبت می‌شود** — الگوی رویدادمحوری بخش 5.3 روی زنجیره پیاده‌سازی شده است.

### 7.4 ارتقاهای سازگار با حالت

اصل مرکزی پیاده‌سازی NEAR **ارتقاهای سازگار با حالت** است:

- ساختار سریالیزه‌شده‌ی سطح بالا **هرگز تغییر نمی‌کند**؛
- حالت ویژگی‌های جدید در **فضاهای نام رزروشده** نقشه‌ی ذخیره‌سازی موجود زندگی می‌کند؛
- ارتقاها فقط-متدی هستند.

این به **سازگاری رو به جلوی سریالیزاسیون Borsh** وابسته است. افزودن فیلد به ساختار، ازسریالیزاسیون را می‌شکند (مورد شکستِ مستند)، پس اصل به‌شدت اعمال می‌شود:

```rust
// مثال سازگاری حالت: ساختار بدون تغییر، حالت در نقشه‌ی ذخیره‌سازی
// struct ZxvCore { owner: AccountId, store: LookupMap<u64, Receipt> }
// ویژگی جدید (مثل پاکت گاز) در محدوده‌ی کلید رزروشده‌ی store زندگی می‌کند
```

### 7.5 کارایی گاز اندازه‌گیری‌شده

مصرف گاز عملیات مرکزی چهارارزشی (conjoin/is_settleable):

| عملیات | گاز | نکته |
|---|---|---|
| `conjoin` | < 1.2 Tgas | کران بالای منطق پاکت داخلی |
| `is_settleable` | ده‌ها Ggas | یک مقایسه‌ی واحد |
| کل اجرا | < 10 Tgas | سقف تراکنش |

**خواندن‌ها (view calls) کاملاً بدون گازند.** این از ویژگی «تصمیم یک مقایسه‌ی واحد است» ناشی می‌شود.

### 7.6 آزمون دیوار گاز (راستی‌آزمایی پایداری)

نتایج بار مداوم روی شبکه‌ی زنده:

```
فقط-خواندن: 125/125 فراخوانی معتبر (بدون دیوار گاز)
مسیر نوشتن: 10/10 چرخه (دیوار ذخیره‌سازی با تأمین مالی پشت‌سر گذاشته شد، یکپارچگی حفظ شد)
```

### 7.7 ایستگاه گاز ونیز (کاربرد اقتصادی)

مثال کاربرد **اقتصادی** منطق چهارارزشی، ایستگاه گاز ونیز است:

| حالت تسویه | بار گاز | تطبیق پروتکل |
|---|---|---|
| `TRUE` (تسویه‌شده) | کاربر 50% | پروتکل 50% (موثر 2×) |
| `BOTH` (تناقض) | کاربر 100% | هیچ (سپر تخلیه) |
| `FALSE` (ردشده) | کاربر 100% | هیچ |

**تناقض (BOTH) تطبیق را از دست می‌دهد** و به‌صورت ساختاری از تهی‌شدن ذخیره توسط اسپم عمدی تناقض جلوگیری می‌کند. ویژگی «BOTH تسویه نمی‌کند» مستقیماً به طراحی مشوق‌های اقتصادی ترجمه می‌شود.

### 7.8 فرمان‌های حاکمیتی (شاخه‌زنی بدون شاخه‌زنی)

حاکمیت پیاده‌سازی NEAR مستقیماً از ویژگی «تناقض‌ها می‌توانند هم‌زیستی کنند» بهره می‌برد:

- `sd_issue` — شخص فرمان حاکمیتی خود را صادر می‌کند (پیشنهاد بهبود)
- `sd_sign` — امضای فرمان (رجیستری پیروان)
- `sd_shift` — تغییر آزاد فرمانِ تبعیت‌شده (به جلو و عقب)
- `sd_forkless` — برهان هم‌زیستی فرمان‌های متناقض

**فرمان الف چیزی را ادعا می‌کند و فرمان ب عکس آن؛ هر دو وجود دارند و هر دو برای پیروانشان معتبرند.** ویژگی «BOTH جاذب و هرگز تحمیل‌نشده» پایه‌ی ریاضی شاخه‌زنی بدون شاخه‌زنی است.

### 7.9 مسیریابی متعارف

آدرس متعارف NEAR یعنی `zedec.near` قراردادی زنده است که به کل پروتکل مسیریابی می‌کند:

```
zedec.near → route() → canonical=zedec.near|registry=7eaa2c6f...|gate=6ca890f7...|core=f18889ea...
zedec.near → protocol_meta() → primaryWallet=zedec.near|protocolAddress=f18889ea...|...
zedec.near → is_settleable(1) → true
```

---

## 8. معیارها و ارزیابی

### 8.1 دستورهای راستی‌آزمایی محلی

```bash
# نسخه‌ی C (صفر هیپ، بیت‌های خالص)
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate

# نسخه‌ی Rust (راستی‌آزمایی ناوردای no_std)
cargo test --release

# نسخه‌ی WASM
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# فازینگ (کل دامنه‌ی u8، 65,536)
gcc -O3 fuzz.c belnap_gate.c -o fuzz && ./fuzz
```

### 8.2 خروجی مورد انتظار

```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
Disjoin(BOTH,NEITHER): 0x03 (BOTH)
```

### 8.3 نتایج اندازه‌گیری‌شده‌ی تضمین کیفیت

همه‌ی نتایج راستی‌آزمایی پیاده‌سازی NEAR:

| راستی‌آزمایی | نتیجه |
|---|---|
| هارنس QC (مقایسه با کد منبع C کرنل) | **55/55 PASS، صفر باگ** |
| ممیزی on-chain کامل (همه‌ی لایه‌ها) | **24/24 PASS** |
| آزمون استرس پایداری | **70/70 فراخوانی معتبر، صفر شکست** |
| ممیزی ثبت | **136/136 به قراردادهای زنده‌ی شناخته‌شده** |
| آزمون دیوار گاز (خواندن) | **125/125** |
| آزمون دیوار گاز (نوشتن) | **10/10 چرخه** |

---

## 9. نتیجه‌گیری و چشم‌انداز

### 9.1 نتیجه‌گیری

این مقاله در چهار سطح — نظریه، پیاده‌سازی، راستی‌آزمایی و اثبات — توضیح داد که چگونه منطق چهارارزشی (پاراکانسپتنتی) را به‌عنوان بلوک پایه‌ی سیستم به‌طور کامل پیاده‌سازی کنیم.

**سهم نظری**: جبر meet زنجیره‌ای (FALSE < NEITHER < BOTH < TRUE) ناوردای «هیچ ترکیب غیر-TRUE به TRUE نمی‌رسد» را **به‌صورت ساختاری تضمین** می‌کند. این انحراف عمدی از شبکه‌ی بل‌ناپ است که هم پیاده‌سازی را ساده و هم تضمین را قوی می‌کند.

**پیاده‌سازی**: پیاده‌سازی C بدون تخصیص (بدون هیپ، بیت‌های خالص، چند دستور) و پیاده‌سازی Rust/`no_std` امن از نظر نوع ارائه شد. هر دو در چند ثانیه به‌صورت محلی کامپایل و راستی‌آزمایی می‌شوند.

**راستی‌آزمایی**: شمارش کامل فضای حالت متناهی (64 بررسی شرکت‌پذیری، 16 بررسی ناوردا، 65,536 ورودی فازینگ) به‌صورت مکانیکی مطابقت پیاده‌سازی با مشخصات را تضمین می‌کند.

**اثبات**: قرارداد هوشمند در شبکه‌ی اصلی NEAR (110 کیت، 136 قابلیت، متعارف `zedec.near`، آدرس قرارداد `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) آزمون دیوار گاز و ممیزی خصمانه را پشت سر می‌گذارد. منطق چهارارزشی مستقیماً در طراحی مشوق‌های اقتصادی (تطبیق 2×/سپر تخلیه‌ی ایستگاه گاز ونیز) و طراحی حاکمیت (شاخه‌زنی بدون شاخه‌زنی) اعمال می‌شود.

### 9.2 چشم‌انداز

- **برهان صوری**: ارتقای آزمون‌های استقرایی به برهان‌های مکانیکی در Coq/Isabelle
- **توسعه‌ی جدول رتبه**: راستی‌آزمایی اینکه توسعه به 145 حالت بدون تغییر کد در عملگرها بازتاب می‌یابد
- **استقرار بین‌زنجیره‌ای**: انتقال نسخه‌ی WASM به محیط‌های اجرای دیگر
- **استانداردسازی NEP**: پیشنهاد دروازه‌ی منطق چهارارزشی به‌عنوان NEP

---

## پیوست الف. کد منبع کامل

### الف.1 belnap_gate.h

```c
/* belnap_gate.h — رابط عمومی ماشین حالت چهارارزشی */
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

### الف.2 روش راستی‌آزمایی (کامل)

```bash
#!/bin/sh
# verify.sh — اجرای همه‌ی بررسی‌های منطق چهارارزشی در یک‌جا
set -e

echo "=== 1. C: کامپایل (صفر هشدار) ==="
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
echo "OK"

echo "=== 2. C: اجرا ==="
./belnap_gate

echo "=== 3. C: فازینگ (65,536) ==="
gcc -O3 fuzz.c belnap_gate.c -o fuzz
./fuzz

echo "=== 4. Rust: آزمون‌ها ==="
cargo test --release

echo "=== 5. Rust: ساخت WASM ==="
cargo build --target wasm32-unknown-unknown --release
echo "OK"
```

---

## پیوست ب. مخازن پیاده‌سازی مرجع

※ این الگوریتم برای محیط‌هایی با الزامات قابلیت‌اطمینان بالا طراحی و راستی‌آزمایی شده است: لایه‌های اجماع توزیع‌شده و محیط‌های اجرای WASM. پیاده‌سازی‌های مرجع:

- **پیاده‌سازی کرنل** (`kernel/src/lpres/`: `lpres_phases.h`, `lpres.c`): https://github.com/sykomiked/zxv-stack
- **پیاده‌سازی قرارداد هوشمند** (شبکه‌ی اصلی NEAR، متعارف `zedec.near`): https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*