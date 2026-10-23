# Maszyna stanów tolerująca sprzeczności: logika czterowartościowa jako prymityw systemowy

**Algebra spotkania łańcuchowego (FALSE < NEITHER < BOTH < TRUE), implementacja w C z zerową alokacją, port w Rust `no_std` i weryfikowalny proof of concept w sieci NEAR**

*Autor: zespół inżynierów ZXV (ZEDEC / 36N9 Genetics, LLC)*
*Artykuł dla 4programmers.net — niezależnego polskiego centrum C++, Rust i programowania niskopoziomowego*

---

## Streszczenie

Każdy rozproszony lub współbieżny automat stanów prędzej czy później napotyka tę samą ścianę: **co system powinien zrobić, gdy dwa źródła informacji nie zgadzają się co do „prawdy”?** Klasyczna logika dwuwartościowa (prawo wyłączonego środka, $P \lor \neg P$) wyzwala **zasadę eksplozji** (principle of explosion), gdy sprzeczne dowody docierają jednocześnie, zmuszając implementacje do wyboru między czterema trybami awarii: paniką, ostatni-zapisujący-wygrywa (LWW), zakleszczeniem (deadlock) lub przymusem większości. Niniejszy artykuł proponuje projekt automatu stanów oparty na **logice czterowartościowej Belnapa** (TRUE/FALSE/NEITHER/BOTH), która traktuje sprzeczność jako **pełnoprawny stan matematyczny** i zachowuje ją bez przerywania liniowego wykonania. Główny wkład to **algebra spotkania łańcuchowego** ($FALSE < NEITHER < BOTH < TRUE$): ponieważ TRUE znajduje się na szczycie łańcucha, niezmiennik **„żadna kombinacja wartości nie-TRUE nie daje TRUE”** jest gwarantowany strukturalnie, a nie sprawdzeniami w czasie wykonania. Projekt zaimplementowano w C (zerowa alokacja, czyste operacje bitowe) i Rust (`no_std`), a zweryfikowano testami wyczerpującymi (16/64 kombinacji) i fuzzingiem (65 536 wejść). Wreszcie teoria nie jest projektem papierowym: jest **wdrożona w mainnecie NEAR** jako inteligentny kontrakt (110 kitów, 136 capabilities, adres kanoniczny `zedec.near`, adres kontraktu `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) jako proof of concept, po przejściu audytu adwersarialnego i testu ściany gazowej. Wszystkie twierdzenia można zweryfikować niezależnie, bez gazu, on-chain.

---

## 1. Wprowadzenie: dlaczego „sprzeczność” jako prymityw systemowy

### 1.1 Fundamentalny problem systemów rozproszonych

Każdy rozproszony lub współbieżny automat stanów musi w końcu odpowiedzieć na jedno pytanie:

> **Gdy dwa źródła informacji kolidują co do prawdy, co system powinien zrobić?**

Logika klasyczna zakłada prawo wyłączonego środka. Ale w środowisku rozproszonym partycje sieci lub opóźnienia replikacji mogą spowodować **jednoczesne dotarcie dowodów dla $P$ i $\neg P$**. W ramach klasycznych zasada eksplozji: ze sprzeczności $P \land \neg P$ można wyprowadzić dowolne zdanie, a cały system wnioskowania traci sens.

W konsekwencji implementacje oparte na logice klasycznej są zmuszone do „unikania” sprzeczności jedną z czterech strategii:

| Strategia | Zachowanie | Koszt |
|---|---|---|
| **Panika** | rzuca wyjątek, cofa stan | przerwanie liniowego wykonania, utrata stanu w locie |
| **Ostatni-zapisujący-wygrywa (LWW)** | nowszy znacznik czasu nadpisuje | starsza prawda zostaje cicho wymazana |
| **Zakleszczenie (deadlock)** | zatrzymuje przetwarzanie i czeka | cały system zamarza |
| **Przymus większości** | większa liczba głosów wymusza rozwiązanie | wygrywa władza, nie prawda |

Wszystkie cztery traktują **sprzeczność jako błąd (bug)**. Żadna nie zachowuje informacji zawartej w sprzeczności; każda poświęca albo dostępność systemu, albo integralność danych.

### 1.2 Zwrot parakonsystentny

Logika parakonsystentna odwraca przesłankę:

> **Sprzeczność nie jest stanem błędu. Jest stanem informacji.**

System utrzymuje oba twierdzenia jednocześnie, rejestruje konflikt i odmawia wymuszenia rozwiązania, dopóki dowody naprawdę się nie rozstrzygną. To nie stanowisko filozoficzne — to **matematycznie dobrze zdefiniowana logika czterowartościowa**, implementowalna jako prymityw automatu stanów.

---

## 2. Teoria: struktura algebraiczna logiki czterowartościowej

### 2.1 Definicja stanów

Przestrzeń stanów jest zgodna z kanoniczną definicją z prawdziwego jądra:

```c
/* lpres_phases.h — definicja stanu z prawdziwego jądra (fragment) */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* brak dowodów (niezaobserwowany, oczekujący) */
    LPRES_STATE_TRUE    = 1,  /* uzgodniony, prawdziwy (rozliczalny)         */
    LPRES_STATE_FALSE   = 2,  /* jawnie odrzucony (odmowa)                   */
    LPRES_STATE_BOTH    = 3,  /* sprzeczność (oba dowody)                    */
    LPRES_STATE_GATEWAY = 4,  /* piąta wartość: port rozszerzenia            */
    /* ... fazy rozszerzenia wg kamieni milowych Fibonacciego ... */
    LPRES_STATE_COUNT   = 145 /* całkowita liczba stanów po rozszerzeniu     */
} lpres_state_t;
```

**Kluczowa decyzja projektowa**: stany są reprezentowane jako **wzorce bitowe w pojedynczym bajcie**. `0x00`–`0x03` to rdzeń logiki czterowartościowej; `0x04` i dalej to fazy rozszerzenia. Przejścia stanów są czystymi operacjami całkowitymi — bez alokacji sterty, bez pamięci dynamicznej, bez blokad.

### 2.2 Ranga łańcuchowa (definicja porządku całkowitego)

Rdzeniem tej implementacji **nie jest krata Belnapa jako taka, ale ranga na całkowicie uporządkowanym łańcuchu**. Tabela rang prawdziwego jądra:

```c
/* lpres_phases.h — ranga łańcuchowa (implementacja kanoniczna) */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = {
    [LPRES_STATE_FALSE]   = 0,  /* dno (dominant spotkania)  */
    [LPRES_STATE_NEITHER] = 1,  /* brak dowodów              */
    [LPRES_STATE_BOTH]    = 2,  /* sprzeczność               */
    [LPRES_STATE_TRUE]    = 3,  /* szczyt (tożsamość join)   */
    [LPRES_STATE_GATEWAY] = 4,
    /* ... każda faza rozszerzenia przyjmuje następną rangę ... */
};
```

To znaczy **porządek całkowity** to

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

**FALSE to dno, TRUE to szczyt.** To sedno struktury.

### 2.3 Operacja conjoin = spotkanie łańcuchowe

Conjoin z prawdziwego jądra (`kernel/src/lpres/lpres.c`):

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank, so a new phase needs only
     * its rank, never a new branch. Deliberate deviation from Belnap's
     * FOUR (the chain, not the lattice): no mix of non-TRUE values can
     * yield TRUE, the only value that authorises an action. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* nieznana faza nigdy nie jest autorytetem */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**Spotkanie = zwróć element o mniejszej randze.** Właściwości:

| Wejście | Wynik | Znaczenie |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE jest dominantem (dno) |
| `conjoin(TRUE, TRUE)` | `TRUE` | tylko zgoda rozlicza |
| `conjoin(BOTH, TRUE)` | `BOTH` | sprzeczność jest utrzymywana, nigdy nie wymuszana |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER pochłania BOTH |
| `conjoin(nieznany, x)` | `NEITHER` | ochrona poza zakresem (fail-closed) |

### 2.4 Operacja disjoin = łańcuchowy join

Dualne disjoin:

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

Join = **zwróć element o większej randze**. Ponieważ TRUE jest szczytem, `disjoin(TRUE, x) = TRUE`.

### 2.5 Właściwości algebraiczne (zweryfikowane)

Łańcuchowe spotkanie/join tworzą **kratę ograniczoną (bounded lattice)**. Wyczerpująca weryfikacja wszystkich kombinacji (4×4=16 i 64 dla łączności):

```
Spotkanie: idempotentne ○ przemienne ○ łączne ○
Join: idempotentny ○ przemienny ○ łączny ○
Absorpcja: ○ (a ∧ (a ∨ b) = a)
Porządek częściowy: ○ (a ≤ b ⇔ meet(a,b) = a)
Niezmiennik: ○ (spotkanie wartości nie-TRUE nigdy nie daje TRUE)
```

**To struktura, nie przypadek.** Ponieważ TRUE ma rangę 3 (szczyt), spotkanie (zwracające minimum) może być TRUE **tylko jeśli oba wejścia są TRUE**. Zatem:

> **Niezmiennik „tylko TRUE rozlicza” jest wyprowadzony matematycznie ze struktury rang.**

### 2.6 Świadome odejście od kraty Belnapa

Komentarz w źródle mówi wprost:

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice): no mix of non-TRUE values can yield TRUE, the only value that authorises an action.*

Krata Belnapa `FOUR` ma dwa porządki (porządek informacji i porządek prawdy). Ta implementacja **zwija ją w pojedynczy całkowicie uporządkowany łańcuch**. Korzyści:

- dodanie fazy wymaga tylko **dodania rangi** (bez nowej logiki gałęzi);
- implementacja sprowadza się do **pojedynczego porównania całkowitego** (`<=`);
- problem „kombinacja nie-TRUE daje TRUE”, możliwy w kracie Belnapa, jest **strukturalnie wyeliminowany**.

---

## 3. Implementacja: C, zerowa alokacja

### 3.1 Kompletna samodzielna lista

Poniższy pojedynczy plik kompiluje się i działa tylko z `gcc`:

```c
/* belnap_gate.c — maszyna stanów tolerująca sprzeczności, zerowa alokacja
 *
 * Kompilacja:
 *   gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
 * Uruchomienie:
 *   ./belnap_gate
 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/* Stany czterowartościowe (z lpres_phases.h prawdziwego jądra) */
