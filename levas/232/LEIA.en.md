# Batch 232 — freeing the staff and award histories and growing the competition history; the club-by-reputation and squad-by-ability orders; the physical and general normalisation of player attributes and the second name by weight; the version 1 converters: clubs, non-players, players; the version 1 competition converter and the nation's English name and the club's English short name; game creation: the senior and B team squads, the B team staff, the per-nation name index; game creation: removing a person from the club, the club scouts with the partner team ones

**Routines:**
- `liberar_historico_de_pessoas()` = 0x527ee0 (Database.cpp, cdecl): with the staff history table present, frees it (GlobalFree in the shared database, free otherwise) and zeroes pointer and count;
- `liberar_historico_de_premios()` = 0x527f20 (Database.cpp, cdecl): the same for the award (staff competition) history;
- `crescer_historico_de_competicoes()` = 0x527f60 (Database.cpp, cdecl): reallocs the competition history to n more rows (out of memory: 0); count, national part pointer and the new rows with their own index at +0 and the rest zero; 1;
- `ordem_dos_clubes_pela_reputacao()` = 0x540020 (Database.cpp, cdecl): qsort comparison of club ids by reputation (+0x80), highest first; negative id = 0;
- `ordem_do_elenco_pela_habilidade()` = 0x5402f0 (Database.cpp, cdecl): qsort comparison of squads: null staff last, then the player word +0xb, highest first;
- `ordem_do_elenco_pela_habilidade_atual()` = 0x549c30 (Database.cpp, cdecl): qsort comparison of a squad: no player last, then current ability (+5) and, when tied, the word +0xb, highest first;
- `normalizar_atributos_fisicos_do_jogador()` = 0x538ec0 (Database.cpp, cdecl): lowers the physical attributes above the current ability (cut by the weighted sum with the .rdata constants) and rebalances the attribute vector without cells 0x18 and 0x1e, target −40 to 0; with +0xf = 20 removes pair and trio averages; removes and restores the training progress;
- `segundo_nome_pelo_peso()` = 0x53a0e0 (Database.cpp, cdecl): a second name: with the nation having more than 10, the heaviest of 3 draws in its range; otherwise a draw over the whole table;
- `normalizar_atributos_do_jogador()` = 0x538bc0 (Database.cpp, cdecl): rebalances the player attribute vector with target −20 (12 cells within the limit, excess above 85 spread); with +0xf ≥ 15 removes the +0x2a/+0x3a average; removes and restores the training progress;
- `ler_clubes_v1()` = 0x54dc90 (Db_files.cpp, cdecl): reads version 1 club.dat field by field (577 bytes on file for 581-byte records, holes at +0x37, +0x52 and +0x244), the reputation in one byte (0 becomes 1) × 500; 0 at the first read that does not come, 1 if everything came;
- `ler_nao_jogadores_v1()` = 0x54e9a0 (Db_files.cpp, cdecl): reads the version 1 non-player table (id 9; the community map calls it players) field by field (65 bytes on file for 68-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came;
- `ler_jogadores_v1()` = 0x54f150 (Db_files.cpp, cdecl): reads the version 1 player table (id 10; the community map calls it non_players) field by field (67 bytes on file for 70-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came;
- `ler_competicoes_v1()` = 0x54ffd0 (Db_files.cpp, cdecl): reads version 1 club_comp.dat / nation_comp.dat field by field (104 bytes on file for 107-byte records, holes at +0x37 and +0x52), the reputation in one byte; 0 at the first read that does not come, 1 if everything came;
- `nome_ingles_da_nacao()` = 0x5453f0 (Database.cpp, cdecl): the nation's name, or its English one from eng.lng (109-byte rows, by id) when another language is loaded;
- `nome_curto_ingles_do_clube()` = 0x545770 (Database.cpp, cdecl): the club's short name, or its English one from eng.lng (79-byte rows, through the id map or by id) when another language is loaded;
- `cortar_elencos_de_a_e_b()` = 0x541e60 (Database.cpp, cdecl): flags every club that has a partner team (+0x244) and, for every senior team, trims the joint senior+B squad to 48 by reputation, from the bottom, keeping 3 goalkeepers;
- `juntar_o_time_b_ao_a()` = 0x544970 (Database.cpp, cdecl): for every senior team with a B team: trims the B squad to what fits in 50 with the senior one and moves the B backroom staff (coaches, scouts, physios and the manager when the senior team has none) to the senior club;
- `montar_indice_de_nomes()` = 0x539750 (Database.cpp, cdecl): allocates and builds the per-nation index of the first, second and common name tables (first row, weight total and count of every group); the last group of each table is left unwritten, as in the original;
- `tirar_pessoa_do_clube()` = 0x53c190 (Database.cpp, cdecl): removes the person from the club slot her job points to (writing the date into her, when given, and, for the senior national team manager, from the under-21 side too) and, if asked, clears the club in the person; 0 if she was not in the slot;
- `olheiros_do_clube()` = 0x53cd60 (Database.cpp, cdecl): copies the 7 club scouts into an array; with a partner team (senior/B), merges both up to 7 and clears the rest;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own; the mutations also went through it): EQUAL (16856662 bytes).
- **Mutations (20):** 11 caught. `dbi_crescer_historico_de_competicoes` escaped: only runs at the end of a competition; the save proof only creates and saves, and the load goes to 2001-08-28. `dbi_ordem_do_elenco_pela_habilidade_atual` escaped: only runs in the AI squad strength during the season, not in the creation or the .qst save. `dbj1_clubes` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj1_nao_jogadores` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj1_jogadores` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj2_competicoes` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbj2_clube_curto` escaped: the factory database and the save are version 2 and the test game is in English: neither the save proof nor the career reach it; caught by the database self-test (CM_AUTOTESTE=banco, levas/232/autoteste_banco.txt). `dbk1_juntar_comissao` escaped: no B team of the test leagues seems to have a manager, scout or physio at creation, so nothing moves to the A team (to be checked against the database). `dbk2_olheiros` escaped: the creation does not call 0x53cd60 (only the career); the save proof does not see it. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 231's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 232`

**Fields touched:**
- [0xae23cc] / [0xae2374]: staff history table and count (17 bytes); pointer; frees and zeroes (liberar_historico_de_pessoas)
- [0xae23e4] / [0xae238c]: award history table and count (58 bytes); pointer; frees and zeroes (liberar_historico_de_premios)
- [0xae2340]: database in the shared block (GlobalAlloc); value; reads (both frees)
- [0xae23d4] / [0xae237c] / [0xae23dc]: competition history (26 bytes), count and national part (from row [0xae2384]); table; writes (crescer_historico_de_competicoes)
- club +0x80: reputation; signed word; reads (ordem_dos_clubes_pela_reputacao)
- staff +0x61: player; pointer; reads. Player +5: current ability; +0xb: the tie-break word; signed words; reads (both squad orders)
- staff +0x61: player; pointer; reads. Player (70 bytes) +0x0f..+0x44: attributes; signed bytes; reads and writes (both normalisations, through 0x53a610/0x53aa10 and directly: +0x1b, +0x36, +0x40, +0x44, +0x2a, +0x3a, +0x1e, +0x24, +0x39)
- player +5: current ability; signed word; reads (normalizar_atributos_fisicos_do_jogador)
- staff +0x58: vector cell 54; byte; writes (through 0x53aa10)
- [0xde1f44]: TRAINING_MANAGER; pointer; reads (both normalisations: remove and restore the progress, 0x8b3640)
- nation name index ([0xae2338] + id × 36) +4 / +0x1c: first and count of second names; value; reads. [0xae23f4] (60 bytes) +0x3b: weight; signed byte; reads (segundo_nome_pelo_peso)
- TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x80 reputation (u16; version 1: byte, 0 becomes 1, × 500); whole record except +0x37, +0x52 and +0x244; writes (ler_clubes_v1)
- TNonPlayer (68 bytes, [0xae23c0]): +0x8/+0xa/+0xc reputations (u16; version 1: bytes); whole record; writes (ler_nao_jogadores_v1)
- TPlayer (70 bytes, [0xae23c4]): +0x9/+0xb/+0xd reputations (u16; version 1: bytes); whole record; writes (ler_jogadores_v1)
- TComp (107 bytes, [0xae23d0 / 0xae23d8]): +0x69 reputation (u16; version 1: byte); whole record except +0x37 and +0x52; writes (ler_competicoes_v1)
- 0x9b5984 names language; value; reads (nome_ingles_da_nacao, nome_curto_ingles_do_clube)
- 0xae2408 eng.lng table (+0 club count, +0x20 clubs in 79-byte rows, +0x2c nations in 109-byte rows); pointer; reads (both)
- 0xae23fc eng.lng club id map; pointer; reads (nome_curto_ingles_do_clube)
- TClub +0x244: has a partner team; byte; writes (cortar_elencos_de_a_e_b), reads (juntar_o_time_b_ao_a)
- TClub +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): club slots; pointers; writes (both, through 0x53c190 and 0x53bd10)
- TStaff +0x39 club, +0x3d job, +0x3e joined date; writes (both, through 0x53c190 and 0x53bd10)
- TNames (60 bytes): +0x37 nation, +0x3b weight; reads (montar_indice_de_nomes)
- name index [0xae2338] (0x24 bytes per nation): +0/+4/+8 first row, +0xc/+0x10/+0x14 weight total, +0x18/+0x1c/+0x20 count (first, second, common); writes (montar_indice_de_nomes)
- TClub +0xbf, +0xc3 (3), +0xcf, +0xd3, +0xd7 (50), +0x19f (5), +0x1b3 (7), +0x1cf (3): club slots; pointers; writes (tirar_pessoa_do_clube)
- TClub +0x53 nation; reads (tirar_pessoa_do_clube)
- TClub +0x1b3 (7) scouts; pointers; reads (olheiros_do_clube)
- TStaff +0x24 national team, +0x28 national job, +0x29 national date, +0x39 club, +0x3d job, +0x3e club date; writes (tirar_pessoa_do_clube)

**Of interest to:** SGE, CMRTE
