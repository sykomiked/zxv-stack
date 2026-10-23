# Parakonsistente Logik: eine vollständige Einführung

**Deutsch** · ZXV — parakonsistente Logik · Zustandsmaschine · Abrechnung


Jedes verteilte System stößt auf dieselbe Wand: zwei Quellen sind sich über die Wahrheit uneinig, und das System muss entscheiden. Klassische Logik löst beim Widerspruch das Explosionsprinzip aus. Parakonsistente Logik antwortet anders: Behandle den Widerspruch als Information.

---

## 1. Das Problem, das klassische Logik nicht löst
Der Satz vom ausgeschlossenen Dritten (P oder nicht-P) setzt voraus, dass jede Aussage wahr oder falsch ist. Aber in einer verteilten Umgebung treffen P und nicht-P zusammen. Klassische Systeme antworten mit Panik, Letztschreiber, Deadlock oder Mehrheitszwang — jedes verliert Informationen.

---

## 2. Die vier Werte
Ein vierwertiger Automat behandelt TRUE, FALSE, NEITHER und BOTH als Zustände erster Klasse. BOTH bedeutet: der Widerspruch wird aufgezeichnet und gehalten — nie erzwungen, nie gelöscht. Das ist Belnaps Logik als System-Primitive.

---

## 3. Die Ketten-Meet-Algebra
Ordne die Zustände auf einer totalen Kette: FALSE < NEITHER < BOTH < TRUE. Das Meet gibt den kleineren Rang zurück, der Join den größeren. Da TRUE an der Spitze liegt, kann keine Kombination von Nicht-TRUE-Werten TRUE ergeben — die Invariante ist strukturell.

---

## 4. Die Invariante
Die Abrechnungsregel folgt aus der Struktur: nur TRUE rechnet ab. Der Widerspruch (BOTH) wird gehalten und protokolliert; nur Zustimmung (TRUE) autorisiert Handlung. Die Algebra ist idempotent, kommutativ, assoziativ und absorptiv — ein beschränkter Verband, verifiziert über alle 16 und 64 Kombinationen.

**Nur TRUE rechnet ab. TRUE+FALSE=BOTH — Widersprüche werden aufgezeichnet, nie erzwungen.**

---

## 5. Fazit
Parakonsistente Logik löst, was klassische Systeme verweigern: was tun, wenn die Wahrheit umstritten ist. Die Antwort: den Widerspruch als Beweis halten und nur bei echter Zustimmung abrechnen.

Das System ist offen.
