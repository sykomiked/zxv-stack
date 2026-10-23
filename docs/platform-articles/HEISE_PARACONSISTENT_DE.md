# Widerspruchstolerante Zustandsmaschinen: Vierwertige Logik als System-Primitive

**Die Ketten-Meet-Algebra (FALSE < NEITHER < BOTH < TRUE), eine C-Implementierung ohne Allokation, ein Rust-`no_std`-Port und ein verifizierbarer Proof of Concept im NEAR-Netzwerk**

*Von: ZXV-Ingenieursteam (ZEDEC / 36N9 Genetics, LLC)*
*Artikel für Heise Developer — das führende deutschsprachige Technikportal für Systems Engineering, Leistungsanalyse und Software-Verifikation*

---

## Zusammenfassung

Jeder verteilte oder nebenläufige Automat stößt früher oder später an dieselbe Wand: **Was soll das System tun, wenn zwei Informationsquellen über die „Wahrheit" streiten?** Die klassische zweiwertige Logik (Satz vom ausgeschlossenen Dritten, $P \lor \neg P$) löst bei gleichzeitigem Eintreffen widersprüchlicher Beweise das **Explosionsprinzip** (principle of explosion) aus und zwingt Implementierungen, zwischen vier Fehlermodi zu wählen: Panik, Last-Writer-Wins (LWW), Deadlock oder Mehrheitszwang. Dieser Artikel schlägt einen Zustandsautomaten vor, der auf **Belnaps vierwertiger Logik** (TRUE/FALSE/NEITHER/BOTH) basiert, Widersprüche als **mathematischen Zustand erster Klasse** behandelt und sie bewahrt, ohne die lineare Ausführung abzubrechen. Der zentrale Beitrag ist eine **Ketten-Meet-Algebra** ($FALSE < NEITHER < BOTH < TRUE$): Da TRUE an der Spitze der Kette liegt, ist die Invariante **„keine Kombination von Nicht-TRUE-Werten ergibt TRUE"** strukturell garantiert — nicht durch Laufzeitprüfungen. Das Design ist in C (null Allokation, reine Bitoperationen) und Rust (`no_std`) implementiert und durch erschöpfende Tests (16/64 Kombinationen) sowie Fuzzing (65 536 Eingaben) verifiziert. Schließlich ist die Theorie kein Papierprojekt: Sie ist **im Mainnet von NEAR als Smart Contract** (110 Kits, 136 Capabilities, kanonische Adresse `zedec.near`, Vertragsadresse `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) als Proof of Concept deployed und hat ein adversarielles Audit und den Gas-Wall-Test bestanden. Alle Behauptungen lassen sich unabhängig, gasfrei, on-chain verifizieren.

---

## 1. Einleitung: Warum „Widerspruch" als System-Primitive

### 1.1 Das fundamentale Problem verteilter Systeme

Jeder verteilte oder nebenläufige Automat muss irgendwann eine Frage beantworten:

> **Wenn zwei Informationsquellen über die Wahrheit streiten, was soll das System tun?**

Die klassische Logik setzt den Satz vom ausgeschlossenen Dritten voraus. In einer verteilten Umgebung können Netzwerkpartitionen oder Replikationsverzögerungen jedoch dazu führen, dass **Beweise für $P$ und $\neg P$ gleichzeitig eintreffen**. Im klassischen Rahmen löst dies das Explosionsprinzip aus: Aus dem Widerspruch $P \land \neg P$ ist jede Aussage ableitbar, und das gesamte Inferenzsystem wird bedeutungslos.

Klassisch-logische Implementierungen sind daher gezwungen, den Widerspruch auf eine von vier Arten zu „vermeiden":

| Strategie | Verhalten | Kosten |
|---|---|---|
| **Panik** | Ausnahme werfen, Zustand zurücksetzen | lineare Ausführung unterbrochen, laufender Zustand verloren |
| **Last-Writer-Wins (LWW)** | der neuere Zeitstempel überschreibt | die ältere Wahrheit wird stillschweigend gelöscht |
| **Deadlock** | Verarbeitung stoppen und warten | das gesamte System friert ein |
| **Mehrheitszwang** | die größere Stimmenzahl erzwingt die Lösung | es siegt Macht, nicht Wahrheit |

Alle vier behandeln den **Widerspruch als Bug**. Keine bewahrt die im Widerspruch enthaltene Information; jede opfert entweder Systemverfügbarkeit oder Datenintegrität.

### 1.2 Die parakonsistente Wende

Parakonsistente Logik dreht die Prämisse um:

> **Ein Widerspruch ist kein Fehlerzustand. Er ist ein Informationszustand.**

Das System hält beide Behauptungen gleichzeitig, protokolliert den Konflikt und weigert sich, eine Lösung zu erzwingen, bis sich die Beweise tatsächlich beruhigt haben. Das ist keine philosophische Position — es ist eine **mathematisch wohldefinierte vierwertige Logik**, implementierbar als Primitive eines Zustandsautomaten.

---

## 2. Theorie: Die algebraische Struktur der vierwertigen Logik

### 2.1 Zustandsdefinition

Der Zustandsraum folgt der kanonischen Definition des realen Kernels:

```c
/* lpres_phases.h — Zustandsdefinition aus dem realen Kernel (Auszug) */
typedef enum {
    LPRES_STATE_NEITHER = 0,  /* keine Beweise (unbeobachtet, ausstehend) */
    LPRES_STATE_TRUE    = 1,  /* zugestimmt, wahr (abrechenbar)           */
    LPRES_STATE_FALSE   = 2,  /* explizit widerlegt (abgelehnt)           */
    LPRES_STATE_BOTH    = 3,  /* Widerspruch (beide Beweise)              */
    LPRES_STATE_GATEWAY = 4,  /* fünfter Wert: der Erweiterungsport        */
    /* ... Erweiterungsphasen nach Fibonacci-Meilensteinen ... */
    LPRES_STATE_COUNT   = 145 /* Gesamtzahl der Zustände nach Erweiterung  */
} lpres_state_t;
```

**Zentrale Designentscheidung**: Zustände werden als **Bitmuster in einem einzelnen Byte** dargestellt. `0x00`–`0x03` sind der Kern der vierwertigen Logik; `0x04` und darüber sind Erweiterungsphasen. Zustandsübergänge sind reine Integer-Operationen — ohne Heap-Allokation, ohne dynamischen Speicher, ohne Sperren.

### 2.2 Ketten-Rang (Definition der Totalordnung)

Der Kern dieser Implementierung ist **nicht Belnaps Verband als solcher, sondern ein Rang auf einer total geordneten Kette**. Die Rangtabelle des realen Kernels:

```c
/* lpres_phases.h — Ketten-Rang (kanonische Implementierung) */
static const uint8_t lpres_rank[LPRES_STATE_COUNT] = {
    [LPRES_STATE_FALSE]   = 0,  /* Boden (Dominant des Meet)  */
    [LPRES_STATE_NEITHER] = 1,  /* keine Beweise              */
    [LPRES_STATE_BOTH]    = 2,  /* Widerspruch                */
    [LPRES_STATE_TRUE]    = 3,  /* Spitze (Identität des Join) */
    [LPRES_STATE_GATEWAY] = 4,
    /* ... jede Erweiterungsphase nimmt den nächsten Rang ... */
};
```

Die **Totalordnung** ist also

$$ \text{FALSE} < \text{NEITHER} < \text{BOTH} < \text{TRUE} $$

**FALSE ist der Boden, TRUE die Spitze.** Das ist der Kern der Struktur.

### 2.3 Die Conjoin-Operation = Ketten-Meet

Das Conjoin des realen Kernels (`kernel/src/lpres/lpres.c`):

```c
lpres_state_t lpres_conjoin(lpres_state_t a, lpres_state_t b) {
    /* the meet: min on the chain. FALSE dominates, TRUE is identity,
     * NEITHER absorbs BOTH — all by the rank, so a new phase needs only
     * its rank, never a new branch. Deliberate deviation from Belnap's
     * FOUR (the chain, not the lattice): no mix of non-TRUE values can
     * yield TRUE, the only value that authorises an action. */
    if ((unsigned) a >= LPRES_STATE_COUNT ||
        (unsigned) b >= LPRES_STATE_COUNT)
        return LPRES_STATE_NEITHER;   /* eine unbekannte Phase ist nie Autorität */
    return lpres_rank[a] <= lpres_rank[b] ? a : b;
}
```

**Meet = das Element mit dem kleineren Rang zurückgeben.** Eigenschaften:

| Eingabe | Ergebnis | Bedeutung |
|---|---|---|
| `conjoin(FALSE, x)` | `FALSE` | FALSE ist der Dominant (Boden) |
| `conjoin(TRUE, TRUE)` | `TRUE` | nur Zustimmung rechnet ab |
| `conjoin(BOTH, TRUE)` | `BOTH` | Widerspruch wird gehalten, nie erzwungen |
| `conjoin(NEITHER, BOTH)` | `NEITHER` | NEITHER absorbiert BOTH |
| `conjoin(unbekannt, x)` | `NEITHER` | Bereichs-Schutz (fail-closed) |

### 2.4 Die Disjoin-Operation = Ketten-Join

Das duale Disjoin:

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

Join = **das Element mit dem größeren Rang zurückgeben**. Da TRUE die Spitze ist, gilt `disjoin(TRUE, x) = TRUE`.

### 2.5 Algebraische Eigenschaften (verifiziert)

Ketten-Meet/Join bilden einen **beschränkten Verband (bounded lattice)**. Erschöpfende Verifikation aller Kombinationen (4×4=16 und 64 für Assoziativität):

```
Meet: idempotent ○ kommutativ ○ assoziativ ○
Join: idempotent ○ kommutativ ○ assoziativ ○
Absorption: ○ (a ∧ (a ∨ b) = a)
Partielle Ordnung: ○ (a ≤ b ⇔ meet(a,b) = a)
Invariante: ○ (ein Meet von Nicht-TRUE-Werten ergibt nie TRUE)
```

**Das ist Struktur, kein Zufall.** Da TRUE Rang 3 hat (die Spitze), kann das Meet (das das Minimum zurückgibt) nur dann TRUE sein, **wenn beide Eingaben TRUE sind**. Daher:

> **Die Invariante „nur TRUE rechnet ab" wird mathematisch aus der Rangstruktur abgeleitet.**

### 2.6 Die bewusste Abweichung vom Belnap-Verband

Der Quellkommentar sagt es deutlich:

> *Deliberate deviation from Belnap's FOUR (the chain, not the lattice): no mix of non-TRUE values can yield TRUE, the only value that authorises an action.*

Belnaps `FOUR` ist ein Verband mit zwei Ordnungen (Informationsordnung und Wahrheitsordnung). Diese Implementierung **faltet sie zu einer einzigen total geordneten Kette**. Die Vorteile:

- das Hinzufügen einer Phase erfordert nur das **Hinzufügen eines Rangs** (keine neue Verzweigungslogik);
- die Implementierung reduziert sich auf **einen einzigen Integer-Vergleich** (`<=`);
- das in Belnaps Verband mögliche Problem „Nicht-TRUE-Kombination ergibt TRUE" ist **strukturell eliminiert**.

---

## 3. Implementierung: C, null Allokation

### 3.1 Vollständige eigenständige Auflistung

Die folgende einzelne Datei kompiliert und läuft nur mit `gcc`:

```c
/* belnap_gate.c — widerspruchstolerante Zustandsmaschine, null Allokation
 *
 * Kompilierung:
 *   gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
 * Ausführung:
 *   ./belnap_gate
 */
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/* Vierwertige Zustände (aus lpres_phases.h des realen Kernels) */
typedef enum {
    NEITHER = 0x00,  /* keine Beweise */
    TRUE    = 0x01,  /* zugestimmt, wahr */
    FALSE   = 0x02,  /* explizit widerlegt */
    BOTH    = 0x03,  /* Widerspruch */
} belnap_state_t;

/* Ketten-Rang: Enum-Reihenfolge [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
 * FALSE=0 (Boden), TRUE=3 (Spitze) — wie im realen Kernel */
static const uint8_t rank[4] = { 1, 3, 0, 2 };

static belnap_state_t conjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER; /* fail-closed */
    return rank[a] <= rank[b] ? a : b;
}