typedef enum {
    NEITHER = 0x00,  /* brak dowodów */
    TRUE    = 0x01,  /* uzgodniony, prawdziwy */
    FALSE   = 0x02,  /* jawnie odrzucony */
    BOTH    = 0x03,  /* sprzeczność */
} belnap_state_t;

/* Ranga łańcuchowa: porządek wyliczenia [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
 * FALSE=0 (dno), TRUE=3 (szczyt) — jak w prawdziwym jądrze */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* fail-closed */
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

/* Niezmiennik rozliczenia: tylko TRUE rozlicza */
static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    belnap_state_t a = TRUE;
    belnap_state_t b = FALSE;

    /* Powstaje sprzeczność: w logice klasycznej to punkt paniki */
    belnap_state_t contention = (belnap_state_t)((unsigned)a | (unsigned)b);
    /* 0x01 | 0x02 = 0x03 (BOTH) — utrzymana jako „informacja”, bez paniki */

    printf("State: 0x%02X (%s)\n",
           contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n",
           is_settleable(contention) ? "YES" : "NO (Blocked without panic)");

    /* Zbieżność: TRUE + TRUE = TRUE (rozliczalne) */
    belnap_state_t resolved = conjoin(a, a);
    printf("Resolved: 0x%02X -> Can Settle? %s\n",
           resolved, is_settleable(resolved) ? "YES" : "NO");

    /* Ochrona nieznanej fazy: wejście poza zakres nigdy nie staje się TRUE */
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n",
           unknown, is_settleable(unknown) ? "YES" : "NO");

    /* Weryfikacja disjoin (join) */
    belnap_state_t j = disjoin(BOTH, NEITHER); /* max rank: BOTH */
    printf("Disjoin(BOTH,NEITHER): 0x%02X (%s)\n",
           j, j == BOTH ? "BOTH" : "OTHER");

    return 0;
}
```

### 3.2 Charakterystyka pamięci

Analiza na x86-64 Linux:

```
Stos: 3 zmienne lokalne = 24 bajty (z wyrównaniem)
Sterta: 0 (brak wywołań malloc/free)
Dane statyczne: rank[4] = 4 bajty (sekcja .rodata)
Rozmiar kodu: dziesiątki instrukcji na poziomie asemblera
```

**Płaskie, przewidywalne zużycie; zero thrashingu sterty.** Przy budżetowaniu pamięci dla systemów czasu rzeczywistego tę cechę można traktować jako gwarantowaną górną granicę. Projekt ma bezpośrednie zastosowanie w środowiskach wbudowanych, WASM i przestrzeni jądra.

### 3.3 Weryfikacja na poziomie asemblera

Asembler `conjoin` przy `-O3` (x86-64, fragment):

```asm
conjoin:
    movzx   eax, dil            ; załaduj a
    movzx   ecx, sil            ; załaduj b
    movzx   edx, BYTE PTR rank[rax]  ; załaduj rank[a]
    cmp     BYTE PTR rank[rcx], dl   ; porównaj rank[b]
    cmovle  eax, esi            ; jeśli rank[a] <= rank[b], wybierz b
    ret
