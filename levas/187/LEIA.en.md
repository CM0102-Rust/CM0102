# Batch 187 — the match screens (5): the latest scores panel, the next screen and the heading colour

**Routines:**
- `painel_dos_ultimos_placares()` = 0x723d20 (match_screens.cpp, cdecl): draws the latest scores panel: title, badge, the day's groups and matches, table and buttons;
- `sem_tela_seguinte()` = 0x7febb0 (scrman.cpp, thiscall): the screen manager has no next screen to go forward to;
- `cor_do_titulo()` = 0x839590 (Setup.cpp, cdecl): the heading text colour (two colours by the flag);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 186's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 187`
