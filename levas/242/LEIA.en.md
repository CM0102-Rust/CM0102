# Batch 242 — national teams: ending the call-ups of a competition and pruning the call-up records; national teams: adding to and removing from the squad and releasing the over-age under-21 players; national teams: the current competition of each nation and calling up a replacement; national teams: the call-up pools of each nation and their two orderings; friendlies: the friendly opponent and the tour opponent; the league table: adding a result and the points for a result; the league table: the order by stored head-to-head, the date order, the club row and whether the league has the club

**Routines:**
- `encerrar_convocacoes_da_competicao()` = 0x774f30 (national_teams.cpp, thiscall, ret 4): in the call-up record list, those of the competition (+4) end tomorrow (+0x10, the day from 0x54c5b0);
- `encerrar_convocacoes_da_nacao()` = 0x774f90 (national_teams.cpp, thiscall, ret 8): like 0x774f30, only the records of the competition (+4) and the nation (+0);
- `podar_registros_de_convocacao()` = 0x7774b0 (national_teams.cpp, thiscall): in each club record (+0x20, 6 bytes), drops the expired entries (0x456770) at the start, moves the rest down (0x777590) and frees the emptied buffer;
- `por_no_elenco_da_selecao()` = 0x776250 (national_teams.cpp, thiscall, ret 0xc): the first of the 50 free squad slots of the team (+0xd7; type 0/1 the senior team, else the under-21) takes the person, who gets the team (+0x24), today's date (+0x29), mark 0xb (+0x28) and squad bit 4 or 8 (0x540980);
- `tirar_do_elenco_da_selecao()` = 0x7762f0 (national_teams.cpp, thiscall, ret 0xc): removes the person from the team tactics (0x896c60) and, if found in a slot, empties it and clears +0x24, +0x28 and the national bits of +0x5e;
- `dispensar_acima_da_idade_do_sub21()` = 0x76d880 (national_teams.cpp, thiscall, ret 4): once the nation record date (+0x62) has passed, removes from the under-21 squad everyone older than 21 (+0x18) and clears record +0x6b;
- `limpar_competicao_atual()` = 0x775000 (national_teams.cpp, thiscall, ret 4): the nation current competition (record +4) becomes −1 and, if there is a next one ([this+8][id]), that one takes its place, notifying the managers (0x6a3770);
- `liberar_competicoes_do_continente()` = 0x775070 (national_teams.cpp, thiscall, ret 8): in the nations of the continent (+0x71), the current and next competitions that ended (competition +0x59 null, or stage [0x9cfa10]) become −1; empty current and a next one: the next moves up, with the notice (0x6a3770);
- `convocar_substituto()` = 0x775f90 (national_teams.cpp, thiscall, ret 0x10): in the pool of the level, the first eligible player of the nation who plays a position of the one leaving (0x5b7d20) is called up (0x776250, leaving another squad first through 0x7762f0) with the news (0x76fd10); nobody: clears the type slot count (+0x45);
- `montar_reservas_das_selecoes()` = 0x76abf0 (national_teams.cpp, thiscall): clears the 4 levels of each nation record and puts every eligible person in the nation pool (and the under-21 one, up to 21 years and under 5 caps), also by the second nationality; full section: enters by rating, and sorts (qsort with 0x778ed0);
- `ordem_das_reservas()` = 0x778ed0 (national_teams.cpp, cdecl): the pool qsort order: rating (+9 of the player data) descending;
- `ordem_pelo_clube()` = 0x778ef0 (national_teams.cpp, cdecl): the order by club (+0x39): no club last, clubs by id, within a club squad bit 4 before 8 (+0x5e);
- `adversario_do_amistoso()` = 0x5c4cb0 (friendly.cpp, thiscall, ret 0x14): searches (bsearch) the candidates of the band (1 to 20, going up until found) and, from there, the first one that is not the club, excluded, already an opponent, out by date, of crossed bands, a rival, of a rival nation (half the time), from the same city or above the drawn band, that passes the nation check by the club reputation and is drawn (1 in 5); else 0;
- `adversario_da_excursao()` = 0x5c5040 (friendly.cpp, thiscall, ret 0x14): searches (bsearch) the candidates of the tour nation and, from its first one, the one not above the maximum band nor out by date, the club itself, already an opponent, a rival, of a rival nation (half the time), from the same city or above the drawn band, drawn half the time; stops at the first one of another nation; else 0;
- `aplicar_resultado_na_tabela()` = 0x685f60 (league.cpp, thiscall, ret 0x18): finds the home and away rows and adds the game at home and away (played, goals, win/draw/loss, by extra time and penalties, marking +0x3d/+0x3e), the win points by the +0x90 virtual and the draw points +0xc3; redoes the totals, returns both rows points and adds the head-to-head (0x6869a0);
- `pontos_do_resultado()` = 0x68aad0 (league.cpp, thiscall, ret 8): the win points (+0xc2) to the winner, 0 to the loser; when level, by extra time (+0xab bit 2), by penalties (bits 0x21: the draw points +0xc3 to the winner) or the draw points;
- `ordem_pela_posicao_do_confronto()` = 0x683410 (league.cpp, cdecl): qsort callback: the own compare, the points, the club id with no games, the stored head-to-head position (+4) and the name;
- `ordem_das_datas_da_liga()` = 0x685cf0 (league.cpp, cdecl): bsearch callback: the year (+2, unsigned) and the day (+0): −1, 0 or 1;
- `linha_do_clube()` = 0x68ab70 (league.cpp, thiscall, ret 8): the club row in the given rows (null: the league table), or 0;
- `liga_tem_o_clube()` = 0x68abd0 (league.cpp, thiscall, ret 4): 1 when the club has a row in the league table;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (20):** 16 caught. `cpd_atual_aviso` escaped: no nation with a next competition before 2002-06-01 (the "seleção a competição seguinte" branch is not in the base); also escaped the game comparing and the pt load (0 divergences). `cpd_continente_aviso` escaped: the next competition becoming the current one in the continent does not happen before 2002-06-01 (the "seleção continente a seguinte vira a atual" branch is not in the base); also escaped the game comparing and the pt load (0 divergences). `cpe_datas_ano` escaped: ordem_das_datas_da_liga 0x685cf0: in the career searches the year always ties, only the day decides; also escaped the game comparing and the pt load (0 divergences). `cpe_linha_do_clube` escaped: linha_do_clube 0x68ab70 (31 direct calls): the row found changes nothing the prints and the comparator see before 2002-06-01; also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 241's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 242`

