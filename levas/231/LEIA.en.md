# Batch 231 — writing the .dat files: staff history, competitions, competition histories; writing the .dat files: staff competitions, officials, colours; writing the .dat files: names; opening and creating the .dat files

**Routines:**
- `gravar_historico_de_pessoas()` = 0x54fe50 (Db_files.cpp, cdecl): writes n 17-byte records (staff_history.dat, id 17) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went;
- `gravar_competicoes()` = 0x5501d0 (Db_files.cpp, cdecl): writes n 107-byte records (club_comp.dat, in two calls: the competitions and the 127 reserve ones; nation_comp.dat) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went;
- `gravar_historicos_de_competicao()` = 0x550320 (Db_files.cpp, cdecl): writes n 26-byte records (club_comp_history.dat and nation_comp_history.dat, ids 19 and 20) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went;
- `gravar_competicoes_de_pessoal()` = 0x550470 (Db_files.cpp, cdecl): writes n 101-byte records (staff_comp.dat, id 11) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went;
- `gravar_arbitros()` = 0x5506b0 (Db_files.cpp, cdecl): writes n 43-byte records (officials.dat, id 7) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went;
- `gravar_registros_58()` = 0x550800 (Db_files.cpp, cdecl): writes n 58-byte records (colour.dat, id 2, and staff_comp_history.dat, id 18: the same routine for both) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went;
- `gravar_nomes()` = 0x550950 (Db_files.cpp, cdecl): writes n 60-byte records (first_names.dat, second_names.dat and common_names.dat, ids 13 to 15) in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went (it only writes, despite the community name);
- `abrir_arquivo_do_banco()` = 0x54d580 (Db_files.cpp, cdecl): builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "rb"); the comparator checks the path;
- `criar_arquivo_do_banco()` = 0x54d5b0 (Db_files.cpp, cdecl): builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "wb"); only when writing the database to the folder; the comparator checks the path;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856662 bytes).
- **Mutations (9):** 7 caught. `dbh_gravar_historico_de_pessoas` escaped: save_database only calls 0x54fe50 when [0xae23cc] ≠ 0 (0x5311ac); in the game the staff history has already become staff history.tmp, so the write only runs when saving the database to the Data folder, never into the .qst. `dbh_criar` escaped: 0x54d5b0 is only called from save_database(…, 0, …), saving the database to the Data folder; the .qst is opened by zopen 0x9326e0. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 230's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 231`

**Fields touched:**
- TStaffHistory (17 bytes, [0xae23cc]): whole record; reads (gravar_historico_de_pessoas)
- TComp (107 bytes, [0xae23d0 / 0xae23d8]): whole record; reads (gravar_competicoes)
- TCompHistory (26 bytes, [0xae23d4 / 0xae23dc]): whole record; reads (gravar_historicos_de_competicao)
- TStaffComp (101 bytes, [0xae23e0]): whole record; reads (gravar_competicoes_de_pessoal)
- TOfficials (43 bytes, [0xae23e8]): whole record; reads (gravar_arbitros)
- TColour (58 bytes, [0xae23ec]) and TStaffCompHistory (58 bytes, [0xae23e4]): whole record; reads (gravar_registros_58)
- TNames (60 bytes, [0xae23f0 / 0xae23f4 / 0xae23f8]): whole record; reads (gravar_nomes)
- 0xde1f64 (.dat path, 260): text; writes (abrir_arquivo_do_banco, criar_arquivo_do_banco)

**Of interest to:** SGE, CMRTE
