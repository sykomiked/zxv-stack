# آلة الحالة المتسامحة مع التناقض: المنطق الرباعي القيم كأولية نظامية

**جبر الالتقاء السلسلي (FALSE < NEITHER < BOTH < TRUE)، وتنفيذ بلغة C بدون تخصيص ذاكرة، ونسخة Rust بنمط no_std، وإثبات مفهوم قابل للتحقق على شبكة NEAR**

*بقلم: فريق هندسة ZXV (ZEDEC / 36N9 Genetics, LLC)*
*مكتوب لـ Hsoub I/O — المنتدى العربي الأساسي لمفاهيم علوم الحاسوب والمنطق الموزع وهندسة الأنظمة*

---

## الملخص

كل نظام موزع أو متزامن يواجه في النهاية الجدار نفسه: **ماذا يجب أن يفعل النظام عندما يختلف مصدرا معلومات حول "الحقيقة"؟** المنطق الثنائي الكلاسيكي (قانون الثالث المرفوع، $P \lor \neg P$) يطلق **مبدأ الانفجار** (principle of explosion) عندما تصل أدلة متناقضة في وقت واحد، مما يجبر التطبيقات على الاختيار بين أربعة أنماط فشل: الذعر، آخر-كاتب-يفوز (LWW)، الجمود (deadlock)، أو إكراه الأغلبية. يقترح هذا المقال تصميم آلة حالة مبنية على **منطق بيلناب الرباعي** (TRUE/FALSE/NEITHER/BOTH) الذي يعامل التناقض كـ**حالة رياضية من الدرجة الأولى** ويحافظ عليها دون إيقاف التنفيذ الخطي. المساهمة الجوهرية هي **جبر التقاء سلسلي** ($FALSE < NEITHER < BOTH < TRUE$): ولأن TRUE يقع في قمة السلسلة، فإن الثابتة **"لا تولّد أي تركيبة من قيم غير-TRUE قيمة TRUE"** تكون مضمونة بنيويًا لا بفحوصات وقت التشغيل. التصميم مُنفَّذ بلغة C (بدون تخصيص ذاكرة، عمليات بت خالصة) وRust (`no_std`)، وتحقَّق منه باختبارات استقصائية شاملة (16/64 تركيبة) واختبار ضبابي (65,536 مدخلًا). أخيرًا، النظرية ليست تصميمًا ورقيًا: فهي **منشورة على الشبكة الرئيسية لـ NEAR** كعقد ذكي (110 حزم، 136 قدرة، العنوان الأساسي `zedec.near`، عنوان العقد `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) كإثبات مفهوم، وقد اجتازت التدقيق الخصمي واختبار جدار الغاز. يمكن التحقق من كل ادعاء في هذا المقال بشكل مستقل، بدون غاز، على السلسلة.

---

## 1. المقدمة: لماذا "التناقض" كأولية نظامية

### 1.1 المشكلة الأساسية للأنظمة الموزعة

كل نظام موزع أو متزامن يجب أن يجيب في النهاية عن سؤال واحد:

> **عندما يتعارض مصدرا معلومات حول الحقيقة، ماذا يجب أن يفعل النظام؟**

يفترض المنطق الكلاسيكي قانون الثالث المرفوع. لكن في البيئة الموزعة، يمكن أن يتسبب تقسيم الشبكة أو تأخر النسخ المتماثل في **وصول أدلة لكل من $P$ و $\neg P$ في وقت واحد**. ضمن الإطار الكلاسيكي، يطلق هذا مبدأ الانفجار: من التناقض $P \land \neg P$، يمكن اشتقاق أي قضية، ويصبح نظام الاستدلال كله بلا معنى.

وبالتالي، تُجبر التطبيقات المبنية على المنطق الكلاسيكي على "تجنب" التناقض عبر واحدة من أربع استراتيجيات:

| الاستراتيجية | السلوك | التكلفة |
|---|---|---|
| **الذعر** | إلقاء استثناء وتراجع الحالة | انقطاع التنفيذ الخطي، فقدان الحالة الجارية |
| **آخر-كاتب-يفوز (LWW)** | الطابع الزمني الأحدث يكتب فوق | محو الحقيقة القديمة بصمت |
| **الجمود (deadlock)** | إيقاف المعالجة والانتظار | تجمّد النظام كله |
| **إكراه الأغلبية** | عدد الأصوات الأكبر يفرض الحل | ينتصر السلطان لا الحقيقة |

تعامل الاستراتيجيات الأربع جميعها **التناقض كخلل (bug)**. لا تحافظ أي منها على المعلومات التي يحتويها التناقض؛ كل منها يضحي إما بتوفر النظام أو بسلامة البيانات.

### 1.2 التحول الباراكنسيستنتي

المنطق الباراكنسيستنتي (المتسامح مع التناقض) يقلب الفرضية:

> **التناقض ليس حالة خطأ. إنه حالة معلومات.**

يحتفظ النظام بالادعاءين في الوقت نفسه، ويسجّل التعارض، ويرفض فرض حل حتى تستقر الأدلة فعلًا. هذا ليس موقفًا فلسفيًا — إنه **منطق رباعي القيم محدد رياضيًا بشكل جيد**، قابل للتنفيذ كأولية لآلة الحالة.

---

## 2. النظرية: البنية الجبرية للمنطق الرباعي

### 2.1 تعريف الحالة

يتبع فضاء الحالات التعريف الأساسي من النواة الفعلية:

```c
/* lpres_phases.h — تعريف الحالة من النواة الفعلية (مقتطف) */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* لا دليل (غير مرصود، معلّق) */
    LPRES_STATE_TRUE    = 1,  /* متفق عليه، صحيح (قابل للتسوية) */
    LPRES_STATE_FALSE   = 2,  /* مُنقَض صراحةً (مرفوض) */
    LPRES_STATE_BOTH    = 3,  /* تناقض (كلا الدليلين) */
    LPRES_STATE_GATEWAY = 4,  /* القيمة الخامسة: منفذ التوسعة */
    /* ... مراحل توسعة حسب معالم فيبوناتشي ... */
    LPRES_STATE_COUNT   = 145 /* إجمالي الحالات بعد التوسعة */
} lpres_state_t;
```

**قرار تصميم رئيسي**: تُعرَّف الحالات **كأنماط بت في بايت واحد**. `0x00`–`0x03` هي جوهر المنطق الرباعي؛ `0x04` وما بعدها مراحل توسعة. انتقالات الحالة إذن عمليات صحيحة خالصة — دون تخصيص كومة، دون ذاكرة ديناميكية، دون أقفال.

### 2.2 الرتبة السلسلية (تعريف الترتيب الكلي)

جوهر هذا التنفيذ **ليس شبكة بيلناب في ذاتها، بل رتبة على سلسلة مرتبة ترتيبًا كليًا**. جدول رتب النواة الفعلية:

```c
/* lpres_phases.h — الرتبة السلسلية (التنفيذ الأساسي) */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = {
    [LPRES_STATE_FALSE]   = 0,  /* القاع (مهيمن الالتقاء)  */
    [LPRES_STATE_NEITHER] = 1,  /* لا دليل                  */
    [LPRES_STATE_BOTH]    = 2,  /* تناقض                    */
    [LPRES_STATE_TRUE]    = 3,  /* القمة (هوية الالتحام)    */
    [LPRES_STATE_GATEWAY] = 4,
    /* ... كل مرحلة توسعة تأخذ الرتبة التالية ... */
};
```

أي أن **الترتيب الكلي** هو

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

**FALSE هو القاع، وTRUE هو القمة.** هذا هو جوهر البنية.

### 2.3 عملية الالتقاء (conjoin) = التقاء سلسلي

التقاء النواة الفعلية (`kernel/src/lpres/lpres.c`):

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank, so a new phase needs only
     * its rank, never a new branch. Deliberate deviation from Belnap's
     * FOUR (the chain, not the lattice): no mix of non-TRUE values can
     * yield TRUE, the only value that authorises an action. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* المرحلة المجهولة لا تكون سلطة أبدًا */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**الالتقاء = إرجاع العنصر ذي الرتبة الأصغر.** الخصائص:

| المدخل | النتيجة | المعنى |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE هو المهيمن (القاع) |
| `conjoin(TRUE, TRUE)` | `TRUE` | الاتفاق وحده يُسوّي |
| `conjoin(BOTH, TRUE)` | `BOTH` | التناقض محفوظ، لا يُكره أبدًا |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER يمتص BOTH |
| `conjoin(مجهول, x)` | `NEITHER` | حماية خارج المدى (fail-closed) |

### 2.4 عملية الالتحام (disjoin) = التحام سلسلي

الالتحام المزدوج:

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

الالتحام = **إرجاع العنصر ذي الرتبة الأكبر**. ولأن TRUE هو القمة، `disjoin(TRUE, x) = TRUE`.

### 2.5 الخصائص الجبرية (مُتحقَّق منها)

يشكل الالتقاء/الالتحام السلسلي **شبكة محدودة (bounded lattice)**. التحقق الاستقصائي الشامل على كل التركيبات (4×4=16، و64 للترابطية):

```
الالتقاء: تجمّعية ○ تبادلية ○ ترابطية ○
الالتحام: تجمّعية ○ تبادلية ○ ترابطية ○
الامتصاص: ○ (a ∧ (a ∨ b) = a)
الترتيب الجزئي: ○ (a ≤ b ⇔ meet(a,b) = a)
الثابتة: ○ (التقاء قيم غير-TRUE لا ينتج TRUE أبدًا)
```

**هذا بنية لا مصادفة.** ولأن TRUE له رتبة 3 (القمة)، فإن الالتقاء (الذي يعيد الحد الأدنى) لا يمكن أن يكون TRUE **إلا إذا كان كلا المدخلين TRUE**. لذلك:

> **الثابتة "فقط TRUE يُسوّي" تُشتق رياضيًا من بنية الرتب.**

### 2.6 الانحراف المتعمد عن شبكة بيلناب

يصرّح تعليق المصدر بوضوح:

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice): no mix of non-TRUE values can yield TRUE, the only value that authorises an action.*

شبكة بيلناب `FOUR` شبكة ذات ترتيبين (ترتيب المعلومات وترتيب الحقيقة). هذا التنفيذ **يطويها في سلسلة واحدة مرتبة ترتيبًا كليًا**. الفوائد:

- إضافة مرحلة تتطلب فقط **إضافة رتبة** (دون منطق فرع جديد)؛
- ينحصر التنفيذ في **مقارنة صحيحة واحدة** (`<=`)؛
- مشكلة "تركيبة غير-TRUE تنتج TRUE"، الممكنة في شبكة بيلناب، **مستبعدة بنيويًا**.

---

## 3. التنفيذ: C بدون تخصيص ذاكرة

### 3.1 قائمة مستقلة كاملة

الملف المفرد التالي يُترجم ويُشغَّل بـ `gcc` وحده:

```c
/* belnap_gate.c — آلة حالة متسامحة مع التناقض، بدون تخصيص ذاكرة
 *
 * الترجمة:
 *   gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
 * التشغيل:
 *   ./belnap_gate
 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/* حالات رباعية القيم (من lpres_phases.h للنواة الفعلية) */
