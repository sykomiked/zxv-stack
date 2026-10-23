# Parametri ingegneristici di ZXV: il kernel e l'architettura

**Italiano** · ZXV — logica paraconsistente · macchina a stati · saldo


Questo documento dettaglia i parametri ingegneristici: il nucleo a quattro valori, la disciplina degli aggiornamenti compatibili, il libro di audit in catena, l'economia del gas e il registro di qualità.

---

## Il nucleo del kernel
Il modulo lpres definisce i quattro stati come pattern di bit in un byte. conjoin e disjoin sono confronti interi puri su una tabella di ranghi — zero allocazione, poche istruzioni.

---

## Aggiornamenti compatibili
La struttura serializzata non cambia mai; le nuove funzioni vivono in namespace riservati della mappa di storage. Questa compatibilità Borsh ha permesso di accumulare 110 kit senza rompere lo stato.

---

## La catena di audit
Ogni saldo si lega in una catena di hash (le ricevute). verify_chain conferma l'integrità — il pattern event sourcing on-chain, una traccia immutabile dalla contraddizione alla risoluzione.

---

## L'economia del gas
Un saldo TRUE riceve un match di gas 2x (la stazione di Venezia); un BOTH perde il match (lo scudo di drenaggio). La contraddizione è costosa, quindi niente spam. Le letture sono senza gas.

---

## Il registro di qualità
L'harness QC incrocia la sorgente C del kernel (55/55); l'audit on-chain copre tutti i layer (24/24); lo stress 70/70; l'audit di registrazione 136/136; il muro del gas 125/125 letture e 10/10 cicli.

---

## Conclusione
ZXV dimostra che la logica paraconsistente è ingegneria: distribuita, auditata e verificabile su NEAR.

Il sistema è aperto.

**Solo TRUE salda. TRUE+FALSE=BOTH — le contraddizioni sono registrate, mai coartate.**

