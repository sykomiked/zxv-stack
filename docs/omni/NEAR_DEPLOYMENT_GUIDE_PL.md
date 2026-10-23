# Korzystanie z wdrożenia ZXV na NEAR: kompletny przewodnik

**Polski** · ZXV — logika parakonsystentna · maszyna stanów · rozliczenie


ZXV to żywa implementacja logiki parakonsystentnej w sieci NEAR. Ten przewodnik wyjaśnia, jak czytać, weryfikować i używać wdrożenia — bez gazu, bez portfela do czytania.

---

## Adresy
Adres kanoniczny to zedec.near, żywy kontrakt routingu. Wskazuje na protokół (mikrokontrakt 110 zestawów), rejestr (136 możliwości) i bramę (niezmiennik jądra).

```
zedec.near · f18889ea8bc108de53b899df3a3417c76995cbb55fb2174ff08d2e0abee99bcf · 7eaa2c6f37ef7dd1dc073c4630533b67f303274d31d411e4047eb0d9401ef01a · 6ca890f766fdaeccd6c934da325f5f06ebc63ca9be4f5f1b44d0686af93c64a8
```

---

## Jak czytać system
Każdy klient czyta system bez gazu przez view calls. Uniwersalny interfejs uczy sam: ux_manifest zwraca pełny schemat, ux_help odpowiada na każdy temat.

---

## Jak dołączyć
Nowy aktor woła mint_onboard: jednorazowy grant 5000 jednostek, wydawalny na platformie, ale nieprzenoszalny. Potem pool_enter daje dostęp przez pulę płynności.

---

## Jak zweryfikować
Niezmiennik jest weryfikowalny on-chain: conjoin zwraca wynik czterowartościowy; is_settleable jest prawdziwe tylko dla TRUE; verify_chain potwierdza łańcuch audytu.

---

## Referencje
Jądro: github.com/sykomiked/zxv-stack · NEP i księga: github.com/sykomiked/ZXV-NEP-paraconsistent · Zestaw deweloperski: github.com/sykomiked/zxv-developer-kit.

System jest otwarty.

**Tylko TRUE rozlicza. TRUE+FALSE=BOTH — sprzeczności są odnotowywane, nigdy nie wymuszane.**

