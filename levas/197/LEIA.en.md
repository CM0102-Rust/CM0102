# Batch 197 — the human manager record (holiday stamp and saved line-up) and the search manager

**Routines:**
- `hora_das_ferias()` = 0x5fdc30 (human_manager.cpp, thiscall): the clock stamp stored when the human manager went on holiday;
- `escalacao_guardada()` = 0x5fed50 (human_manager.cpp, thiscall): the address of the human manager's saved line-up (16 pairs);
- `tecnico_da_busca()` = 0x7ae570 (player_search.cpp, thiscall): stores the human manager id in the player search;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 196's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 197`