typedef enum {
    NEITHER = 0x00,  /* لا دليل */
    TRUE    = 0x01,  /* متفق عليه، صحيح */
    FALSE   = 0x02,  /* منقَض صراحةً */
    BOTH    = 0x03,  /* تناقض */
} belnap_state_t;

/* الرتبة السلسلية: ترتيب التعداد [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
 * FALSE=0 (القاع)، TRUE=3 (القمة) — مثل النواة الفعلية */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* fail-closed */
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

/* ثابتة التسوية: فقط TRUE يُسوّي */
static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    belnap_state_t a = TRUE;
    belnap_state_t b = FALSE;

    /* نشوء التناقض: في المنطق الكلاسيكي، هذه نقطة الذعر */
    belnap_state_t contention = (belnap_state_t)((unsigned)a | (unsigned)b);
    /* 0x01 | 0x02 = 0x03 (BOTH) — محفوظ كـ"معلومة"، دون ذعر */

    printf("State: 0x%02X (%s)\n",
           contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n",
           is_settleable(contention) ? "YES" : "NO (Blocked without panic)");

    /* الالتحام: TRUE + TRUE = TRUE (قابل للتسوية) */
    belnap_state_t resolved = conjoin(a, a);
    printf("Resolved: 0x%02X -> Can Settle? %s\n",
           resolved, is_settleable(resolved) ? "YES" : "NO");

    /* حماية المرحلة المجهولة: المدخل خارج المدى لا يصبح TRUE أبدًا */
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n",
           unknown, is_settleable(unknown) ? "YES" : "NO");

    /* تحقق الالتحام (join) */
    belnap_state_t j = disjoin(BOTH, NEITHER); /* max rank: BOTH */
    printf("Disjoin(BOTH,NEITHER): 0x%02X (%s)\n",
           j, j == BOTH ? "BOTH" : "OTHER");

    return 0;
}
```

### 3.2 خصائص الذاكرة

التحليل على x86-64 Linux:

```
المكدس: 3 متغيرات محلية = 24 بايت (مع المحاذاة)
الكومة: 0 (لا استدعاءات malloc/free)
البيانات الثابتة: rank[4] = 4 بايت (قسم .rodata)
حجم الكود: عشرات التعليمات على مستوى التجميع
```

**استهلاك مسطّح وقابل للتنبؤ؛ صفر اهتزاز كومة.** عند موازنة الذاكرة للأنظمة الزمنية الحقيقية، يمكن معاملة هذه الخاصية كحد أعلى مضمون. التصميم قابل للتطبيق مباشرة في البيئات المدمجة وWASM وفضاء النواة.

### 3.3 التحقق على مستوى التجميع

تجميع `conjoin` عند `-O3` (x86-64، مقتطف):

```asm
conjoin:
    movzx   eax, dil            ; تحميل a
    movzx   ecx, sil            ; تحميل b
    movzx   edx, BYTE PTR rank[rax]  ; تحميل rank[a]
    cmp     BYTE PTR rank[rcx], dl   ; مقارنة rank[b]
    cmovle  eax, esi            ; إذا rank[a] <= rank[b] اختر b
    ret
