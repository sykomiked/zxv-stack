# Parámetros de ingeniería de ZXV: el kernel y la arquitectura

**Español** · ZXV — lógica paraconsistente · máquina de estados · liquidación


Este documento detalla los parámetros de ingeniería: el núcleo de cuatro valores, la disciplina de actualización compatible, el libro de auditoría en cadena, la economía del gas y el registro de calidad.

---

## El núcleo del kernel
El módulo lpres define los cuatro estados como patrones de bits en un byte. conjoin y disjoin son comparaciones enteras puras sobre una tabla de rangos — cero asignación, unas pocas instrucciones.

---

## Actualizaciones compatibles
La estructura serializada nunca cambia; las nuevas funciones viven en espacios de nombres reservados del mapa de almacenamiento. Esta compatibilidad Borsh permitió acumular 110 kits sin romper el estado.

---

## La cadena de auditoría
Cada liquidación se enlaza en una cadena de hash (los recibos). verify_chain confirma la integridad — el patrón event sourcing en cadena, un rastro inmutable de la contradicción a la resolución.

---

## La economía del gas
Una liquidación TRUE recibe un emparejamiento de gas 2x (la estación de Venecia); un BOTH pierde el emparejamiento (el escudo de drenaje). La contradicción es costosa, no se puede hacer spam. Las lecturas son sin gas.

---

## El registro de calidad
El harness QC contrasta con la fuente C del kernel (55/55); la auditoría on-chain cubre todas las capas (24/24); el estrés de durabilidad 70/70; la auditoría de registro 136/136; la pared de gas 125/125 lecturas y 10/10 ciclos.

---

## Conclusión
ZXV prueba que la lógica paraconsistente es ingeniería: desplegada, auditada y verificable en NEAR.

El sistema está abierto.

**Solo TRUE liquida. TRUE+FALSE=BOTH — las contradicciones se registran, nunca se coaccionan.**

