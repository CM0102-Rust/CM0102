# Batch 307 — the Spanish season turnover, promotion and relegation; the French season turnover and the CFA and National exchange; the Italian promotion and relegation, the Serie C2 exchange and the Serie C1 regrouping; the Italian season turnover; the Conference exchange when the Conference is not played; the Korean League Cup clubs, group A and next stage

**Routines:**
- `spa_first_process_promotion_relegation_chain()` = 0x850700 (spa_first.cpp, thiscall, ret 4): rule: promotion and relegation in Spain: records the final positions of the First and Second Divisions and swaps the marked clubs between them; with Segunda B, records its own and its first three groups' positions and swaps the Second Division with Segunda B and with each of those groups;
- `spa_first_exchange_second_division_clubs_unplayed()` = 0x8500c0 (spa_first.cpp, thiscall, ret 0): rule: without Segunda B, the Second Division's relegated clubs go down to group B1 and as many come up, first from the lower division table and then from the clubs of groups B1 to B4; a B team never goes up into its senior club's division, and a B team whose senior club is now in the Second Division is forced down, with news;
- `spa_first_start_new_season()` = 0x84feb0 (spa_first.cpp, thiscall, ret 0): rule: the Spanish turnover, led by the First Division: the B team clashes (with Segunda B), promotion and relegation and the exchange with Segunda B (or, without it, with the lower division); the First Division rebuilt (next year, rules, 20 clubs, fixtures, order) and the turnovers of the Second Division and Segunda B (or the lower division); then the predicted positions, the board expectations and the referee renewal;
- `fra_first_exchange_cfa_national_clubs()` = 0x5bc780 (fra_first.cpp, thiscall, ret 0): rule: the best CFA clubs (by reputation, shuffled in fours) go up to National, six in the 2001 turnover and four later; the clubs relegated from National go back to the CFA;
- `fra_first_start_new_season()` = 0x5bc4d0 (fra_first.cpp, thiscall, ret 0): rule: the French turnover, led by the First Division: promotion and relegation, the CFA and National exchange, the First Division rebuilt (next year, rules, 18 clubs, fixtures, order), the division tier of the National clubs capped, the turnovers of the Second Division and National; then the predicted positions, the board expectations and the referee renewal; finally a CFA and a lower league for the new year are built and dropped at once;
- `ita_ser_a_process_promotion_relegation_chain()` = 0x642410 (ita_ser_a.cpp, thiscall, ret 4): rule: promotion and relegation in Italy: records the final positions of Serie A, B, C1 A and C1 B; four up and four down between A and B; swaps between B and each C1 group; with Serie C2 loaded, its three groups' positions and the swaps between each C1 group and each C2 group;
- `ita_ser_a_exchange_serie_c2_clubs_unplayed()` = 0x641fc0 (ita_ser_a.cpp, thiscall, ret 0): rule: without Serie C2 loaded, the six best C2 clubs (by reputation, shuffled in sixes) go up to C1 A and the clubs relegated from both C1 groups go down to C2 A;
- `ita_ser_a_regroup_serie_c1_clubs()` = 0x642630 (ita_ser_a.cpp, thiscall, ret 0): rule: the 36 Serie C1 clubs are regrouped from north to south (by their stadium's city): the first 18 into C1 B and the others into C1 A; the human manager of a club that changes group gets the news;
- `ita_ser_a_start_new_season()` = 0x641700 (ita_ser_a.cpp, thiscall, ret 0): rule: the Italian turnover, led by Serie A: promotion and relegation, the exchange with the level below (Serie D and the C2 regrouping with Serie C2 loaded, otherwise the C2 exchange), the C1 regrouping, Serie A rebuilt (next year, rules, 18 clubs, fixtures, order), the division tier capped in the lowest loaded groups and the turnovers of Serie B, C1 A, C1 B (and C2); then the predicted positions, the board expectations and the referee renewal;
- `eng_prm_exchange_conference_clubs_unplayed()` = 0x574e60 (eng_prm.cpp, thiscall, ret 0): rule: without a played Conference, its strongest club (by reputation) goes up to the Third Division if its ground meets the Third Division's requirements, and the Third Division's relegated clubs go down to the Conference; otherwise the Third Division's bottom club stays up and the Conference club gets the news;
- `kor_league_cup_select_teams()` = 0x66cf20 (kor_league_cup.cpp, thiscall, ret 0): rule: the Korean League Cup has the ten K League clubs, sorted by last season's position;
- `kor_league_cup_create_group_a_table()` = 0x66c770 (kor_league_cup.cpp, thiscall, ret 0): rule: Korean League Cup group A (the cup itself) has the five even-numbered seeds (1st, 3rd, 5th, 7th and 9th in the order);
- `kor_league_cup_advance_stage()` = 0x66cad0 (kor_league_cup.cpp, thiscall, ret 0): rule: after the groups the Korean League Cup goes on to the knockout (built when leaving stage 0); going past the last stage is an error;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (10):** 10 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (570739b5).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 31731bcf, original 8e889891 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 306's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 307`

**Fields touched:**
- table row +0x37 (status 3 = relegated; the exchange marks the forced-down B team); club +0x57 and +0x5b (moved); value; writes
- club +0x57, +0x5b, +0x60 (reserve division: the CFA); table row +0x37 (status 3 = relegated); value/pointer; reads/writes
- club +0x57 (division: the C1 group), +0x5b; table row +0x37 (status 3 = relegated); value/pointer; reads/writes
- league +0x4c (writes 1 at the end of the turnover), +0xc and +0x2c (zeroed), +0x30, +0x40, +0xb1, +0xba; value/pointer; writes
- league +0xe2/+0xe4 (ground requirements); table row +0x37 (3 = relegated, 0xfe = stays); club +0x57, +0x5b; value; reads/writes
- Korean cup +0xee (list of the ten clubs), +0xf2 (places); league +0x3e, +0xb1, +0x30; value/pointer; writes
