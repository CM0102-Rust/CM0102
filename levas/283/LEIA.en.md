# Batch 283 — Brazilian Series C: the final, the season turnover and the predicted positions; the English Football League Trophy: the 56 clubs of the North and South sections, the section stage and the next stage; the English Football League Trophy: the final between the section winners and the final ranking; the cups' knockout stage (Cup_Stage): the constructor and the destructor; the Libertadores and Merconorte knockouts: the group qualifiers and the next stage; the Merconorte invited clubs and the Mercosur and Copa América next stage; the Copa América: the 12 nations, the predicted positions and the new season; the state league constructors: Bahia, Centre-West and Rio Grande do Sul; the state league constructors: Goias, Minas Gerais and North; the state league constructors: Northeast, Parana and Pernambuco; the state league constructors: Rio, Santa Catarina and Sao Paulo; and the clubs average attendance at the end of the season; the Oceania Club Championship: the draw day and the group A and B tables; the Oceania Club Championship: the 8 clubs, the next stage and the semi-finals

**Routines:**
- `cqs4_final_da_serie_c()` = 0x42e980 (bra_nat_third.cpp, thiscall, ret 0): The Series C final: the winner of third-round group A against the winner of group B, in a two-club cup stage [inference: home and away]; the winner is the Series C champion;
- `cqs4_virada_da_serie_c()` = 0x42def0 (bra_nat_third.cpp, thiscall, ret 0): The Series C turnover: archives the season, clears the table, the schedule and the stages, moves to the next year and rebuilds the rules, the 56-club list and the table, the fixtures and the order; then the board expectations, the referee renewal, the seven first-round groups and the predicted positions; the 56-club list is discarded at the end;
- `cqs4_posicoes_previstas_da_serie_c()` = 0x42f0a0 (bra_nat_third.cpp, thiscall, ret 0): The Series C predicted positions: the 56 ordered by reputation; the favourite 1st, the second 2nd, the next two 3rd (the semi-finals), two 5th and two 7th, eight 9th (the second round) and the other forty in bands of eight: 16th, 24th, 32nd, 40th and 48th;
- `cqs5_trofeu_clubes()` = 0x56b730 (eng_auto_cup.cpp, thiscall, ret 0): rule: the League Trophy (LDV Vans) has 56 clubs: all of the second and third divisions, the club relegated from the third to the Conference and, to complete it, Conference clubs (with the nation rule on, those already in it by category, 6 or less first; otherwise up to 30 Conference clubs or other English clubs outside the generic division, sorted by reputation with the best shuffled); the list is split into North and South by the latitude of the stadium city and each half goes from lowest reputation to highest;
- `cqs5_trofeu_fase_das_chaves()` = 0x56bab0 (eng_auto_cup.cpp, thiscall, ret 0): rule: the Trophy section stage is a knockout (Cup_Stage) with the 56 in list order (North, then South), with the rounds of stage 0 of the cup table, no seeding;
- `cqs5_trofeu_fase_seguinte()` = 0x56bc50 (eng_auto_cup.cpp, thiscall, ret 0): rule: the Trophy stage moves on by one; when the sections end (stage 1), it builds the final between the North and South winners; at the last stage, the error box;
- `cqs6_trofeu_final()` = 0x56bce0 (eng_auto_cup.cpp, thiscall, ret 0): rule: when the sections end, the Trophy final is a single tie (2-team Cup_Stage, stage 1) between the one still alive in the cup list (status 1) and the one still alive in the section stage; the round is drawn at once (home side);
- `cqs6_trofeu_classificacao()` = 0x56c010 (eng_auto_cup.cpp, thiscall, ret 0): rule: the Trophy final ranking in the statistics: the highest-reputation club of each half takes 1st and 2nd (the one with the higher reputation is 1st); the others, round by round from the last to the first, share one position per round (3, then 3 + 2 × those already placed...);
- `cqs7_fase_de_copa()` = 0x522e00 (cup_stage.cpp, thiscall, ret 0x3c): rule: a cup knockout stage takes the round table copied from the cup's template (dates, draws), the year, the stage, the id and the parent's rules (+0x49, +0x4a); each team enters with status 0 and its pot seed (given by the cup, or the list position when the cup asks for order, or 0); the fixture tree is built at once: the stage one (Cup_Stage::map_fixture_tree, which links the winners and places the seeds in the first round) or the cup one (Cup::map_fixture_tree, which links at random);
- `cqs7_fim_da_fase_de_copa()` = 0x523300 (cup_stage.cpp, thiscall, ret 0): rule: the end of a knockout stage: the team list, the round table and each round's fixtures freed, and the base destroyed;
- `cqs10_libertadores_mata_mata()` = 0x4c3d10 (conmebol_liber.cpp, thiscall, ret 0): rule: the Libertadores knockout has 16 clubs: the top 2 (+0xbe) of each of the 8 groups (group 1 in table order, the others by position in each stage's table), in a cup stage (stage 7) with the cup's round table and year; the first round is drawn at once;
- `cqs10_merconorte_semifinais()` = 0x4c2330 (con_merc_cup.cpp, thiscall, ret 0): rule: the Merconorte semi-finals have 4 clubs: the winner (+0xbe = 1) of each of the 4 groups, in a cup stage (stage 3); the first round is drawn at once;
- `cqs10_merconorte_fase_seguinte()` = 0x4c20b0 (con_merc_cup.cpp, thiscall, ret 0): rule: the Merconorte moves on one stage when the current one ends; after the 3 group stages (on reaching 3) it builds the semi-finals; going past the last stage is an error;
- `cqs11_merconorte_convidados()` = 0x4c2560 (con_merc_cup.cpp, thiscall, ret 0): rule: the Merconorte invites 16: the best 4 of Mexico, 5 of Colombia, 3 of Peru and 2 of Bolivia (each list sorted by reputation, with the top band shuffled: 3 times the places) and 2 from the American league (with its competition: the latest MLS champion and that of its other competition, and the MLS runner-up if they are the same club; without it, the 2 best MLS clubs); the final list is shuffled (3 × 16 swaps) before the groups;
- `cqs11_mercosul_fase_seguinte()` = 0x4c5140 (conmebol_merc.cpp, thiscall, ret 0): rule: the Mercosur moves on one stage when the current one ends; after the 4 group stages (on reaching 4) it builds the quarter-finals; going past the last stage is an error;
- `cqs11_copa_america_fase_seguinte()` = 0x5e1620 (goldcup.cpp, thiscall, ret 0): rule: the Copa América moves on one stage when the current one ends; after the 2 group stages (on reaching 2) it builds the knockout; going past the last stage is an error;
- `cqs12_copa_america_selecoes()` = 0x5e1cd0 (goldcup.cpp, thiscall, ret 0): rule: the Copa América has 12 nations; in 2001 a fixed list (the groups interleaved in the places: A = Colombia, Chile, Ecuador and Venezuela; B = Brazil, Mexico, Peru and Paraguay; C = Costa Rica, Honduras, Uruguay and Bolivia); in other years the hosts first, then the South American nations and the guests (Japan in 1999 [inference: the nation of 0x9cf36c], in other years that of 0x9cf280, and Mexico always), without repeating the hosts, up to 12, sorted by reputation; without 12, an error;
- `cqs12_copa_america_previstas()` = 0x5e2340 (goldcup.cpp, thiscall, ret 0): rule: the Copa América predicted positions by reputation: 1, 2, 3, 3, four 5ths, 9 and three 10ths;
- `cqs12_copa_america_temporada_nova()` = 0x5e1530 (goldcup.cpp, thiscall, ret 0): rule: the Copa América is every 2 years: when it ends, the table and stages are undone, the season goes to the statistics history, the year goes up 2 and the rules are set up again;
- `cqt1_construir_baiano()` = 0x42f2d0 (bra_reg_bahia.cpp, thiscall, ret 8): The Bahia state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt1_construir_centro_oeste()` = 0x4304b0 (bra_reg_central.cpp, thiscall, ret 8): The Centre-West state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt1_construir_gaucho()` = 0x4317c0 (bra_reg_gaucho.cpp, thiscall, ret 8): The Rio Grande do Sul state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt2_construir_goiano()` = 0x432b50 (bra_reg_goias.cpp, thiscall, ret 8): The Goias state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt2_construir_mineiro()` = 0x433d50 (bra_reg_minas_gerais.cpp, thiscall, ret 8): The Minas Gerais state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt2_construir_norte()` = 0x435340 (bra_reg_north.cpp, thiscall, ret 8): The North state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt3_construir_nordeste()` = 0x4363f0 (bra_reg_northeast.cpp, thiscall, ret 8): The Northeast state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt3_construir_paranaense()` = 0x437400 (bra_reg_parana.cpp, thiscall, ret 8): The Parana state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt3_construir_pernambucano()` = 0x4388f0 (bra_reg_pern.cpp, thiscall, ret 8): The Pernambuco state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt4_construir_carioca()` = 0x439a40 (bra_reg_rio.cpp, thiscall, ret 8): The Rio state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, two halves and the final (three stages besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt4_construir_catarinense()` = 0x43b130 (bra_reg_santa.cpp, thiscall, ret 8): The Santa Catarina state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt4_construir_paulista()` = 0x43c280 (bra_reg_sp.cpp, thiscall, ret 8): The Sao Paulo state league at career creation: the year and the database competition, disciplinary rule 7 and, with no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with the state clubs, the fixtures, the order, the statistics and the predicted positions;
- `cqt4_atualizar_media_de_publico()` = 0x5d6280 (game.cpp, cdecl): The clubs average attendance at the end of each league season: it moves with the final position against the predicted one and with the competition reputation, with a weight by nation (0.1, 0.25 or 0.5 in some, 1 in the others): the champion and runner-up without promotion gain (reputation + places above the prediction) × 200/100 or 125/65 (stadium owner or not); promoted clubs gain by club reputation and the reputation gap to the league above, with that league floor; relegated clubs lose by the gap to the league below, with its ceiling; promotion play-off and mid-table clubs gain or lose by a draw, mixed with the season real attendance (1/5 of the average home crowd + 4/5 of the average) and reputation floors and ceilings (2000, 3000 or 5000 times); at the end, at least 500 and between the club minimum and record;
- `cqt10_oceania_dia()` = 0x797b40 (oceania_club_champ.cpp, thiscall, ret 8): rule: the Oceania Club Championship is drawn on the start day (+0xb6/+0xb8) and only on the call without `completo`: the 8 clubs, the predicted positions, group A in the cup's own table, the fixtures and group B as a stage; before that day the cup does not even run the league day; an error while building returns 1 without the league day;
- `cqt10_oceania_grupo_a()` = 0x7978c0 (oceania_club_champ.cpp, thiscall, ret 0): rule: group A of the Oceania Club Championship has the first 4 chosen clubs, each in the row of its order; human managers get the group draw news;
- `cqt10_oceania_grupo_b()` = 0x797e80 (oceania_club_champ.cpp, thiscall, ret 0): rule: group B of the Oceania Club Championship is a league stage with chosen clubs 4 to 7, the cup's rules and year and the cup's round table for stage 0 (10, 12 and 14 January); human managers get the news;
- `cqt11_oceania_clubes()` = 0x798220 (oceania_club_champ.cpp, thiscall, ret 0): rule: the Oceania Club Championship has 8 clubs in 4 pairs, each pair split between groups A and B by a draw: the Australian league champion (without one, the best Australian club) and the best New Zealand club; the best of Fiji and of Tahiti; two Melanesian clubs of different nations; two Polynesian clubs of different nations (each list sorted by reputation, with the top band shuffled);
- `cqt11_oceania_fase_seguinte()` = 0x797df0 (oceania_club_champ.cpp, thiscall, ret 0): rule: the Oceania Club Championship moves on one stage when the current one ends; after the groups (on reaching 1) it builds the semi-finals; going past the last stage is an error;
- `cqt11_oceania_semifinais()` = 0x798050 (oceania_club_champ.cpp, thiscall, ret 0): rule: the Oceania Club Championship semi-finals cross the groups: A winner × B runner-up and A runner-up × B winner, in a cup stage (stage 1) with the first round set up at once;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (40):** 40 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original 5b041fc1 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 282's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 283`

**Fields touched:**
- league +0xc: the stages (pointer array); pointer; reads slots 8 and 9, writes 10 (final); destroys, frees and reallocates (turnover)
- league +0x30: the current stage; value; writes 10 (final), −1 (turnover)
- league +0x40: the season year; value; reads and writes + 1
- league +0xee: the 56-club list; pointer; sorts (predicted positions), frees and clears (turnover)
- league +0x2c: the stage count; value; reads
- league +0x4c, +0xba: [inference] season flag, the schedule; writes 0, frees
- statistics, club record +0x72: the predicted position; value; writes
- [0xb74340] the officials manager; pointer; reads
- cup +0xb2 club list, +0xb6 how many (56); pointer/value; writes (clubs) and reads (section stage)
- club +0x57 division, +0x5b previous season division [inference], +0x53 nation; pointer; reads
- club +0x5f category (signed byte) [inference]; value; reads
- nation +0x11c bit 4 (the Conference-by-category rule) [inference]; value; reads
- [0x9cf5c4], [0x9cf5c8], [0x9cf69c], [0x9cf764], [0x9cf2e4]: the second and third divisions, the Conference, the generic division and England [inference]; id; reads
- cup +0xc stages, +0x2c stages, +0x30 current stage; pointer/value; writes
- cup +0xa7 entrants list (6 bytes: club, +5 status); pointer; reads
- cup +0xc stages ([0] sections, [1] final); pointer; reads and writes
- cup +0xa3 round table (0x68 bytes per round, +0x1a the round's clubs), +0x3c rounds; reads
- cup +8 statistics (Comp_Stats); pointer; writes each club's final position
- club +0x80 reputation (word); value; reads
- stage +4, +0x28, +0x3a id, +0x3c rounds, +0x3e teams, +0x40 year, +0x42, +0x43, +0x44 stage, +0x49, +0x4a, +0x51, +0x2c, +0x30, +0xab, +0xac, +0xb1; value; writes (constructor)
- stage +0xa3 round table (0x68 bytes per round), +0xa7 teams (6 bytes: club, +4 seed, +5 status), +0xad fixtures per round; pointer; writes (constructor) and frees (destructor)
- cup +0xbe qualifiers per group (signed byte), +0xb1 group 1 table; value/pointer; reads (knockout)
- cup +0xc stages (group League_Stages; the knockout Cup_Stage written to [+0xc][7] or [3]); pointer; reads and writes (knockout)
- cup +0x2c stages, +0x30 current stage; value; reads and writes (next stage)
- club +0x53 nation (its +0 is the id), +0x57 league (its +0 is the competition id); pointer; reads (invited)
- [0x9cf3d0] Mexico, [0x9cf29c] Colombia, [0x9cf428] Peru, [0x9cf254] Bolivia, [0x9cf590] the MLS, [0x9cf728] the American league competition; id; reads
- cup +0xee invited list, +0xf2 its count (16); pointer/value; writes (invited)
- cup +0x2c stages, +0x30 current stage; value; reads and writes (next stages)
- cup +0x40 year (2001 = 0x7d1), +0xee nation list, +0xf2 its count (12); value/pointer; reads and writes (the 12)
- nation +0x71 continent (its +0 is the id), +0 id; national team +0x53 nation; pointer/id; reads (the 12)
- statistics (+8) predicted position of the club entry (+0x72); value; writes (predictions)
- cup +0x4c, +0xba round table, +0x45, +0x30; value/pointer; writes (new season)
- league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7
- league +0x51: [inference] the importance; value; writes 0xf
- league +0x40: the season year; value; writes
- league +4: the competition record in the database; pointer; writes
- league +0x44: [inference] new league flag; value; writes 0xff
- league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes
- league +8: the statistics block (Comp_Stats); pointer; writes
- league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7
- league +0x51: [inference] the importance; value; writes 0xf
- league +0x40: the season year; value; writes
- league +4: the competition record in the database; pointer; writes
- league +0x44: [inference] new league flag; value; writes 0xff
- league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes
- league +8: the statistics block (Comp_Stats); pointer; writes
- league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7
- league +0x51: [inference] the importance; value; writes 0xf
- league +0x40: the season year; value; writes
- league +4: the competition record in the database; pointer; writes
- league +0x44: [inference] new league flag; value; writes 0xff
- league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes
- league +8: the statistics block (Comp_Stats); pointer; writes
- league +0x50: the disciplinary rule (index into the disciplinary bodies table); value; writes 7
- league +0x51: [inference] the importance; value; writes 0xf
- league +0x40: the season year; value; writes
- league +4: the competition record in the database; pointer; writes
- league +0x44: [inference] new league flag; value; writes 0xff
- league +0x2c / +0x30 / +0xc: the stages / the current one / the array; writes
- league +8: the statistics block (Comp_Stats); pointer; writes
- club +0x73 / +0x77 / +0x7b: the average attendance / the minimum / the record; value; reads and writes (attendance)
- club +0x80: the reputation (word); +0x6d: stadium owner; reads
- league +0x1c / +0x20: the league above / below (id); +0xbe, +0xbf, +0xc1: promotion, play-off and relegation places; reads
- competition +0x69: the reputation (word); reads
- cup +0x45 draw done, +0xb6/+0xb8 draw day and relative year, +0x40 year; value; reads and writes +0x45 (day)
- cup +0xee the 8 chosen clubs (club pointers), +0xf2 their count; pointer/value; reads (groups), frees and clears (day)
- cup +0x3e teams, +0xb1 table (0x3b-byte rows: +0 club, +4 position); value/pointer; writes (group A)
- cup +0xc stages ([0] group B), +0x30 current stage; pointer/value; writes (group B)
- club +0x53 nation (+0 the id, +0x71 the continent; [0x9cfa18] Oceania); pointer/id; reads (the 8)
- [0x9cf3f4] New Zealand, [0x9cf224] Australia, [0x9cf4b4] Tahiti, [0x9cf2f8] Fiji, [0x9cf89c] the Australian league; id; reads
- cup +0xee the 8 (group A in places 0..3, B in 4..7), +0xf2 their count; pointer/value; writes (the 8)
- cup +0x2c stages, +0x30 current stage, +0xc stages ([1] the semi-finals); value/pointer; reads and writes (next stage, semi-finals)

**Of interest to:** CMRTE