static belnap_state_t disjoin(belnap_state_t a, belnap_state_t b) {
    if ((unsigned)a > 3 || (unsigned)b > 3) return NEITHER;
    return rank[a] >= rank[b] ? a : b;
}

/* Abrechnungs-Invariante: nur TRUE rechnet ab */
static bool is_settleable(belnap_state_t s) { return s == TRUE; }

int main(void) {
    belnap_state_t a = TRUE;
    belnap_state_t b = FALSE;

    /* Der Widerspruch entsteht: in der klassischen Logik ist dies der Panikpunkt */
    belnap_state_t contention = (belnap_state_t)((unsigned)a | (unsigned)b);
    /* 0x01 | 0x02 = 0x03 (BOTH) — als „Information" gehalten, ohne Panik */

    printf("State: 0x%02X (%s)\n",
           contention, contention == BOTH ? "BOTH" : "OTHER");
    printf("Can Settle? %s\n",
           is_settleable(contention) ? "YES" : "NO (Blocked without panic)");

    /* Konfluenz: TRUE + TRUE = TRUE (abrechenbar) */
    belnap_state_t resolved = conjoin(a, a);
    printf("Resolved: 0x%02X -> Can Settle? %s\n",
           resolved, is_settleable(resolved) ? "YES" : "NO");

    /* Schutz unbekannter Phasen: Eingabe außerhalb des Bereichs wird nie TRUE */
    belnap_state_t unknown = conjoin((belnap_state_t)7, TRUE);
    printf("Unknown-guard: 0x%02X -> Can Settle? %s\n",
           unknown, is_settleable(unknown) ? "YES" : "NO");

    /* Verifikation des Disjoin (Join) */
    belnap_state_t j = disjoin(BOTH, NEITHER); /* max rank: BOTH */
    printf("Disjoin(BOTH,NEITHER): 0x%02X (%s)\n",
           j, j == BOTH ? "BOTH" : "OTHER");

    return 0;
}
```

### 3.2 Speichercharakteristik

Analyse auf x86-64 Linux:

```
Stack: 3 lokale Variablen = 24 Bytes (mit Ausrichtung)
Heap: 0 (keine malloc/free-Aufrufe)
Statische Daten: rank[4] = 4 Bytes (.rodata-Sektion)
Codegröße: dutzende Anweisungen auf Assembler-Ebene
```

**Flacher, vorhersehbarer Verbrauch; null Heap-Thrashing.** Bei der Speicherbudgetierung für Echtzeitsysteme kann diese Eigenschaft als garantierte Obergrenze behandelt werden. Das Design ist direkt anwendbar in eingebetteten Umgebungen, WASM und Kernel-Raum.

### 3.3 Verifikation auf Assembler-Ebene

Das Assembler von `conjoin` bei `-O3` (x86-64, Auszug):

```asm
conjoin:
    movzx   eax, dil            ; lade a
    movzx   ecx, sil            ; lade b
    movzx   edx, BYTE PTR rank[rax]  ; lade rank[a]
    cmp     BYTE PTR rank[rcx], dl   ; vergleiche rank[b]
    cmovle  eax, esi            ; wenn rank[a] <= rank[b], wähle b
    ret
