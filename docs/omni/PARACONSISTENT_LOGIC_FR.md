# Logique paraconsistante : introduction complète

**Français** · ZXV — logique paraconsistante · machine à états · règlement


Tout système distribué bute sur le même mur : deux sources divergent sur la vérité, et le système doit décider. La logique classique déclenche le principe d'explosion devant la contradiction. La logique paraconsistante répond autrement : traiter la contradiction comme information.

---

## 1. Le problème que la logique classique ne résout pas
Le tiers exclu (P ou non-P) suppose que toute proposition est vraie ou fausse. Mais en environnement distribué, les partitions et les retards font arriver P et non-P ensemble. Les systèmes classiques répondent par la panique, le dernier-écrivain, l'interblocage ou la coercition majoritaire — chacun perd de l'information.

---

## 2. Les quatre valeurs
Une machine à états à quatre valeurs traite TRUE, FALSE, NEITHER et BOTH comme des états de première classe. BOTH signifie que la contradiction est enregistrée et tenue — jamais coercée, jamais effacée. C'est la logique de Belnap adaptée en primitive.

---

## 3. L'algèbre de rencontre en chaîne
Ordonnez les états sur une chaîne totale : FALSE < NEITHER < BOTH < TRUE. La rencontre renvoie le rang mineur ; le join le majeur. Comme TRUE est au sommet, aucune combinaison de valeurs non-TRUE ne peut produire TRUE — l'invariant est structurel.

---

## 4. L'invariant
La règle de règlement découle de la structure : seul TRUE règle. La contradiction (BOTH) est tenue et journalisée ; seul l'accord (TRUE) autorise l'action. L'algèbre est idempotente, commutative, associative et absorbante — un treillis borné vérifié sur les 16 et 64 combinaisons.

**Seul TRUE règle. TRUE+FALSE=BOTH — les contradictions sont enregistrées, jamais coercées.**

---

## 5. Conclusion
La logique paraconsistante résout ce que les systèmes classiques refusent : que faire quand la vérité est disputée. La réponse : tenir la contradiction comme preuve et ne régler que sur l'accord véritable.

Le système est ouvert.
