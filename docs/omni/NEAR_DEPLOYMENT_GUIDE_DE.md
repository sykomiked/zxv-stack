# Die ZXV-Bereitstellung auf NEAR nutzen: vollständiger Leitfaden

**Deutsch** · ZXV — parakonsistente Logik · Zustandsmaschine · Abrechnung


ZXV ist eine lebende Implementierung parakonsistenter Logik im NEAR-Netzwerk. Dieser Leitfaden erklärt, wie man die Bereitstellung liest, verifiziert und nutzt — gasfrei, ohne Wallet zum Lesen.

---

## Die Adressen
Die kanonische Adresse ist zedec.near, ein lebender Routing-Vertrag. Sie zeigt auf das Protokoll (den 110-Kit-Mikrovertrag), das Register (136 Fähigkeiten) und das Gate (die Kernel-Invariante).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Wie man das System liest
Jeder Client liest das System gasfrei über View-Calls. Die universelle Schnittstelle lehrt sich selbst: ux_manifest liefert das volle Schema, ux_help beantwortet jedes Thema.

---

## Wie man beitritt
Ein neuer Akteur ruft mint_onboard: ein einmaliges Geschenk von 5000 Einheiten, auf der Plattform ausgabefähig, aber nicht übertragbar. Dann gibt pool_enter Zugang über den Liquiditätspool.

---

## Wie man verifiziert
Die Invariante ist on-chain verifizierbar: conjoin liefert das vierwertige Ergebnis; is_settleable ist nur für TRUE wahr; verify_chain bestätigt die Audit-Kette.

---

## Referenzen
Kernel: github.com/sykomiked/zxv-stack · NEP und Hauptbuch: github.com/sykomiked/ZXV-NEP-paraconsistent · Entwickler-Kit: github.com/sykomiked/zxv-developer-kit.

Das System ist offen.

**Nur TRUE rechnet ab. TRUE+FALSE=BOTH — Widersprüche werden aufgezeichnet, nie erzwungen.**

