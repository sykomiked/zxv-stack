# ZXV-Ingenieurparameter: der Kernel und die Architektur

**Deutsch** · ZXV — parakonsistente Logik · Zustandsmaschine · Abrechnung


Dieses Dokument beschreibt die Ingenieurparameter: den vierwertigen Kern, die Disziplin zustandskompatibler Upgrades, das Ketten-Auditbuch, die Gas-Ökonomie und das gemessene Qualitätsniveau.

---

## Der Kernel-Kern
Das lpres-Modul definiert die vier Zustände als Bitmuster in einem Byte. conjoin und disjoin sind reine Integer-Vergleiche über eine Rangtabelle — null Allokation, wenige Anweisungen.

---

## Zustandskompatible Upgrades
Die serialisierte Top-Level-Struktur ändert sich nie; neue Funktionen leben in reservierten Namensräumen der Speicherkarte. Diese Borsh-Kompatibilität ließ 110 Kits akkumulieren, ohne den Zustand zu brechen.

---

## Die Audit-Kette
Jede Abrechnung wird in einer Hash-Kette verknüpft (die Quittungen). verify_chain bestätigt die Integrität — das Event-Sourcing-Muster on-chain, ein unveränderlicher Pfad vom Widerspruch zur Lösung.

---

## Die Gas-Ökonomie
Eine TRUE-Abrechnung erhält einen 2x-Gas-Match (die Gas-Station von Venedig); ein BOTH verliert den Match (der Drain-Shield). Widerspruch ist teuer, also kein Spam. Lesen ist völlig gasfrei.

---

## Das Qualitätsniveau
Der QC-Harness gleicht den C-Kernel-Quellcode ab (55/55); das On-Chain-Audit deckt alle Schichten ab (24/24); der Belastungstest 70/70; das Registrierungsaudit 136/136; die Gas-Wall 125/125 Lese- und 10/10 Schreibzyklen.

---

## Fazit
ZXV beweist, dass parakonsistente Logik Ingenieurwesen ist: bereitgestellt, geprüft und auf NEAR verifizierbar.

Das System ist offen.

**Nur TRUE rechnet ab. TRUE+FALSE=BOTH — Widersprüche werden aufgezeichnet, nie erzwungen.**

