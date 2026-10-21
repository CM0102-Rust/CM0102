# Batch 207 — the match screens (15): the team selection check and the match statistics index in the saved game

**Routines:**
- `desenhar_conferencia_da_escalacao()` = 0x7260f0 (match_screens.cpp, cdecl): draws the team selection check: warns (with linked names) about players on international duty, on loan, unavailable, ineligible, banned, injured or without a work permit, the bench limit or missing starters; otherwise asks for confirmation (captain, set piece takers, substitutes) with "No" and "Yes";
- `carregar_estatisticas_da_partida()` = 0x72aa80 (match_stats.cpp, thiscall): reads the record index, the four bucket cursors and the counts from match_stats.dat in the saved game;
- `salvar_estatisticas_da_partida()` = 0x72adf0 (match_stats.cpp, thiscall): writes the record index, the four bucket cursors and the counts to match_stats.dat in the saved game;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 206's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 207`
