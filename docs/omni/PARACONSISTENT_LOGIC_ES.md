# Lógica paraconsistente: una introducción completa

**Español** · ZXV — lógica paraconsistente · máquina de estados · liquidación


Todo sistema distribuido se enfrenta al mismo muro: dos fuentes discrepan sobre la verdad, y el sistema debe decidir. La lógica clásica dispara el principio de explosión ante la contradicción. La lógica paraconsistente responde distinto: trata la contradicción como información.

---

## 1. El problema que la lógica clásica no resuelve
El tercio excluido (P o no-P) supone que toda proposición es verdadera o falsa. Pero en un entorno distribuido, particiones y retrasos significan que P y no-P pueden llegar a la vez. Los sistemas clásicos responden con pánico, último-escritor, interbloqueo o coerción mayoritaria — cada uno pierde información.

---

## 2. Los cuatro valores
Una máquina de estados de cuatro valores trata TRUE, FALSE, NEITHER y BOTH como estados de primera clase. BOTH significa que la contradicción se registra y se mantiene — nunca se coacciona, nunca se borra. Es la lógica de Belnap adaptada como primitiva.

---

## 3. El álgebra de meet en cadena
Ordene los estados en una cadena total: FALSE < NEITHER < BOTH < TRUE. El meet devuelve el rango menor; el join el mayor. Como TRUE está en la cima, ninguna combinación de valores no-TRUE puede producir TRUE — el invariante es estructural.

---

## 4. El invariante
La regla de liquidación surge de la estructura de rangos: solo TRUE liquida. La contradicción (BOTH) se mantiene y registra; solo el acuerdo (TRUE) autoriza la acción. El álgebra es idempotente, conmutativa, asociativa y absorbente — un retículo acotado verificado en las 16 y 64 combinaciones.

**Solo TRUE liquida. TRUE+FALSE=BOTH — las contradicciones se registran, nunca se coaccionan.**

---

## 5. Conclusión
La lógica paraconsistente resuelve lo que los sistemas clásicos rehúsan: qué hacer cuando la verdad está en disputa. La respuesta es sostener la contradicción como evidencia y liquidar solo ante el acuerdo genuino.

El sistema está abierto.
