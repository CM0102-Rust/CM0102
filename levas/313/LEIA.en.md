# Batch 313 — the Korean FA Cup; the Korean Super Cup; the constructors of the K League and the Korean League Cup; the constructors of the Argentine Premier and Second Divisions

**Routines:**
- `construct_kor_fa_cup()` = 0x66a460 (kor_fa_cup.cpp, thiscall, ret 8): rule: the Korean FA Cup at career creation: discipline rule 42 and, with no saved state, the 32 entrants, a 200,000 prize fund, the five rounds, the fixture tree, the statistics and the predicted positions (it does not write the bench);
- `kor_fa_cup_create_round_table()` = 0x66a720 (kor_fa_cup.cpp, thiscall, ret 0x10): rule: the Korean FA Cup has five single-match rounds: the first (32 clubs) drawn 21/10 and played 30/10; the round of 16 31/10 and 3/11; the quarter-finals 4/11 and 7/11; the semi-finals 8/11 and 17/11; the final 18/11 and 25/11; no prize per round (a share of the fund);
- `kor_fa_cup_collect_entrants()` = 0x66aaf0 (kor_fa_cup.cpp, thiscall, ret 0): rule: the Korean FA Cup takes 32 clubs: 22 clubs of the nation outside the K League (up to 100 found, sorted by reputation and shuffled), then the K League clubs; returns 1 with the list full; defect of the original: it copies 22 even when it finds fewer;
- `construct_kor_super_cup()` = 0x66d820 (kor_super_cup.cpp, thiscall, ret 8): rule: the Korean Super Cup is born with discipline rule 42 and the bench (5 substitutes, 3 substitutions); with no saved state, its two clubs, the round table, the tie tree and the statistics, no predicted positions;
- `kor_super_cup_create_round_table()` = 0x66dad0 (kor_super_cup.cpp, thiscall, ret 0x10): rule: the Korean Super Cup is a single match between its two clubs, drawn on 1/2 and played on 10/3 (moved to the weekday), category 6, no prize;
- `kor_super_cup_create_table()` = 0x66dc10 (kor_super_cup.cpp, thiscall, ret 0): rule: the Korean Super Cup is between the K League champion and the Korean FA Cup winner; a missing one is replaced by the first club of the nation; if the same club won both, the FA Cup runner-up comes in; without one, the error box;
- `construct_kor_league()` = 0x66ae70 (kor_league.cpp, thiscall, ret 8): rule: the K League at career creation: discipline rule 42, only the league (no stages); with no saved state, the rules, the table with its ten clubs, the fixtures, the order, the statistics and the predicted positions;
- `construct_kor_league_cup()` = 0x66bf00 (kor_league_cup.cpp, thiscall, ret 8): rule: the Korean League Cup at career creation: 5 substitutes and 3 substitutions, discipline rule 42, importance 12; with no saved state, two stages (group B and the knockout), the ten clubs, the statistics, the predicted positions, the rules, group A table (the cup itself), its fixtures and order and group B stage;
- `construct_arg_prm()` = 0x404250 (arg_prm.cpp, thiscall, ret 8): rule: the Argentine Premier Division at career creation: discipline rule 3 and, with no saved state, importance 15, two stages (Apertura and Clausura), the rules, the table, the fixtures, the statistics, the two half-season stages, the order, the relegation averages and the predicted positions;
- `construct_arg_second()` = 0x406bf0 (arg_second.cpp, thiscall, ret 8): rule: the Argentine Second Division (Nacional B) at career creation: discipline rule 3 and, with no saved state, importance 15, six stages (the zones, the promotion cup and the play-offs), the rules, the table, the fixtures, the order, the statistics, the relegation averages and its own predicted positions;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (4):** 4 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (0c0c36d5).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 8fc46a0d, original 29b211de (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 312's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 313`

**Fields touched:**
- cup +0x3e (entrants), +0xa7 (6 bytes: club, seed, status); competition +0x5d (nation); club +0x53 (nation), +0x57 (division)
- cup +0x49, +0x4a (bench), +0x50 (discipline), +0x3e and +0xa7 (the two clubs), round +0x11
- league +0xd9 (flags), +0x2c and +0xc (stages), +0x49, +0x4a (bench), +0x50 (discipline), +0x51 (importance)
- league +0x2c and +0xc (stages), +0x50 (discipline), +0x51 (importance), +8 (statistics)
