# Batch 327 — loading a cup's temporary file (Cup::load_from_disk); loading a league's temporary file (League::load_from_temp_file)

**Routines:**
- `cup_load_from_disk()` = 0x51fc00 (Cup.cpp, thiscall, ret 4): reads CM3_TEMP\<competition>_<id or year>.tmp in the order it was saved (MAPA_SAVE §6): name, statistics (a new Comp_Stats, or the old one destroyed with the pointer left dangling), the class data (virtual +0x20), prize fund, +0x52, +0x51, +0x4b, closed (+0x4c; below version 0x3e013 one byte and three skipped), stadium, bench, substitutions, +0x45, +0xb1, rounds, teams, year, +0xab, +0xac, +0x42, +0x3a, the divisions above and below, the average table (+0x18/+0x34), the competition (id → record), the rounds (0x68), the teams (6 bytes, id → club), each round's ties (0x35), the stages (type 1 Cup_Stage, type 2 League_Stage, each built and read) and the entrant book (id → club); without the file (a new game), 0; on a short read the box (some return 0 with the file left open, most stop the game);
- `league_load_from_temp_file()` = 0x687b10 (league.cpp, thiscall, ret 4): reads CM3_TEMP\<competition>_<id>.tmp (or <competition>_<year>.bak with 0) in the order it was saved (MAPA_SAVE §6): name, statistics (a new Comp_Stats, the old one not destroyed; with 0, +8 cleared), the class data (virtual +0x20), the header (required capacity and seats, prize, complete, the last update date, games per club, +0x52, importance, +0x4b, closed, stadium, bench, substitutions, the draw, active, stopped, legs, clubs, year, type, +0x42, +0x3a, the competition (id → record), rounds, points for a win and a draw, category, tie-breaks, the four bands, rules, divisions above and below, +0xe6), the round dates (0x41), the average table (+0x18/+0x34), the table (0x3b per club, id → club) with the head-to-head (n − 1 records of 0x0e, id → club only when row 0 has them) and the position history (games per club, rebuilt from −1 as (n − 1) × legs), the stages (type 1 Cup_Stage, type 2 League_Stage) and the entrant book (the old one freed; id → club); without the file (a new game), 0; on a short read the box, returning 0 with the file left open;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL fora dos campos ignorados; carga EQUAL (parou em 2001-239, 718412161 sorteios, impressao 7146f25fc4cbd6f5).
- **Mutations (6):** 5 caught. `gra29_clube_do_livro` escaped: MEASURED GAP. When loading a league's .tmp, the club of each entrant book place becomes the next club. The loop ran in two leagues of the proof save, and the proof follows the loaded career to 2001-239 with the load EQUAL to the base: nothing read those places in that stretch (the entrant book is read by the European cup draws and by building the next season). The same change in a cup's load (gra28_clube_do_livro) was caught: the load crashed. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (f6740b3f).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 ac155f01, original e8ece790 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 326's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 327`

**Fields touched:**
- Cup +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (file name), +0xa3, +0xa7, +0xab, +0xac, +0xad, +0xb1; reads from the .tmp and writes
- League +4, +8, +0xc, +0x14, +0x18, +0x1c, +0x20, +0x24, +0x28, +0x2c, +0x30, +0x34, +0x36, +0x38, +0x3a, +0x3c, +0x3e, +0x40, +0x42, +0x43, +0x44, +0x45, +0x49, +0x4a, +0x4b, +0x4c, +0x51, +0x52, +0x53 (file name), +0xa7, +0xa9, +0xab, +0xb1 (rows +0, +5, +9), +0xb5, +0xb6, +0xb8, +0xba, +0xbe, +0xc2..+0xc5, +0xd1, +0xd9, +0xdb, +0xdd, +0xe1, +0xe2, +0xe4, +0xe6; reads from the .tmp and writes

**Of interest to:** SGE
