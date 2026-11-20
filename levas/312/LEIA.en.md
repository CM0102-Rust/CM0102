# Batch 312 — the Finnish Cup; the Norwegian Cup; the Portuguese Cup; the Portuguese Super Cup constructor

**Routines:**
- `construct_fin_cup()` = 0x592250 (fin_cup.cpp, thiscall, ret 8): rule: the Finnish Cup at career creation: discipline rule 11, 7 substitutes and 3 substitutions and, with no saved state, the 76 entrants, a 200,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted positions;
- `fin_cup_create_round_table()` = 0x592510 (fin_cup.cpp, thiscall, ret 0x10): rule: the Finnish Cup has seven single-leg rounds, settled on the day: the first (64 clubs, the Second Division and lower ones) drawn 15/4 and played 29/4; the second 3/5 and 16/5; the third (the 8 First Division clubs come in) 5/6 and 21/6; the fourth (the Premier Division top 4 come in) 22/6 and 25/7; the quarter-finals 27/7 and 19/9; the semi-finals 21/9 and 17/10; the final 18/10 and 10/11; no prize per round (a share of the fund);
- `fin_cup_collect_entrants()` = 0x592a20 (fin_cup.cpp, thiscall, ret 0): rule: the Finnish Cup takes 76 clubs: 44 from the Second Division and below (up to 80 found, sorted by reputation and shuffled), then the First Division clubs, then the Premier Division clubs that finished below 5th and last the top 5; the Premier Division clubs with seed 1; returns 1 with the list full; defect of the original: it copies 44 even when it finds fewer;
- `construct_nor_cup()` = 0x78f020 (nor_cup.cpp, thiscall, ret 8): rule: the Norwegian Cup at career creation: discipline rule 22, 7 substitutes and 3 substitutions and, with no saved state, the 128 entrants, an 800,000 prize fund, the seven rounds, the fixture tree, the statistics and the predicted positions;
- `nor_cup_create_round_table()` = 0x78f2e0 (nor_cup.cpp, thiscall, ret 0x10): rule: the Norwegian Cup has seven rounds: the first (128 clubs) drawn 7/4 and played 5/5; the second 6/5 and 26/5; the third 27/5 and 2/6; the fourth (a replay on a draw) 3/6 and 30/6; the quarter-finals 1/7 and 4/8; the semi-finals 5/8 and 25/9; the final drawn 29/9 and played on 24 or 31 October (31/10 moved to the weekday; if that passes 1/11, the 24th), with a replay 3 days later; no prize per round (a share of the fund);
- `nor_cup_collect_entrants()` = 0x78f890 (nor_cup.cpp, thiscall, ret 0): rule: the Norwegian Cup takes 128 clubs, none with "2" in the short name (the reserve sides): first the Premier Division (seed 1), First Division and the eight Second Division groups clubs; then the Third Division; last the Norwegian clubs without a division; a club whose division points at the cup itself has its division cleared; returns 1 with the list full;
- `construct_por_cup()` = 0x7cd980 (por_cup.cpp, thiscall, ret 8): rule: the Portuguese Cup at career creation: discipline rule 23 and, with no saved state, the 164 entrants, an 800,000 prize fund, the eight rounds, the fixture tree, the statistics and the predicted positions (it does not write the bench);
- `por_cup_create_round_table()` = 0x7cdc40 (por_cup.cpp, thiscall, ret 0x10): rule: the Portuguese Cup has eight single-match rounds, with a replay on a draw up to the semi-finals: the first (128 lower clubs) drawn 10/9 and played 19/9; the second (the 18 Second Division clubs come in) 23/9 and 5/10; the third (17 Premier Division clubs come in) 18/10 and 14/11; the fourth (the last one comes in) 29/12 and 9/1; the fifth (15 clubs in 7 ties, as the original) 10/1 and 26/1; the quarter-finals 3/2 and 9/2; the semi-finals 20/3 and 12/4; the final 13/4 and 21/5; no prize per round (a share of the fund);
- `por_cup_collect_entrants()` = 0x7ce220 (por_cup.cpp, thiscall, ret 0): rule: the Portuguese Cup takes 164 clubs: up to 128 from the lower divisions (the Second Division B groups, then the Third Division and the Portuguese clubs of the lower division), without the B teams linked to no club; then the Second Division and the Premier Division clubs; last it swaps the last entrant with one drawn up to 18 places before; returns 1 with the list full;
- `construct_por_super()` = 0x7d2e40 (por_super.cpp, thiscall, ret 8): rule: the Portuguese Super Cup is born with discipline rule 23; with no saved state, its two clubs, the round table, the tie tree and the statistics, no predicted positions;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (6):** 5 caught. `mwe19_reservas_entram` escaped: EQUIVALENT MUTATION IN THIS DATABASE (measured). `nor_cup_collect_entrants` leaves out the reserve sides, the clubs with "2" in the short name (`crt_strstr(short_name, "2")`); the mutation lets them in. Measured: the counter `mwe_19 norwegian cup entrants` shows 1 run in the batch's real game (the routine runs in the window), and the comparer `compare_nor_cup_collect_entrants` -- side by side, with the random generator restored, checking every entry -- gave 0 divergences with the mutation applied (comparing to the chain date and in the holiday save load). So in this database (DATABASE=MINIMUM) no club of the loaded Norwegian divisions the routine walks has "2" in its short name: the reserve rule only shows with a database that carries the Norwegian B teams. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (f1f23a87).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 ae792160, original ff0d5539 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 311's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 312`

**Fields touched:**
- cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); club +0x57 (division), +0x5f (last season position)
- cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); club +0x38 (short name), +0x53 (nation), +0x57 (division)
- cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); club +0x53 (nation), +0x57 (division)
- cup +0x50 (discipline)
