# Batch 168 — closing the engine (2): the side's centre-backs, the referee and the match player

**Routines:**
- `zagueiros_do_lado_sem_conflito()` = 0x6d5b20 (match_eng.cpp, thiscall): 0x5b8dc0 on the side's formation;
- `construir_arbitro()` = 0x6e9460 (match_official.cpp, thiscall): the referee constructor (zeroes +0 and +4);
- `destruir_jogador_da_partida()` = 0x6ebd60 (match_pl.cpp, thiscall): the match player destructor (zeroes +0x19e);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 167's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 168`
