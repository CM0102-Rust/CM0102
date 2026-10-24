# Batch 230 — writing the .dat files: continents, nations, cities; writing the .dat files: stadiums, clubs, staff 110; writing the .dat files: non-players, players, preferences

**Routines:**
- `gravar_continentes()` = 0x54d6e0 (Db_files.cpp, cdecl): writes the continent.dat (id 3) table of 198-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_nacoes()` = 0x54d840 (Db_files.cpp, cdecl): writes the nation.dat (id 4) table of 290-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_cidades()` = 0x54d9a0 (Db_files.cpp, cdecl): writes the city.dat (id 21) table of 56-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_estadios()` = 0x54daf0 (Db_files.cpp, cdecl): writes the stadium.dat table of 78-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_clubes()` = 0x54e5d0 (Db_files.cpp, cdecl): writes the club.dat and nat_club.dat (ids 0, 1) table of 581-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_pessoas_110()` = 0x54e720 (Db_files.cpp, cdecl): writes the staff.dat (staff, 110 bytes; id 6) table of 110-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_nao_jogadores()` = 0x54efd0 (Db_files.cpp, cdecl): writes the staff.dat non-player (id 9; the community map calls it players) table of 68-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_jogadores()` = 0x54fbb0 (Db_files.cpp, cdecl): writes the staff.dat player (id 10; the community map calls it non_players) table of 70-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;
- `gravar_preferencias()` = 0x54fd00 (Db_files.cpp, cdecl): writes the staff preferences (id 22) table of 52-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything went out;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856662 bytes).
- **Mutations (9):** 9 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 229's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 230`

**Fields touched:**
- TContinent (198 bytes, [0xae23a4]): whole record (+0xbe regional strength, double); reads (gravar_continentes: goes to the file)
- TNation (290 bytes, [0xae23a8]): whole record (rivals already turned into indexes by save_database); reads (gravar_nacoes)
- TCities (56 bytes, [0xae23ac]): whole record; reads (gravar_cidades)
- TStadium (78 bytes): whole record (+0x3c capacity); reads (gravar_estadios)
- TClub (581 bytes, [0xae23b4 / 0xae23b8]): whole record (pointers already turned into indexes); reads (gravar_clubes)
- TStaff (110 bytes, [0xae23bc]): whole record (pointers already turned into indexes); reads (gravar_pessoas_110)
- TNonPlayer (68 bytes, [0xae23c0]): whole record; reads (gravar_nao_jogadores)
- TPlayer (70 bytes, [0xae23c4]): whole record; reads (gravar_jogadores)
- TPreferences (52 bytes, [0xae23c8]): whole record; reads (gravar_preferencias)

**Of interest to:** SGE, CMRTE
