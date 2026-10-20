# Batch 188 — the shortlist manager, the training entry and the training load

**Routines:**
- `lista_de_observados()` = 0x5d5ab0 (game.cpp, thiscall): the shortlist manager kept in the transfer manager;
- `entrada_do_treino()` = 0x5d5ac0 (game.cpp, thiscall): the player's 6-byte entry in the training table;
- `gravar_carga_de_treino()` = 0x62c660 (injury.cpp, thiscall): stores the player's training load in the fitness record;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 187's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 188`
