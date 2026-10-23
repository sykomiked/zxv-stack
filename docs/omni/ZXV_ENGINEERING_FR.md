# Paramètres d'ingénierie de ZXV : le noyau et l'architecture

**Français** · ZXV — logique paraconsistante · machine à états · règlement


Ce document détaille les paramètres d'ingénierie : le cœur à quatre valeurs, la discipline de mise à niveau compatible, le livre d'audit en chaîne, l'économie du gaz et le registre de qualité.

---

## Le cœur du noyau
Le module lpres définit les quatre états comme motifs de bits dans un octet. conjoin et disjoin sont des comparaisons entières pures sur une table de rangs — zéro allocation, quelques instructions.

---

## Mises à niveau compatibles
La structure sérialisée ne change jamais ; les nouvelles fonctions vivent dans des espaces de noms réservés du stockage. Cette compatibilité Borsh a permis d'accumuler 110 kits sans casser l'état déployé.

---

## La chaîne d'audit
Chaque règlement s'enchaîne dans une chaîne de hachage (les reçus). verify_chain confirme l'intégrité — le pattern event sourcing on-chain, une trace immuable de la contradiction à la résolution.

---

## L'économie du gaz
Un règlement TRUE reçoit un match de gaz 2x (la station de Venise) ; un BOTH perd le match (le bouclier de drainage). La contradiction coûte cher, donc pas de spam. Les lectures sont sans gaz.

---

## Le registre de qualité
Le harnais QC recoupe la source C du noyau (55/55) ; l'audit on-chain couvre toutes les couches (24/24) ; le stress 70/70 ; l'audit d'enregistrement 136/136 ; le mur de gaz 125/125 lectures et 10/10 cycles.

---

## Conclusion
ZXV prouve que la logique paraconsistante est de l'ingénierie : déployée, auditée et vérifiable sur NEAR.

Le système est ouvert.

**Seul TRUE règle. TRUE+FALSE=BOTH — les contradictions sont enregistrées, jamais coercées.**

