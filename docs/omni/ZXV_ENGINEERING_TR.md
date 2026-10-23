# ZXV mühendislik parametreleri: çekirdek ve mimari

**Türkçe** · ZXV — parakonsistent mantık · durum makinesi · mutabakat


Bu belge ZXV'nin mühendislik parametrelerini anlatır: dört değerli çekirdek, durumla uyumlu yükseltme disiplini, zincir denetim defteri, gaz ekonomisi ve ölçülen kalite.

---

## Çekirdek
lpres modülü dört durumu tek baytta bit desenleri olarak tanımlar. conjoin ve disjoin derece tablosunda saf tamsayı karşılaştırmalarıdır — sıfır tahsis, birkaç talimat.

---

## Durumla uyumlu yükseltme
Seri hale getirilmiş üst yapı asla değişmez; yeni özellikler depolama haritasının ayrılmış ad alanlarında yaşar. Bu Borsh uyumluluğu, durumu bozmadan 110 kitin birikmesini sağladı.

---

## Denetim zinciri
Her mutabakat bir hash zincirine bağlanır (makbuzlar). verify_chain bütünlüğü onaylar — event sourcing deseni, çelişkiden çözüme değişmez bir iz.

---

## Gaz ekonomisi
TRUE mutabakatı 2x gaz eşleşmesi alır (Venedik Gaz İstasyonu); BOTH eşleşmeyi kaybeder (tahliye kalkanı). Çelişki pahalıdır, bu yüzden spam olamaz. Okuma tamamen gazsızdır.

---

## Kalite kaydı
QC donanımı çekirdek C kaynağıyla örtüşür (55/55); zincir üstü denetim tüm katmanlar (24/24); dayanıklılık stresi 70/70; kayıt denetimi 136/136; gaz duvarı 125/125 okuma ve 10/10 döngü.

---

## Sonuç
ZXV, parakonsistent mantığın mühendislik olduğunu kanıtlar: dağıtılmış, denetlenmiş, NEAR'da doğrulanabilir.

Sistem açıktır.

**Yalnızca TRUE mutabakat sağlar. TRUE+FALSE=BOTH — çelişkiler kaydedilir, asla zorlanmaz.**

