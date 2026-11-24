# Batch 331 — club records: the record types of each club and the end of a nation's season

**Routines:**
- `club_records_record_type_applies()` = 0x449590 (club_records.cpp, thiscall (ecx nao lido), ret 0xc): tells whether record type (1 to 32) applies to the club in the period (1 all-time, 2 the season): the club-only types (league positions, attendance, results, league appearances and goals, transfers...) apply to anyone who is not a national team; 27 (caps) only to national teams; 32 (the fans' player of the year) only to clubs and never in the all-time period; the others, and a type outside 1..32, always. A null club: the error box and 0;
- `club_records_end_season_for_nation()` = 0x447880 (club_records.cpp, thiscall, ret 4): closes the season of every club of the key nation: a season average attendance above -1 that beats the all-time one (-1 = none) goes to the all-time record with the year; a season rating still at -1.0 is worked out; a season average rating above -1.0 that beats the all-time one is copied with the year; for each broken record the news is queued (a human club or national team with 5 seasons or more, without repeats); with a season record, the fans' player of the year (type 32): when there is one, news item 0x3e8 for a club with a human manager and the person's supporters' record (with the year and the nation's split-season flag); the season goes to the history file (the year and the 0x38b-byte record set) and the season record is reset (-1); always, one more season in the club's entry and the season counters cleared. Returns 1; a null nation: the error box and 0. Defect of the original: the attendance and rating checks read the all-time record before testing it for null (and only then create it); since the match creates both together, the creation never runs;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (5):** 5 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (40d35960).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 e8d3d34b, original 531044b3 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 330's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 331`
