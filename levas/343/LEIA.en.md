# Batch 343 — the Swedish awards; the Swedish teams of the week and the Swedish transfer rules; a nation's player of the year by the season statistics and the orders of its candidates; the Norwegian month awards

**Routines:**
- `sweden_awards_construct()` = 0x893720 (award/sweden_awards.cpp, thiscall, ret 4): builds the Swedish awards: the base with the class vtable, Sweden as the nation, id 0x13 and type 1; with a file, the saved awards; otherwise the ten awards;
- `sweden_awards_init()` = 0x8937b0 (award/sweden_awards.cpp, thiscall, ret 4): creates the ten Swedish awards: manager of the year, player, fans' player, goalkeeper, defender, midfielder and forward of the year, the Premier Division team of the week, its top goalscorer and the First Division team of the week, each with its national and world reputations;
- `sweden_awards_daily_update()` = 0x893bf0 (award/sweden_awards.cpp, thiscall, ret 4): on 1 December, the Swedish year awards: the manager (from the Premier Division), the player and the fans' player from the year ratings, goalkeeper, defender, midfielder and forward from the ratings' top three by position, and the Premier Division top goalscorer; then, every day, the base daily update;
- `sweden_awards_team_of_the_week()` = 0x893d60 (award/sweden_awards.cpp, thiscall, ret 4): builds the team of the week of a Swedish award: only for the Premier Division and First Division teams of the week (awards 7 and 9), by the award's club competition;
- `construct_sweden_rules()` = 0x893d80 (award/sweden_awards.cpp, thiscall, ret 0x14): builds the Swedish transfer rules: the base, the class vtable, no transfer windows and the +0x15 flag = 1 (a type 5 contract does not make a player ineligible);
- `nation_awards_award_player_of_year_by_stats()` = 0x769380 (nation_awards.cpp, thiscall, ret 0x24): rule: each player whose club (if any) plays in a running competition and who passes the nation filters (the club's, his own, the excluded one, his own or his club's) gets a score from the mode's statistic (category 2 in mode 2, else 6) and his games; in mode 2 a score of 0 leaves him out; in mode 3 the score is per game; outside mode 2 it is multiplied by half the reputation weight ((club reputation / 500 + division reputation x 4) / 5, or 1). Sorted by score (lowest first in mode 3), the first three who play one of the positions (a centre back who also plays full back does not count), have enough games (a share of 25 games: 0.66, or 0.01 in modes 2 and 5, down by 0.1) and pass the age rule go to the award history. Defects: without a candidate the block is not freed; outside mode 2 the record's score keeps stack garbage;
- `nation_awards_compare_score_ascending()` = 0x76a480 (nation_awards.cpp, cdecl): rule: the candidates by score, the lowest first (unordered counts as lower);
- `nation_awards_compare_score_descending()` = 0x76a4b0 (nation_awards.cpp, cdecl): rule: the candidates by score, the highest first (unordered counts as lower);
- `norway_awards_on_month_change()` = 0x794430 (award/norway_awards.cpp, thiscall): at the turn of the month, the manager of the month of each division (the best manager of the month, with his club) and the player of the month of each from the month ratings;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (10):** 10 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 77f1b62b, original 18b0f378 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 342's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 343`

**Fields touched:**
- statistics award candidate (10 bytes): +0 person, +4 score (f32), +8 games (word); values; writes and reads (nation_awards_award_player_of_year_by_stats and the orders)