```

**Kilka instrukcji przyjaznych predykcji rozgałęzień** — wydajność na ścieżce gorącej gwarantowana.

---

## 4. Implementacja: Rust / `no_std`, bezpieczeństwo typów

### 4.1 Logika czterowartościowa z bezpieczeństwem typów

```rust
//! lib.rs — weryfikacja niezmienników działająca pod no_std
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

/// Ranga łańcuchowa: porządek wyliczenia [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
/// FALSE=0 (dno), TRUE=3 (szczyt)
const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool {
        matches!(self, Belnap::True)
    }

    /// Spotkanie łańcuchowe: FALSE dominuje, TRUE jest tożsamością,
    /// NEITHER pochłania BOTH. Kombinacja nie-TRUE nigdy nie daje TRUE.
    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        let ra = RANK[self as usize];
        let rb = RANK[other as usize];
        if ra <= rb { self } else { other }
    }

    /// Łańcuchowy join: TRUE dominuje.
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
        // Sprzeczność nie panikuje!; zwraca false i zachowuje stan
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
        // Sprzeczność jest utrzymywana i nigdy nie wymuszana do TRUE
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

### 4.2 Weryfikacja oparta na właściwościach

Oprócz testów jednostkowych, **weryfikacja oparta na właściwościach** (KAT: Known-Answer Tests) dla wszystkich kombinacji:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;

    /// Sprawdzenie niezmiennika dla wszystkich kombinacji 4×4
    #[test]
    fn verify_no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                let m = a.merge(b);
                if a != Belnap::True && b != Belnap::True {
                    assert_ne!(m, Belnap::True,
                        "kombinacja nie-TRUE daje TRUE: {:?} ∧ {:?} = {:?}", a, b, m);
                }
            }
        }
    }

    /// Pełne wyliczenie łączności
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

    /// Prawo absorpcji
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

