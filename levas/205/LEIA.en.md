# Batch 205 — the match screens (12): the team stats rows, the latest scores refresh and the wait for the other players

**Routines:**
- `preencher_estatisticas_do_time()` = 0x721cd0 (match_screens.cpp, cdecl): writes the 20 player rows of the team stats table (22 columns from the param 0x1c table), building the cells or updating what changed;
- `atualizar_ultimos_placares()` = 0x724520 (match_screens.cpp, cdecl): rewrites the latest scores rows in pages: stage, fixture, score, period, event scorers, aggregate and the page footer;
- `esperar_os_outros_jogadores()` = 0x727fb0 (match_screens.cpp, cdecl): in a network game, waits for the other humans of the same fixture or closes the screens, shows the notice and opens the match screen;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 204's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 205`
