# Batch 233 — the contract rules: club in season and club and nation among the six major ones; the contract opinion links: clear, add and remove; the club staff with the partner team ones: the assistant, the coaches and the coach count; the English names of the nation's three-letter code, the competition and the staff competition; the stadium's English name, does the language's .lng exist? and freeing the English tables; game creation: generating the missing backroom staff, trimming the club table, pruning the competition history; database load: resolving indexes to pointers and the club vector by reputation

**Routines:**
- `clube_em_temporada()` = 0x547770 (Database.cpp, cdecl): null club: 0; otherwise key_nation_club_in_season 0x669bd0(club, null, null): 1 when today is inside the season of the club nation (the whole eax);
- `clube_em_nacao_grande()` = 0x549e50 (Database.cpp, cdecl): 1 when the club nation (+0x53) is England, France, Germany, Italy, Spain or Holland (the index.cpp keys); null club or nation: 0;
- `nacao_grande()` = 0x549ea0 (Database.cpp, cdecl): the same six-key test on the nation itself; null: 0;
- `limpar_vinculos_de_opiniao()` = 0x549c90 (Database.cpp, cdecl): −1 in both links (id and value) of the person contract; null person or id ≥ [0xae2364]: nothing;
- `juntar_vinculo_de_opiniao()` = 0x549cf0 (Database.cpp, cdecl): writes the other person id and the value into the first empty link (+0x27 = −1) or else into the second, overwriting; both persons must be valid;
- `tirar_vinculo_de_opiniao()` = 0x549d60 (Database.cpp, cdecl): empties the first link with the given value or, if not, the second;
- `auxiliar_do_clube()` = 0x53cb80 (Database.cpp, cdecl): the club assistant (+0xd3); without one, the partner (senior/B) team one; none: 0;
- `treinadores_do_clube()` = 0x53cc30 (Database.cpp, cdecl): copies the 5 club coaches into an array; with a partner team (senior/B), merges both up to 5 and clears the rest;
- `contar_treinadores_do_clube()` = 0x53c8d0 (Database.cpp, cdecl): how many valid coaches (non-null, id < [0xae2364]) the club and the partner team have together;
- `sigla_inglesa_da_nacao()` = 0x5454c0 (Database.cpp, cdecl): the nation's three-letter code, or its English one from eng.lng (109-byte rows, by id, code at +0x4f) when another language is loaded;
- `nome_ingles_da_competicao()` = 0x5455a0 (Database.cpp, cdecl): the competition's name, or its English one from eng.lng (83-byte rows: club ones by id, nation ones by id minus the club count) when another language is loaded;
- `nome_ingles_da_competicao_de_pessoal()` = 0x5456a0 (Database.cpp, cdecl): the staff competition's name, or its English one from eng.lng (79-byte rows, by id) when another language is loaded;
- `nome_ingles_do_estadio()` = 0x545990 (Database.cpp, cdecl): the stadium's name, or its English one from eng.lng (52-byte rows, by id) when another language is loaded;
- `arquivo_do_idioma_existe()` = 0x535930 (Database.cpp, cdecl): builds CM3_TEMP\<language>.lng (0, 1, 11 and 12: eng.lng; 2 to 10: fra, ger, ita, esp, dan, hol, nor, por, swe) and tells whether it exists; any other language, 0;
- `liberar_tabelas_em_ingles()` = 0x544f30 (Database.cpp, thiscall): frees the 8 non-null rows of the eng.lng table, +0x20 to +0x3c, without clearing the pointers;
- `gerar_comissao_extra()` = 0x527240 (Database.cpp, cdecl): fills coaches, scouts and physios of the clubs of chosen nations by reputation (reserve rows, names of the nation, random birth date) and creates unattached managers for clubs without one; over 23,000: box and 0;
- `cortar_tabela_de_clubes()` = 0x5267c0 (Database.cpp, cdecl): trims the club table by the database size: the best per nation, those referred by staff and histories, national teams, keys and clubs of chosen nations; swaps the pointers, compacts, builds [0xae23fc] and reallocates in place;
- `podar_historico_de_competicoes()` = 0x539ae0 (Database.cpp, cdecl): keeps in the club competition history only the rows of competitions with no nation or a chosen one, reallocates, sets the national part and sorts (qsort with 0x540780);
- `resolver_indices_do_banco()` = 0x5315a0 (Database.cpp, cdecl): replaces every id field of the 12 tables with the record pointer, with the limits, the 0xde1f64 log lines, the boxes for staff with bad player, preferences or non-player data and the original bugs (club +0x5b clears +0x60; stadium +0x48 clears the city);
- `ordenar_clubes_pela_reputacao()` = 0x53db40 (Database.cpp, cdecl): with rebuild, fills the vector [0xae2d1c] with the club ids; then sorts it by reputation (qsort with 0x540020);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Save proof** (the `.qst` saved in Rust mode with and without the batch's routines, outside the bytes that vary on their own, and the holiday save loaded to 2001-08-28 with and without them, same fingerprint; the mutations also went through it): EQUAL (16856662 bytes); carga EQUAL (parou em 2001-239, 728264576 sorteios, impressao 7a1167744a713c73).
- **Mutations (21):** 9 caught. `dbl_clube_em_temporada` escaped: the 0x547770 copy is switched off (DESLIGADAS: the hook, even passthrough, changes the career in both modes), so the mutation does not run. `dbl_nacao_grande` escaped: only runs in the youth regeneration (0x7abac0), during the season; the save proof does not reach it. `dbl_juntar_vinculo` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_tirar_vinculo` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_auxiliar` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_treinadores` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbl_contar_treinadores` escaped: runs during the season (staff distribution, contract events, regeneration), not in the creation nor the .qst save; the comparator runs in the real game comparing (0 divergences). `dbm1_competicao` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm1_pessoal` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm2_estadio` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm2_lng` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). `dbm2_soltar` escaped: the test game is in English (language 0): the .lng path does not run in the creation nor the load; caught by the database self-test (CM_AUTOTESTE=banco, below in cadeia.txt). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 232's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 233`

**Fields touched:**
- TClub +0x53: nation; pointer; reads (clube_em_nacao_grande)
- TNation +0: id; integer; reads (clube_em_nacao_grande, nacao_grande). Keys 0x9cf2e4, 0x9cf300, 0x9cf314, 0x9cf360, 0x9cf488, 0x9cf338 (England, France, Germany, Italy, Spain, Holland); reads
- [0xae2d0c] (contracts, 79 bytes per person) +0x27/+0x3b: link person id; +0x37/+0x4b: link value; integers, −1 = empty; reads and writes (all three)
- TStaff +0: id; integer; reads (all three). [0xae2364]: staff count; reads
- TClub +0xd3: assistant; pointer; reads (auxiliar_do_clube)
- TClub +0x19f (5): coaches; pointers; reads (treinadores_do_clube, contar_treinadores_do_clube)
- TClub +0x244: has partner team; byte; cleared by 0x540a50 when it refuses the partner (all three)
- 0x9b5984 names language; value; reads (all three)
- 0xae2408 eng.lng table (+0x14 club competition count, +0x28 nation and +0x38 club competitions in 83-byte rows, +0x2c nations in 109-byte rows, +0x3c staff competitions in 79-byte rows); pointer; reads (all three)
- 0x9b5984 names language; value; reads (nome_ingles_do_estadio)
- 0xae2408 eng.lng table (+0x34 stadiums in 52-byte rows; +0x20..+0x3c the 8 rows); pointer; reads (nome_ingles_do_estadio, liberar_tabelas_em_ingles)
- 0xde279c path buffer (0x90bde0); text; writes (arquivo_do_idioma_existe)
- TStaff +4/+8/+0xc names, +0x10 birth date, +0x1a nation, +0x24 national team, +0x28 national job, +0x39 club, +0x3d job, +0x5f type; writes (gerar_comissao_extra)
- TClub +0x19f (5), +0x1b3 (7), +0x1cf (3): backroom staff; pointers; writes (gerar_comissao_extra)
- [0x9b59b4]: first empty common name row; writes (gerar_comissao_extra)
- TClub +0 id, +0x80 reputation, +0xb3/+0xb7/+0xbb clubs; [0xae23fc] new row → old id; [0xae235c], [0xae23b8]; writes (cortar_tabela_de_clubes)
- TNation +0x11c: chosen nation; byte; reads (all three)
- club competition history (26 bytes): +4 competition; [0xae23d4], [0xae237c], [0xae23dc]; writes (podar_historico_de_competicoes)
- TClub +0x53..+0xbb, +0xbf..+0xd3, +0xd7 (50), +0x19f, +0x1b3, +0x1cf, +0x1e0 (20); TNation +0x71..+0x118; official +4..+0x1a; stadium +0x38, +0x48; preferences +4..+0x30; non-player +0x23..+0x3f; TStaff +4..+0x24, +0x61, +0x65, +0x69; staff comp +0x53..+0x5f; staff history +4, +0xa; city +0x1f; comp history +4..+0x16; club comp +0x59..+0x65: id → pointer; writes (resolver_indices_do_banco)
- [0xae2d1c]: club id vector by reputation; writes (ordenar_clubes_pela_reputacao)

**Of interest to:** SGE, CMRTE, CMScout