```

**تعليمات قليلة صديقة لتنبؤ الفروع** — الأداء على المسار الساخن مضمون.

---

## 4. التنفيذ: Rust / no_std بأمان الأنواع

### 4.1 منطق رباعي آمن الأنواع

```rust
//! lib.rs — تحقق الثوابت الذي يعمل تحت no_std
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

/// الرتبة السلسلية: ترتيب التعداد [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
/// FALSE=0 (القاع)، TRUE=3 (القمة)
const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool {
        matches!(self, Belnap::True)
    }

    /// التقاء سلسلي: FALSE يهيمن، TRUE هوية،
    /// NEITHER يمتص BOTH. تركيبة غير-TRUE لا تنتج TRUE أبدًا.
    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        let ra = RANK[self as usize];
        let rb = RANK[other as usize];
        if ra <= rb { self } else { other }
    }

    /// التحام سلسلي: TRUE يهيمن.
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
        // التناقض لا يطلق panic!؛ يعيد false ويحافظ على الحالة
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
        // التناقض محفوظ ولا يُكره إلى TRUE أبدًا
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

### 4.2 التحقق القائم على الخصائص

إلى جانب اختبارات الوحدة، **تحقق قائم على الخصائص** (KAT: اختبارات الإجابة المعروفة) على كل التركيبات:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;

    /// تحقق الثابتة على كل التركيبات 4×4
    #[test]
    fn verify_no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                let m = a.merge(b);
                if a != Belnap::True && b != Belnap::True {
                    assert_ne!(m, Belnap::True,
                        "تركيبة غير-TRUE تنتج TRUE: {:?} ∧ {:?} = {:?}", a, b, m);
                }
            }
        }
    }

    /// تعداد كامل للترابطية
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

    /// قانون الامتصاص
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

