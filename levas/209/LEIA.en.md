# Batch 209 — the match screens (14): building and handling the live match screen and building the match details screen; the match screens (13): the possession bar, the match overview panel and the live league table

**Routines:**
- `montar_tela_da_partida()` = 0x718e90 (match_screens.cpp, cdecl): builds the live match screen: team headers and score, commentary, clock, control, tactics buttons, commentary speeds, tabs, the selected tab panel, the stadium and the commentary panel;
- `responder_tela_da_partida()` = 0x71a240 (match_screens.cpp, cdecl): handles the live match screen: commentary volume and speed, the new event (clock, commentary, modes), watching or not, a team's tactics, switching tabs and each tab's input;
- `montar_detalhes_da_partida()` = 0x71b630 (match_screens.cpp, cdecl): builds the details screen of a stored match: loads the match record and snapshot from the match stats, the team headers, the 7 tabs and the selected tab panel;
- `atualizar_barra_da_posse()` = 0x71d9e0 (match_screens.cpp, cdecl): redraws the overview possession bar when the share changes (percentages and the colour of the club with more of the ball) and stores the match state in the param 0x1d copy;
- `painel_da_visao_geral_da_partida()` = 0x71c270 (match_screens.cpp, cdecl): draws the match overview: period, possession bar and commentary, date, score with half time/extra time/penalties, competition, scorers and cards, shoot-out, referee, attendance, weather and abandoned match;
- `tabela_da_liga_ao_vivo()` = 0x7280e0 (match_screens.cpp, cdecl): draws the live league table (league, top, bottom) on a copy of the league with the day scores applied (or the draw date notice): title, headers, separators, position, up/down arrow, club and columns;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (034ff3eb3e55dfd3).

**Redo**, on top of batch 208's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 209`
