# Parameter rekayasa ZXV: kernel dan arsitektur

**Bahasa Indonesia** · ZXV — logika parakonsisten · mesin keadaan · penyelesaian


Dokumen ini merinci parameter rekayasa: inti empat nilai, disiplin peningkatan kompatibel, buku audit rantai, ekonomi gas, dan catatan kualitas.

---

## Inti kernel
Modul lpres mendefinisikan empat keadaan sebagai pola bit dalam satu byte. conjoin dan disjoin adalah perbandingan bilangan bulat murni pada tabel peringkat — nol alokasi, beberapa instruksi.

---

## Peningkatan kompatibel
Struktur berseri tingkat atas tidak pernah berubah; fitur baru hidup di namespace cadangan peta penyimpanan. Kompatibilitas Borsh ini memungkinkan 110 kit terkumpul tanpa merusak keadaan.

---

## Rantai audit
Setiap penyelesaian terhubung dalam rantai hash (kwitansi). verify_chain mengonfirmasi integritas — pola event sourcing on-chain, jejak tak berubah dari kontradiksi ke resolusi.

---

## Ekonomi gas
Penyelesaian TRUE mendapat pencocokan gas 2x (Stasiun Gas Venesia); BOTH kehilangan pencocokan (perisai drainase). Kontradiksi mahal, jadi tidak bisa spam. Pembacaan sepenuhnya tanpa gas.

---

## Catatan kualitas
Harness QC mencocokkan sumber C kernel (55/55); audit on-chain mencakup semua lapisan (24/24); stres ketahanan 70/70; audit pendaftaran 136/136; dinding gas 125/125 baca dan 10/10 siklus.

---

## Kesimpulan
ZXV membuktikan bahwa logika parakonsisten adalah rekayasa: diterapkan, diaudit, dan dapat diverifikasi di NEAR.

Sistem ini terbuka.

**Hanya TRUE yang menyelesaikan. TRUE+FALSE=BOTH — kontradiksi dicatat, tidak pernah dipaksa.**

