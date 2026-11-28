# Batch 344 — the Turkish awards; the Turkish teams of the week; the Polish awards; the Polish managers of the month; the Russian awards

**Routines:**
- `turkey_awards_construct()` = 0x901c70 (award/turkey_awards.cpp, thiscall, ret 4): builds the Turkish awards: the base with the class vtable, Turkey, id 0x1a and type 1; with a file, the saved ones; otherwise the six awards;
- `turkey_awards_init()` = 0x901d00 (award/turkey_awards.cpp, thiscall, ret 4): creates the six Turkish awards: player of the year, manager of the year and the teams of the week and of the year of the first and second divisions (without reputations);
- `turkey_awards_daily_update()` = 0x901fd0 (award/turkey_awards.cpp, thiscall, ret 4): on 30 May, the Turkish awards: the player of the year from the year ratings' top three, the manager of the year and the teams of the year of both divisions; then, every day, the base daily update;
- `turkey_awards_team_of_the_week()` = 0x902080 (award/turkey_awards.cpp, thiscall, ret 4): builds the team of the week of a Turkish award: only for the first and second division teams of the week (awards 2 and 4), by the award's club competition;
- `poland_awards_construct()` = 0x7cd130 (award/poland_awards.cpp, thiscall, ret 4): builds the Polish awards: the base with the class vtable, Poland, id 0x15 and type 1, with no month ratings; with a file, the saved ones; otherwise the ten awards;
- `poland_awards_init()` = 0x7cd1c0 (award/poland_awards.cpp, thiscall, ret 4): creates the ten awards: in each division the team of the week, player, young player and manager of the year and the manager of the month (July to May);
- `poland_awards_daily_update()` = 0x7cd630 (award/poland_awards.cpp, thiscall, ret 4): on 30 May, in each division the player and the young player (up to 21) of the year from the stats and the manager of the year; on the season end day, the managers of the month move on; then, every day, the base daily update;
- `poland_awards_on_month_change()` = 0x7cd790 (award/poland_awards.cpp, thiscall): at the turn of the month, the manager of the month of each division; never runs, because Poland has no month ratings (defect of the original);
- `russia_awards_construct()` = 0x7ec210 (award/russia_awards.cpp, thiscall, ret 4): builds the Russian awards: the base with the class vtable, Russia, id 0x1b and type 1, with no month ratings; with a file, the saved ones; otherwise the seven awards;
- `russia_awards_init()` = 0x7ec2a0 (award/russia_awards.cpp, thiscall, ret 4): creates the seven awards: the Russian player of the year and, in each division, the team of the week, the team of the year and the player of the month (April to November);
- `russia_awards_daily_update()` = 0x7ec600 (award/russia_awards.cpp, thiscall, ret 4): on 30 November, the Russian player of the year from the year ratings top three and the teams of the season of both divisions; then, every day, the base daily update;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (10):** 10 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 f632fb18, original ced0d1c2 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 343's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 344`
