# Batch 282 — the halves of the Rio, Minas Gerais and Parana state leagues and of the J1: each half's round-robin stage; the season turnover of the Rio, Minas Gerais and Parana state leagues; the Australian NSL: the next stage, the top-six finals series and the grand final; the MLS: the next stage, the eight-club play-off and the MLS Cup final; the cups' season turn: the Brazilian Cup, the Emperor's Cup and the Japanese League Cup; the season turn of the Japanese Super Cup, the US Open Cup and the MLS All-Star Game, and the two all-star teams; the Copa Libertadores: the draw day, the first group table and the other groups' stages; the Copa Mercosur: the draw day, the first group table and the other groups' stages; the Copa Merconorte: the draw day, the first group table and the other groups' stages; the Copa América: the draw day, the first group table and the other two groups' stages; the three MLS divisions: the first division table and the second and third division stages; the MLS: the season turnover, the predicted positions and the list of the twelve clubs; Brazilian Series C: the seven first-round groups, the sixteen-club second round and the two third-round groups

**Routines:**
- `cqr1_turno_do_carioca()` = 0x43afa0 (bra_reg_rio.cpp, thiscall, ret 4): One half of the Rio state league: when the first game of a half is processed, it builds that half's round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final;
- `cqr1_turno_do_mineiro()` = 0x4351b0 (bra_reg_minas_gerais.cpp, thiscall, ret 4): One half of the Minas Gerais state league: when the first game of a half is processed, it builds that half's round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final;
- `cqr1_turno_do_paranaense()` = 0x438760 (bra_reg_parana.cpp, thiscall, ret 4): One half of the Parana state league: when the first game of a half is processed, it builds that half's round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 12, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final;
- `cqr1_turno_da_j1()` = 0x663c20 (jap_j1.cpp, thiscall, ret 4): One half of the J1: when the first game of a half is processed, it builds that half's round-robin stage (a league with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and one top place, the half winner's); the first half goes to stage 0 and the second (from round 16, by the match +0x34) to stage 1; each half winner (the top of the stage table) goes to the final;
- `cqr2_virada_do_carioca()` = 0x43a6a0 (bra_reg_rio.cpp, thiscall, ret 0): The Rio state league turnover: archives the season in the statistics, clears the table, the schedule and the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the referee renewal. The halves are only built at each one's first game (cqr_1);
- `cqr2_virada_do_mineiro()` = 0x434910 (bra_reg_minas_gerais.cpp, thiscall, ret 0): The Minas Gerais state league turnover: archives the season in the statistics, clears the table, the schedule and the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the referee renewal. The halves are only built at each one's first game (cqr_1);
- `cqr2_virada_do_paranaense()` = 0x438060 (bra_reg_parana.cpp, thiscall, ret 0): The Parana state league turnover: archives the season in the statistics, clears the table, the schedule and the stages (the two halves and the final) and prepares three new stage slots; moves to the next year and rebuilds the rules, the table with the twelve state clubs, the fixtures and the order; then the predicted positions and the referee renewal. The halves are only built at each one's first game (cqr_1);
- `cqr3_fase_seguinte_da_nsl()` = 0x411f70 (aus_nsl.cpp, thiscall, ret 0): The NSL has the regular season (round robin), the finals series (stage 0) and the grand final (stage 1): the next stage moves the current stage on and builds the finals series at the end of the regular season and the grand final at the end of the finals series; past the last stage it is an error (the box);
- `cqr3_serie_final_da_nsl()` = 0x412010 (aus_nsl.cpp, thiscall, ret 0): The NSL finals series: the top six of the regular season play their own round-robin stage (with the league points and tie-breakers) and its top two go to the grand final; sends the draw news. [inference] The real NSL used a double-chance knockout; here it is a short league of the six;
- `cqr3_grande_final_da_nsl()` = 0x4121c0 (aus_nsl.cpp, thiscall, ret 0): The NSL grand final: the top two of the finals series in a two-club cup stage [inference: a single game, the first as home side]; the winner is the Australian champion;
- `cqr4_fase_seguinte_da_mls()` = 0x909400 (usa_mls.cpp, thiscall, ret 0): The MLS has three divisions of four (the league and stages 0 and 1, played together in the regular season), the play-off (stage 2) and the final (stage 3): the next stage goes from stage 1 to the play-off and from the play-off to the final, building each; past the last stage it is an error (the box);
- `cqr4_play_off_da_mls()` = 0x9094c0 (usa_mls.cpp, thiscall, ret 0): The MLS play-off: eight clubs. The three division winners, ordered by points, and the five best of the rest (second to fourth of each division, by points; ties with no own rule). The pairs: the best winner against the fifth best of the rest, the second winner against the fourth, the third winner against the third and the best of the rest against the second [inference: pairs follow the list]; the winners go to the final;
- `cqr4_final_da_mls()` = 0x9097d0 (usa_mls.cpp, thiscall, ret 0): The MLS Cup final: the two play-off winners (the first two with the winner mark, in play-off list order) in a two-club cup stage [inference: a single game]; the winner is the MLS champion;
- `cqr5_virada_copa_do_brasil()` = 0x427d10 (bra_cup.cpp, thiscall, ret 0): rule: at the season turn the Brazilian Cup is rebuilt from scratch: last edition's tables are freed, the season history is archived (Comp_Stats), the year goes up by one and the cup builds again its entrants (the team table), the rounds and the fixture tree, and the predicted positions;
- `cqr5_virada_copa_do_imperador()` = 0x662950 (jap_emp_cup.cpp, thiscall, ret 0): rule: the same turn in the Emperor's Cup (Japan): tables freed, history archived, year +1, the entrants (JapEmpCup::collect_entrants), the rounds, the tree and the predicted positions;
- `cqr5_virada_copa_da_liga_japonesa()` = 0x665bc0 (jap_j_cup.cpp, thiscall, ret 0): rule: the same turn in the Japanese League Cup (J-League Cup): tables freed, history archived, year +1, the 28 entrants (JapJCup::collect_entrants), the rounds, the tree and the predicted positions;
- `cqr6_virada_supercopa_japonesa()` = 0x6664e0 (jap_super.cpp, thiscall, ret 0): rule: at the season turn the Japanese Super Cup is rebuilt: last edition's tables freed, history archived, year +1, the two clubs (JapSuper::select_teams: the J1 champion and the Emperor's Cup winner [inference]), the rounds, the tree and the predicted positions;
- `cqr6_virada_copa_dos_eua()` = 0x90b790 (usa_open_cup.cpp, thiscall, ret 0): rule: the same turn in the US Open Cup: tables freed, history archived, year +1, the entrants (UsaOpenCup::collect_entrants), the rounds, the tree and the predicted positions;
- `cqr6_virada_jogo_das_estrelas()` = 0x90a760 (usa_mls_all_stars.cpp, thiscall, ret 0): rule: the MLS All-Star Game turn: tables freed, history archived, year +1, the two all-star teams, the rounds and the tree (no predicted positions);
- `cqr6_times_do_jogo_das_estrelas()` = 0x90a540 (usa_mls_all_stars.cpp, thiscall, ret 0): rule: the MLS All-Star Game pits the two conference all-star teams ([0x9d00a4] and [0x9d00a8], clubs in the table); in odd years the [0x9d00a4] team is at home, in even years the [0x9d00a8] one; the stadium is that of an MLS club drawn (1 in 6, in table order) among the home conference's clubs (0x413 in odd years, 0x414 in even ones); both teams get the USA nation [inference];
- `cqr10_libertadores_dia()` = 0x4c4410 (conmebol_liber.cpp, thiscall, ret 8): rule: the Libertadores has 32 clubs (those the South American rules put in it, shuffled) in 8 groups of 4, home and away (3 points a win, 1 a draw; two qualify from each group, to the round of 16); the draw is on the start day (15 January of the cup year): group 1 stays in the cup's own table (drawn clubs 0 to 3, in order) and groups 2 to 8 are stages (drawn clubs 4g + 4 to 4g + 7); on the draw day: the clubs (0x4c3f40 (the 32 clubs with the Libertadores as their season competition, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted positions; then the league day;
- `cqr10_libertadores_primeiro_grupo()` = 0x4c37a0 (conmebol_liber.cpp, thiscall, ret 0): rule: group 1 of the Copa Libertadores has 4 clubs, the first 4 drawn, each in the row of its order; human managers get the group draw news;
- `cqr10_libertadores_grupo()` = 0x4c3b10 (conmebol_liber.cpp, thiscall, ret 4): rule: each of the other groups of the Copa Libertadores is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get the news;
- `cqr11_mercosul_dia()` = 0x4c5c90 (conmebol_merc.cpp, thiscall, ret 8): rule: the Mercosur has 20 invited clubs (the clubs the South American rules invite, shuffled) in 5 groups of 4 [inference: six rounds on Tuesdays, from 29 July to 14 October]; the season turns on 1 June; on the draw day group 1 stays in the cup's own table (drawn clubs 0 to 3) and groups 2 to 5 are stages (4g + 4 to 4g + 7); the 5 winners and the 3 best runners-up go to the quarter-finals; on the draw day: the clubs (0x4c5850 (the 20 invited clubs, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted positions; then the league day;
- `cqr11_mercosul_primeiro_grupo()` = 0x4c4e60 (conmebol_merc.cpp, thiscall, ret 0): rule: group 1 of the Copa Mercosur has 4 clubs, the first 4 drawn, each in the row of its order; human managers get the group draw news;
- `cqr11_mercosul_grupo()` = 0x4c51d0 (conmebol_merc.cpp, thiscall, ret 4): rule: each of the other groups of the Copa Mercosur is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get the news;
- `cqr12_merconorte_dia()` = 0x4c2c90 (con_merc_cup.cpp, thiscall, ret 8): rule: the Merconorte has 16 invited clubs (the best of four nations and two clubs of the American league [inference], shuffled) in 4 groups of 4, home and away (3/1 points, one qualifies from each group, to the semi-finals); it starts on 1 August; on the draw day group 1 stays in the cup's own table (drawn clubs 0 to 3) and groups 2 to 4 are stages (4g + 4 to 4g + 7, always 4 teams); on the draw day: the clubs (0x4c2560 (the 16 invited clubs, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages and the predicted positions; then the league day;
- `cqr12_merconorte_primeiro_grupo()` = 0x4c1de0 (con_merc_cup.cpp, thiscall, ret 0): rule: group 1 of the Copa Merconorte has 4 clubs, the first 4 drawn, each in the row of its order; human managers get the group draw news;
- `cqr12_merconorte_grupo()` = 0x4c2140 (con_merc_cup.cpp, thiscall, ret 4): rule: each of the other groups of the Copa Merconorte is a league stage with the next 4 drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get the news;
- `cqr13_copa_america_dia()` = 0x5e2120 (goldcup.cpp, thiscall, ret 8): rule: the Copa América has 12 nations in 3 groups of 4 (single round, 3/1 points, two qualify from each group plus the two best thirds to the quarter-finals); the draw is on the start day (20 March of the cup year, moved to weekday 5) and only on the call without `completo`: the list of 12, the predicted positions, group A in the cup's own table, the fixtures and groups B and C as stages; before that day the cup does not even run the league day;
- `cqr13_copa_america_primeiro_grupo()` = 0x5e11e0 (goldcup.cpp, thiscall, ret 0): rule: group A of the Copa América has 4 nations, drawn entries 0, 3, 6 and 9 (the groups are interleaved in the list), each in the row of its order; human managers get the group draw news;
- `cqr13_copa_america_grupo()` = 0x5e16b0 (goldcup.cpp, thiscall, ret 4): rule: groups B and C of the Copa América are league stages with drawn entries g + 1, g + 4, g + 7 and g + 10, the cup's rules and year and the cup's round table for the group (6, 10 and 13 July); human managers get the news;
- `cqs1_tabela_da_mls()` = 0x908560 (usa_mls.cpp, thiscall, ret 0): The MLS table is the first division one: the four clubs of the competition with division code 0x413, in database order; succeeds if it found all four;
- `cqs1_segunda_divisao_da_mls()` = 0x908fc0 (usa_mls.cpp, thiscall, ret 0): The second MLS division (code 0x47b): a round-robin stage with its four clubs (in database order), the same rounds, points, tie-breakers and places as the league, as stage 0; the three division winners and the five best of the rest go to the play-off;
- `cqs1_terceira_divisao_da_mls()` = 0x9091e0 (usa_mls.cpp, thiscall, ret 0): The third MLS division (code 0x414): the same template as the second, as stage 1; the current stage becomes 1, from which the next stage goes to the play-off;
- `cqs2_virada_da_mls()` = 0x908e00 (usa_mls.cpp, thiscall, ret 0): The MLS turnover: clears the table, the schedule and the stages (the other two divisions, the play-off and the final), archives the season in the statistics, moves to the next year and rebuilds the rules, the first division, the season fixtures (with the other two division stages) and the order; then the predicted positions, the board expectations and the referee renewal;
- `cqs2_posicoes_previstas_da_mls()` = 0x909bb0 (usa_mls.cpp, thiscall, ret 0): The MLS predicted positions: in each division the four clubs are ordered by reputation and get positions 1, 4, 7 and 10 (each division favourite is predicted 1st, the second 4th, the third 7th and the last 10th), one scale shared by the three divisions;
- `cqs2_clubes_da_mls()` = 0x909a40 (usa_mls.cpp, thiscall, ret 8): The list of the twelve MLS clubs: the four of the first division, then the four of the second and the four of the third, in table order [inference: used by the MLS All-Star game];
- `cqs3_grupo_da_serie_c()` = 0x42ee80 (bra_nat_third.cpp, thiscall, ret 4): A Series C first-round group: seven clubs of the 56-club list (the first seven slots are the league's own group; group g takes slots 7(g+1) to 7(g+1)+6), home and away round robin, with the league points, tie-breakers and places; the top two of each of the eight groups go to the second round;
- `cqs3_segunda_fase_da_serie_c()` = 0x42e430 (bra_nat_third.cpp, thiscall, ret 0): The Series C second round: the sixteen (the top two of each group) seeded by first-round points (ties go to the earlier one: the league, then groups 0 to 6, first before second), in a knockout where the best meets the 16th, the 2nd the 15th and so on [inference: pairs follow the list];
- `cqs3_terceira_fase_da_serie_c()` = 0x42e700 (bra_nat_third.cpp, thiscall, ret 0): The Series C third round: the eight second-round winners, drawn (shuffled) into two groups of four, home and away, with 3 points for a win, 1 for a draw, tie-breakers {1, 2, 4} and two top places in each group [inference: the top two of each group go up to Series B and the group winners play the final];

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (39):** 39 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original a6f0e662 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 281's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 282`

**Fields touched:**
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads
- league +0x3e: the club count; value; reads
- league +0xc2 / +0xc3: points for a win / a draw; value; reads
- league +0xc4: the match category; value; reads
- league +0xc5: the tie-breakers; pointer (the address); reads
- league +0x40: the season year; value; reads
- league +4: the competition record in the database; pointer; reads
- league +0xc: the stages (pointer array); pointer; writes slot k
- league +0x30: the current stage; value; writes k
- league +0x4c: [inference] season flag; value; writes 0
- league +0xba: the schedule (0x41-byte rounds); pointer; frees and clears
- league +8: the statistics block (Comp_Stats); pointer; reads
- league +0xc: the stages (pointer array); pointer; destroys the stages, frees and writes a new 3-slot array
- league +0x2c: the stage count; value; writes 3
- league +0x30: the current stage; value; reads and writes −1
- league +0x40: the season year; value; reads and writes + 1
- league +4: the competition record in the database; pointer; reads (referees)
- [0xb74340] the officials manager; pointer; reads
- league +0x30 / +0x2c: the current stage / the stage count; value; reads and writes (next stage, finals series)
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads the top six
- league +0xc: the stages (pointer array); pointer; writes slots 0 and 1, reads stage 0's table
- league +0xc2 / +0xc3 / +0xc4 / +0xc5: points for a win and a draw, match category, tie-breakers; value; reads
- league +0x40 / +0x51 / +0xd9 / +4: the year, the importance, the rule bits, the competition record; value; reads
- cup stage +0xa7 / +0x3e: the teams (6 bytes) / how many; reads (comparator)
- league +0x30 / +0x2c: the current stage / the stage count; value; reads and writes (next stage)
- league +0xb1: the table (0x3b-byte rows: +0 the club, +0x19 the points); pointer; reads rows 0 to 3 of the league and of stages 0 and 1
- league +0xc: the stages (pointer array); pointer; reads slots 0, 1 and 2, writes 2 and 3
- cup stage +0xa7: the teams (6-byte records: +0 the club, +5 the winner mark); pointer; reads (final)
- cup stage +0x3e: the team count; value; reads
- league +0x40 / +4: the year / the competition record; value; reads
- cup +0x40 year (word); value; reads and writes +1 (season turn)
- cup +0x4c, +0xab, +0xb1 (byte); value; writes 0 (season turn)
- cup +0xa7 entrants, +0xa3 rounds, +0xad fixtures per round (+0x3c rounds); pointer; frees and writes 0 (season turn)
- cup +8 competition statistics (Comp_Stats); pointer; reads (season turn)
- cup +0x40 year (word); value; reads and writes +1 (season turns), reads (all-star teams: home side by odd/even year)
- cup +0x4c, +0xab, +0xb1 (byte); value; writes 0 (season turns)
- cup +0xa7 entrants (6 bytes: club, +4, +5), +0xa3 rounds, +0xad fixtures per round; pointer; frees and writes (season turns, all-star teams)
- [0x9d00a4], [0x9d00a8] the two all-star clubs; index into the club table; reads
- club +0x57 division (competition); pointer; reads. club +0x69 stadium; pointer; reads and writes (the all-star home team); club +0x6d byte; writes 0; club +0x53 nation; pointer; writes
- [0x9cf590] the MLS; id; reads. [0x9cf4f8] the USA [inference]; index into the nation table [0xae23a8] (0x122 bytes); reads
- cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)
- cup +0xee drawn club list (club pointers); pointer; reads (table and stages) and frees and clears (day)
- cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)
- cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)
- cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)
- cup +0xee drawn club list (club pointers); pointer; reads (table and stages) and frees and clears (day)
- cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)
- cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)
- cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)
- cup +0xee drawn club list (club pointers); pointer; reads (table and stages) and frees and clears (day)
- cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)
- cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)
- cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)
- cup +0xee drawn list (national club pointers), +0xf2 its count; pointer/value; reads (table and stages), frees and clears (day)
- cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (table)
- cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)
- club +0x57: the club competition (the record); pointer; reads
- [0xae23b4] / [0xae235c]: the database clubs (0x245 bytes) / how many; reads
- league +0x3e: the club count; value; writes 4 (table), reads
- league +0xb1: the table; pointer; writes (table)
- league +0xc: the stages (pointer array); pointer; writes slots 0 and 1
- league +0x30: the current stage; value; writes 0 and 1
- league +0x3c, +0xa7, +0xab, +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: rounds, games per club, places, points, category, tie-breakers, rule bits, year, importance, record; value; reads
- stage +0x3a: the main stage code; value; writes the division code
- league +0x4c: [inference] season flag; value; writes 0
- league +0xba: the schedule; pointer; frees and clears
- league +0xc: the stages (pointer array); pointer; destroys the stages (turnover), reads slots 0 and 1
- league +0x30: the current stage; value; reads and writes −1
- league +0x40: the season year; value; reads and writes + 1
- league +8: the statistics block (Comp_Stats); pointer; reads
- statistics, club record +0x72: the predicted position; value; writes (predicted positions)
- league +0xb1 / +0x3e: the table / how many clubs; reads
- club reputation: read by 0x4b07d0 (qsort); reads
- [0xb74340] the officials manager; pointer; reads
- league +0xee: the 56-club Series C list (only in the constructor and the turnover); pointer; reads
- league +0xb1: the table (0x3b-byte rows: +0 the club, +0x19 the points); pointer; reads the top two rows of the league and the groups
- league +0xc: the stages (pointer array); pointer; writes slots 0 to 9
- league +0x30: the current stage; value; writes g, 7 and 9
- cup stage +0xa7: the teams (6-byte records: +0 the club, +5 the winner mark); pointer; reads (third round)
- league +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: places, points, category, tie-breakers, rule bits, year, importance, record; value; reads

**Of interest to:** CMRTE
