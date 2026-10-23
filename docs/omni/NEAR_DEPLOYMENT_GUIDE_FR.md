# Utiliser le déploiement ZXV sur NEAR : guide complet

**Français** · ZXV — logique paraconsistante · machine à états · règlement


ZXV est une implémentation vivante de la logique paraconsistante sur NEAR. Ce guide explique comment lire, vérifier et utiliser le déploiement — sans gaz, sans portefeuille pour la lecture.

---

## Les adresses
L'adresse canonique est zedec.near, un contrat de routage vivant. Elle pointe vers le protocole (micro-contrat de 110 kits), le registre (136 capacités) et la porte (l'invariant du noyau).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Comment lire le système
Tout client lit le système sans gaz via des view calls. L'interface universelle s'enseigne seule : ux_manifest renvoie le schéma complet, ux_help répond à tout sujet.

---

## Comment rejoindre
Un nouvel acteur appelle mint_onboard : une subvention unique de 5000 unités, dépensable sur la plateforme mais non transférable. Puis pool_enter donne accès via le pool de liquidité.

---

## Comment vérifier
L'invariant est vérifiable on-chain : conjoin renvoie le résultat à quatre valeurs ; is_settleable est vrai uniquement pour TRUE ; verify_chain confirme la chaîne d'audit.

---

## Références
Noyau : github.com/sykomiked/zxv-stack · NEP et grand livre : github.com/sykomiked/ZXV-NEP-paraconsistent · Kit de développement : github.com/sykomiked/zxv-developer-kit.

Le système est ouvert.

**Seul TRUE règle. TRUE+FALSE=BOTH — les contradictions sont enregistrées, jamais coercées.**