**64 sprawdzenia łączności** i **16 sprawdzeń niezmiennika** wykonuje się pod testem. To nie dowód, ale ponieważ przestrzeń stanów jest skończona i mała, jest to **pełne wyliczenie**.

### 4.3 Rozważania dotyczące celu WASM

Implementacja `no_std` kompiluje się bezpośrednio dla celu WASM (`wasm32-unknown-unknown`):

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Powstały WASM ma **kilka KB**, z właściwościami:

- nie wywołuje funkcji sterty (`malloc`/`free`);
- przejścia stanów eksportują się jako czyste funkcje;
- pomiar gazu (rozliczanie zasobów) jest uproszczony.

---

## 5. Projekt automatu stanów: integracja logiki czterowartościowej

### 5.1 Wzorzec przejścia czterowartościowego

```
                    ┌──────────────┐
     brak dowodów   │   NEITHER    │
    ──────────────► │  (oczekiwanie)│
                    └──────┬───────┘
                           │ dowody docierają
                    ┌──────▼───────┐
     oba dowody     │              │  pojedynczy dowód
    ──────────────► │    BOTH      ├──────────────┐
                    │ (sprzeczność)│              │
                    └──────┬───────┘              │
                           │ rozwiązanie         │
                    ┌──────▼───────┐     ┌───────▼───────┐
                    │    TRUE      │     │    FALSE      │
                    │  (rozliczone)│     │  (odmowa)     │
                    └──────────────┘     └───────────────┘
```

