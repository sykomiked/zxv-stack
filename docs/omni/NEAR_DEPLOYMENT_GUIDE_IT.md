# Usare la distribuzione ZXV su NEAR: guida completa

**Italiano** · ZXV — logica paraconsistente · macchina a stati · saldo


ZXV è un'implementazione viva della logica paraconsistente sulla rete NEAR. Questa guida spiega come leggere, verificare e usare la distribuzione — senza gas, senza portafoglio per leggere.

---

## Gli indirizzi
L'indirizzo canonico è zedec.near, un contratto di routing vivo. Punta al protocollo (micro-contratto di 110 kit), al registro (136 capacità) e alla porta (l'invariante del kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Come leggere il sistema
Qualsiasi client legge il sistema senza gas via view calls. L'interfaccia universale si insegna da sola: ux_manifest restituisce lo schema completo, ux_help risponde a ogni tema.

---

## Come unirsi
Un nuovo attore chiama mint_onboard: una sovvenzione una tantum di 5000 unità, spendibile sulla piattaforma ma non trasferibile. Poi pool_enter dà accesso tramite il pool di liquidità.

---

## Come verificare
L'invariante è verificabile on-chain: conjoin restituisce il risultato a quattro valori; is_settleable è vero solo per TRUE; verify_chain conferma la catena di audit.

---

## Riferimenti
Kernel: github.com/sykomiked/zxv-stack · NEP e libro mastro: github.com/sykomiked/ZXV-NEP-paraconsistent · Kit di sviluppo: github.com/sykomiked/zxv-developer-kit.

Il sistema è aperto.

**Solo TRUE salda. TRUE+FALSE=BOTH — le contraddizioni sono registrate, mai coartate.**