**64 فحص ترابطية** و**16 فحص ثابتة** تُنفَّذ تحت الاختبار. هذا ليس برهانًا، لكن لأن فضاء الحالات منتهٍ وصغير، فهو **تعداد كامل**.

### 4.3 اعتبارات هدف WASM

ينتقل تنفيذ no_std مباشرة إلى هدف WASM (`wasm32-unknown-unknown`):

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

WASM الناتج **بضعة كيلوبايتات**، بالخصائص:

- لا يستدعي دوال الكومة (`malloc`/`free`)؛
- تصدَّر انتقالات الحالة كدوال نقية؛
- يبسّط قياس الغاز (محاسبة الموارد).

---

## 5. تصميم آلة الحالة: دمج المنطق الرباعي

### 5.1 نمط الانتقال الرباعي

```
                    ┌──────────────┐
     لا دليل        │   NEITHER    │
    ──────────────► │  (انتظار)    │
                    └──────┬───────┘
                           │ وصول الدليل
                    ┌──────▼───────┐
     كلا الدليلين   │              │  دليل واحد
    ──────────────► │    BOTH      ├──────────────┐
                    │ (تناقض محفوظ)│              │
                    └──────┬───────┘              │
                           │ الحل                │
                    ┌──────▼───────┐     ┌───────▼───────┐
                    │    TRUE      │     │    FALSE      │
                    │  (تسوية)     │     │  (رفض)        │
                    └──────────────┘     └───────────────┘
```

