# Batch 284 — the Asian Club Championship: the next stage, the two groups and the semi-finals; the African Nations Cup: the draw day, the draw of the 16 and the group B to D stages; the African Nations Cup and Asian Cup Winners' Cup knockouts: the quarter-finals, the next stage and the finals; the Italian Cup and the Serie C Cup: the group 1 table and the next stage; the Italian Cup and the Serie C Cup: each group's stage; the Italian Cup and the Serie C Cup: the knockout after the groups; the European Championship qualifiers: the rules and the draw date, the group sizes, a team's status and group 1; the UEFA Cup: the entrants; the King's Cup day and the season history of the Champions League, the Greek Cup, the Irish League Cup and the Serie C Cup

**Routines:**
- `cqt12_asia_fase_seguinte()` = 0x40c070 (asia_club_champ.cpp, thiscall, ret 0): rule: the Asian Club Championship moves on one stage when the current one ends; after the two zones' knockouts (on reaching 1) it builds the two groups; after the groups (on reaching 3) the semi-finals; going past the last stage is an error;
- `cqt12_asia_grupos()` = 0x40c110 (asia_club_champ.cpp, thiscall, ret 0): rule: the 4 survivors of the first zone's knockout make group 1 and the 4 of the second zone group 2 (league stages 1 and 2, single round [inference], the cup's rules and year); human managers get the news;
- `cqt12_asia_semifinais()` = 0x40c3d0 (asia_club_champ.cpp, thiscall, ret 0): rule: the Asian Club Championship semi-finals cross the groups: group 1 winner × group 2 runner-up and group 2 winner × group 1 runner-up, in a cup stage (stage 3) with the first round drawn at once;
- `cqs13_africana_dia()` = 0x4025a0 (african_nations.cpp, thiscall, ret 8): rule: the African Nations Cup is drawn on the start day (+0xb6/+0xb8): the draw of the 16, the predicted positions, group A in the cup's own table, the fixtures and groups B to D as stages; before that day the cup does not even run the league day;
- `cqs13_africana_sorteio()` = 0x4022c0 (african_nations.cpp, thiscall, ret 0): rule: the African Nations Cup has 16 nations: the hosts first and the African nations (up to 50, in nation order, without the hosts) sorted by reputation; the pots are the list's blocks of 4 (the hosts and the best in pot 1); each of the 4 groups gets one nation drawn from each pot; with fewer than 16 African nations there is no draw;
- `cqs13_africana_grupo()` = 0x401ea0 (african_nations.cpp, thiscall, ret 4): rule: groups B to D of the African Nations Cup are league stages with the 4 nations drawn into them, the cup's rules and year and the cup's round table for the group; human managers get the news;
- `cqt13_africana_quartas()` = 0x402090 (african_nations.cpp, thiscall, ret 0): rule: the African Nations Cup quarter-finals cross the groups: A winner × B runner-up, C winner × D runner-up, C runner-up × D winner and B winner × A runner-up, in a cup stage (stage 3) with the first round set up at once;
- `cqt13_recopa_asiatica_fase_seguinte()` = 0x40dfa0 (asia_cup_winner.cpp, thiscall, ret 0): rule: the Asian Cup Winners' Cup moves on one stage when the current one ends; after the two zones' knockouts (on reaching 1) it builds the finals; going past the last stage is an error;
- `cqt13_recopa_asiatica_finais()` = 0x40e030 (asia_cup_winner.cpp, thiscall, ret 0): rule: the Asian Cup Winners' Cup finals have the 4 survivors of the two zones' knockouts, with the middle pair crossed (the two of each zone do not meet in the semi-final), in a cup stage (stage 1) with the first round drawn at once;
- `cqr7_copa_da_italia_grupo_um()` = 0x63f250 (ita_cup.cpp, thiscall, ret 0): rule: the Italian Cup starts with groups of 4; group 1 is the cup object itself (the other 7 are separate stages): the first 4 entrants go to the table rows, leave the list, and each human gets the group news;
- `cqr7_copa_da_serie_c_grupo_um()` = 0x63d5e0 (ita_c_cup.cpp, thiscall, ret 0): rule: the Serie C Italian Cup starts with groups of 5 (16 groups); group 1 is the cup object itself: the first 5 entrants go to the table rows and leave the list;
- `cqr7_copa_da_italia_fase_seguinte()` = 0x63f5b0 (ita_cup.cpp, thiscall, ret 0): rule: the Italian Cup stage moves on by one; after stage 6 (the 8 groups are over) it builds the knockout (the 8 group winners and the 16 seeds); at the last stage, the error box;
- `cqr7_copa_da_serie_c_fase_seguinte()` = 0x63d950 (ita_c_cup.cpp, thiscall, ret 0): rule: the Serie C Cup stage moves on by one; after stage 14 (the 16 groups are over) it builds the 32-team knockout;
- `cqr8_copa_da_italia_grupo()` = 0x63f640 (ita_cup.cpp, thiscall, ret 4): rule: groups 2 to 8 of the Italian Cup are separate leagues (League_Stage) of 4 teams: the next 4 entrants on the list, which leave it, with the rounds of the cup table for that stage, single round-robin (1) and 2 points [inference: the constructor's last arguments]; each human gets the group news;
- `cqr8_copa_da_serie_c_grupo()` = 0x63d9e0 (ita_c_cup.cpp, thiscall, ret 4): rule: groups 2 to 16 of the Serie C Cup are separate leagues of 5 teams, taken from the entrants list in order, as in the Italian Cup;
- `cqr9_copa_da_italia_mata_mata()` = 0x63f870 (ita_cup.cpp, thiscall, ret 0): rule: after the 8 groups, the Italian Cup knockout has 24 teams: the 8 group winners and the 16 seeds entering directly (the first entrants still on the list: the Serie A clubs [inference]); in the draw the winners go with seed 3 and 2, the seeds with 2 and 1 (8 slots per seed) [inference: each slot's pot]; the first round is drawn at once;
- `cqr9_copa_da_serie_c_mata_mata()` = 0x63dc10 (ita_c_cup.cpp, thiscall, ret 0): rule: after the 16 groups, the Serie C Cup knockout has 32 teams: the 16 winners, the 6 best runners-up (the 16 runners-up in a table sorted by criteria 1, 2, 3: points and tie-breakers; their group marked with 2 and the row with 6) and 10 seeds entering directly; no seeding; the first round is drawn at once;
- `cqs8_eurocopa_regras()` = 0x57e8f0 (euro_champ_qual.cpp, thiscall, ret 0): rule: the European Championship qualifiers are home-and-away groups (2 rounds), 3 points per win and 1 per draw, tie-breakers 1, 2, 4, 5, each group winner goes straight through and the runner-up to the play-off, nobody is relegated; the draw is on 10 March of the cup year, moved to the Friday [inference];
- `cqs8_eurocopa_tamanho_do_grupo()` = 0x57d9d0 (euro_champ_qual.cpp, thiscall, ret 4): rule: in format 1 groups 1, 3 and 6 have 6 teams and the others 5; in format 2 groups 1, 3, 6 and 8 have 6 and the others 5;
- `cqs8_eurocopa_situacao()` = 0x580b50 (euro_champ_qual.cpp, thiscall, ret 4): rule: a team's status in the qualifiers: −2 when its stat 0x12 in the cup is below 14 (the position [inference]), else −3;
- `cqs8_eurocopa_grupo_um()` = 0x57e7e0 (euro_champ_qual.cpp, thiscall, ret 0): rule: qualifier group 1 is the cup object itself, with 5 teams (the first of the drawn list), (5 − 1) × 2 home-and-away rounds, and the group news;
- `cqs9_uefa_inscritos()` = 0x902b90 (uefa_cup.cpp, thiscall, ret 0): rule: the UEFA Cup takes the clubs the UEFA ranking put in it, each with its coefficient class (those of 10 or more count as preliminary-round clubs); if, minus half of the preliminary ones, they are fewer than 118, the first clubs without a European cup from nations without the +0x11c mark and coefficient below 2000 complete it, with class 3 (and they become UEFA Cup clubs); beyond 118, the last ones leave; the list is sorted by class; with an even number of preliminary clubs, the class 1 ones go to a separate list (with class 3) and the cup gets entrants + 27 slots (the 27 coming later from the Champions League and the Intertoto [inference]); with an odd number, the cup keeps 145 slots and returns 0;
- `cqt5_copa_do_rei_dia()` = 0x84d740 (spa_cup.cpp, thiscall, ret 8): rule: the King's Cup is rebuilt once per edition, on the first day (with the day phase) on which the UEFA ranking year reaches the cup year: old tables freed, the entrants (Primera, Segunda and Segunda B clubs and lower clubs, by the nation rule), the rounds, the tree and the predicted positions; every day the cup plays its rounds;
- `cqt5_historico_liga_dos_campeoes()` = 0x584ad0 (european_cup.cpp, thiscall, ret 0): rule: the Champions League season history is that of its last stage (stage 12: the final);
- `cqt5_historico_copa_da_grecia()` = 0x5e93e0 (gre_cup.cpp, thiscall, ret 0): rule: the Greek Cup season history is that of stage 10 (the final);
- `cqt5_historico_copa_da_liga_irlandesa()` = 0x637750 (ire_lge_cup.cpp, thiscall, ret 0): rule: the Irish League Cup season history is that of stage 7 (the final);
- `cqt5_historico_copa_da_serie_c()` = 0x63d770 (ita_c_cup.cpp, thiscall, ret 0): rule: the Serie C Cup season history is that of the knockout (stage 15);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (27):** 25 caught. `cqs8_grupo_um_com_rodada_a_mais` escaped: every reader of `games_per_club` (+0xa7) in the exe combines it with the row's games played (+0x11), which stay zero until the group 1 matches in September, and the form buffer (malloc of 8 or 10 bytes) lands in the same 16-byte allocator block; the comparer runs only the original against the fixed formula (proved from the code). `cqs9_preliminar_de_11` escaped: the `EURO_SEEDING` of a UEFA Cup club is the stage `assign_uefa_cup_places` gives it, 1, 2 or `count_extra` x 10 + 3 with `count_extra` >= 2 (23, 33...), never 10, so `>= 0xa` and `>= 0xb` count the same clubs (proved from the code). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original f3bf21f5 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 283's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 284`

**Fields touched:**
- cup +0xa7 zone 1 knockout table (6 bytes: club, +4 class, +5 status: 1 = alive), +0x3e teams; pointer/value; reads (groups)
- cup +0xc stages ([0] zone 2 knockout, [1] and [2] the groups, [3] the semi-finals); pointer; reads and writes
- cup +0x2c stages, +0x30 current stage; value; reads and writes (next stage, groups)
- cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)
- cup +0xee list (50 nations, then the 16 of the groups), +0xf2 its count (word, 16); pointer/value; writes (draw), reads (stages), clears (day)
- nation +0x71 continent (its +0 is the id, [0x9cfa08] Africa); national team +0x53 nation; pointer/id; reads (draw)
- cup +0xc stages, +0x30 current stage; pointer/value; writes (stage)
- cup +0xb1 group A table, +0xc stages ([0..2] groups B to D, [3] the quarter-finals); pointer; reads and writes (quarter-finals)
- cup +0xa7 zone 1 knockout table (6 bytes: +5 status, 1 = alive), +0x3e teams, +0xc stages ([0] zone 2, [1] the finals); pointer/value; reads and writes (finals)
- cup +0x2c stages, +0x30 current stage; value; reads and writes
- cup +0x3e teams; value; writes 4 or 5 (group 1)
- cup +0xb1 table (0x3b-byte rows); pointer; writes (group 1)
- cup +0xee entrants waiting for a stage (club vector); pointer; reads and clears (group 1)
- cup +0x2c stages, +0x30 current stage; value; reads and writes +1 (next stage)
- cup +0x3e teams per group; value; reads
- cup +0xee entrants waiting for a stage (club vector); pointer; reads and clears
- cup +0xc stages; pointer; writes stage g
- cup +0x30 current stage; value; writes g
- cup +4, +0x40, +0x51, +0xbe, +0xc2..+0xc5, +0xd9; rule values passed to the stage; reads
- cup +0xb1 table (0x3b-byte rows: +0 club, +5 and +9 cleared in the copy, +0x37 best runner-up mark 6); pointer; reads and writes
- cup +0xc stages (the group ones and the knockout at 7 or 15); pointer; reads and writes
- cup +0xee entrants (up to +0xf2); pointer; reads and clears
- cup +0xbe group mark (2: has a best runner-up); value; writes
- cup +0xc2..+0xc8 points and tie-breakers, +0x3c rounds, +0x43, +0x49, +0x4a, +0xbe..+0xc1 places; value; writes (rules)
- cup +0xb6 draw day, +0xb8; value; writes (rules)
- cup +0xba round table, +0xa9 rounds, +0x3a id; pointer/value; writes (rules)
- cup +0xf6 format (signed byte); value; reads (group size)
- cup +0xf7 drawn teams (vector); pointer; reads (group 1). +0x3e, +0xa7, +0xb1; writes (group 1)
- club +0x1db European cup (id), +0x1df coefficient (signed byte); id/value; reads, and writes +0x1db for fillers
- nation +0x11c mark, +0x8e coefficient (signed word); value; reads
- cup +0x3e slots, +0xb2 preliminary clubs, +0x36/+0x38 counts of list +0x14; value; writes
- cup +0xa7 entrants (6 bytes: club, +4 class, +5 status), +0x14 class-1 list; pointer; writes
- [0x9cf6f0] the UEFA Cup; id; reads
- cup +0x45 already rebuilt this edition; value; reads and writes 1 (day)
- [0xde1f58] +0x439 the UEFA ranking year (word); value; reads
- cup +0xa7, +0xa3, +0xad the edition tables; pointer; frees (day)
- cup +0xc stages; pointer; reads the last one (histories)

**Of interest to:** CMRTE
