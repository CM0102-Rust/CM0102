# Batch 220 — the saved game: creating the game from the setup file (quick start and automatic career); the saved game: saving the whole game

**Routines:**
- `criar_jogo_pela_configuracao()` = 0x827110 (Setup.cpp, cdecl): reads auto_setup.cfg/quick_start.cfg (database, stats, foreground and background leagues, language, engine, bids, holiday), creates the game (0x828110) and saves the .qst (0x82e430 with 0x17) and frees everything, or puts the human manager on holiday at a drawn club;
- `gravar_jogo()` = 0x82e430 (Setup.cpp, cdecl): saves the game: deletes the old file, the database (0x524d30), general.dat (16 fields and the summary), the subsystem saves in the original order with disk and memory checks between them, packs the *.tmp files, the FIFA rules; on a network game, writes to temp_save_file and renames;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856662 bytes).
- **Mutations (5):** 3 caught. `sv3_criar_id` escaped: the 0x827110 copy is switched off (DESLIGADAS: running on the private stack it changes the residue create_game 0x828110 reads and club.dat comes out different), so the mutation does not run. `sv3_criar_frente` escaped: the 0x827110 copy is switched off (DESLIGADAS: running on the private stack it changes the residue create_game 0x828110 reads and club.dat comes out different), so the mutation does not run. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 219's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 220`

**Fields touched:**
- [0xae3138] game options (game.cfg) +4: bit 2 (league type); value; reads; +0x113: game name ("auto_setup_game"); text; writes (criar_jogo_pela_configuracao)
- nation +0x11c: league mark (foreground 2 or 6, background 1 or 5, by options bit 2); value; reads and writes (criar_jogo_pela_configuracao)
- [0xae2d00] marked leagues, [0xae2d04] foreground leagues; value; writes; [0xae2d08] default nation; pointer; writes (criar_jogo_pela_configuracao)
- [0xa5a390] match engine enabled (byte), [0x9cd1f8] computer bids; value; writes (criar_jogo_pela_configuracao)
- [0xdd7124], [0xdd713c], [0xdd7130] game flags; value; writes 1 (criar_jogo_pela_configuracao)
- current human 0xb74c26 + 0x300 × (i16)[0xb77c7a]: manager person; pointer; reads and writes (criar_jogo_pela_configuracao)
- person (0x6e) +0x5f: human (1); value; reads and writes; +0x18 year 1966, +0x10 date, +0x24 and +0x39 = 0, +0xc text cleared; +0x1a nation; pointer; +4 and +8 names; id; writes (criar_jogo_pela_configuracao)
- club (0x245) +0x53: nation; pointer; reads (criar_jogo_pela_configuracao)
- [0xdd7110] save version (0xfffc1fb9); value; writes (gravar_jogo)
- general.dat: [0xdd7110] version, summary (0xf74), [0xdd7120] unique id, [0xdd7114] database size (1 byte), [0xdd6ec8] and [0xae2c90] dates, [0xae2c88] time, [0xae2c78] last date, [0xae2cfc] last time, [0xae2d00] and [0xae2d04] leagues, default nation (id at +0 of [0xae2d08] or −1), [0xae2d10] × 0x4f from [0xae2d0c], [0xae2d18] × 6 from [0xae2d14], [0xae235c] × 4 from [0xae2d1c]; bytes; reads and writes into the save (gravar_jogo)
- [0xae3138] game options +4: bits 0x8000 (no compression), 0x20000000 (network), 0x40000000 (check disk first); value; reads (gravar_jogo)
- [0xdd7134] out of disk, [0xdd7138] out of memory; value; writes 1 (gravar_jogo)
- the subsystem globals (PASSOS_DO_SAVE; [0xb6b7c8], the Media one, is also the FOG_OF_WAR [0xae2a60] condition) and [0xb5ded8] (hall of fame); pointer; reads (gravar_jogo)
- [0xdd7130] game saved, [0xa801bc]; value; writes 1 and 0; 0xdd70a0 last save date; value; writes (gravar_jogo)

**Of interest to:** SGE, CMRTE