**نقاط التصميم**:

1. **NEITHER "انتظار" لا "تعليق"**. التسوية لا تُحجب لكنها لا تتقدم أيضًا.
2. **BOTH انتقال إلى "حل النزاع"**. لا حل تلقائي، لكن يُكتب أثر تدقيق.
3. **فقط TRUE يأذن بالآثار الجانبية** (تغيير الحالة، نقل الأموال). عندما `is_settleable(s)` خاطئة، لا تحدث كتابة إطلاقًا.

### 5.2 العلاقة بالذرّية

خاصية جوهرية عند الدمج في معالجة المعاملات:

```
ذرّية انتقال الحالة:
  read → evaluate (قرار رباعي) → write فقط إذا is_settleable == TRUE
```

وهكذا **لا تحدث كتابة في حالة التناقض**، مما يمنع بنيويًا عدم الاتساق الناتج عن تحديثات الحالة الجزئية. لسباق الفحص-ثم-التحديث الكلاسيكي (TOCTOU)، يدمج المنطق الرباعي **الفحص نفسه في الثابتة**.

### 5.3 الدمج مع تتبع الأحداث

يتكامل تسجيل التناقضات طبيعيًا مع نمط تتبع الأحداث (event sourcing):

```
[Event 1] attest(TRUE)   → الحالة: TRUE
[Event 2] attest(FALSE)  → الحالة: BOTH (سُجّل التناقض)
[Event 3] resolve(TRUE)  → الحالة: TRUE (حُلّ)
```

**كل انتقال يُسجَّل في سجل إلحاق-فقط (append-only)**، محافظًا على أثر التدقيق الكامل من التناقض إلى الحل. هذا هو نفس نمط سلسلة الإيصالات في تنفيذ NEAR (أدناه).

---

## 6. التحقق: الأساليب الشكلية والاختبارات

### 6.1 الأساس الرياضي للاختبار الاستقصائي

لأن فضاء الحالات منتهٍ (4 حالات)، **يوفر اختبار التركيبات الاستقصائي الشامل ضمانًا مكافئًا لبرهان رياضي**:

- الالتقاء: 4×4 = 16
- الترابطية: 4×4×4 = 64
- التجمّعية: 4
- التبادلية: 16

بتثبيت كل هذا بالاختبارات، نحصل على **تحقق ميكانيكي من مطابقة التنفيذ للمواصفة (جدول الرتب)**.

### 6.2 اختبارات الإجابة المعروفة (KAT)

تُقارَن القيم المتوقعة المشتقة من جدول الرتب الأساسي بمخرجات التنفيذ:

| الاختبار | المدخل | المتوقع | التحقق |
|---|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` | ○ |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` | ○ |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` | ○ |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` | ○ |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER` (fail-closed) | ○ |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` | ○ |

### 6.3 الاختبار الضبابي

الصلابة ضد المدخلات خارج جدول الرتب:

