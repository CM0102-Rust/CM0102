# Batch 305 — the season turnovers of the Veikkausliiga, the Tippeligaen and the Allsvenskan; the Portuguese turnover and its promotion and relegation chain; the season turnovers of the Finnish, Norwegian and Portuguese cups; the Portuguese Super Cup: its teams, its round table and its turnover

**Routines:**
- `fin_prm_start_new_season()` = 0x595d40 (fin_prm.cpp, thiscall, ret 0): the Finnish turnover, led by the Veikkausliiga: promotion and relegation, two unnamed steps [inference: the Ykkönen groups redone], the Veikkausliiga rebuilt for the next year (rules, 12-club table, fixtures, order), then the Ykkönen turnover, the predicted positions, the board expectations and the referee renewal;
- `nor_prm_start_new_season()` = 0x7927a0 (nor_prm.cpp, thiscall, ret 0): the Norwegian turnover, led by the Tippeligaen: promotion and relegation, the club swap between the First and Second Divisions, an unnamed step, the Tippeligaen rebuilt for the next year, then the First Division turnover, the predicted positions, the expectations and the referees;
- `swe_prm_start_new_season()` = 0x8905f0 (swe_prm.cpp, thiscall, ret 0): the Swedish turnover, led by the Allsvenskan: promotion and relegation; without a playable Second Division, three regional clubs go up to the Superettan; the clubs' division tier limited and the Second Division sections dealt again; the Allsvenskan rebuilt for the next year, then the Superettan and Second Division (when it exists) turnovers, the predicted positions, the expectations and the referees;
- `por_prm_process_promotion_relegation_chain()` = 0x7cfbf0 (por_prm.cpp, thiscall, ret 4): promotion and relegation in Portugal: records the final positions of the Premier Division and of the Liga de Honra and swaps the marked clubs between them; with the Second Division B played, records the positions of its three zones and the Liga de Honra swaps clubs with each one;
- `por_prm_start_new_season()` = 0x7cf9e0 (por_prm.cpp, thiscall, ret 0): the Portuguese turnover, led by the Premier Division: the B team checks (with the Second Division B), promotion and relegation, the Liga de Honra exchange with the regional zones (without the Second Division B), the Premier Division rebuilt for the next year, then the Liga de Honra and Second Division B turnovers, the predicted positions, the board expectations and the referee renewal;
- `fin_cup_start_new_season()` = 0x592d00 (fin_cup.cpp, thiscall, ret 0): the Finnish Cup turnover: last edition's entrant, round and tie tables freed, the season archived in the statistics and the year + 1; then the entrants, the rounds, the tie tree and the predicted positions;
- `nor_cup_start_new_season()` = 0x78fb90 (nor_cup.cpp, thiscall, ret 0): the same turnover in the Norwegian Cup;
- `por_cup_start_new_season()` = 0x7ce540 (por_cup.cpp, thiscall, ret 0): the same turnover in the Portuguese Cup;
- `por_super_select_teams()` = 0x7d3260 (por_super.cpp, thiscall, ret 0): the Super Cup is between the Premier Division champion and the Portuguese Cup winner; if the same club won both, the Cup runner-up takes the place; if a club is missing, the error box;
- `por_super_create_round_table()` = 0x7d30f0 (por_super.cpp, thiscall, ret 0x10): the Super Cup is one round: draw on 5 July and one match on 12 August at a neutral ground [inference: venue 4] (in 2000, two legs at the clubs' grounds); category 6; no prizes;
- `por_super_start_new_season()` = 0x7d3490 (por_super.cpp, thiscall, ret 0): the Super Cup turnover: last edition's tables freed, the season archived and the year + 1; then the two teams, the rounds and the tie tree (no predicted positions);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (11):** 10 caught. `mwe16_copa_portuguesa_dois_anos` escaped: ESCAPE EXPLAINED BY THE WINDOW, measured: the mutation puts the Portuguese Cup two years ahead at its turnover (`por_cup_start_new_season`, `PorCup::start_new_season` 0x7ce540). The turnover runs inside the window (the branch `mwe_16 portuguese cup turnover` counts 1 in a Rust career to 2002-08-01, between the Portuguese league turnover and the Super Cup one), but the first round of the next edition falls after 2002-08-01: until then the mutant spends the same draws as the base (4857694775). Measured on 18/11/2026: base and mutant to 2002-12-01 diverge -- base 6909527833 draws, 61692 matches, print 81eb94f9c7dfb53d; mutant 6812199597 draws, 61630 matches, print 316080bd39606dac. The 62 missing matches are the cup rounds that did not happen. The Norwegian and Finnish siblings (calendar-year seasons) are caught inside the window. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (48171c52).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6c09145c, original bf442e79 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 304's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 305`

**Fields touched:**
- cup round +0x11 unnamed word = 3 (the Portuguese Super Cup); value; writes
