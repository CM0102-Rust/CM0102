# Batch 208 — the interface areas (1): laying out the grid, initialising and drawing an area (with the 30-row clamp: the 1080 report "Area.cpp 507")

**Routines:**
- `organizar_area()` = 0x403350 (Area.cpp, thiscall): splits the area width and height by the column and row weights (up to 30 each; more raises the Area.cpp 497/507 box) and sets up the scrollbar;
- `iniciar_area()` = 0x402ac0 (Area.cpp, thiscall): initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights; the visible rows clamped at 30 (the grid size), the rest goes to the scroll bar;
- `desenhar_area()` = 0x402e70 (Area.cpp, thiscall): draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 207's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 208`