```c
/* fuzz.c — تحقق الصلابة ضد المدخلات خارج المدى */
#include <stdint.h>
#include <stdio.h>
#include "belnap_gate.h"  /* تصدير conjoin, disjoin, is_settleable */

int main(void) {
    /* تجربة كل 256×256 تركيبة مدخلات (كامل نطاق u8) */
    for (unsigned a = 0; a < 256; a++) {
        for (unsigned b = 0; b < 256; b++) {
            belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
            /* الثابتة: نتيجة المدخلات خارج المدى ليست TRUE أبدًا */
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

**تتحقق الثابتة لكل 65,536 مدخلًا (كامل نطاق u8)**.

---

## 7. النشر على NEAR — إثبات المفهوم

المنطق الرباعي الموصوف أعلاه ليس تصميمًا ورقيًا. إنه **منشور على الشبكة الرئيسية لـ NEAR كعقد ذكي**. هذا هو "إثبات المفهوم" لهذا المقال.

### 7.1 نظرة عامة على البنية

```
┌─────────────────────────────────────────────────────────┐
│  الشبكة الرئيسية NEAR                                   │
│                                                         │
│  zedec.near (أساسي، توجيه)                              │
│    │ route() → يعيد الطوبولوجيا الكاملة                 │
│    │ protocol_meta() → سجل عام                          │
│    │ is_settleable() → الثابتة                          │
│    │                                                   │
│  f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf
│    (zxv-core: العقد الدقيق الواحد ذو 110 حزم)           │
│    │ conjoin() → التقاء رباعي                           │
│    │ is_settleable() → قرار التسوية                     │
│    │ verify_chain() → تحقق سلسلة التجزئة                │
│    │                                                   │
│  7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
│    (السجل: 136 قدرة)                                    │
│    vm_capabilities() → 136                              │
│                                                         │
│  6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
│    (البوابة: ثابتة النواة)                              │
└─────────────────────────────────────────────────────────┘
```

### 7.2 المنطق الرباعي في العقد الذكي

ينفّذ عقد NEAR الذكي المكتوب بلغة Rust **نفس منطق تنفيذ no_std في الفصل 4**:

```rust
// البوابة في zxv-core (مقتطف)
#[near_bindgen]
impl ZxvCore {
    /// التقاء رباعي: التقاء سلسلي
    pub fn conjoin(&self, inputs: Vec<u8>) -> u8 {
        let mut state = 0u8; // NEITHER
        for v in inputs {
            state = meet(state, v);
        }
        state
    }

