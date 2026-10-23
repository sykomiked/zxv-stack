# Lógica paraconsistente: uma introdução completa

**Português** · ZXV — lógica paraconsistente · máquina de estados · liquidação


Todo sistema distribuído enfrenta o mesmo muro: duas fontes discordam sobre a verdade, e o sistema deve decidir. A lógica clássica dispara o princípio da explosão diante da contradição. A lógica paraconsistente responde de outro modo: tratar a contradição como informação.

---

## 1. O problema que a lógica clássica não resolve
O terceiro excluído (P ou não-P) supõe que toda proposição é verdadeira ou falsa. Mas em um ambiente distribuído, partições e atrasos fazem P e não-P chegarem juntos. Os sistemas clássicos respondem com pânico, último-escritor, impasse ou coerção majoritária — cada um perde informação.

---

## 2. Os quatro valores
Uma máquina de estados de quatro valores trata TRUE, FALSE, NEITHER e BOTH como estados de primeira classe. BOTH significa que a contradição é registrada e mantida — nunca coagida, nunca apagada. É a lógica de Belnap adaptada como primitiva.

---

## 3. A álgebra de meet em cadeia
Ordene os estados em uma cadeia total: FALSE < NEITHER < BOTH < TRUE. O meet retorna o menor posto; o join o maior. Como TRUE está no topo, nenhuma combinação de valores não-TRUE pode produzir TRUE — o invariante é estrutural.

---

## 4. O invariante
A regra de liquidação decorre da estrutura: apenas TRUE liquida. A contradição (BOTH) é mantida e registrada; só o acordo (TRUE) autoriza a ação. A álgebra é idempotente, comutativa, associativa e absorvente — um reticulado limitado verificado nas 16 e 64 combinações.

**Apenas TRUE liquida. TRUE+FALSE=BOTH — as contradições são registradas, nunca coagidas.**

---

## 5. Conclusão
A lógica paraconsistente resolve o que os sistemas clássicos recusam: o que fazer quando a verdade está em disputa. A resposta é sustentar a contradição como evidência e liquidar apenas diante do acordo genuíno.

O sistema está aberto.
