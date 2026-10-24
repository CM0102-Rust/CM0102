# Batch 229 — reading the .dat files: officials, colours, names; reading the .dat files with a converter: clubs, competitions, non-players; reading the .dat files: players, staff 110, preferences

**Routines:**
- `ler_arbitros()` = 0x5505c0 (Db_files.cpp, cdecl): reads the officials.dat table (id 7) of 43-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_cores()` = 0x550710 (Db_files.cpp, cdecl): reads the colour.dat table (id 2) of 58-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_nomes()` = 0x550860 (Db_files.cpp, cdecl): reads the first_names.dat, second_names.dat and common_names.dat tables (ids 13, 14, 15) of 60-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_clubes()` = 0x54db50 (Db_files.cpp, cdecl): reads the club.dat and nat_club.dat table (ids 0, 1) of 581-byte records in one go, through zread (save) or fread (Data), with the entry version 2, and turns reputation 0 into 1; version 1: the 0x54dc90 converter (other: box and exit); 1 if everything came;
- `ler_competicoes()` = 0x54feb0 (Db_files.cpp, cdecl): reads the club_comp.dat and nation_comp.dat table (ids 12, 16) of 107-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54ffd0 converter (other: box and exit); 1 if everything came;
- `ler_nao_jogadores()` = 0x54e880 (Db_files.cpp, cdecl): reads the non-player table of staff.dat (id 9; the community map calls it players) of 68-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54e9a0 converter (other: box and exit); 1 if everything came;
- `ler_jogadores()` = 0x54f030 (Db_files.cpp, cdecl): reads the player table of staff.dat (id 10; the community map calls it non_players) of 70-byte records in one go, through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54f150 converter (other: box and exit); 1 if everything came;
- `ler_pessoas_110()` = 0x54e630 (Db_files.cpp, cdecl): reads the staff.dat (staff, 110-byte file version: the save and community databases) table (id 6) of 110-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came;
- `ler_preferencias()` = 0x54fc10 (Db_files.cpp, cdecl): reads the staff preferences table (id 22: the save and community databases) of 52-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutations (9):** 9 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 228's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 229`

**Fields touched:**
- index.dat entry +0x37: record count; value; reads (all three)
- index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)
- TOfficials (43 bytes, [0xae23e8]): +0x0 id; +0x4 first name and +0x8 second name (ids); +0xc birth date; +0x14 year; +0x16 nation; +0x1a city; +0x1e current ability; +0x20 potential; +0x22 reputation; +0x24..+0x2a ratings (allowing flow, discipline, important matches, pressure, refereeing, running the line, time keeping); whole record; writes (ler_arbitros)
- TColour (58 bytes, [0xae23ec]): +0x0 id; +0x4 name (51); +0x37 red; +0x38 green; +0x39 blue; whole record; writes (ler_cores)
- TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): +0x0 name (51); +0x33 id; +0x37 nation; +0x3b count (weight); whole record; writes (ler_nomes)
- index.dat entry +0x37: record count; value; reads (all three)
- index.dat entry +0x3f: table version; value; reads (all three; 1 goes to the converter; outside 1 and 2 the original opens the box)
- TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x0 id; +0x4 name (51, the name key); +0x53 nation; +0x65 cash; +0x69 stadium; +0x73..+0x7b attendance; +0x80 reputation (u16; 0 becomes 1); +0x83..+0x97 colours; +0xb3..+0xbb rivals; +0xd7 squad (50 ids); whole record; writes (ler_clubes)
- TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x0 id; +0x4 name (51, the name key); +0x38 short name; +0x53 three letters; +0x57 scope; +0x58 selected; +0x59 continent; +0x5d nation; +0x61/+0x65 colours; +0x69 reputation; whole record; writes (ler_competicoes)
- TNonPlayer (68 bytes, [0xae23c0]): +0x0 id; +0x4 current ability; +0x6 potential; +0x8/+0xa/+0xc reputations (home, current, world); +0xe.. staff attributes (+0xe attacking, +0xf business, +0x10 coaching...); whole record; writes (ler_nao_jogadores)
- index.dat entry +0x37: record count; value; reads (all three)
- index.dat entry +0x3f: table version; value; reads (all three; players in version 1 go to the converter; outside the accepted version the original opens the box)
- TPlayer (70 bytes, [0xae23c4]): +0x0 id; +0x4 squad number; +0x5 current ability; +0x7 potential; +0x9/+0xb/+0xd reputations (home, current, world); +0xf.. positions (+0xf goalkeeper, +0x10 sweeper, +0x11 defender...); attributes; whole record; writes (ler_jogadores)
- TStaff (110 bytes, [0xae23bc]): +0x0 id; +0x4/+0x8/+0xc names (ids); +0x10 birth date; +0x18 year; +0x1a nation; +0x39 club; +0x3e/+0x46 contract; +0x4e wage; +0x52 value; +0x56..+0x5d traits; +0x61 player, +0x65 preferences, +0x69 non-player (indexes); whole record; writes (ler_pessoas_110)
- TPreferences (52 bytes, [0xae23c8]): +0x0 id; +0x4..+0xc favourite clubs; +0x10..+0x18 disliked clubs; +0x1c..+0x24 favourite staff; +0x28..+0x30 disliked staff (ids); whole record; writes (ler_preferencias)

**Of interest to:** SGE, CMRTE