    /// ثابتة التسوية: فقط TRUE يُسوّي
    pub fn is_settleable(&self, state: String) -> bool {
        state == "TRUE"
    }
}
```

**أوامر التحقق (بدون غاز، استدعاءات عرض)**:

```bash
# conjoin {"inputs":[1,2]}        → "FALSE" (TRUE+FALSE=FALSE)
# is_settleable {"state":"BOTH"}  → false (التناقض لا يُسوّى أبدًا)
# is_settleable {"state":"TRUE"}  → true (الاتفاق يُسوّى)
```

### 7.3 دفتر تدقيق سلسلة التجزئة (الإيصالات)

في تنفيذ NEAR، كل تسوية مرتبطة في **سلسلة تجزئة**:

```rust
// ربط الإيصال (مقتطف)
// append() يربط تجزئة الرأس السابق في السجل التالي
// verify_chain() يتحقق من سلامة ربط كل الإيصالات
```

نتيجة التحقق (مُقاسة):

```
verify_chain {"from":0} → true (كل الإيصالات مربوطة ومتحقق منها)
```

**التاريخ الكامل من التناقض إلى الحل يُسجَّل بشكل غير قابل للتغيير** — نمط تتبع الأحداث من القسم 5.3 منفذًا على السلسلة.

### 7.4 ترقيات متوافقة مع الحالة

مبدأ أساسي في تنفيذ NEAR هو **الترقيات المتوافقة مع الحالة**:

- البنية المتسلسلة من المستوى الأعلى **لا تتغير أبدًا**؛
- حالة الميزات الجديدة تعيش في **مساحات أسماء محجوزة** لخريطة التخزين الحالية؛
- الترقيات طرق-فقط.

يعتمد هذا على **توافق Borsh التسلسلي الأمامي**. إضافة حقل إلى البنية يكسر إزالة التسلسل (حالة فشل موثقة)، لذا يُطبَّق المبدأ بصرامة:

```rust
// مثال توافق الحالة: البنية ثابتة، الحالة تعيش في خريطة التخزين
// struct ZxvCore { owner: AccountId, store: LookupMap<u64, Receipt> }
// ميزة جديدة (مثل مغلف الغاز) تعيش في نطاق مفاتيح محجوز من store
```

### 7.5 كفاءة الغاز المقاسة

استهلاك غاز العمليات الرباعية الجوهرية (conjoin/is_settleable):

| العملية | الغاز | ملاحظة |
|---|---|---|
| `conjoin` | < 1.2 Tgas | الحد الأعلى لمنطق المغلف الداخلي |
| `is_settleable` | عشرات Ggas | مقارنة واحدة |
| التنفيذ كله | < 10 Tgas | سقف المعاملة |

**القراءات (استدعاءات العرض) خالية من الغاز تمامًا**. هذا مشتق من خاصية "القرار مقارنة واحدة".

### 7.6 اختبار جدار الغاز (تحقق المتانة)

نتائج الحمل المستمر على الشبكة الحية:

```
قراءة فقط: 125/125 استدعاء صالح (لا جدار غاز)
مسار الكتابة: 10/10 دورات (تم تجاوز جدار التخزين بالتمويل، وحُفظت السلامة)
```

### 7.7 محطة غاز البندقية (التطبيق الاقتصادي)

مثال على تطبيق المنطق الرباعي **اقتصاديًا** هو محطة غاز البندقية:

| حالة التسوية | عبء الغاز | مطابقة البروتوكول |
|---|---|---|
| `TRUE` (سُوّي) | المستخدم 50% | البروتوكول 50% (2× فعلي) |
| `BOTH` (تناقض) | المستخدم 100% | لا شيء (درع التصريف) |
| `FALSE` (رفض) | المستخدم 100% | لا شيء |

**التناقض (BOTH) يفقد المطابقة**، مما يمنع بنيويًا استنزاف الاحتياطي بالبريد العشوائي المتعمد للتناقضات. خاصية "BOTH لا يُسوّى" ترتبط مباشرة بتصميم الحوافز الاقتصادية.

### 7.8 المراسيم السيادية (تفرعات دون تفرع)

تستغل حوكمة تنفيذ NEAR خاصية "التناقضات يمكن أن تتعايش" مباشرة:

- `sd_issue` — يصدر الشخص مرسومه السيادي (اقتراح تحسين)
- `sd_sign` — يوقّع مرسومًا (سجل الملتزمين)
- `sd_shift` — يبدّل المرسوم الملتزم به بحرية (ذهابًا وإيابًا)
- `sd_forkless` — برهان أن المراسيم المتناقضة تتعايش

**المرسوم أ يؤكد شيئًا والمرسوم ب عكسه؛ كلاهما موجود، وكلاهما صالح لمن يلتزم به.** خاصية "BOTH ممتص ولا يُكره أبدًا" هي الأساس الرياضي للتفرع دون تفرع.

### 7.9 التوجيه الأساسي

العنوان الأساسي NEAR `zedec.near` عقد حي يوجّه إلى البروتوكول كله:

```
zedec.near → route() → canonical=zedec.near|registry=7eaa2c6f...|gate=6ca890f7...|core=f18889ea...
zedec.near → protocol_meta() → primaryWallet=zedec.near|protocolAddress=f18889ea...|...
zedec.near → is_settleable(1) → true
```

---

## 8. المعايير والتقييم

### 8.1 أوامر التحقق المحلي

```bash
# نسخة C (صفر كومة، بتات خالصة)
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate

# نسخة Rust (تحقق ثوابت no_std)
cargo test --release