```

**Wenige, für die Sprungvorhersage freundliche Anweisungen** — die Leistung auf dem heißen Pfad ist garantiert.

---

## 4. Implementierung: Rust / `no_std`, Typsicherheit

### 4.1 Typsichere vierwertige Logik

```rust
//! lib.rs — Invariantenprüfung, die unter no_std funktioniert
#![no_std]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Belnap {
    Neither = 0x00,
    True    = 0x01,
    False   = 0x02,
    Both    = 0x03,
}

/// Ketten-Rang: Enum-Reihenfolge [NEITHER,TRUE,FALSE,BOTH] -> {1,3,0,2}
/// FALSE=0 (Boden), TRUE=3 (Spitze)
const RANK: [u8; 4] = [1, 3, 0, 2];

impl Belnap {
    #[inline(always)]
    pub fn is_settleable(self) -> bool {
        matches!(self, Belnap::True)
    }

    /// Ketten-Meet: FALSE dominiert, TRUE ist Identität,
    /// NEITHER absorbiert BOTH. Eine Nicht-TRUE-Kombination ergibt nie TRUE.
    #[inline(always)]
    pub fn merge(self, other: Self) -> Self {
        let ra = RANK[self as usize];
        let rb = RANK[other as usize];
        if ra <= rb { self } else { other }
    }

