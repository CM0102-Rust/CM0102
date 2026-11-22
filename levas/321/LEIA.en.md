# Batch 321 — the competition statistics in the save: writing them; the competition statistics in the save: reading them; the club history: building, destroying and loading from the save

**Routines:**
- `comp_stats_save()` = 0x4ac4e0 (comp_screens.cpp, thiscall): writes into the competition's temporary file, in order: the club table (capacity, count, 0x76-byte entries), the player table (0x2d-byte entries), the block +0x56a, both record sets, the archive keys, the season end count, the record flags and the saved runs (freed first when the count is 0); a short write stops the game;
- `comp_stats_load()` = 0x4ab890 (comp_screens.cpp, thiscall): reads the competition's temporary file in the order it was written: new club and player tables (in data before version 0x3e015, player entries of 0x1d bytes with the rest zeroed), the block +0x56a (only after version 0x3e007), the records, the keys, the flags and the saved runs; a short read stops the game;
- `club_history_constructor()` = 0x442ab0 (club_history.cpp, thiscall, ret 4): searches the CD (the "Insert CD" box until found; a no-CD exe patches the search and the copy reads the bytes), creates the league history file list (CM3_TEMP\club league history.tmp, 21-byte records) and the cup one (club cup history.tmp, 11 bytes); with a save, loads the records; otherwise one 16-byte record per club with the four offsets at -1;
- `club_history_destructor()` = 0x442e20 (club_history.cpp, thiscall): destroys both file lists and frees the club records, zeroing the pointers and the count;
- `club_history_load()` = 0x443cd0 (club_history.cpp, thiscall, ret 4): reads from the save's "club history.dat" item the count and each club's 16-byte records; a short read or no memory stops the game;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL fora dos campos ignorados; carga EQUAL (parou em 2001-239, 718412161 sorteios, impressao 7146f25fc4cbd6f5).
- **Mutations (5):** 3 caught. `tel41_6_club_attendance` escaped: EXPLAINED GAP. In the competition statistics load, the first club's home attendance total goes up by 1 after it is read. The load proof follows the career to 2001-239 after loading, and the attendance total is only read by the screens' attendance averages and the season-end attendance record; nothing the simulation reads before then depends on it. Saving the same table was caught by the sibling tel41_5_player_entry_short (the Copa America item changes size). `tel41_6_season_records` escaped: EXPLAINED GAP. On load, the season records go into the all-time records area. Records are only read at each competition's end (update_season_end_records, after 2001-239) and by the statistics screens; the loaded career to 2001-239 does not read them. Saving them was caught by the sibling tel41_5_season_records (the English first division item changes at +0xe41). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (b99a226d).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6edd502b, original b3431db2 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 320's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 321`

**Fields touched:**
- Club_History +0 league history file list (FILE_LLIST, 0x15-byte records), +4 the cup one (0xb), +8 per-club records (0x10: +0/+4 first and last offsets of the league seasons, +8/+0xc of the cup runs, -1 none), +0xc the count; pointer and value; writes (club_history_constructor, club_history_load) and frees (club_history_destructor)

**Of interest to:** SGE, CMRTE
