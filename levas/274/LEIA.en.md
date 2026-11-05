# Batch 274 — the group rules of the Copa Libertadores, the Copa Merconorte and the Oceania Club Championship; the World Cup qualifier rules in Europe, South America and Asia; the group rules of the Copa America, the African Nations Cup and the Oceania qualifiers; the Copa Libertadores: the qualified clubs, each club's fate and the next stage; the African Nations Cup: the next stage, the group A table and the season honours; the rules of the three Brazilian national divisions (Series A, B and C); the rules of the Centre-West, Gaucho, Northeast, Paulista, Bahia and Santa Catarina state leagues; the rules of the Parana, North, Pernambuco, Minas Gerais and Rio state leagues; the rules of the Goias state league and the two J-League divisions; the rules of the Australian NSL and the MLS, and the season honours of the state leagues and Series A and B; the fate of each club at the end of the stages of Series A, Series C and seven state leagues; the fate of each club at the end of the stages of the Centre-West, Goias, Minas Gerais, Parana, Rio and the NSL; the club progress in the competition statistics in Minas Gerais, Parana, Rio, the MLS and Series C

**Routines:**
- `cqj11_libertadores_regras()` = 0x4c3890 (conmebol_liber.cpp, thiscall, ret 0): rule: the Libertadores groups are home and away (double round robin), 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3 [inference: goal difference, goals for, wins]; two qualifiers per group; clubs from several nations; category 7 on each match; 7 substitutes on the bench and 3 changes; starts on day 15 of month 0 of the competition year [inference: 15 January, months counted from 0]; prize fund of 2,000,000;
- `cqj11_merconorte_regras()` = 0x4c1ed0 (con_merc_cup.cpp, thiscall, ret 0): rule: the Copa Merconorte groups are home and away, 3/1 points, tiebreak 1, 2, 3, one qualifier per group, category 7, starts on day 1 of month 7 [inference: 1 August], prize fund of 2,000,000;
- `cqj11_oceania_clubes_regras()` = 0x7979b0 (oceania_club_champ.cpp, thiscall, ret 0): rule: the Oceania Club Championship groups are single round robin, 3/1 points, tiebreak by codes 1, 2 and 4, two qualifiers per group (the top ones go on, the others are out: rule bits 0x28), category 8 (the deciding stages one), 3 substitutes on the bench and 3 changes; starts on day 20 of month 7 moved to weekday 5 [inference]; no prize fund;
- `cqj12_eliminatoria_europa_regras()` = 0x9191e0 (wc_europe_league.cpp, thiscall, ret 0): rule: the European groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2, 4 and 5 [inference: goal difference, goals for, head-to-head, away goals]; each group winner goes straight through; national teams (+0x43 = 2); category 10 (national teams) on each match; 7 substitutes and 3 changes; starts on 25 of month 2 [inference: 25 March] moved to weekday 5;
- `cqj12_eliminatoria_sul_americana_regras()` = 0x921040 (wc_south_american_league.cpp, thiscall, ret 0): rule: the South American qualifier is a single home and away league, 3/1 points, tiebreak 1, 2 and 3; the direct places are four minus the exempt nations (the world champion and South American hosts, counted in +0xee and +0xef [inference]); the fifth goes to the play-off (+0xbf = 1); category 10, 7 substitutes and 3 changes; starts on 25 of month 0 on weekday 5;
- `cqj12_eliminatoria_asia_regras()` = 0x912eb0 (wc_asia_league.cpp, thiscall, ret 0): rule: the Asian groups are home and away, 3/1 points, tiebreak 1, 2 and 3, each group winner goes on, category 10, starts on 25 of month 2 on weekday 5, +0xb8 = 1 [inference: the stage starts the following year];
- `cqk10_copa_america_regras()` = 0x5e12d0 (goldcup.cpp, thiscall, ret 0): rule: the Copa America groups are single round robin, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3 [inference: goal difference, goals for, wins]; two qualifiers per group; national teams; category 10 on each match; 7 substitutes and 3 changes; the date of 20 of month 2 of the competition year on weekday 5 [inference: the draw date];
- `cqk10_copa_africana_regras()` = 0x401ab0 (african_nations.cpp, thiscall, ret 0): rule: the African Nations Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group, category 10, 7 substitutes and 3 changes; the date of 5 of month 9 of the year before the tournament on weekday 5 [inference: the draw in the previous year] and +0xb8 = −1;
- `cqk10_eliminatoria_oceania_regras()` = 0x91ecf0 (wc_oceania_league.cpp, thiscall, ret 0): rule: the Oceania qualifier groups are single round robin, 3/1 points, tiebreak 1, 2, 3, one qualifier per group (to the Oceania final), category 10, 7 substitutes and 3 changes, starting on 15 of month 0 on weekday 5;
- `cqk11_libertadores_classificados()` = 0x4c3f40 (conmebol_liber.cpp, thiscall, ret 0): rule: the Libertadores takes the 32 clubs marked with it as the season's continental competition (the mark comes from league qualification), in database order, then shuffled (96 random swaps) for the groups; any other count is an error (the box);
- `cqk11_libertadores_registros()` = 0x4c4080 (conmebol_liber.cpp, thiscall, ret 0x18): rule: in the seven groups (stages 0 to 6), a club going through gets the qualification record (weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 7), a club going through gets the qualification by round (weight 50 to 100), one going out the knocked-out record; the winner gets the title record and goes to the Intercontinental Cup, in the South American place (place 0 if a club of continent [0x9cfa1c] is there, otherwise place 1), with the Intercontinental season archive and predicted positions; the runner-up gets the runner-up record;
- `cqk11_libertadores_fase_seguinte()` = 0x4c3a80 (conmebol_liber.cpp, thiscall, ret 0): rule: the Libertadores has seven group stages (0 to 6) and the knockout at stage 7 (the top two of each group, 16 clubs); going past the last stage is an error (the box);
- `cqk12_africana_fase_seguinte()` = 0x401e10 (african_nations.cpp, thiscall, ret 0): rule: the African Nations Cup has three group stages (stages 0 to 2, one per group round [inference]) and, at stage 3, the knockout; going past the last stage is an error (the box);
- `cqk12_africana_grupo_a()` = 0x4019c0 (african_nations.cpp, thiscall, ret 0): rule: group A has four national teams, the first four of the draw list, in order; the managers get the draw news;
- `cqk12_africana_historico()` = 0x401b90 (african_nations.cpp, thiscall, ret 0): rule: the season honours (winner, runner-up, third) are the knockout ones;
- `cqi7_regras_da_serie_a()` = 0x42a5a0 (bra_nat_first.cpp, thiscall): Series A: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no league above, drops to Series B; 4 top places that, by the rule word 8, lead to the next stage (the play-offs; no promotion and no champion band: the champion comes from the knockout) [inference: the next stage is the slot 11 one]; 4 relegated; 7 substitutes on the bench and 3 substitutions; fixed squad numbers; category 2 (league) matches. Then the main stage schedule;
- `cqi7_regras_da_serie_b()` = 0x42cb70 (bra_nat_second.cpp, thiscall): Series B: single round robin; win 3, draw 1; head-to-head and goal difference; promotes to Series A and relegates to Series C; 4 go up (rule word 2: the top places are promotion) and 4 go down; 7 substitutes, 3 substitutions, fixed numbers. Then the schedule;
- `cqi7_regras_da_serie_c()` = 0x42de50 (bra_nat_third.cpp, thiscall): Series C: double round robin (home-and-away groups [inference: the groups of 7 of the 0x42d620 schedule]); win 3, draw 1; tie-break by goal difference, goals scored and away goals; promotes to Series B, no league below; 2 top places leading to the next stage (rule word 8) and 2 bottom ones [inference: no effect, there is no league below]; 7 substitutes, 3 substitutions, fixed numbers. Then the schedule;
- `cqi8_regras_do_centro_oeste()` = 0x430ff0 (bra_reg_central.cpp, thiscall): Centre-West: single round robin; 8 top places (the 8-club knockout [inference: the slot 11 stage]); 3 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage schedule;
- `cqi8_regras_do_gaucho()` = 0x4322d0 (bra_reg_gaucho.cpp, thiscall): Gaucho, Northeast Cup and Paulista (the same routine in the three vtables): single round robin; 4 top places (the 4-club knockout); 2 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage schedule;
- `cqi8_regras_do_catarinense()` = 0x43bc70 (bra_reg_santa.cpp, thiscall): Bahia and Santa Catarina (the same routine in the two vtables): double round robin; 4 top places (the 4-club semi-final); 1 relegated; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion band: the champion comes from the final stage); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage schedule;
- `cqi9_regras_do_paranaense()` = 0x437fc0 (bra_reg_parana.cpp, thiscall): Parana: no top places in the overall table (each half is a league stage of its own, created by add_result 0x4386e0, and decides the title [inference]); 4 relegated; double round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage schedule;
- `cqi9_regras_do_pernambucano()` = 0x439430 (bra_reg_pern.cpp, thiscall): North and Pernambuco (the same routine in the two vtables): 4 top places (the final stage of 4); 2 relegated; double round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage schedule;
- `cqi9_regras_do_carioca()` = 0x43a600 (bra_reg_rio.cpp, thiscall): Minas Gerais and Rio (the same routine in the two vtables): no top places in the overall table (the halves have stages of their own, created by add_result 0x43af20 / 0x435130 [inference: they decide the title]); 2 relegated; double round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no play-offs in the table; no league above or below (the relegated go to the non-playable pool [inference]); rule word 8 (the top places lead to the next stage; no champion band); 7 bench substitutes and 3 substitutions; category 2 (league) matches. Then the main stage schedule;
- `cqj7_regras_do_goiano()` = 0x433640 (bra_reg_goias.cpp, thiscall): Goias: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; 8 top places (rule word 8: they lead to the next stage, the knockout [inference]); 4 relegated, no league above or below; 7 bench substitutes, 3 substitutions; category 2 matches. Then the schedule;
- `cqj7_regras_da_j1()` = 0x663620 (jap_j1.cpp, thiscall): J1: double round robin (the two stages of the year, created by add_result 0x663940 [inference]); win 3, draw 1; tie-break by goal difference, goals scored and away goals; no top places in the overall table (rule word 8, no champion band: the champion comes from the final between the stage winners [inference]); 2 go down to J2; no league above; 5 bench substitutes and 3 substitutions; fixed squad numbers; the word +0xab = 0x82 copied to each match. Then the schedule;
- `cqj7_regras_da_j2()` = 0x665140 (jap_j2.cpp, thiscall): J2: four round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; 2 go up to J1 (the rule word stays the base one, 3: champion and promotion), no relegation, no league below; 5 bench substitutes, 3 substitutions. Then the schedule;
- `cqj8_regras_da_nsl()` = 0x411ce0 (aus_nsl.cpp, thiscall): NSL: three round robins; win 3, draw 1; tie-break by goal difference, goals scored and away goals; the top 6 go to the finals series (rule word 0x28: 0x8 next stage, no champion band, and 0x20, clubs without a chance marked eliminated); nobody goes up or down; 4 bench substitutes and 3 substitutions; fixed squad numbers. Then the schedule;
- `cqj8_regras_da_mls()` = 0x908660 (usa_mls.cpp, thiscall): MLS: 32 games per team (+0xa7; four round robins in +0x3c); win 3, draw 1; tie-break by goal difference, goals scored and head-to-head; 1 top place in the overall table (rule word 0x48: 0x8 next stage, the play-offs; 0x40 fixed round order); nobody goes up or down; the word +0xab = 0xc2 copied to each match. No schedule here: the games are built by hand (0x9086e0);
- `cqj8_historico_do_estadual()` = 0x432970 (bra_reg_gaucho.cpp, thiscall): end of a state league (nine of them): records the honours of the first stage (the table) and calls 0x42bba0, which, once the twelve state leagues are over, puts the champions into the Champions Cup;
- `cqj8_historico_do_primeiro_estagio()` = 0x408eb0 (arg_second.cpp, thiscall): end of the Argentine second division and Series A and B: the honours come from the first stage, not the overall table;
- `cqj9_destino_na_serie_a()` = 0x42abc0 (bra_nat_first.cpp, thiscall): Series A: at the end of the round robin, the clubs through to the knockout get the qualification record (stage 0x41c, weight 0x28) and the relegated the relegation record; in the knockout, the champion gets the champion record and fate 5 in the table, the eliminated fate -2;
- `cqj9_destino_na_serie_c()` = 0x42eb00 (bra_nat_third.cpp, thiscall): Series C: in the groups, the qualified get the qualification record; in stage 7, those who go through; in the final group (stages 8 and 9), the two promoted get the promotion (weight 0x32) and qualification records and fate 0 (promotion) in the first table that has them; in the final (stage 10), the champion gets the champion record;
- `cqj9_destino_no_baiano()` = 0x430210 (bra_reg_bahia.cpp, thiscall): Bahia, Gaucho, North, Northeast, Pernambuco, Santa Catarina and Paulista: at the end of the round robin, the clubs through to the final stage get the qualification record (round 0x82, weight 0x32) and the relegated the relegation record; in the knockout, the champion gets the champion record and fate 5; the eliminated, the knocked-out record weighted 30 to 50 by the round they fell in, and fate -2;
- `cqk7_destino_no_centro_oeste()` = 0x4314e0 (bra_reg_central.cpp, thiscall): Centre-West and Goias: at the end of the round robin, the clubs through to the knockout get the qualification record (round 0x82, weight 0x32) and the relegated the relegation record; in the knockout (stage 0), the champion gets the champion record and fate 5; the eliminated, the knocked-out record weighted 30 to 50 by round (over 3 rounds: quarter-finals, semi-finals, final) and fate -2;
- `cqk7_destino_no_mineiro()` = 0x434d90 (bra_reg_minas_gerais.cpp, thiscall): Minas Gerais, Parana and Rio: the round robin as in the Centre-West; the two halves (stages 0 and 1) record nothing and return -1; the final knockout (stage 2, a single round: weight 40 for the loser) gives the champion (fate 5) and the runner-up (fate -2);
- `cqk7_destino_na_nsl()` = 0x411980 (aus_nsl.cpp, thiscall): NSL: at the end of the regular season, the six clubs through to the finals series get the qualification record (stage 0x435, weight 0x28); in the finals, those who go through get the record (round 0x96) and those who fall the failed-to-qualify one; the champion (grand final) gets the champion record. No fate written to the table;
- `cqk8_progresso_no_mineiro()` = 0x4350d0 (bra_reg_minas_gerais.cpp, thiscall): Minas Gerais, Parana and Rio: writes the club progress codes (current, lowest and highest position); in the round robin, the lowest under 3 becomes 1 and the highest under 3 becomes 2 [inference: the two halves have tables of their own and the overall one gives no title];
- `cqk8_progresso_na_mls()` = 0x909dd0 (usa_mls.cpp, thiscall): MLS: in the regular season and play-off stages 0 and 1, the current position 1, 2, 3, 4 becomes 1, 3, 7, 10, the lowest 1 to 3 becomes 1 and 4 becomes 10, the highest 1 to 3 becomes 7 and 4 becomes 10 (positions on the whole-league scale [inference]); in stage 2, each code becomes 2 × code − 1; after that, as they come;
- `cqk8_progresso_na_serie_c()` = 0x42e100 (bra_nat_third.cpp, thiscall): Series C: in the groups (stages up to 6), the group position becomes the position on the 49-place Series C scale [inference]: 1 and 2 → 1 (the highest → 9), 3 → 17, 4 → 25, 5 → 33, 6 → 41, 7 → 49; in later stages, the codes as they come;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (39):** 33 caught. `cqj12_europa_uma_semana_antes` escaped: the start day only applies to the 2006 World Cup qualifiers, which begin after 2002-06-01 (outside the test career; confirmed with 0 divergences in comparison mode) [inference]. `cqj12_america_do_sul_uma_semana_antes` escaped: the start day only applies to the 2006 World Cup qualifiers, which begin after 2002-06-01 (outside the test career; confirmed with 0 divergences in comparison mode) [inference]. `cqj12_asia_uma_semana_antes` escaped: the start day only applies to the 2006 World Cup qualifiers, which begin after 2002-06-01 (outside the test career; confirmed with 0 divergences in comparison mode) [inference]. `cqk10_oceania_uma_semana_depois` escaped: the start day only applies to the 2006 World Cup qualifiers, which begin after 2002-06-01 (outside the test career; confirmed with 0 divergences in comparison mode) [inference]. `cqk11_campeao_na_vaga_1` escaped: the Libertadores champion slot in the club world cup is only read when it is played, after 2002-06-01 (0 divergences in comparison mode) [inference]. `cqk7_classificado_estadual` escaped: the "qualified" record only goes to the managers' history, which the simulation does not read up to the date (0 divergences in comparison mode) [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (d2c61798).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original 4bce14f7 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 273's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 274`

**Fields touched:**
- group league +0xc2/+0xc3 points, +0xc4 category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places, +0x3c rounds, +0x42 type (2 international clubs), +0x43, +0x49 bench substitutes, +0x4a changes, +0x28 prize fund, +0xd9 rule bits, +0xb6 start date (the first word of the Date), +0xb8, +0xba schedule; value/pointer; writes (all three)
- competition +0x40 year; value; reads (all three)
- qualifier league +0xc2/+0xc3 points, +0xc4 category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places, +0x3c rounds, +0x43, +0x49/+0x4a bench and changes, +0xa7, +0xab, +0xdb, +0xb6 start date, +0xb8, +0xba schedule; value/pointer; writes (all three)
- South American qualifier +0xee/+0xef exempt nations [inference]; [0x9cf76c] the competition +0xef comes from; value/id; reads and writes +0xef (cqj12_eliminatoria_sul_americana_regras)
- group league +0xc2/+0xc3 points, +0xc4 category, +0xc5..+0xc8 tiebreaks, +0xbe..+0xc1 places, +0x3c rounds, +0x43 (2 national teams), +0x49/+0x4a bench and changes, +0xb6 date, +0xb8, +0xba schedule; value/pointer; writes (all three)
- competition +0x40 year; value; reads (all three)
- club +0x1db continental competition of the season; id; reads (cqk11_libertadores_classificados)
- Libertadores +0xee qualified list (pointers), +0xf2 count (32); pointer/value; writes (cqk11_libertadores_classificados)
- Intercontinental Cup ([0x9cf6e4]) +0xa7 teams (6 bytes; places 0 and 1), +8 statistics; nation +0x71 continent (+0 id), [0x9cfa1c] South American continent; pointer/id; reads and writes the place (cqk11_libertadores_registros)
- competition +0x2c stages, +0x30 current stage; value; reads and writes +0x30 (cqk11_libertadores_fase_seguinte)
- competition +0x2c stages, +0x30 current stage, +0xc stages (stage 3: the knockout); value/pointer; reads and writes +0x30 (cqk12_africana_fase_seguinte, cqk12_africana_historico)
- African Nations Cup +0xee drawn teams list (pointers), +0x3e teams, +0xb1 table (0x3b-byte rows: +0 team, +4 position); pointer/value; reads and writes (cqk12_africana_grupo_a)
- league +0x3c: rounds robin; value (word); writes (all three)
- league +0xc2 / +0xc3: points per win / per draw; value; writes
- league +0xc4: category copied to each match (2 = league, goes to the match +0x3a); value; writes
- league +0x42: competition type (1 = national club); value; writes
- league +0xc5..+0xc7: tie-breaks (1 goal difference, 2 goals for, 3 head-to-head, 4 away goals, 5 wins); value; writes
- league +0xbe / +0xbf / +0xc0 / +0xc1: top places, upper play-off, lower play-off, relegated; value; writes
- league +0x1c / +0x20: league above / below; id (fixed key, -1 = none); writes
- league +0xd9: rule bit word (2 promotion, 8 next stage); value (word); writes
- league +0x49 / +0x4a: bench substitutes / substitutions; value; writes
- league +0x52: stage flags (2 = fixed squad numbers); value; writes
- league +0xa9 / +0x3a: schedule outputs (rounds, stage code); value (word); writes (through slot 16)
- league +0xba: schedule (0x41-byte rounds); pointer; writes
- [0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: Series A, B and C keys; id; reads
- league +0x3c: round robins; value (word); writes
- league +0xc2 / +0xc3: points per win / per draw; value; writes
- league +0xc4: category copied to each match (2 = league); value; writes
- league +0x42: competition type (1 = national club); value; writes
- league +0xc5..+0xc7: tie-breaks (3 head-to-head, 1 goal difference, 0 none); value; writes
- league +0xbe..+0xc1: top places, upper and lower play-offs (0), relegated; value; writes
- league +0x1c / +0x20: league above / below (-1 = none); id; writes
- league +0xd9: rule bit word (8 = next stage); value (word); writes
- league +0x49 / +0x4a: bench substitutes / substitutions; value; writes
- league +0xab / +0xdb: words copied to each match (match +0x38 and +0x36) [inference: stage codes]; value; writes 0
- league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (through slot 16)
- league +0x3c: round robins (2); value (word); writes
- league +0xc2 / +0xc3: points per win / per draw; value; writes
- league +0xc4: category copied to each match (2 = league); value; writes
- league +0x42: competition type (1 = national club); value; writes
- league +0xc5..+0xc7: tie-breaks (3 head-to-head, 1 goal difference, 0 none); value; writes
- league +0xbe..+0xc1: top places, upper and lower play-offs (0), relegated; value; writes
- league +0x1c / +0x20: league above / below (-1 = none); id; writes
- league +0xd9: rule bit word (8 = next stage); value (word); writes
- league +0x49 / +0x4a: bench substitutes / substitutions; value; writes
- league +0xab / +0xdb: words copied to each match (match +0x38 and +0x36) [inference: stage codes]; value; writes 0
- league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (through slot 16)
- league +0x3c: round robins; value (word); writes
- league +0xc2 / +0xc3: points per win / per draw; value; writes
- league +0xc4: category copied to each match (2 = league); value; writes
- league +0x42: competition type (1 = national club); value; writes
- league +0xc5..+0xc8: tie-breaks (1 goal difference, 2 goals for, 3 head-to-head, 4 away goals); value; writes
- league +0xbe..+0xc1: top places, upper and lower play-offs, relegated; value; writes
- league +0x1c / +0x20: league above / below (fixed key, -1 = none); id; writes
- league +0xd9: rule bit word (8 = next stage); value (word); writes (Goias and J1)
- league +0x49 / +0x4a: bench substitutes / substitutions; value; writes
- league +0x52: stage flags (2 = fixed numbers); value; writes (J1)
- league +0xab / +0xdb: words copied to each match (match +0x38 and +0x36) [inference: stage codes]; value; writes (0x82 and 0 in the J-League, 0 and 0 in Goias)
- league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (through slot 16)
- [0x9cf65c] / [0x9cf768]: J1 and J2 keys; id; reads
- league +0x3c: round robins; value (word); writes
- league +0xa7: games per team in the season; value (word); writes (MLS: 32)
- league +0xc2 / +0xc3: points per win / per draw; value; writes
- league +0xc4: category copied to each match (2 = league); value; writes
- league +0x42: competition type (1 = national club); value; writes
- league +0xc5..+0xc8: tie-breaks (1 goal difference, 2 goals for, 3 head-to-head, 4 away goals); value; writes
- league +0xbe..+0xc1: top places, play-offs, relegated; value; writes
- league +0x1c / +0x20: league above / below (-1 = none); id; writes
- league +0xd9: rule bit word (0x8 next stage, 0x20 eliminated, 0x40 fixed order); value (word); writes
- league +0x49 / +0x4a: bench substitutes / substitutions; value; writes (NSL)
- league +0x52: stage flags (2 = fixed numbers); value; writes (NSL)
- league +0xab: word copied to each match (match +0x38) [inference: stage code]; value; writes (0 in the NSL, 0xc2 in the MLS)
- league +0xa9 / +0x3a / +0xba: schedule outputs and the schedule; value / pointer; writes (NSL, through slot 16)
- league +0xc: stage vector; pointer; reads the first (both honours routines)
- league +4: competition record in the database; pointer; reads (goes into the career records)
- league +0xb1 / +0x3e: table (0x3b rows) and team count; pointer / value; reads
- table row +0: club; pointer; reads
- table row +0x37: fate (5 champion, 0 promoted, -2 eliminated); value; writes
- league +0xc: stage vector; pointer; reads stages 0..6 (Series C)
- match +0x30 / +0x32 / +0x34: round, stage and the scale round; value (word); reads (Bahia)
- [0xdd7f00]: Staff_Records; pointer; reads
- league +4: competition record in the database; pointer; reads (goes into the career records)
- league +0xb1 / +0x3e: table (0x3b rows) and team count; pointer / value; reads
- table row +0: club; pointer; reads
- table row +0x37: fate (5 champion, -2 eliminated); value; writes (state leagues)
- match +0x30 / +0x32 / +0x34: round, stage and the scale round; value (word); reads
- [0xdd7f00]: Staff_Records; pointer; reads
- league +8: Comp_Stats (competition statistics); pointer; reads
- Comp_Stats entry +0x73 / +0x74 / +0x75: club progress (current, lowest, highest position); value; writes
- Comp_Stats +0x18: table order flag; value; reads and writes (through 0x4a4850)

**Of interest to:** CMRTE
