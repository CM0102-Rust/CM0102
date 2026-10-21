# Batch 202 — the match screens (8): the team stats, player ratings and match stats panels

**Routines:**
- `painel_das_estatisticas_do_time()` = 0x7217a0 (match_screens.cpp, cdecl): draws one team's stats table (page header, arrows, column titles) or the no-stats message;
- `painel_das_notas_dos_jogadores()` = 0x722ed0 (match_screens.cpp, cdecl): draws both teams' player ratings: shirt number, name (button, with the captain) and rating;
- `painel_das_estatisticas_da_partida()` = 0x71e040 (match_screens.cpp, cdecl): draws the match stats: 13 labelled rows (shots, corners, fouls, passes, tackles, headers, cards);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 201's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 202`
