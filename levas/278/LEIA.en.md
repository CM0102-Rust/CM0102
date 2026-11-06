# Batch 278 — the knockout of the state and regional leagues: Rio Grande do Sul, Centre-West and Goias; the end-of-season tie-break of the Italian Serie A and Serie B (the "spareggio"); the promotion play-off and the play-out of the Italian Serie C1; the stage advance of the English Third Division; the constructors of the four English divisions; the constructors of the Italian Serie A, Serie B and the two Serie C1 groups; the constructors of the French second and third divisions and the Spanish Primera and Segunda; the Spanish lower division (constructor and table) and the Spanish Segunda table; the French background leagues: the CFA and the lower division (constructors, rules and destructors); the real 2001/02 fixture list of the Italian Serie A and Serie B; the real 2001/02 fixture list of the two Italian Serie C1 groups; the stubs that return 1

**Routines:**
- `cqn9_mata_mata_do_gaucho()` = 0x4325b0 (bra_reg_gaucho.cpp, thiscall, ret 0): Rio Grande do Sul: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);
- `cqn9_mata_mata_do_centro_oeste()` = 0x4312d0 (bra_reg_central.cpp, thiscall, ret 0): Centre-West: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th (the seeds are the positions [inference: two legs]);
- `cqn9_mata_mata_do_goiano()` = 0x433930 (bra_reg_goias.cpp, thiscall, ret 0): Goias: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th (the seeds are the positions [inference: two legs]);
- `desempate_da_serie_a()` = 0x642880 (ita_ser_a.cpp, thiscall, ret 8): the Serie A day; at season end, two clubs level on points in different bands (the title; Italy's Champions League places; the UEFA Cup places; the next two [inference: the Intertoto]; the relegated, the last +0xc1 of 18) are not split by goal difference: the first such pair from the top is ordered by head-to-head and goes to an extra match (the next stage); with no tie on a boundary, the season closes;
- `desempate_da_serie_b()` = 0x647050 (ita_ser_b.cpp, thiscall, ret 8): the Serie B day; the fixed bands are the champion, 3 more promotion places (4 go up) and the 4 relegated of 20; each tied pair on a boundary is ordered by head-to-head and, if any, the tie-break match;
- `ordem_dos_empatados_na_italia()` = 0x6471a0 (ita_ser_b.cpp, thiscall, ret 4): clubs level on points are ordered by the head-to-head among them (the mini-table 0x682b30) and the table positions are renumbered;
- `resultado_do_desempate_da_serie_a()` = 0x641dc0 (ita_ser_a.cpp, thiscall, ret 0x18): in the league, the base records; in the tie-break match, the winner (status 1 [inference]) is placed above the other in the table (rows swap if it was below) and, with no play-off pending, the season closes;
- `fase_seguinte_da_c1a()` = 0x64b9e0 (ita_ser_c1a.cpp, thiscall, ret 0): after the league, the current stage goes up; on the first, the play-off and the play-out are created; past the last, the error box;
- `fases_do_fim_da_c1a()` = 0x64ba70 (ita_ser_c1a.cpp, thiscall, ret 0): the C1/A rule (18 clubs): the champion goes up directly and enters the Serie C1 Super Cup; 2nd v 5th and 3rd v 4th play the promotion play-off (the second place); 14th v 17th and 15th v 16th play the play-out (the losers go down with the 18th [inference]); each is a cup stage with the league's own schedule;
- `fase_seguinte_da_c1b()` = 0x64fba0 (ita_ser_c1b.cpp, thiscall, ret 0): the same as C1/A;
- `fases_do_fim_da_c1b()` = 0x64fc30 (ita_ser_c1b.cpp, thiscall, ret 0): the same rule in C1/B;
- `fase_seguinte_da_third()` = 0x578330 (eng_third.cpp, thiscall, ret 0): after the Third Division league, the current stage goes up and, on the first, the promotion play-off is created (0x5783c0);
- `construir_premier()` = 0x573380 (eng_prm.cpp, thiscall, ret 8): the Premiership: discipline rule 9 (the English leagues one), minimum ground of 10,000 all seated, only the league (no play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted positions;
- `construir_first()` = 0x571680 (eng_first.cpp, thiscall, ret 8): the First Division: discipline 9, no minimum ground, the league and 1 stage (the promotion play-off);
- `construir_second()` = 0x5754a0 (eng_second.cpp, thiscall, ret 8): the Second Division: discipline 9, no minimum ground, the league and the play-off;
- `construir_third()` = 0x576dd0 (eng_third.cpp, thiscall, ret 8): the Third Division: discipline 9, minimum ground of 6,000 with 1,000 seated, the league and the play-off;
- `construir_serie_a()` = 0x6406d0 (ita_ser_a.cpp, thiscall, ret 8): Serie A: discipline rule 0x11 (the Italian leagues one), only the league (the tie-break is born at season end); with no saved state, the rules, the table, the real 2001/02 fixture list by club name, the fixtures, the order, the stats and the predicted positions;
- `construir_serie_b()` = 0x645810 (ita_ser_b.cpp, thiscall, ret 8): Serie B: discipline 0x11, only the league, the real 2001/02 fixture list;
- `construir_c1a()` = 0x64aa70 (ita_ser_c1a.cpp, thiscall, ret 8): C1/A: discipline 0x11, the league and 2 stages (the promotion play-off and the play-out), the real 2001/02 fixture list;
- `construir_c1b()` = 0x64ec20 (ita_ser_c1b.cpp, thiscall, ret 8): C1/B, as C1/A;
- `construir_franca_2()` = 0x5be330 (fra_second.cpp, thiscall, ret 8): the French second division: discipline rule 0xc (the French one), only the league; with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted positions;
- `construir_franca_3()` = 0x5bfb40 (fra_third.cpp, thiscall, ret 8): the French third division (the National): discipline 0xc, only the league;
- `construir_segunda()` = 0x8539a0 (spa_second.cpp, thiscall, ret 8): the Spanish Segunda: discipline rule 0x1a (the Spanish one), only the league;
- `construir_primera()` = 0x84e130 (spa_first.cpp, thiscall, ret 8): the Primera: discipline 0x1a, only the league; finally, if the nation lacks bit 4 of +0x11c [inference: the lower division playable], it creates the Spanish lower division (the B teams and the best of Segunda B) at +0xee;
- `construir_divisao_de_baixo_espanhola()` = 0x851020 (spa_lower.cpp, thiscall, ret 4): the Spanish lower division is a background league with no discipline (0xff), in the competition of id [0x9cf6b0]; with no saved state, the rules, the table and the fixtures (no stats nor predicted positions);
- `tabela_da_segunda()` = 0x855b40 (spa_second.cpp, thiscall, ret 0): the Segunda has 22 clubs (the competition's, in database order); the B team of a Primera club is marked as not promotable (fate −4); the prize fund starts from 50,000;
- `tabela_da_divisao_de_baixo_espanhola()` = 0x8533d0 (spa_lower.cpp, thiscall, ret 0): the lower division has 22 clubs: first the Spanish B teams whose parent club is in the Primera, then those of the Segunda (database order); the rest come from the Segunda B clubs [inference: competitions [0x9cf62c..0x9cf638]] and the Spanish generic division that are not B teams, ordered by reputation with the best shuffled; each club placed keeps the competition at +0x60, but the Segunda B-team pass zeroes +0x60 of every Spanish side not entering it, including the Primera ones already placed [observed in the comparison];
- `regras_da_cfa()` = 0x5ba0a0 (fra_cfa.cpp, thiscall, ret 0): the rules of the two French background leagues: home and away, 3 points a win and 1 a draw, tie-breaks 4, 1, 2 [inference: position, goal difference, goals], no promotion, play-off or relegation places, rule bits 0x43 (a champion, top places as promotion and fixed round order); the league above gets the id at [0x9cf624] (the same global the Spanish Segunda uses for the Primera) [inference: an original slip, harmless since there is no promotion];
- `construir_cfa()` = 0x5b91c0 (fra_cfa.cpp, thiscall, ret 4): the CFA (the French fourth level, background): up to 72 clubs, all of the first and second divisions [inference: the reserve sides] and up to 34 of the generic division whose reserve competition is the CFA, drawn into 4 groups of 18; each group gets the table and the fixtures (in the game calendar) and the table is freed;
- `construir_divisao_de_baixo_francesa()` = 0x5bd600 (fra_lower.cpp, thiscall, ret 4): the French lower division (background): up to 40 clubs, those of the National and up to 20 French clubs of the generic division with no reserve competition, drawn into 2 groups of 20;
- `destruir_cfa()` = 0x5b9460 (fra_cfa.cpp, thiscall, ret 0): frees the table, the schedule, the stages and their vector and destroys the league;
- `destruir_divisao_de_baixo_francesa()` = 0x5bd870 (fra_lower.cpp, thiscall, ret 0): the same for the lower division;
- `calendario_fixo_da_serie_a()` = 0x642a70 (ita_ser_a.cpp, thiscall, ret 0): the 2001/02 Serie A does not draw its fixtures: the 306 matches (34 rounds of 9) are fixed, with date and both clubs by name; if any name is missing from the database, the whole list is dropped and the league falls back to the drawn schedule;
- `calendario_fixo_da_serie_b()` = 0x647230 (ita_ser_b.cpp, thiscall, ret 0): the 2001/02 Serie B: the 380 matches (38 rounds of 10) fixed, with the same rule;
- `calendario_fixo_da_c1a()` = 0x64be90 (ita_ser_c1a.cpp, thiscall, ret 0): the 2001/02 C1/A: the 306 matches (34 rounds of 9) fixed by club name; with a name missing from the database, the list is dropped and the league draws its schedule;
- `calendario_fixo_da_c1b()` = 0x650100 (ita_ser_c1b.cpp, thiscall, ret 0): the 2001/02 C1/B, with the same rule;
- `toco_um_com_dois()` = 0x5bb540 (fra_first.cpp, thiscall, ret 8): the add_result stub of competitions that play no matches: returns 1;
- `toco_um_com_um()` = 0x5bb550 (fra_first.cpp, thiscall, ret 4): the default of competition slot 25 and ruling body slot 6: returns 1;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (28):** 27 caught. `cql4_serie_a_todo_empate` escaped: the mutation only changes anything when two neighbours in the final Serie A table tie on points inside the same band; the 2001/02 Serie A of this career had no such tie (the branch ran: its neighbour `cql4_serie_a_sem_zerar_o_historico` was caught on 2002-142), and comparison mode also gave 0 divergences. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original fd53022e (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 277's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 278`

**Fields touched:**
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads
- league +0xc: the stages (pointer array); pointer; writes the first
- league +0x30: the current stage; value; writes 0
- league +0x40: the season year; value; reads
- cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)
- league +0xdd season over, +0xe6 [inference: table closed], +0x2c stages, +0xc stage vector, +0x4c history mark; value and pointer; reads and writes
- league +0xc1 relegated; value; reads
- table row +0x19 points, +4 position, +0x37 fate, +0 club; value; reads and writes
- competition record +0x5d nation; pointer; reads
- fixture +0x1c and +0x20 the two clubs; pointer; reads
- league +0x30 current stage, +0x2c stages, +0xc stage vector; value and pointer; reads and writes
- league +0xbf play-off places, +0x40 year, +4 competition record; value; reads
- [[0xadadfc] + [0x9cf938] × 4] the Serie C1 Super Cup, +0xa7 entrants (6 bytes; +0 club); pointer; writes the champion
- table row +0 club; pointer; reads
- league +0 vtable, +4 competition record, +0x40 year, +0x50 discipline rule, +0xe2/+0xe4 minimum ground (capacity, seated); value and pointer; writes
- league +0x44 stage (0xff = the league), +0x30 current stage, +0x2c stages, +0xc stage vector, +8 Comp_Stats; value and pointer; writes
- league +0 vtable, +4 competition record, +0x40 year, +0x50 discipline rule; value and pointer; writes
- league +0x44 stage, +0x30 current stage, +0x2c stages, +0xc stage vector, +8 Comp_Stats, +0x4c history mark, +0xee (class byte in A and B) [inference: tie-break in progress]; value and pointer; writes
- league +0 vtable, +4 competition record, +0x40 year, +0x50 discipline rule; value and pointer; writes
- league +0x44 stage, +0x30 current stage, +0x2c stages, +0xc stage vector, +8 Comp_Stats; value and pointer; writes
- Primera +0xee the Spanish lower division; pointer; writes
- nation +0x11c bit 4 [inference: lower division playable]; value; reads
- table row +0 club, +0x37 fate (−4 = B team that cannot go up); pointer and value; writes
- club +0x57 competition (id at +0), +0x53 nation; pointer; reads and writes (the lower division moves its clubs' +0x57 to the generic one)
- club +0x60 the reserve competition [inference: ClubReserveDivision]; pointer; writes
- league +0x28 prize fund, +0xe1 mark; value; writes
- [0xae2350] [inference: clubs created later]; value; reads
- league +0x3c rounds, +0xc2/+0xc3 points, +0xc4 match category, +0xc5..+0xc7 tie-breaks, +0xbe..+0xc1 places, +0x1c/+0x20 leagues above and below, +0xd9 rule bits, +0xba schedule, +0x42 type; value; writes
- club +0x57 competition, +0x53 nation, +0x60 reserve competition; pointer; reads
- [0xae2350] [inference: clubs created later]; value; reads
- league +0xea the fixed fixture list (16 bytes each; count before, at −4); +8/+0xc the club ids, −1 if not found; pointer and value; writes
- league +0x3e clubs, +0xa9 rounds; value; reads
- league +0xea the fixed fixture list (16 bytes each; count before, at −4); +8/+0xc the club ids, −1 if not found; pointer and value; writes

**Of interest to:** CMRTE, SGE
