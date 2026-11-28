# Batch 346 — the Welsh awards; the Welsh manager of the month and team of the week; the Scottish awards

**Routines:**
- `wales_awards_construct()` = 0x90ead0 (award/wales_awards.cpp, thiscall, ret 4): builds the Welsh awards: the base with the class vtable, Wales, id 0x1c and type 1; with a file, the saved ones; otherwise the month ratings (out of memory, the error box and no awards) and the six awards;
- `wales_awards_init()` = 0x90ec10 (award/wales_awards.cpp, thiscall, ret 4): creates the six Welsh Premier Division awards: the team of the week, player, young player, top goalscorer and manager of the year, and the manager of the month (August of the current year to April of the next);
- `wales_awards_daily_update()` = 0x90ef10 (award/wales_awards.cpp, thiscall, ret 4): on 11 May, the Welsh awards: player, top goalscorer and young player (up to 21) of the year and the manager of the year; on the nation's season end day, the month award moves to the next season; then, every day, the base daily update;
- `wales_awards_on_month_change()` = 0x90f020 (award/wales_awards.cpp, thiscall): at the turn of the month, the Welsh Premier Division manager of the month: the staff records' best manager of the month takes the month award, with his club;
- `wales_awards_team_of_the_week()` = 0x90f060 (award/wales_awards.cpp, thiscall, ret 4): builds the team of the week of a Welsh award: only for the team of the week (award 0);
- `scotland_awards_construct()` = 0x7f5170 (award/scotland_awards.cpp, thiscall, ret 4): builds the Scottish awards: the base with the class vtable, Scotland, id 0x11 and type 1; with a file, the saved ones; otherwise the month ratings and the 27 awards;
- `scotland_awards_init()` = 0x7f52b0 (award/scotland_awards.cpp, thiscall, ret 4): creates the 27 awards: the Scottish player, top goalscorer and young player of the year and, in each of the four divisions, the team of the week, manager, player and young player of the month (August to May), manager of the year and team of the year;
- `scotland_awards_daily_update()` = 0x7f5dc0 (award/scotland_awards.cpp, thiscall, ret 4): on the Saturday of 23 May, the manager of the year of each division, the Scottish player, young player (up to 21) and top goalscorer of the year from the year ratings top three and the team of the season of each division; on the season end day, the twelve month awards move on; then, every day, the base daily update;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (8):** 8 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 abec784e, original eb48bed6 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 345's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 346`
