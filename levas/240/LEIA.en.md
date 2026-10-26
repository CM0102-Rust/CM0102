# Batch 240 — the nationality adjective by language and form

**Routines:**
- `adjetivo_da_nacao()` = 0x6181d0 (index.cpp, cdecl): the nation's adjective into the text (size >= 50): with gender 0 or outside languages 9, 4, 5, 10 and 6 the nation's name; in those, the form from 0x545a60 and the chains of 937 nations (generated from the exe by adjetivos_nacao.py, texts by address), with the name if the text stays empty (except in languages 4 and 5); returns 1 (0 with a null nation or text or a short size);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 3 caught. `dbx_forma` escaped: form 5 (min(5) → min(4)) only appears in the Italian and Spanish tables, which the English game does not use; also escaped the real game comparing (0 divergences). `dbx_vazio` escaped: the name at the end only changes something in languages 4 and 5 (Italian, Spanish); also escaped the real game comparing (0 divergences). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 239's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 240`

**Fields touched:**
- TNation +0 id, +0x57 name; [0x9cf178..0x9d0527] nation keys; [0x9eb2dc] language; reads (adjetivo_da_nacao)

**Of interest to:** SGE, CMRTE, CMScout
