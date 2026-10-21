# Batch 200 — the match screens (9): the team block and player row copies and the live score

**Routines:**
- `atribuir_bloco_do_time()` = 0x71dd70 (match_screens.cpp, thiscall): copies the match team block (0x625 bytes) field by field, with its 20 player rows;
- `atribuir_linha_do_jogador()` = 0x71df00 (match_screens.cpp, thiscall): copies one player row of the team block (0x49 bytes), the date through Date::operator=;
- `contar_placar_ao_vivo()` = 0x728e90 (match_screens.cpp, cdecl): counts the goals and shoot-out goals of a match up to the current minute of the live match;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 199's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 200`
