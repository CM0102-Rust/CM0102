# Batch 275 — the MLS: the fate of each club at the end of the stages, the result in the two conference tables and the All-Star conference; the rules of the four English divisions: rounds, points, promotion, play-offs, relegation and tie-breaks; the table of the four English divisions: the clubs, the prize fund and the TV money; the rules of the Italian divisions: rounds, points, promotion, play-off, play-out, relegation and the tie-break match; the table of the Italian divisions: the clubs, the prize fund and Serie A TV money; the rules of the three French divisions: rounds, points, promotion and relegation by year (the top flight grows to 20 clubs in 2002) and tie-breaks; the table of the three French divisions: the clubs, the prize fund and TV money by previous division; the rules of the Spanish divisions: rounds, points, promotion, relegation and head to head first in the tie-breaks; La Primera: the table, the TV money and the progress in the stats; Serie C2/A: the records by status; the tables of the French CFA and lower division; the progress in the stats of Italian Serie C1

**Routines:**
- `cqk9_destino_na_mls()` = 0x908200 (usa_mls.cpp, thiscall): MLS: in the season and the first two play-off rounds, those who go through get the qualification record (stage 0xa0, round 0x78, weight 0x28) and those left out the failed-to-qualify one; in the final, the winner gets the final record (weight 0x50) and the others the round one, weighted 50 to 70; then the champion gets the champion record;
- `cqk9_resultado_na_mls()` = 0x9099d0 (usa_mls.cpp, thiscall): MLS: each game goes to the overall table and, in the regular season, also to the two conference tables (East and West [inference]), without counting the statistics twice;
- `cqk9_conferencia_das_estrelas()` = 0x90a000 (usa_mls.cpp, cdecl): MLS All-Star game: East (0x413) for New England, MetroStars, Tampa Bay, D.C. United, Columbus and Miami; West (0x414) for Colorado, Dallas, Kansas City, Los Angeles, San Jose and Chicago; clubs outside the MLS, none;
- `regras_da_premier()` = 0x574b70 (eng_prm.cpp, thiscall, ret 0): the Premier (20 clubs): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go straight down to the First; tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; builds the round schedule;
- `regras_da_first()` = 0x572d90 (eng_first.cpp, thiscall, ret 0): the First (24): the top 2 go up directly, 3rd to 6th go to the promotion play-offs, the bottom 3 go down to the Second; 5 substitutes and 3 substitutions; 2 rounds, 3 points;
- `regras_da_second()` = 0x576780 (eng_second.cpp, thiscall, ret 0): the Second (24): 2 go up directly, 3rd to 6th to the play-offs, the bottom 4 go down to the Third;
- `regras_da_third()` = 0x5780c0 (eng_third.cpp, thiscall, ret 0): the Third (24): 3 go up directly, 4th to 7th to the play-offs, the bottom club goes down to the Conference when the nation has its lower divisions loaded [inference from nation bit 4] (otherwise there is no league below);
- `tabela_da_premier()` = 0x574a20 (eng_prm.cpp, thiscall, ret 0): the Premier has 20 clubs; the prize fund starts at 250,000 per place; TV pays 12,000,000 to each club (5,000,000 up to 1999);
- `tabela_da_second()` = 0x576630 (eng_second.cpp, thiscall, ret 0): the Second has 24 clubs; prize fund of 20,000 per place; TV pays 2,500,000 to each club (300,000 up to 1999);
- `tabela_da_third()` = 0x577f70 (eng_third.cpp, thiscall, ret 0): the Third has 24 clubs; prize fund of 10,000 per place; TV pays 1,000,000 to each club (200,000 up to 1999);
- `tabela_da_first()` = 0x572bf0 (eng_first.cpp, thiscall, ret 0): the First has 24 clubs; prize fund of 70,000 per place; TV pays by the division the club came from: 5,877,777 to those relegated from the Premier and 4,099,999 to those already in the First and those promoted from the Second (1,000,000 to all up to 1999);
- `regras_da_serie_a()` = 0x641640 (ita_ser_a.cpp, thiscall, ret 0): Serie A (18 clubs): 2 rounds, 3 points for a win and 1 for a draw, nobody goes up, the bottom 4 go down to B; 7 substitutes and 3 substitutions; tie-breaks 1, 2 and 4 [codes; inference: goal difference, goals for, head to head]; a points tie across a place border goes to a tie-break match (+0xe6); fixed round order;
- `regras_da_serie_b()` = 0x6468a0 (ita_ser_b.cpp, thiscall, ret 0): Serie B (20): the top 4 go up to A, the bottom 4 go down to C1; the rest as in A (tie-break match too);
- `regras_da_c1a()` = 0x64b770 (ita_ser_c1a.cpp, thiscall, ret 0): Serie C1/A (18): the champion goes up to B, 2nd to 5th go to the promotion play-off, 14th to 17th to the relegation play-out and the last goes down; tie-breaks 1 and 2; the league below stored is C1/B (as in the original);
- `regras_da_c1b()` = 0x64f920 (ita_ser_c1b.cpp, thiscall, ret 0): Serie C1/B (18): as C1/A; below it C2/A when the nation has its lower divisions loaded [inference from nation bit 4];
- `tabela_da_serie_a()` = 0x641510 (ita_ser_a.cpp, thiscall, ret 0): Serie A has 18 clubs; the prize fund starts at 250,000 per place (paid by table position when the league ends); TV pays 5,000,000 to each club;
- `tabela_da_serie_b()` = 0x646780 (ita_ser_b.cpp, thiscall, ret 0): Serie B has 20 clubs; prize fund of 70,000 per place; no TV money;
- `tabela_da_c1a()` = 0x64b670 (ita_ser_c1a.cpp, thiscall, ret 0): Serie C1/A has 18 clubs; no prize fund and no TV money;
- `tabela_da_c1b()` = 0x64f820 (ita_ser_c1b.cpp, thiscall, ret 0): Serie C1/B has 18 clubs; no prize fund and no TV money;
- `regras_da_franca_1()` = 0x5bc410 (fra_first.cpp, thiscall, ret 0): the top division: 2 rounds, 3 points for a win and 1 for a draw; nobody goes up; 2 go down in the 2001 season (18 clubs, the league grows to 20) and 3 afterwards; tie-breaks 1, 2 and 3 [codes]; 5 substitutes and 3 substitutions; round order split in halves (+0xd9 = 0x83);
- `regras_da_franca_2()` = 0x5bf0a0 (fra_second.cpp, thiscall, ret 0): the second: 4 go up and 2 go down in 2001, then 3 up and 3 down; the rest as the top division;
- `regras_da_franca_3()` = 0x5c0700 (fra_third.cpp, thiscall, ret 0): the National: 4 go up in 2001 and 3 afterwards, 4 go down (no league below in the game); 3 substitutes and 3 substitutions;
- `tabela_da_franca_1()` = 0x5bc2b0 (fra_first.cpp, thiscall, ret 0): the top division has 18 clubs in the 2001 season and 20 afterwards; prize fund of 200,000 per place; TV pays 6,000,000 to each club;
- `tabela_da_franca_2()` = 0x5bef10 (fra_second.cpp, thiscall, ret 0): the second has 20 clubs; no prize fund; TV pays by the division the club came from: 2,000,000 to those relegated from the top flight and 1,500,000 to those who stayed and those promoted from the National;
- `tabela_da_franca_3()` = 0x5c05d0 (fra_third.cpp, thiscall, ret 0): the National has 20 clubs; prize fund of 20,000 per place; TV pays 200,000 to each club;
- `regras_da_primera()` = 0x84fdf0 (spa_first.cpp, thiscall, ret 0): La Primera (20 clubs, 38 rounds): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go down to the Segunda; tie-breaks 4, 1, 2 and 3 [codes; inference: head to head, goal difference, goals for, wins]; 7 substitutes and 3 substitutions; fixed round order;
- `regras_da_segunda()` = 0x855cc0 (spa_second.cpp, thiscall, ret 0): La Segunda (22): the top 3 go up directly, the bottom 4 go down to Segunda B when it is loaded [inference from nation bit 4]; B teams cannot go up (marked by the table, 0x855b40);
- `regras_da_divisao_de_baixo()` = 0x853790 (spa_lower.cpp, thiscall, ret 0): the lower division (22: B teams and the best minor clubs by reputation): 2 go up directly and 2 go to the promotion play-off, 4 go down; the league above stored is La Primera (as in the original);
- `tabela_da_primera()` = 0x84fc70 (spa_first.cpp, thiscall, ret 0): La Primera has 20 clubs; first, the clubs of the Spanish lower division move to the generic lower division; prize fund of 250,000 per place; TV pays 5,000,000 to each club;
- `registros_pela_situacao_na_c2a()` = 0x654180 (ita_ser_c2a.cpp, thiscall, ret 0x18): Serie C2/A records: in the table, the title, promotion (weight 100), the play-off place (weight 40) and relegation; in the promotion play-off, the winner goes up (weight 50) and the loser gets the knock-out record (weight by round); in the play-out, the winner stays up and the loser goes down;
- `situacao_na_primera()` = 0x850880 (spa_first.cpp, thiscall, ret 0x14): the club's current, best and worst possible position in La Primera stats; places 17 and 18 become 16 for the best and 19 for the worst [inference: they are not a band of their own];
- `tabela_do_grupo_da_cfa()` = 0x5b9fa0 (fra_cfa.cpp, thiscall, ret 4): a CFA group has 18 clubs (4 groups, drawn by the constructor); clubs that are in the first or second division get the row mark [inference: the reserve sides of professional clubs, which cannot go up];
- `tabela_da_divisao_de_baixo_francesa()` = 0x5be230 (fra_lower.cpp, thiscall, ret 4): the French lower division has 20 clubs (chosen by the constructor); those in the National get the mark;
- `situacao_na_c1a()` = 0x64bd80 (ita_ser_c1a.cpp, thiscall, ret 0x14): the club's place forecast in C1/A: in the league, best and worst place become bands (2 to 5 = the promotion play-off: best 2, worst 5; 14 to 17 = the play-out: best 14, worst 17); in the play-off, places + 1; in the play-out, the winner stays at 14 and the loser at 16;
- `situacao_na_c1b()` = 0x64ff40 (ita_ser_c1b.cpp, thiscall, ret 0x14): the same rule in C1/B;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (30):** 30 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (bdb97d45).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original c62bf4a7 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 274's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 275`

