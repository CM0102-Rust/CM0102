# Batch 170 — closing the engine (4): the match and player orderings and the event variant

**Routines:**
- `ordem_das_partidas_do_dia()` = 0x6b6510 (match_eng.cpp, cdecl): the day's match order (qsort): competition, record fields and the club name;
- `ordem_de_acao()` = 0x6bd840 (match_eng.cpp, cdecl): the players' order on the engine's turn (qsort): the marked ones first, then by strength;
- `variante_do_evento()` = 0x6d5d80 (match_events.cpp, stdcall): swaps the commentary event for a variant, by a draw and by the play type;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 4 caught. `o_campos` escaped: in the English league the day matches tied up to +0x36 never disagree in sign between +0x38 and +0x34 (missing data; goes to the broad career). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 169's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 170`
