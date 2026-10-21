# Batch 198 — the match screens (6): the day fixtures screen and the default captain

**Routines:**
- `abrir_jogos_do_dia()` = 0x717cc0 (match_screens.cpp, cdecl): opens the day fixtures screen in a slot with the fixture group (error box without a group);
- `responder_jogos_do_dia()` = 0x718930 (match_screens.cpp, cdecl): the day fixtures screen input: another fixture group, the competition progress or the menu bar;
- `vaga_do_mais_bem_avaliado()` = 0x727e20 (match_screens.cpp, cdecl): the first eleven slot with the highest staff rating (the default captain), -1 for none;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 197's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 198`