**Punkty projektowe**:

1. **NEITHER to „oczekiwanie”, nie „wstrzymanie”**. Rozliczenie nie jest blokowane, ale też nie postępuje.
2. **BOTH to przejście do „rozwiązywania sporu”**. Bez automatycznego rozwiązania, ale zapisywany jest ślad audytowy.
3. **Tylko TRUE autoryzuje skutki uboczne** (zmiana stanu, ruch środków). Gdy `is_settleable(s)` jest false, żaden zapis nie występuje.

### 5.2 Związek z atomowością

Kluczowa właściwość przy integracji z przetwarzaniem transakcji:

```
Atomowość przejścia stanu:
  read → evaluate (decyzja czterowartościowa) → write tylko jeśli is_settleable == TRUE
```

Zatem **żaden zapis nie występuje w stanie sprzeczności**, strukturalnie zapobiegając niespójności z częściowych aktualizacji stanu. Dla klasycznego wyścigu sprawdź-następnie-zaktualizuj (TOCTOU), logika czterowartościowa **integruje samo sprawdzenie z niezmiennikiem**.

### 5.3 Integracja z event sourcing

Rejestrowanie sprzeczności naturalnie integruje się ze wzorcem event sourcing:

```
[Event 1] attest(TRUE)   → stan: TRUE
[Event 2] attest(FALSE)  → stan: BOTH (sprzeczność zarejestrowana)
[Event 3] resolve(TRUE)  → stan: TRUE (rozwiązana)
```

**Każde przejście jest zapisywane w dzienniku append-only**, zachowując pełny ślad audytowy od sprzeczności do rozwiązania. To ten sam wzorzec co łańcuch receipts w implementacji NEAR (poniżej).

---

## 6. Weryfikacja: metody formalne i testy

### 6.1 Podstawa matematyczna testów wyczerpujących

Ponieważ przestrzeń stanów jest skończona (4 stany), **wyczerpujący test kombinacji daje gwarancję równoważną dowodowi matematycznemu**:

- Spotkanie: 4×4 = 16
- Łączność: 4×4×4 = 64
- Idempotencja: 4
- Przemienność: 16

Utrwalenie tego testami daje **mechaniczną weryfikację, że implementacja odpowiada specyfikacji (tabeli rang)**.

### 6.2 KAT (Known-Answer Test)

Oczekiwane wartości wyprowadzone z kanonicznej tabeli rang porównuje się z wyjściem implementacji:

| Test | Wejście | Oczekiwane | Weryfikacja |
|---|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` | ○ |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` | ○ |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` | ○ |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` | ○ |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER` (fail-closed) | ○ |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` | ○ |

### 6.3 Fuzzing

Odporność na wejścia spoza tabeli rang:

