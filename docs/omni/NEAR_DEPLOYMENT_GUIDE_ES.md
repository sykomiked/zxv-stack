# Usar el despliegue ZXV en NEAR: guía completa

**Español** · ZXV — lógica paraconsistente · máquina de estados · liquidación


ZXV es una implementación viva de la lógica paraconsistente en NEAR. Esta guía explica cómo leer, verificar y usar el despliegue, sin gas y sin billetera para leer.

---

## Las direcciones
La dirección canónica es zedec.near, un contrato de enrutamiento vivo. Apunta al protocolo (micro-contrato de 110 kits), al registro (136 capacidades) y a la puerta (el invariante del kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Cómo leer el sistema
Cualquier cliente lee el sistema sin gas mediante view calls. La interfaz universal se enseña sola: ux_manifest devuelve el esquema completo, ux_help responde cualquier tema.

---

## Cómo unirse
Un nuevo actor llama mint_onboard: una subvención única de 5000 unidades, gastable en la plataforma pero no transferible. Luego pool_enter da acceso a través del pool de liquidez.

---

## Cómo verificar
El invariante es verificable on-chain: conjoin devuelve el resultado de cuatro valores; is_settleable es verdadero solo para TRUE; verify_chain confirma la cadena de auditoría.

---

## Referencias
Kernel: github.com/sykomiked/zxv-stack · NEP y libro mayor: github.com/sykomiked/ZXV-NEP-paraconsistent · Kit de desarrollo: github.com/sykomiked/zxv-developer-kit.

El sistema está abierto.

**Solo TRUE liquida. TRUE+FALSE=BOTH — las contradicciones se registran, nunca se coaccionan.**