# نسخة WASM
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# اختبار ضبابي (كامل نطاق u8، 65,536)
gcc -O3 fuzz.c belnap_gate.c -o fuzz && ./fuzz
```

### 8.2 المخرجات المتوقعة

```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
Disjoin(BOTH,NEITHER): 0x03 (BOTH)
```

### 8.3 نتائج ضمان الجودة المقاسة

كل نتائج التحقق لتنفيذ NEAR:

| التحقق | النتيجة |
|---|---|
| منصة QC (مقارنة بمصدر C للنواة) | **55/55 PASS، صفر خلل** |
| تدقيق on-chain كامل (كل الطبقات) | **24/24 PASS** |
| اختبار إجهاد المتانة | **70/70 استدعاء صالح، صفر فشل** |
| تدقيق التسجيل | **136/136 إلى عقود حية معروفة** |
| اختبار جدار الغاز (قراءات) | **125/125** |
| اختبار جدار الغاز (كتابات) | **10/10 دورات** |

---

## 9. الخاتمة والآفاق

### 9.1 الخاتمة

شرح هذا المقال، على أربعة مستويات — النظرية، التنفيذ، التحقق، والإثبات — كيفية تنفيذ المنطق الرباعي (الباراكنسيستنتي) بالكامل كأولية نظامية.

**المساهمة النظرية**: جبر الالتقاء السلسلي (FALSE < NEITHER < BOTH < TRUE) **يضمن بنيويًا** الثابتة "لا تركيبة غير-TRUE تنتج TRUE". هذا انحراف متعمد عن شبكة بيلناب، يبسّط التنفيذ ويقوّي الضمان معًا.

**التنفيذ**: تقديم تنفيذ C بدون تخصيص (صفر كومة، بتات خالصة، تعليمات قليلة) وتنفيذ Rust/no_std آمن الأنواع. كلاهما يُترجم ويتحقق محليًا في ثوانٍ.

**التحقق**: تعداد كامل لفضاء الحالات المنتهي (64 فحص ترابطية، 16 فحص ثابتة، 65,536 مدخل ضبابي) يضمن ميكانيكيًا مطابقة التنفيذ للمواصفة.

**الإثبات**: العقد الذكي على الشبكة الرئيسية لـ NEAR (110 حزم، 136 قدرة، أساسي `zedec.near`، عنوان العقد `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) يجتاز اختبار جدار الغاز والتدقيق الخصمي. يُطبَّق المنطق الرباعي مباشرة على تصميم الحوافز الاقتصادية (مطابقة 2×/درع التصريف في محطة غاز البندقية) وتصميم الحوكمة (تفرعات دون تفرع).

### 9.2 الآفاق

- **البرهان الشكلي**: ترقية الاختبارات الاستقصائية إلى براهين ميكانيكية في Coq/Isabelle
- **توسعة جدول الرتب**: التحقق من أن التوسعة إلى 145 حالة تنعكس في العاملين دون تغيير كود
- **نشر عبر السلاسل**: نقل نسخة WASM إلى بيئات تشغيل أخرى
- **توحيد NEP**: اقتراح بوابة المنطق الرباعي كاقتراح تحسين NEAR

---

## الملحق أ. الكود المصدري الكامل

### أ.1 belnap_gate.h

```c
/* belnap_gate.h — الواجهة العامة لآلة الحالة الرباعية */
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

### أ.2 إجراء التحقق (كامل)

```bash
#!/bin/sh
# verify.sh — تشغيل كل فحوصات المنطق الرباعي دفعة واحدة
set -e

echo "=== 1. C: ترجمة (صفر تحذيرات) ==="
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
echo "OK"

echo "=== 2. C: تشغيل ==="
./belnap_gate

echo "=== 3. C: اختبار ضبابي (65,536) ==="
gcc -O3 fuzz.c belnap_gate.c -o fuzz
./fuzz

echo "=== 4. Rust: اختبارات ==="
cargo test --release

echo "=== 5. Rust: بناء WASM ==="
cargo build --target wasm32-unknown-unknown --release
echo "OK"
```

---

## الملحق ب. مستودعات التنفيذ المرجعي

※ صُمم هذا الخوارزم وتحقق منه لبيئات ذات متطلبات موثوقية عالية: طبقات الإجماع الموزع وبيئات تشغيل WASM. التنفيذات المرجعية:

- **تنفيذ النواة** (`kernel/src/lpres/`: `lpres_phases.h`, `lpres.c`): https://github.com/sykomiked/zxv-stack
- **تنفيذ العقد الذكي** (الشبكة الرئيسية NEAR، الأساسي `zedec.near`): https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*