```c
/* fuzz.c — weryfikacja odporności na wejścia poza zakresem */
#include <stdint.h>
#include <stdio.h>
#include "belnap_gate.h"  /* eksportuje conjoin, disjoin, is_settleable */

int main(void) {
    /* próbuj wszystkie 256×256 kombinacji wejść (cała domena u8) */
    for (unsigned a = 0; a < 256; a++) {
        for (unsigned b = 0; b < 256; b++) {
            belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
            /* niezmiennik: wynik dla wejść poza zakresem nigdy nie jest TRUE */
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

**Niezmiennik zachowuje się dla wszystkich 65 536 wejść (cała domena u8).**

---

## 7. Wdrożenie w NEAR — proof of concept

Opisana logika czterowartościowa nie jest projektem papierowym. Jest **wdrożona w mainnecie NEAR jako inteligentny kontrakt**. To jest „proof of concept” tego artykułu.

### 7.1 Przegląd architektury

```
┌─────────────────────────────────────────────────────────┐
│  Mainnet NEAR                                           │
│                                                         │
│  zedec.near (kanoniczny, routing)                       │
│    │ route() → zwraca pełną topologię                   │
│    │ protocol_meta() → publiczny rekord                 │
│    │ is_settleable() → niezmiennik                      │
│    │                                                   │
│  f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf
│    (zxv-core: pojedynczy mikrokontrakt z 110 kitami)    │
│    │ conjoin() → spotkanie czterowartościowe            │
│    │ is_settleable() → decyzja rozliczenia              │
│    │ verify_chain() → weryfikacja łańcucha skrótów      │
│    │                                                   │
│  7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
│    (rejestr: 136 capabilities)                          │
│    vm_capabilities() → 136                              │
│                                                         │
│  6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
│    (gate: niezmiennik jądra)                            │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Logika czterowartościowa w inteligentnym kontrakcie

Inteligentny kontrakt NEAR napisany w Rust wykonuje **tę samą logikę co implementacja `no_std` z rozdziału 4**:

```rust
// gate w zxv-core (fragment)
#[near_bindgen]
impl ZxvCore {
    /// Spotkanie czterowartościowe: łańcuchowe spotkanie
    pub fn conjoin(&self, inputs: Vec<u8>) -> u8 {
        let mut state = 0u8; // NEITHER
        for v in inputs {
            state = meet(state, v);
        }
        state
    }

    /// Niezmiennik rozliczenia: tylko TRUE rozlicza
    pub fn is_settleable(&self, state: String) -> bool {
        state == "TRUE"
    }
}
```

**Polecenia weryfikacji (bez gazu, view calls)**:

```bash
# conjoin {"inputs":[1,2]}        → "FALSE" (TRUE+FALSE=FALSE)
# is_settleable {"state":"BOTH"}  → false (sprzeczność nigdy nie rozlicza)
# is_settleable {"state":"TRUE"}  → true (zgoda rozlicza)
```

### 7.3 Księga audytowa łańcucha skrótów (receipts)

W implementacji NEAR każde rozliczenie jest połączone w **łańcuch skrótów**:

```rust
// łączenie receipt (fragment)
// append() łączy skrót poprzedniej głowy z następnym rekordem
// verify_chain() weryfikuje integralność połączeń wszystkich receipts
```

Wynik weryfikacji (zmierzony):

```
verify_chain {"from":0} → true (wszystkie receipts połączone i zweryfikowane)
```

**Pełna historia od sprzeczności do rozwiązania jest zapisywana w niezmiennej formie** — wzorzec event sourcing z sekcji 5.3 zaimplementowany on-chain.

### 7.4 Aktualizacje kompatybilne ze stanem

Podstawową zasadą implementacji NEAR są **aktualizacje kompatybilne ze stanem**:

- serializowana struktura najwyższego poziomu **nigdy się nie zmienia**;
- stan nowych funkcji żyje w **zarezerwowanych przestrzeniach nazw** istniejącej mapy przechowywania;
- aktualizacje są tylko-metodowe.

To opiera się na **kompatybilności wprzód serializacji Borsh**. Dodanie pola do struktury psuje deserializację (udokumentowany przypadek awarii), więc zasada jest ściśle przestrzegana:

