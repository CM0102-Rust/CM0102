# Batch 186 — the match screens (4): opening the match screen and the responses of the latest scores and team stats panels

**Routines:**
- `abrir_tela_da_partida()` = 0x718aa0 (match_screens.cpp, cdecl): opens the live match screen, writes its parameters and marks the human as a spectator;
- `responder_ultimos_placares_da_partida()` = 0x725860 (match_screens.cpp, cdecl): the latest scores panel of the match screen: the page switch every 5 s and the clicks (competition, pages, pause);
- `responder_estatisticas_do_time()` = 0x722dc0 (match_screens.cpp, cdecl): the team stats panel: the redraw with new data and the clicks that move the row up and down;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 185's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 186`
