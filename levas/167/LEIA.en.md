# Batch 167 — closing the engine (1): the ball's goal end, the abandoned match and the side's slots

**Routines:**
- `bola_na_area()` = 0x6d58f0 (match_eng.cpp, thiscall): 1 with the ball in the top box, −1 in the bottom one, otherwise 0; returns only `al`;
- `partida_abandonada()` = 0x6d5930 (match_eng.cpp, thiscall): the abandonment minute × 11 below the clock (16-bit); returns only `al`;
- `vagas_do_lado_sem_conflito()` = 0x6d5af0 (match_eng.cpp, thiscall): 0x5b8960 on the side's formation;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (4):** 3 caught. `a_onze` escaped: no match is abandoned in the automatic career (the abandonment minute stays at −1). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 166's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 167`
