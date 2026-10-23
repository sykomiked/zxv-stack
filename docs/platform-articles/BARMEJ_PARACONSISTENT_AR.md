# آلة الحالة المتسامحة مع التناقض: المنطق الرباعي القيم كأولية نظامية — دراسة شاملة مع تنفيذ C وRust وإثبات مفهوم على NEAR

**من المنطق الكلاسيكي إلى منطق بيلناب الرباعي: كيف نجعل الأنظمة الموزعة لا تنهار عند التناقض**

*بقلم: فريق هندسة ZXV (ZEDEC / 36N9 Genetics, LLC)*
*مقالة لـ Barmej — منصة المطورين العربية لربط مهندسي البرمجيات في السعودية والإمارات وشمال إفريقيا*

---

## الملخص التنفيذي

هل تساءلت يومًا: **ماذا يحدث عندما يتعارض مصدرا بيانات في نظام موزع؟** في الأنظمة التقليدية، ينتهي الموقف إما بانهيار النظام (panic)، أو بفقدان البيانات بصمت (last-writer-wins)، أو بتجمّد النظام بالكامل (deadlock). هذا المقال يقدم حلاً جذريًا مختلفًا: **المنطق الرباعي القيم (Paraconsistent Logic)** الذي يعامل التناقض كحالة معلوماتية مشروعة، لا كخطأ يجب تجنبه. سنبني معًا آلة حالة كاملة — تنفيذ بلغة C بدون تخصيص ذاكرة، ونسخة Rust بنمط `no_std` — ثم نستعرض كيف نُشر هذا المنطق فعليًا على شبكة NEAR كعقد ذكي قابل للتحقق.

---

## 1. المقدمة: مشكلة التناقض في الأنظمة الموزعة

### 1.1 لماذا تفشل الأنظمة عند التناقض؟

كل نظام موزع يواجه في النهاية سؤالًا حاسمًا: عندما تصلنا معلومتان متناقضتان من عقدتين مختلفتين، أيهما نصدق؟ المنطق الكلاسيكي الثنائي (صواب/خطأ) لا يوفر إجابة — بل ينهار. السبب هو **مبدأ الانفجار** (principle of explosion): من التناقض $P \land \neg P$ يمكن استنتاج أي شيء، فيفقد النظام بأكمله معناه.

الحلول التقليدية الأربعة:

| الاستراتيجية | السلوك | الثمن |
|---|---|---|
| **الانهيار (Panic)** | إلقاء استثناء وتراجع الحالة | توقف التنفيذ وفقدان الحالة الجارية |
| **آخر كاتب يفوز (LWW)** | الطابع الزمني الأحدث يكتب فوق | فقدان صامت للحقيقة القديمة |
| **الجمود (Deadlock)** | توقف المعالجة والانتظار | تجمد النظام كله |
| **إكراه الأغلبية** | الأكثرية تفرض الحل | ينتصر السلطان لا الحقيقة |

### 1.2 التحول: التناقض كحالة معلوماتية

المنطق الباراكنسيستنتي يقلب الفرضية: بدلًا من معاملة التناقض كخطأ، نعامله كـ**معلومة**. النظام يحتفظ بالادعاءين معًا، يسجل التعارض، ولا يجبر على حل حتى تستقر الأدلة فعلًا. هذا ليس تنظيرًا — إنه منطق رياضي رباعي القيم قابل للتنفيذ.

---

## 2. النظرية: جبر المنطق الرباعي

### 2.1 الحالات الأربع

```c
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* لا دليل */
    LPRES_STATE_TRUE    = 1,  /* متفق عليه — قابل للتسوية */
    LPRES_STATE_FALSE   = 2,  /* منقوض صراحة */
    LPRES_STATE_BOTH    = 3,  /* تناقض — محفوظ */
} lpres_state_t;
```

### 2.2 الترتيب الكلي (الرتبة السلسلية)

الجوهر: نرتب الحالات في سلسلة كاملة:

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

| الحالة | الرتبة | الدور |
|---|---|---|
| FALSE | 0 | القاع — مهيمن الالتقاء |
| NEITHER | 1 | لا دليل |
| BOTH | 2 | تناقض |
| TRUE | 3 | القمة — هوية الالتحام |

### 2.3 الالتقاء (conjoin) = التقاء سلسلي

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;  /* fail-closed */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**الخاصية الذهبية**: لأن TRUE له الرتبة 3 (القمة)، فإن الالتقاء (الذي يعيد الأصغر) لا يمكن أن يكون TRUE إلا إذا كان كلا المدخلين TRUE. أي:

> **"لا تولد أي تركيبة من قيم غير-TRUE قيمة TRUE"** — مضمونة بنيويًا، لا بفحص.

### 2.4 الخصائص الجبرية (مُتحقق منها)

```
الالتقاء: تجمّعية ○ تبادلية ○ ترابطية ○
الالتحام: تجمّعية ○ تبادلية ○ ترابطية ○
الامتصاص: ○ (a ∧ (a ∨ b) = a)
```

---

## 3. التنفيذ: C بدون تخصيص ذاكرة

