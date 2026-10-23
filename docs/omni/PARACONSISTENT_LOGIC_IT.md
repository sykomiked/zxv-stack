# Logica paraconsistente: un'introduzione completa

**Italiano** · ZXV — logica paraconsistente · macchina a stati · saldo


Ogni sistema distribuito affronta lo stesso muro: due fonti divergono sulla verità, e il sistema deve decidere. La logica classica innesca il principio di esplosione davanti alla contraddizione. La logica paraconsistente risponde diversamente: trattare la contraddizione come informazione.

---

## 1. Il problema che la logica classica non risolve
Il terzo escluso (P o non-P) presuppone che ogni proposizione sia vera o falsa. Ma in un ambiente distribuito, partizioni e ritardi fanno arrivare P e non-P insieme. I sistemi classici rispondono con panico, ultimo-scrittore, stallo o coercizione maggioritaria — ciascuno perde informazione.

---

## 2. I quattro valori
Una macchina a stati a quattro valori tratta TRUE, FALSE, NEITHER e BOTH come stati di prima classe. BOTH significa che la contraddizione è registrata e tenuta — mai coartata, mai cancellata. È la logica di Belnap adattata come primitiva.

---

## 3. L'algebra di meet in catena
Ordinate gli stati su una catena totale: FALSE < NEITHER < BOTH < TRUE. Il meet restituisce il rango minore; il join il maggiore. Poiché TRUE è in cima, nessuna combinazione di valori non-TRUE può produrre TRUE — l'invariante è strutturale.

---

## 4. L'invariante
La regola di saldo deriva dalla struttura: solo TRUE salda. La contraddizione (BOTH) è tenuta e registrata; solo l'accordo (TRUE) autorizza l'azione. L'algebra è idempotente, commutativa, associativa e assorbente — un reticolo limitato verificato su tutte le 16 e 64 combinazioni.

**Solo TRUE salda. TRUE+FALSE=BOTH — le contraddizioni sono registrate, mai coartate.**

---

## 5. Conclusione
La logica paraconsistente risolve ciò che i sistemi classici rifiutano: cosa fare quando la verità è contesa. La risposta: tenere la contraddizione come prova e saldare solo su un accordo genuino.

Il sistema è aperto.
