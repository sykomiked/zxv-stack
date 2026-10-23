# Usar a implantação ZXV na NEAR: guia completo

**Português** · ZXV — lógica paraconsistente · máquina de estados · liquidação


ZXV é uma implementação viva da lógica paraconsistente na rede NEAR. Este guia explica como ler, verificar e usar a implantação — sem gás, sem carteira para leitura.

---

## Os endereços
O endereço canônico é zedec.near, um contrato de roteamento vivo. Ele aponta para o protocolo (micro-contrato de 110 kits), o registro (136 capacidades) e a porta (o invariante do kernel).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Como ler o sistema
Qualquer cliente lê o sistema sem gás via view calls. A interface universal se ensina: ux_manifest retorna o esquema completo, ux_help responde qualquer tópico.

---

## Como entrar
Um novo ator chama mint_onboard: uma doação única de 5000 unidades, gastável na plataforma mas não transferível. Depois pool_enter dá acesso pelo pool de liquidez.

---

## Como verificar
O invariante é verificável on-chain: conjoin retorna o resultado de quatro valores; is_settleable é verdadeiro apenas para TRUE; verify_chain confirma a cadeia de auditoria.

---

## Referências
Kernel: github.com/sykomiked/zxv-stack · NEP e livro-razão: github.com/sykomiked/ZXV-NEP-paraconsistent · Kit de desenvolvimento: github.com/sykomiked/zxv-developer-kit.

O sistema está aberto.

**Apenas TRUE liquida. TRUE+FALSE=BOTH — as contradições são registradas, nunca coagidas.**

