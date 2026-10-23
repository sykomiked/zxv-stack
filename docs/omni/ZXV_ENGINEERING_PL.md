# Parametry inżynieryjne ZXV: jądro i architektura

**Polski** · ZXV — logika parakonsystentna · maszyna stanów · rozliczenie


Ten dokument opisuje parametry inżynieryjne: czterowartościowe jądro, dyscyplinę zgodnych aktualizacji, łańcuchową księgę audytu, ekonomię gazu i zmierzoną jakość.

---

## Rdzeń jądra
Moduł lpres definiuje cztery stany jako wzorce bitów w jednym bajcie. conjoin i disjoin to czyste porównania całkowite na tabeli rzędów — zero alokacji, kilka instrukcji.

---

## Zgodne aktualizacje
Serializowana struktura najwyższego poziomu nigdy się nie zmienia; nowe funkcje żyją w zarezerwowanych przestrzeniach nazw mapy przechowywania. Ta zgodność Borsh pozwoliła zgromadzić 110 zestawów bez łamania stanu.

---

## Łańcuch audytu
Każde rozliczenie wiąże się w łańcuch skrótów (paragony). verify_chain potwierdza integralność — wzorzec event sourcing on-chain, niezmienny ślad od sprzeczności do rozwiązania.

---

## Ekonomia gazu
Rozliczenie TRUE otrzymuje 2x dopasowanie gazu (Wenecka Stacja Gazowa); BOTH traci dopasowanie (tarcza drenażu). Sprzeczność jest kosztowna, więc nie można spamować. Odczyt jest całkowicie bez gazu.

---

## Zapis jakości
Harness QC krzyżuje się z kodem źródłowym jądra C (55/55); audyt on-chain pokrywa wszystkie warstwy (24/24); stres trwałości 70/70; audyt rejestracji 136/136; ściana gazu 125/125 odczytów i 10/10 cykli.

---

## Podsumowanie
ZXV dowodzi, że logika parakonsystentna jest inżynierią: wdrożona, zbadana i weryfikowalna na NEAR.

System jest otwarty.

**Tylko TRUE rozlicza. TRUE+FALSE=BOTH — sprzeczności są odnotowywane, nigdy nie wymuszane.**

