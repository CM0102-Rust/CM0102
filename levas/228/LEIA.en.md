# Batch 228 — reading the .dat files: continents, nations, cities; reading the .dat files: stadiums, staff 157, staff history; reading the .dat files: competition histories, staff competitions, staff competition history

**Routines:**
- `ler_continentes()` = 0x54d5e0 (Db_files.cpp, cdecl): reads the continent.dat table (id 3) of 198-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_nacoes()` = 0x54d740 (Db_files.cpp, cdecl): reads the nation.dat table (id 4) of 290-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came;
- `ler_cidades()` = 0x54d8a0 (Db_files.cpp, cdecl): reads the city.dat table (id 21) of 56-byte records in one go, through zread (save) or fread (Data), with the entry version 1 to 2 (other: box and exit); 1 if everything came;
- `ler_estadios()` = 0x54da00 (Db_files.cpp, cdecl): reads the stadium.dat table (id 5) of 78-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_pessoas_157()` = 0x54e780 (Db_files.cpp, cdecl): reads the staff.dat (staff, 157-byte file version) table (id 6) of 157-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_historico_de_pessoas()` = 0x54fd60 (Db_files.cpp, cdecl): reads the staff_history.dat table (id 17) of 17-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_historicos_de_competicao()` = 0x550230 (Db_files.cpp, cdecl): reads the club_comp_history.dat and nation_comp_history.dat table (id 19, 20) of 26-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;
- `ler_competicoes_de_pessoal()` = 0x550380 (Db_files.cpp, cdecl): reads the staff_comp.dat table (id 11) of 101-byte records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if everything came;
- `ler_historico_de_competicoes_de_pessoal()` = 0x5504d0 (Db_files.cpp, cdecl): reads the staff_comp_history.dat table (id 18) of 58-byte records in one go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856662 bytes).
- **Mutations (9):** 8 caught. `dbf_historico_de_competicoes_de_pessoal` escaped: the staff competition history table is not a .qst item (there is no staff_comp_history.dat in the container) and the quick start writes nothing derived from it; the save proof does not see it. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 227's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 228`

**Fields touched:**
- index.dat entry +0x37: record count; value; reads (all three)
- index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)
- TContinent (198 bytes, [0xae23a4]): +0x0 id; +0x4 name (26); +0x1f three letters; +0x23 continentality; +0x3d federation (101); +0xa3 short federation; +0xbe regional strength (double); whole record; writes (ler_continentes)
- TNation (290 bytes, [0xae23a8]): +0x0 id; +0x4 name (51, the name key); +0x57 nationality; +0x71 continent; +0x7a capital; +0x80 stadium; +0x8e reputation; +0x90..+0xa4 colours; +0xa8..+0x108 FIFA/UEFA coefficients (double); +0x110..+0x118 rivals (ids that become pointers in resolve); whole record; writes (ler_nacoes)
- TCities (56 bytes, [0xae23ac]): +0x0 id; +0x4 name (26); +0x1f nation; +0x23 latitude and +0x2b longitude (double); +0x33 attraction; +0x34 weather; whole record; writes (ler_cidades)
- index.dat entry +0x37: record count; value; reads (all three)
- index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)
- TStadiums (78 bytes, [0xae23b0]): +0x0 id; +0x4 name (51); +0x38 city; +0x3c capacity; +0x40 seating; +0x44 expansion; +0x48 nearby stadium; +0x4c covered; +0x4d undersoil heating; whole record; writes (ler_estadios)
- TStaff v1 (157 bytes, [0xae2404 (temporária; convertida para 110 bytes em 0xae23bc)]): 157 bytes; the first fields match the 110-byte TStaff (birth +0x10, year +0x18, contract +0x3e/+0x46); the full layout is in CM0102-Editor (P8); whole record; writes (ler_pessoas_157)
- TStaffHistory (17 bytes, [0xae23cc]): +0x0 id; +0x4 staff; +0x8 year; +0xa club; +0xe on loan; +0xf apps; +0x10 goals; whole record; writes (ler_historico_de_pessoas)
- index.dat entry +0x37: record count; value; reads (all three)
- index.dat entry +0x3f: table version; value; reads (all three; outside the accepted version the original opens the box)
- TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): +0x0 id; +0x4 competition; +0x8 year; +0xa winners; +0xe runners-up; +0x12 third place; +0x16 host; whole record; writes (ler_historicos_de_competicao)
- TStaffComp (101 bytes, [0xae23e0]): +0x0 id; +0x4 name (51, the name key); +0x38 short name; +0x53 continent; +0x57 nation; +0x5b/+0x5f colours; +0x63 reputation; whole record; writes (ler_competicoes_de_pessoal)
- TStaffCompHistory (58 bytes, [0xae23e4]): +0x0 id; +0x4 competition; +0x8 year; +0xa..+0x36 winner, runner-up and third (first name, second name, staff, info); whole record; writes (ler_historico_de_competicoes_de_pessoal)

**Of interest to:** SGE, CMRTE
