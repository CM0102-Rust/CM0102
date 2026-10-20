# Batch 183 — the match screens (3): the match details screen

**Routines:**
- `abrir_detalhes_da_partida()` = 0x71b4a0 (match_screens.cpp, cdecl): opens the details screen of a match and writes its parameters;
- `responder_detalhes_da_partida()` = 0x71bdc0 (match_screens.cpp, cdecl): the match details screen's response to a click (tab switch, club tactics, tab redraw, menu);
- `fechar_detalhes_da_partida()` = 0x71c170 (match_screens.cpp, cdecl): closes the details screen: destroys and frees the objects kept in the parameters;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 182's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 183`