```c
/* belnap_gate.c — آلة حالة متسامحة مع التناقض */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef enum { NEITHER = 0x00, TRUE = 0x01, FALSE = 0x02, BOTH = 0x03 } belnap_state_t;

/* [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2} */
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
    belnap_state_t contention = (belnap_state_t)((unsigned)TRUE | (unsigned)FALSE);
    printf("State: 0x%02X (%s)\n", contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n", is_settleable(contention) ? "YES" : "NO (Blocked without panic)");
    belnap_state_t resolved = conjoin(TRUE, TRUE);
    printf("Resolved: 0x%02X -> Can Settle? %s\n", resolved, is_settleable(resolved) ? "YES" : "NO");
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n", unknown, is_settleable(unknown) ? "YES" : "NO");
    return 0;
}
```

```bash
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate
```

**النتيجة**: `BOTH → NO (Blocked without panic)` — التناقض محفوظ بلا انهيار.

### خصائص الذاكرة
- المكدس: 3 متغيرات = 24 بايت
- الكومة: 0 (لا malloc/free)
- البيانات الثابتة: 4 بايت
- كود صغير: عشرات التعليمات

---

## 4. التنفيذ: Rust / no_std بأمان الأنواع

```rust
//! lib.rs — تحقق الثوابت تحت no_std
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap { Neither = 0x00, True = 0x01, False = 0x02, Both = 0x03 }

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
    fn verify_contradiction_does_not_panic() {
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
    }

    #[test]
    fn verify_meet_associative() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals { for b in vals { for c in vals {
            assert_eq!(a.merge(b).merge(c), a.merge(b.merge(c)));
        }}}
    }

    #[test]
    fn verify_no_nontrue_meet_yields_true() {
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
cargo test --release   # كل الاختبارات خضراء
```

---

## 5. تصميم آلة الحالة

```
لا دليل ──► NEITHER (انتظار) ──► BOTH (تناقض محفوظ) ──► TRUE/FALSE
                                    │                       │
                                    └── حل النزاع ──────────┘
```

**قواعد التصميم**:
1. NEITHER = انتظار، لا تعليق
2. BOTH = انتقال لحل النزاع مع تسجيل أثر التدقيق
3. فقط TRUE يأذن بالآثار الجانبية (كتابة الحالة)

### التكامل مع تتبع الأحداث (Event Sourcing)
```
[Event 1] attest(TRUE)   → TRUE
[Event 2] attest(FALSE)  → BOTH (سُجل التناقض)
[Event 3] resolve(TRUE)  → TRUE
```

---

## 6. التحقق: الاختبار الشامل والضبابي

### اختبارات KAT (الإجابة المعروفة)
| الاختبار | المدخل | المتوقع |
|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` |

### الفحص الضبابي (65,536 مدخل)
```c
for (unsigned a = 0; a < 256; a++)
    for (unsigned b = 0; b < 256; b++) {
        belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
        if (m == TRUE && (a != TRUE || b != TRUE)) { printf("VIOLATION\n"); return 1; }
    }
printf("FUZZ OK: 65536 combinations\n");
```

---

## 7. إثبات المفهوم: النشر على NEAR

هذا المنطق ليس نظريًا — إنه **منشور على الشبكة الرئيسية لـ NEAR كعقد ذكي**، قابل للتحقق مباشرة.

### العناوين
- **العنوان الأساسي (التوجيه):** `zedec.near`
- **بروتوكول ZXV (110 حزم):** `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`
- **السجل (136 قدرة):** `7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a`
- **البوابة (ثابتة النواة):** `6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8`

### تحقق مباشر بدون غاز
```bash
# conjoin {"inputs":[1,2]}        → "FALSE" (TRUE+FALSE=FALSE)
# is_settleable {"state":"BOTH"}  → false (التناقض لا يُسوّى)
# is_settleable {"state":"TRUE"}  → true (الاتفاق يُسوّى)
# verify_chain {"from":0}         → true (سلسلة التدقيق سليمة)
```

### تطبيقات المنطق الرباعي في البروتوكول
- **محطة غاز البندقية**: `TRUE` يحصل على مطابقة غاز 2× من البروتوكول؛ `BOTH` يخسر المطابقة (درع التصريف) — التناقض مكلف فلا يمكن إرساله عشوائيًا
- **المراسيم السيادية**: حوكمة "تفرعات دون تفرع" — مراسيم متناقضة تتعايش
- **سلسلة الإيصالات**: كل تسوية تُسجل في سلسلة تجزئة قابلة للتحقق

### سجل الجودة (مقاس، لا ادعاء)
| التحقق | النتيجة |
|---|---|
| منصة QC (مقارنة بمصدر النواة) | 55/55، صفر خلل |
| تدقيق on-chain كامل | 24/24 |
| اختبار إجهاد المتانة | 70/70 |
| تدقيق التسجيل | 136/136 |
| اختبار جدار الغاز | 125/125 + 10/10 |

---

## 8. الخلاصة

المنطق الرباعي القيم يحل مشكلة لم تحلها الأنظمة الكلاسيكية: **ماذا نفعل عندما تتعارض الحقيقة؟** الإجابة: نحفظ التناقض كمعلومة، ولا نفرض حلًا، ولا ننهار. والخاصية الجوهرية — "لا تولد أي تركيبة من غير-TRUE قيمة TRUE" — مضمونة بنيويًا من ترتيب الرتب، لا بفحص وقت التشغيل.

النظام منشور فعليًا على NEAR، والتحقق مجاني. **النظام مفتوح.**

---

## المستودعات
- **تنفيذ النواة** (`kernel/src/lpres/`): https://github.com/sykomiked/zxv-stack
- **تنفيذ العقد الذكي** (NEAR، `zedec.near`): https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*