**Fields touched:**
- league +4: competition record in the database; pointer; reads (goes into the career records)
- league +8: statistics block (Comp_Stats); pointer; reads, writes 0 and restores (add_result)
- league +0xc: stage vector (the two conferences); pointer; reads
- league +0x30: current stage (1 = the MLS regular season [inference]); value; reads
- match +0x42: conference flag (-1 overall, 0 and 1 the conferences) [inference]; value; reads and writes
- match +0x30 / +0x32 / +0x34: round, stage and the scale round; value (word); reads
- club +0: id; club +0x57: the club league (club_comp); pointer; reads
- [0x9cf590] MLS key; [0x9d037c..0x9d03a8] the twelve club keys; id; reads
- [0xdd7f00]: Staff_Records; pointer; reads
- league +0x3c rounds; +0xc2/+0xc3 points per win/draw; +0xc4 match category (2 = league); +0xc5..+0xc7 tie-breaks; value; writes (all four)
- league +0xbe/+0xbf/+0xc0/+0xc1 direct promotion, promotion play-off, relegation play-off and direct relegation places; value; writes (all four)
- league +0x1c/+0x20 the league above/below; id (−1 none); writes (all four)
- league +0x42 type (1 national clubs), +0x49 substitutes, +0x4a substitutions, +0x52 [unnamed], +0xab/+0xdb words copied into matches; value; writes
- league +0xba round schedule, +0xa9 rounds, +0x3a stage code; pointer and value; writes through virtual +0x3c (all four)
- nation +0x11c bit 4 lower divisions loaded [inference]; value; reads (regras_da_third)
- league +0x3e clubs; value; writes (all four)
- league +0xb1 table (0x3b-byte rows, +0 club); pointer; writes (all four, through 0x687430)
- league +0x28 prize fund; value; writes; +0xe1 = 1 [unnamed]; value; writes (all four)
- club +0x57 competition (id at +0); pointer; reads (all four); +0x5b previous competition; reads through 0x683010 (tabela_da_first)
- [0xae2c92] game year; value; reads (all four)
- league +0x3c rounds; +0xc2/+0xc3 points; +0xc4 match category; +0xc5..+0xc8 tie-breaks; value; writes (all four)
- league +0xbe/+0xbf/+0xc0/+0xc1 direct promotion, promotion play-off, play-out, relegation; value; writes
- league +0x1c/+0x20 league above/below; id; writes
- league +0xd9 rule bits (0x40 = fixed round order); value; reads and writes (all four)
- league +0xe6 a tie across a border goes to a tie-break match; value; writes (regras_da_serie_a, regras_da_serie_b)
- league +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; value; writes; +0xba, +0xa9, +0x3a through virtual +0x3c
- nation +0x11c bit 4 [inference: lower divisions loaded]; value; reads (regras_da_c1b)
- league +0x3e clubs; +0xb1 table (0x3b-byte rows, +0 club); value and pointer; writes (all four)
- league +0x28 prize fund; +0xe1 prize mode (1 = by table, 2 = none, 3 = by the competition, through 0x682e10); value; writes (tabela_da_serie_a, tabela_da_serie_b)
- club +0x57 competition (id at +0); pointer; reads (all four)
- league +0x40 league year; value; reads (all three)
- league +0x3c rounds; +0xc2/+0xc3 points; +0xc4 category; +0xc5..+0xc7 tie-breaks; +0xbe..+0xc1 places; value; writes (all three)
- league +0xd9 rule bits = 0x83; value; writes (all three)
- league +0x1c/+0x20 league above/below; id; writes
- league +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; value; writes; +0xa7 = −1 (regras_da_franca_3); +0xba, +0xa9, +0x3a through virtual +0x3c
- league +0x3e clubs; +0xb1 table (0x3b-byte rows, +0 club); value and pointer; writes (all three)
- league +0x28 prize fund; +0xe1 prize mode (1 = by table, 2 = none); value; writes (all three)
- league +0x40 league year; value; reads; +0xa7 = −1; value; writes (tabela_da_franca_1)
- club +0x57 competition (id at +0); pointer; reads; +0x5b previous competition; reads through 0x683010
- league +0x3c rounds; +0xc2/+0xc3 points; +0xc4 category; +0xc5..+0xc8 tie-breaks; +0xbe..+0xc1 places; value; writes (all three)
- league +0xd9 rule bits = 0x43; value; writes (all three)
- league +0x1c/+0x20 league above/below; id; writes
- league +0xa9 rounds (38 in La Primera, before the virtual); value; writes (regras_da_primera)
- league +0x42, +0x49, +0x4a, +0x52, +0xab, +0xdb; value; writes; +0xba, +0x3a through virtual +0x3c
- nation +0x11c bit 4 [inference: lower divisions loaded]; value; reads (regras_da_segunda)
- club +0x57 competition (id at +0); pointer; reads and writes (tabela_da_primera: the Spanish lower division becomes the generic one, [0xae23d0] + [0x9cf764] × 0x6b)
- league +0x3e, +0xb1, +0x28, +0xe1; value and pointer; writes (tabela_da_primera)
- table row +0x37 status (0 promoted, 3 relegated, 0xfe out); value; writes (registros_pela_situacao_na_c2a)
- match +0x30 round, +0x32 stage, +0x34 round for the weight; value; reads (registros_pela_situacao_na_c2a)
- Comp_Stats club entry +0x73 current place, +0x74 best, +0x75 worst; value; writes (situacao_na_primera)
- table row +0 club, +0x38 mark (1 = club of a higher division); pointer and value; writes through 0x687430 (both tables)
- club +0x57 competition (id at +0); pointer; reads (both tables)
- Comp_Stats club entry +0x73 current place, +0x74 best, +0x75 worst; value; writes (situacao_na_c1a, situacao_na_c1b)

**Of interest to:** CMRTE