**Fields touched:**
- call-up record: +0 nation, +4 competition, +0x10 end day (short); reads and writes (encerrar_convocacoes_*)
- national team manager +0x20: club records (6 bytes: +0 buffer of 9-byte entries, date and mark; +4 mark; +5 count); reads and writes (podar_registros_de_convocacao)
- team (TClub) +0xd7: the 50 national squad slots; reads and writes (all three)
- person (0x6e bytes) +0x18 age, +0x24 national team, +0x28 called-up mark, +0x29 date, +0x5e squad bits; reads and writes (all three)
- nation record in the national team manager ([this+4] + 0x6e × id) +0x62 date, +0x6b; reads and writes (dispensar_acima_da_idade_do_sub21)
- nation record in the national team manager +4 current competition, +0x3a dates and +0x45 slots per type (20 bytes); [this+8] next competition per nation; reads and writes
- TNation (0x122 bytes) +0x71 continent; competition ([0xae23d0], 107 bytes) +0x59; reads (liberar_competicoes_do_continente)
- nation record ([this+4] + 0x6e × id): +8/+0x1d pool start, +0xd/+0x22 count, +0xe/+0x23 slots, +0xf/+0x24 lowest rating (4 levels of 4 bytes); [this+0xc]/[this+0x10] pools; reads and writes (montar_reservas_das_selecoes)
- person (0x6e bytes) +0x18, +0x1a/+0x1e nationalities, +0x22 caps, +0x24, +0x28, +0x39 club, +0x3d, +0x5e, +0x61 player data (+9 rating); reads
- friendly candidate (0x16 bytes): +0 club id, +4 drawn band, +5 band, +6 nation, +0xa opponent list, +0xe/+0x10 bands (shorts); reads and writes the list cursor (+0xa) (adversario_do_amistoso, adversario_da_excursao)
- club (0x245 bytes): +0x53 nation, +0x57 city, +0x80 reputation, +0xb3/+0xb7/+0xbb rivals, +0x1db; nation +0x71, +0x75, +0x110/+0x114/+0x118 rivals; Friendly +0x34e8 year; reads (both)
- table row (0x3b bytes): +0 club, +5 head-to-head, +0x11..+0x1a totals (played, W, D, L, goals for, against, points), +0x1f..+0x28 home, +0x2d..+0x36 away; reads and writes (aplicar_resultado_na_tabela)
- fixture: +0x1c/+0x20 clubs, +0x38 rules (2 extra time, 0x20/1 penalties), +0x43/+0x44 score, +0x47/+0x48 extra time (0xff = none), +0x49/+0x4a penalties; writes +0x3d/+0x3e (aplicar_resultado_na_tabela); League +0xab, +0xc2, +0xc3; reads (both)
- table row (0x3b bytes): +0 club, +4 head-to-head position, +0x11 played and +0x19 points per column; league date: +0 day, +2 year; reads (ordem_pela_posicao_do_confronto, ordem_das_datas_da_liga, linha_do_clube, liga_tem_o_clube)

**Of interest to:** SGE, CMRTE
