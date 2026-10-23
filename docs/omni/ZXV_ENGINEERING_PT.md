# Parâmetros de engenharia da ZXV: o kernel e a arquitetura

**Português** · ZXV — lógica paraconsistente · máquina de estados · liquidação


Este documento detalha os parâmetros de engenharia: o núcleo de quatro valores, a disciplina de atualização compatível, o livro de auditoria em cadeia, a economia do gás e o registro de qualidade.

---

## O núcleo do kernel
O módulo lpres define os quatro estados como padrões de bits em um byte. conjoin e disjoin são comparações inteiras puras sobre uma tabela de postos — zero alocação, poucas instruções.

---

## Atualizações compatíveis
A estrutura serializada nunca muda; novas funções vivem em namespaces reservados do mapa de armazenamento. Essa compatibilidade Borsh permitiu acumular 110 kits sem quebrar o estado.

---

## A cadeia de auditoria
Cada liquidação se liga numa cadeia de hash (os recibos). verify_chain confirma a integridade — o padrão event sourcing on-chain, um rastro imutável da contradição à resolução.

---

## A economia do gás
Uma liquidação TRUE recebe um match de gás 2x (a estação de Veneza); um BOTH perde o match (o escudo de drenagem). A contradição é cara, logo não há spam. Leituras são sem gás.

---

## O registro de qualidade
O harness QC cruza com a fonte C do kernel (55/55); a auditoria on-chain cobre todas as camadas (24/24); o estresse 70/70; a auditoria de registro 136/136; a parede de gás 125/125 leituras e 10/10 ciclos.

---

## Conclusão
ZXV prova que a lógica paraconsistente é engenharia: implantada, auditada e verificável na NEAR.

O sistema está aberto.

**Apenas TRUE liquida. TRUE+FALSE=BOTH — as contradições são registradas, nunca coagidas.**