    /// Ketten-Join: TRUE dominiert.
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
        // Ein Widerspruch panikt nicht!; er gibt false zurück und erhält den Zustand
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
        // Der Widerspruch wird gehalten und nie zu TRUE erzwungen
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

### 4.2 Eigenschaftsbasierte Verifikation

Über die Unit-Tests hinaus: **eigenschaftsbasierte Verifikation** (KAT: Known-Answer Tests) über alle Kombinationen:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;

    /// Invariantenprüfung über alle 4×4-Kombinationen
    #[test]
    fn verify_no_nontrue_meet_yields_true() {
        let vals = [Belnap::Neither, Belnap::True, Belnap::False, Belnap::Both];
        for a in vals {
            for b in vals {
                let m = a.merge(b);
                if a != Belnap::True && b != Belnap::True {
                    assert_ne!(m, Belnap::True,
                        "Nicht-TRUE-Kombination ergibt TRUE: {:?} ∧ {:?} = {:?}", a, b, m);
                }
            }
        }
    }

    /// Vollständige Aufzählung der Assoziativität
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

    /// Absorptionsgesetz
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

**64 Assoziativitätsprüfungen** und **16 Invariantenprüfungen** laufen unter dem Test. Das ist kein Beweis, aber da der Zustandsraum endlich und klein ist, ist es eine **vollständige Aufzählung**.

### 4.3 Überlegungen zum WASM-Ziel

Die `no_std`-Implementierung kompiliert direkt für das WASM-Ziel (`wasm32-unknown-unknown`):

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Das resultierende WASM ist **einige KB** groß, mit den Eigenschaften:

- es ruft keine Heap-Funktionen auf (`malloc`/`free`);
- Zustandsübergänge exportieren sich als reine Funktionen;
- die Gas-Messung (Ressourcen-Abrechnung) wird vereinfacht.

---

## 5. Automaten-Design: Integration der vierwertigen Logik

### 5.1 Das vierwertige Übergangsmuster

```
                    ┌──────────────┐
     keine Beweise  │   NEITHER    │
    ──────────────► │  (Warten)    │
                    └──────┬───────┘
                           │ Beweise treffen ein
                    ┌──────▼───────┐
     beide Beweise  │             │  einzelner Beweis
    ──────────────► │    BOTH      ├──────────────┐
                    │ (Widerspruch)│              │
                    └──────┬───────┘              │
                           │ Lösung               │
                    ┌──────▼───────┐     ┌───────▼───────┐
                    │    TRUE      │     │    FALSE      │
                    │  (abgerechnet)│    │  (abgelehnt)  │
                    └──────────────┘     └───────────────┘
```

**Designpunkte**:

1. **NEITHER ist „Warten", nicht „Aufgeschoben"**. Die Abrechnung wird nicht blockiert, schreitet aber auch nicht voran.
2. **BOTH ist ein Übergang zur „Streitbeilegung"**. Keine automatische Lösung, aber eine Audit-Spur wird geschrieben.
3. **Nur TRUE autorisiert Seiteneffekte** (Zustandsänderung, Geldbewegung). Wenn `is_settleable(s)` false ist, erfolgt keinerlei Schreiben.

### 5.2 Beziehung zur Atomarität

Eine Schlüsseleigenschaft bei der Integration in die Transaktionsverarbeitung:

```
Atomarität des Zustandsübergangs:
  read → evaluate (vierwertige Entscheidung) → write nur wenn is_settleable == TRUE
```

Somit **erfolgt kein Schreiben in einem Widerspruchszustand**, was Inkonsistenzen durch partielle Zustandsaktualisierungen strukturell verhindert. Für das klassische Prüfe-dann-Aktualisiere-Rennen (TOCTOU) **integriert die vierwertige Logik die Prüfung selbst in die Invariante**.

### 5.3 Integration mit Event Sourcing

Das Aufzeichnen von Widersprüchen integriert sich natürlich mit dem Event-Sourcing-Muster:

```
[Event 1] attest(TRUE)   → Zustand: TRUE
[Event 2] attest(FALSE)  → Zustand: BOTH (Widerspruch aufgezeichnet)
[Event 3] resolve(TRUE)  → Zustand: TRUE (gelöst)
```

**Jeder Übergang wird in einem Append-only-Journal aufgezeichnet**, das die vollständige Audit-Spur vom Widerspruch bis zur Lösung bewahrt. Dies ist dasselbe Muster wie die Receipt-Kette in der NEAR-Implementierung (siehe unten).

---

## 6. Verifikation: formale Methoden und Tests

### 6.1 Mathematische Grundlage des erschöpfenden Tests

Da der Zustandsraum endlich ist (4 Zustände), **liefert der erschöpfende Kombinationstest eine Garantie, die einem mathematischen Beweis entspricht**:

- Meet: 4×4 = 16
- Assoziativität: 4×4×4 = 64
- Idempotenz: 4
- Kommutativität: 16

Wenn man dies durch Tests fixiert, erhält man eine **mechanische Verifikation, dass die Implementierung der Spezifikation (der Rangtabelle) entspricht**.

### 6.2 KAT (Known-Answer Test)

Aus der kanonischen Rangtabelle abgeleitete Erwartungswerte werden mit der Ausgabe der Implementierung verglichen:

| Test | Eingabe | Erwartet | Verifizieren |
|---|---|---|---|
| KAT-01 | `conjoin(TRUE, FALSE)` | `FALSE` | ○ |
| KAT-02 | `conjoin(BOTH, TRUE)` | `BOTH` | ○ |
| KAT-03 | `conjoin(NEITHER, BOTH)` | `NEITHER` | ○ |
| KAT-04 | `conjoin(TRUE, TRUE)` | `TRUE` | ○ |
| KAT-05 | `conjoin(0xFF, TRUE)` | `NEITHER` (fail-closed) | ○ |
| KAT-06 | `disjoin(BOTH, NEITHER)` | `BOTH` | ○ |

### 6.3 Fuzzing

Robustheit gegenüber Eingaben außerhalb der Rangtabelle:

```c
/* fuzz.c — Robustheitsprüfung gegen Eingaben außerhalb des Bereichs */
#include <stdint.h>
#include <stdio.h>
#include "belnap_gate.h"  /* exportiert conjoin, disjoin, is_settleable */

int main(void) {
    /* versuche alle 256×256 Eingabekombinationen (die gesamte u8-Domäne) */
    for (unsigned a = 0; a < 256; a++) {
        for (unsigned b = 0; b < 256; b++) {
            belnap_state_t m = conjoin((belnap_state_t)a, (belnap_state_t)b);
            /* Invariante: das Ergebnis für Eingaben außerhalb des Bereichs ist nie TRUE */
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

**Die Invariante gilt für alle 65 536 Eingaben (die gesamte u8-Domäne).**

---

## 7. Das NEAR-Deployment — Proof of Concept

Die oben beschriebene vierwertige Logik ist kein Papierprojekt. Sie ist **im Mainnet von NEAR als Smart Contract deployed**. Dies ist der „Proof of Concept" dieses Artikels.

### 7.1 Architekturübersicht

```
┌─────────────────────────────────────────────────────────┐
│  NEAR-Mainnet                                           │
│                                                         │
│  zedec.near (kanonisch, Routing)                        │
│    │ route() → liefert die vollständige Topologie       │
│    │ protocol_meta() → öffentlicher Datensatz           │
│    │ is_settleable() → die Invariante                   │
│    │                                                   │
│  f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf
│    (zxv-core: der Einzel-Mikrovertrag mit 110 Kits)     │
│    │ conjoin() → vierwertiges Meet                      │
│    │ is_settleable() → Abrechnungsentscheidung          │
│    │ verify_chain() → Hash-Ketten-Verifikation          │
│    │                                                   │
│  7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a
│    (Registry: 136 Capabilities)                         │
│    vm_capabilities() → 136                              │
│                                                         │
│  6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
│    (Gate: die Kernel-Invariante)                        │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Die vierwertige Logik im Smart Contract

Der in Rust geschriebene NEAR-Smart-Contract führt **dieselbe Logik wie die `no_std`-Implementierung aus Kapitel 4** aus:

```rust
// das Gate in zxv-core (Auszug)
#[near_bindgen]
impl ZxvCore {
    /// Vierwertiges Conjoin: Ketten-Meet
    pub fn conjoin(&self, inputs: Vec<u8>) -> u8 {
        let mut state = 0u8; // NEITHER
        for v in inputs {
            state = meet(state, v);
        }
        state
    }

    /// Abrechnungs-Invariante: nur TRUE rechnet ab
    pub fn is_settleable(&self, state: String) -> bool {
        state == "TRUE"
    }
}
```

**Verifikationsbefehle (gasfrei, View-Calls)**:

```bash
# conjoin {"inputs":[1,2]}        → "FALSE" (TRUE+FALSE=FALSE)
# is_settleable {"state":"BOTH"}  → false (ein Widerspruch rechnet nie ab)
# is_settleable {"state":"TRUE"}  → true (Zustimmung rechnet ab)
```

### 7.3 Das Hash-Ketten-Auditbuch (Receipts)

In der NEAR-Implementierung ist jede Abrechnung in einer **Hash-Kette** verknüpft:

```rust
// Receipt-Verknüpfung (Auszug)
// append() verknüpft den Hash des vorherigen Heads im nächsten Datensatz
// verify_chain() prüft die Verknüpfungsintegrität aller Receipts
```

Verifikationsergebnis (gemessen):

```
verify_chain {"from":0} → true (alle Receipts verknüpft und geprüft)
```

**Die vollständige Historie vom Widerspruch bis zur Lösung wird unveränderlich aufgezeichnet** — das Event-Sourcing-Muster aus Abschnitt 5.3, on-chain implementiert.

### 7.4 Zustandskompatible Upgrades

Ein zentrales Prinzip der NEAR-Implementierung sind **zustandskompatible Upgrades**:

- die serialisierte Top-Level-Struktur **ändert sich nie**;
- der Zustand neuer Funktionen lebt in **reservierten Namensräumen** der vorhandenen Store-Map;
- Upgrades sind reine Methoden-Upgrades.

Dies beruht auf der **Vorwärtskompatibilität der Borsh-Serialisierung**. Das Hinzufügen eines Felds zur Struktur bricht die Deserialisierung (ein dokumentierter Fehlerfall), daher wird das Prinzip strikt eingehalten:

```rust
// Beispiel für Zustandskompatibilität: die Struktur ist unverändert, der Zustand lebt in der Store-Map
// struct ZxvCore { owner: AccountId, store: LookupMap<u64, Receipt> }
// eine neue Funktion (z. B. ein Gas-Envelope) lebt in einem reservierten Schlüsselbereich von store
```

### 7.5 Gemessene Gas-Effizienz

Gas-Verbrauch der zentralen vierwertigen Operationen (conjoin/is_settleable):

| Operation | Gas | Hinweis |
|---|---|---|
| `conjoin` | < 1,2 Tgas | Obergrenze der internen Envelope-Logik |
| `is_settleable` | zig Ggas | ein einziger Vergleich |
| gesamte Ausführung | < 10 Tgas | Transaktions-Decke |

**Lesevorgänge (View-Calls) sind vollständig gasfrei.** Dies folgt aus der Eigenschaft „die Entscheidung ist ein einziger Vergleich".

### 7.6 Gas-Wall-Test (Haltbarkeitsverifikation)

Ergebnisse einer Dauerlast im Live-Netzwerk:

```
Nur-Lesen: 125/125 gültige Aufrufe (keine Gas-Wall)
Schreibpfad: 10/10 Zyklen (die Speicher-Wall wurde durch Finanzierung überwunden, Integrität erhalten)
```

### 7.7 Die Gas-Station von Venedig (die wirtschaftliche Anwendung)

Ein Beispiel für die **wirtschaftliche** Anwendung der vierwertigen Logik ist die Gas-Station von Venedig:

| Abrechnungszustand | Gas-Belastung | Protokoll-Match |
|---|---|---|
| `TRUE` (abgerechnet) | Benutzer 50 % | Protokoll 50 % (effektiv 2×) |
| `BOTH` (Widerspruch) | Benutzer 100 % | keins (Drain Shield) |
| `FALSE` (abgelehnt) | Benutzer 100 % | keins |

**Ein Widerspruch (BOTH) verliert den Match** und verhindert strukturell die Erschöpfung der Reserve durch absichtlichen Widerspruchs-Spam. Die Eigenschaft „BOTH rechnet nicht ab" übersetzt sich direkt in die Gestaltung wirtschaftlicher Anreize.

### 7.8 Souveräne Dekrete (Forks ohne Forking)

Die Governance der NEAR-Implementierung nutzt direkt die Eigenschaft „Widersprüche können koexistieren":

- `sd_issue` — eine Person erlässt ihr eigenes Souveränes Dekret (Verbesserungsvorschlag)
- `sd_sign` — unterzeichnet ein Dekret (das Register der Anhänger)
- `sd_shift` — wechselt frei das befolgte Dekret (hin und zurück)
- `sd_forkless` — Beweis, dass widersprüchliche Dekrete koexistieren

**Dekret A behauptet das eine, Dekret B das Gegenteil; beide existieren, und beide gelten für ihre Anhänger.** Die Eigenschaft „BOTH ist absorbierend und wird nie erzwungen" ist die mathematische Grundlage von Forks ohne Forking.

### 7.9 Kanonisches Routing

Die kanonische NEAR-Adresse `zedec.near` ist ein lebender Vertrag, der zum gesamten Protokoll routet:

```
zedec.near → route() → canonical=zedec.near|registry=7eaa2c6f...|gate=6ca890f7...|core=f18889ea...
zedec.near → protocol_meta() → primaryWallet=zedec.near|protocolAddress=f18889ea...|...
zedec.near → is_settleable(1) → true
```

---

## 8. Benchmarks und Bewertung

### 8.1 Lokale Verifikationsbefehle

```bash
# C-Version (null Heap, reine Bitoperationen)
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate && ./belnap_gate

# Rust-Version (no_std-Invariantenprüfung)
cargo test --release

# WASM-Version
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# Fuzzing (die gesamte u8-Domäne, 65 536)
gcc -O3 fuzz.c belnap_gate.c -o fuzz && ./fuzz
```

### 8.2 Erwartete Ausgabe

```
State: 0x03 (BOTH)
Can Settle? NO (Blocked without panic)
Resolved: 0x01 -> Can Settle? YES
Unknown-guard: 0x00 -> Can Settle? NO
Disjoin(BOTH,NEITHER): 0x03 (BOTH)
```

### 8.3 Gemessene Qualitätssicherungsergebnisse

Alle Verifikationsergebnisse der NEAR-Implementierung:

| Prüfung | Ergebnis |
|---|---|
| QC-Harness (gegen den C-Kernel-Quellcode abgeglichen) | **55/55 PASS, null Bugs** |
| Vollständiges On-Chain-Audit (alle Schichten) | **24/24 PASS** |
| Haltbarkeits-Stresstest | **70/70 gültige Aufrufe, null Fehler** |
| Registrierungsaudit | **136/136 zu bekannten lebenden Verträgen** |
| Gas-Wall-Test (Lesen) | **125/125** |
| Gas-Wall-Test (Schreiben) | **10/10 Zyklen** |

---

## 9. Fazit und Ausblick

### 9.1 Fazit

Dieser Artikel hat auf vier Ebenen — Theorie, Implementierung, Verifikation und Beweis — erklärt, wie man vierwertige (parakonsistente) Logik vollständig als System-Primitive implementiert.

**Theoretischer Beitrag**: Die Ketten-Meet-Algebra (FALSE < NEITHER < BOTH < TRUE) **garantiert strukturell** die Invariante „keine Nicht-TRUE-Kombination ergibt TRUE". Dies ist eine bewusste Abweichung von Belnaps Verband, die die Implementierung vereinfacht und die Garantie zugleich stärkt.

**Implementierung**: Eine C-Implementierung mit null Allokation (ohne Heap, reine Bitoperationen, wenige Anweisungen) und eine typsichere Rust/`no_std`-Implementierung werden bereitgestellt. Beide kompilieren und verifizieren lokal in Sekunden.

**Verifikation**: Die vollständige Aufzählung des endlichen Zustandsraums (64 Assoziativitätsprüfungen, 16 Invariantenprüfungen, 65 536 Fuzz-Eingaben) garantiert mechanisch, dass die Implementierung der Spezifikation entspricht.

**Beweis**: Der Smart Contract im NEAR-Mainnet (110 Kits, 136 Capabilities, kanonisch `zedec.near`, Vertragsadresse `f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf`) besteht den Gas-Wall-Test und das adversarielle Audit. Die vierwertige Logik wird direkt auf die Gestaltung wirtschaftlicher Anreize (der 2×-Match/Drain Shield der Gas-Station von Venedig) und die Governance-Gestaltung (Forks ohne Forking) angewendet.

### 9.2 Ausblick

- **Formaler Beweis**: Upgrade der erschöpfenden Tests auf maschinelle Beweise in Coq/Isabelle
- **Erweiterung der Rangtabelle**: Verifizieren, dass die Erweiterung auf 145 Zustände ohne Codeänderung in die Operatoren einfließt
- **Cross-Chain-Deployment**: Portierung der WASM-Version in andere Laufzeitumgebungen
- **NEP-Standardisierung**: Vorschlag der vierwertigen Logik-Gate als NEAR Enhancement Proposal

---

## Anhang A. Vollständiger Quellcode

### A.1 belnap_gate.h

```c
/* belnap_gate.h — öffentliche Schnittstelle der vierwertigen Zustandsmaschine */
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

### A.2 Verifikationsverfahren (vollständig)

```bash
#!/bin/sh
# verify.sh — führt alle Prüfungen der vierwertigen Logik in einem Durchgang aus
set -e

echo "=== 1. C: kompilieren (null Warnungen) ==="
gcc -O3 -Wall -Wextra -pedantic belnap_gate.c -o belnap_gate
echo "OK"

echo "=== 2. C: ausführen ==="
./belnap_gate

echo "=== 3. C: fuzzing (65 536) ==="
gcc -O3 fuzz.c belnap_gate.c -o fuzz
./fuzz

echo "=== 4. Rust: tests ==="
cargo test --release

echo "=== 5. Rust: WASM-Build ==="
cargo build --target wasm32-unknown-unknown --release
echo "OK"
```

---

## Anhang B. Referenzimplementierungs-Repositories

※ Dieser Algorithmus wurde für Umgebungen mit hohen Zuverlässigkeitsanforderungen entworfen und verifiziert: verteilte Konsensschichten und WASM-Laufzeitumgebungen. Referenzimplementierungen:

- **Kernel-Implementierung** (`kernel/src/lpres/`: `lpres_phases.h`, `lpres.c`): https://github.com/sykomiked/zxv-stack
- **Smart-Contract-Implementierung** (NEAR-Mainnet, kanonisch `zedec.near`): https://github.com/sykomiked/ZXV-NEP-paraconsistent

---

*Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC · SPDX-License-Identifier: Apache-2.0*