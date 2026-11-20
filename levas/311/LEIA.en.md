# Batch 311 — the Polish season turnover; the Polish and Greek cup entrants; the Turkish category B turnover and stages and the Polish First Division stages

**Routines:**
- `pol_first_start_new_season()` = 0x7c9740 (pol_first.cpp, thiscall, ret 0): rule: the Polish turnover, led by the First Division: promotion and relegation along the chain; the four best clubs of the lowest division up into the Second Division and its bottom clubs down; the First Division rebuilt for the next year (rules, table, fixtures, order) and its second group stage; the Second Division turned over; the predicted positions, the board expectations and the officials;
- `pol_first_promote_lower_division_clubs()` = 0x7ca030 (pol_first.cpp, thiscall (ecx = a Primeira Divisão), ret 0): rule: at the Polish turnover, the four best Polish clubs of the lowest division (sorted by strength, the best twelve shuffled) go up to the Second Division, and the Second Division's bottom clubs (as many as its direct relegation places) go down; each candidate keeps its division as the last one; with fewer than 4 candidates nobody moves;
- `pol_second_start_new_season()` = 0x7cc000 (pol_second.cpp, thiscall, ret 0): rule: the Polish Second Division turnover (called by the First Division's): the season reopened, last season's stages destroyed, the next year, the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials;
- `pol_cup_collect_entrants()` = 0x7c8550 (pol_cup.cpp, thiscall, ret 0): rule: the Polish Cup takes 52 clubs: the Second Division clubs placed 13th to 20th last season, then those up to 12th (class 2); then the lowest division clubs (the promoted first, then the others, sorted by strength with the best ones shuffled) until 36 are in (class 3); then last season's First Division clubs (class 1);
- `pol_lge_cup_collect_entrants()` = 0x7cad30 (pol_lge_cup.cpp, thiscall, ret 0): rule: the Polish League Cup takes 36 clubs in 2001 and 34 afterwards, all class 0: the Second Division clubs that came up from below, the other Second Division clubs, the First Division clubs placed below 7th and those up to 7th, in database order until the list is full;
- `gre_cup_collect_entrants()` = 0x5e9a80 (gre_cup.cpp, thiscall, ret 0): rule: the Greek Cup takes 44 clubs: 16 Third Division clubs (sorted by strength, the best 48 shuffled), then the Second and First Division clubs in database order until the list is full; short of clubs, the error box;
- `tur_second_b_start_new_season()` = 0x900500 (tur_second_b.cpp, thiscall, ret 0): rule: the Turkish second division category B turnover (called by the First Division's): the season reopened, the stages destroyed, the next year; the rules, the table, the order and the fixtures (in this order); the four group stages built again; the predicted positions and the officials (no board expectations);
- `tur_second_b_advance_stage()` = 0x9006c0 (tur_second_b.cpp, thiscall, ret 0): rule: the next stage of the Turkish category B: past the last stage, the error box; otherwise the current stage goes up by one; on reaching stage 4 the promotion group and the classification groups are built, on reaching stage 10 the play-off;
- `pol_first_advance_stage()` = 0x7c9ad0 (pol_first.cpp, thiscall, ret 0): rule: the next stage of the Polish First Division: past the last stage, the error box; otherwise the current stage goes up by one; leaving stage 0 (reaching 1) the final stage is built;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (3):** 3 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (db2706cd).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 be15cf32, original 8dd48626 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 310's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 311`

**Fields touched:**
- league +0x4c, +0xba, +8, +0xc, +0x30, +0x40, +0x3e, +0xc1; club +0x53 nation, +0x57 division, +0x5b last division
- cup +0x3e team count, +0x40 year, +0xa7 6-byte list; Greek cup +0xee club list, +0xf2 its size; club +0x57 division, +0x5b last division, +0x5f last position
- league +0x2c stage count, +0x30 current stage, +0x4c, +0xba, +8, +0xc, +0x40