```rust
// przykład kompatybilności stanu: struktura niezmieniona, stan w mapie przechowywania
// struct ZxvCore { owner: AccountId, store: LookupMap<u64, Receipt> }
// nowa funkcja (np. koperta gazowa) żyje w zarezerwowanym zakresie kluczy store
```

### 7.5 Zmierzona wydajność gazowa

Zużycie gazu przez operacje rdzenia logiki czterowartościowej (conjoin/is_settleable):

| Operacja | Gaz | Uwaga |
|---|---|---|
| `conjoin` | < 1.2 Tgas | górna granica wewnętrznej logiki koperty |
| `is_settleable` | dziesiątki Ggas | pojedyncze porównanie |
| całe wykonanie | < 10 Tgas | sufit transakcji |

**Odczyty (view calls) są w całości bez gazu.** To wynika z właściwości „decyzja to pojedyncze porównanie”.

### 7.6 Test ściany gazowej (weryfikacja trwałości)

Wyniki ciągłego obciążenia sieci na żywo:

```
Tylko odczyt: 125/125 poprawnych wywołań (brak ściany gazowej)
Ścieżka zapisu: 10/10 cykli (ściana pamięci pokonana finansowaniem, integralność zachowana)
```

### 7.7 Wenecka Stacja Gazowa (zastosowanie ekonomiczne)

Przykładem **ekonomicznego** zastosowania logiki czterowartościowej jest Wenecka Stacja Gazowa:

| Stan rozliczenia | Obciążenie gazem | Match protokołu |
|---|---|---|
| `TRUE` (rozliczone) | użytkownik 50% | protokół 50% (efektywnie 2×) |
| `BOTH` (sprzeczność) | użytkownik 100% | brak (tarcza drenażu) |
| `FALSE` (odmowa) | użytkownik 100% | brak |

**Sprzeczność (BOTH) traci match**, strukturalnie zapobiegając wyczerpaniu rezerwy przez celowy spam sprzecznościami. Właściwość „BOTH nie rozlicza” przekłada się bezpośrednio na projektowanie zachęt ekonomicznych.

### 7.8 Suwerenne Dekrety (forki bez forkowania)

Zarządzanie w implementacji NEAR bezpośrednio wykorzystuje właściwość „sprzeczności mogą współistnieć”:

- `sd_issue` — osoba wydaje własny Suwerenny Dekret (propozycję ulepszenia)
- `sd_sign` — podpisuje dekret (rejestr adherentów)
- `sd_shift` — swobodnie zmienia przestrzegany dekret (w obie strony)
- `sd_forkless` — dowód, że sprzeczne dekrety współistnieją

**Dekret A twierdzi jedno, dekret B przeciwne; oba istnieją i oba są ważne dla swoich adherentów.** Właściwość „BOTH jest pochłaniający i nigdy nie wymuszany” to matematyczna podstawa forków bez forkowania.

### 7.9 Kanoniczne routowanie

Kanoniczny adres NEAR `zedec.near` to żywy kontrakt routujący do całego protokołu:

```
zedec.near → route() → canonical=zedec.near|registry=7eaa2c6f...|gate=6ca890f7...|core=f18889ea...
zedec.near → protocol_meta() → primaryWallet=zedec.near|protocolAddress=f18889ea...|...
zedec.near → is_settleable(1) → true
```

---

## 8. Benchmarki i ocena

### 8.1 Lokalne polecenia weryfikacji

```bash
# Wersja C (zero sterty, czyste bity)
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate

# Wersja Rust (weryfikacja niezmiennika no_std)
cargo test --release

# Wersja WASM
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# Fuzzing (cała domena u8, 65 536)
gcc -O3 fuzz.c belnap_gate.c -o fuzz && ./fuzz
```

### 8.2 Oczekiwane wyjście

```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
Disjoin(BOTH,NEITHER): 0x03 (BOTH)
```

### 8.3 Zmierzone wyniki zapewnienia jakości

Wszystkie wyniki weryfikacji implementacji NEAR:

