# Batch 204 — the match screens (10): the action zones and match report panels and the day fixtures screen

**Routines:**
- `painel_das_zonas_de_acao()` = 0x71f500 (match_screens.cpp, cdecl): draws the action zones: the period selector, the pitch split into three zones with both clubs' colour bars, or the no-zones message;
- `painel_do_relatorio_da_partida()` = 0x721000 (match_screens.cpp, cdecl): draws the match report: the minutes and the event texts (building the report outside the match), or the no-report message;
- `montar_jogos_do_dia()` = 0x717db0 (match_screens.cpp, cdecl): builds the day fixtures screen: title, the day's competition tree, the rounds and matches table and the scroll to the human's match;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 203's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 204`
