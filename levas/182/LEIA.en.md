# Batch 182 — the match screens (2): the latest scores screen

**Routines:**
- `abrir_ultimos_placares()` = 0x71b1b0 (match_screens.cpp, cdecl): opens the latest scores screen of a competition and writes its parameters;
- `montar_ultimos_placares()` = 0x71b300 (match_screens.cpp, cdecl): builds the latest scores screen: background, title, menu, panel and back and next buttons;
- `responder_ultimos_placares()` = 0x71b3d0 (match_screens.cpp, cdecl): the latest scores screen's response to a click (competition switch, redraw, menu);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 181's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 182`