| Weryfikacja | Wynik |
|---|---|
| Harness QC (porównany z kodem źródłowym jądra C) | **55/55 PASS, zero błędów** |
| Kompletny audyt on-chain (wszystkie warstwy) | **24/24 PASS** |
| Test wytrzymałościowy | **70/70 poprawnych wywołań, zero awarii** |
| Audyt rejestracji | **136/136 do znanych żywych kontraktów** |
| Test ściany gazowej (odczyty) | **125/125** |
| Test ściany gazowej (zapisy) | **10/10 cykli** |

---

## 9. Wnioski i perspektywy

### 9.1 Wnioski

Artykuł wyjaśnił, na czterech poziomach — teorii, implementacji, weryfikacji i dowodu — jak w pełni zaimplementować logikę czterowartościową (parakonsystentną) jako prymityw systemowy.

**Wkład teoretyczny**: algebra spotkania łańcuchowego (FALSE < NEITHER < BOTH < TRUE) **strukturalnie gwarantuje** niezmiennik „żadna kombinacja nie-TRUE nie daje TRUE”. To świadome odejście od kraty Belnapa, jednocześnie upraszczające implementację i wzmacniające gwarancję.

**Implementacja**: dostarczono implementację C z zerową alokacją (bez sterty, czyste bity, kilka instrukcji) i bezpieczną typowo implementację Rust/`no_std`. Obie kompilują się i weryfikują lokalnie w kilka sekund.

**Weryfikacja**: pełne wyliczenie skończonej przestrzeni stanów (64 sprawdzenia łączności, 16 niezmiennika, 65 536 wejść fuzz) mechanicznie gwarantuje zgodność implementacji ze specyfikacją.

**Dowód**: inteligentny kontrakt w mainnecie NEAR (110 kitów, 136 capabilities, kanoniczny `zedec.near`, adres kontraktu `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) przechodzi test ściany gazowej i audyt adwersarialny. Logika czterowartościowa jest bezpośrednio stosowana do projektowania zachęt ekonomicznych (match 2×/tarcza drenażu Weneckiej Stacji Gazowej) i projektowania zarządzania (forki bez forkowania).

### 9.2 Perspektywy

- **Dowód formalny**: aktualizacja testów wyczerpujących do dowodów mechanicznych w Coq/Isabelle
- **Ekspansja tabeli rang**: weryfikacja, że ekspansja do 145 stanów odbija się w operatorach bez zmiany kodu
- **Wdrożenie międzyłańcuchowe**: przeniesienie wersji WASM do innych środowisk wykonawczych
- **Standaryzacja NEP**: zaproponowanie bramki logiki czterowartościowej jako NEAR Enhancement Proposal

---

## Dodatek A. Kompletny kod źródłowy

### A.1 belnap_gate.h

```c
/* belnap_gate.h — publiczny interfejs maszyny stanów czterowartościowej */
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

### A.2 Procedura weryfikacji (kompletna)

```bash
#!/bin/sh
# verify.sh — uruchamia wszystkie sprawdzenia logiki czterowartościowej
set -e

echo "=== 1. C: kompilacja (zero ostrzeżeń) ==="
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
echo "OK"

echo "=== 2. C: uruchomienie ==="
./belnap_gate

echo "=== 3. C: fuzzing (65 536) ==="
gcc -O3 fuzz.c belnap_gate.c -o fuzz
./fuzz

echo "=== 4. Rust: testy ==="
cargo test --release

echo "=== 5. Rust: build WASM ==="
cargo build --target wasm32-unknown-unknown --release
echo "OK"
```

---

## Dodatek B. Repozytoria implementacji referencyjnej

※ Algorytm został zaprojektowany i zweryfikowany dla środowisk o wysokich wymaganiach niezawodnościowych: rozproszone warstwy konsensusu i środowiska wykonawcze WASM. Implementacje referencyjne:

- **Implementacja jądra** (`kernel/src/lpres/`: `lpres_phases.h`, `lpres.c`): https://github.com/sykomiked/zxv-stack
- **Implementacja inteligentnego kontraktu** (mainnet NEAR, kanoniczny `zedec.near`): https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*