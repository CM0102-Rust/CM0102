# Batch 238 — the game's fixed keys: ids from the database, colours and continents, and the report of the missing ones

**Routines:**
- `definir_chaves_fixas()` = 0x601a40 (index.cpp, cdecl): the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutations (5):** 4 caught. `dbw_continentes` escaped: the continent keys (0x9cfa08..0x9cfa1c) decide nothing in the .qst creation nor in the load to 2001-08-28; Asia and Europe swapped did not change the save. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 237's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 238`

**Fields touched:**
- [0x9cf178..0x9d0527], [0xb63c9c..0xb63d03]: the fixed keys (-1 and the database row); writes (definir_chaves_fixas)
- [0x9b5984] language; CM3_TEMP\eng.lng (8 counts and the tables); [0xae23fc] club list; reads (definir_chaves_fixas)

**Of interest to:** SGE, CMRTE, CMScout
