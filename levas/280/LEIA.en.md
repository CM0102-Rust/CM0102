# Batch 280 — the schedule of the English first division and the Premiership (rounds, Christmas and the play-offs); the schedule of the Spanish Primera (38 rounds, even and odd year tables, Christmas); the season turnover and the next stage of the state leagues: Bahia, Centre-West and Rio Grande do Sul; the season turnover and the next stage of the state leagues: Goias, North and Northeast; the season turnover and the next stage of the state leagues: Pernambuco, Santa Catarina and Sao Paulo; the schedule of the Spanish Segunda (42 rounds, even and odd year tables); the schedule of the Spanish lower division (42 rounds, even and odd year tables, Christmas); the French first division constructor (the placeholders, the CFA and lower division clubs and the background leagues); the play-offs of Series A and B: the next stage and the knockout of the top 4; the Brazilian football turnover: Series A, the promotion and relegation chain and Series B; the J1: the next stage, the final between the two stage winners and the season turnover; tournament hosts: the default table, the bids news and loading HostCountry.tmp

**Routines:**
- `calendario_da_primeira_inglesa()` = 0x571980 (eng_first.cpp, thiscall, ret 0x10): the Division One schedule: the same 46 dates as Division Two, with TV slots (usually one or two matches on day 6 or 4, slot 1 or 2); the same Christmas by the day of the 16 December round. The promotion play-offs (stage 0): two-legged semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 28 (weekday 0, slot 2) May [inference: venue 4 is the neutral ground];
- `calendario_da_premiership()` = 0x573660 (eng_prm.cpp, thiscall, ret 0x10): the Premiership schedule: 38 rounds (20 clubs), from 19 August to 20 May; Christmas (four fixed-date rounds) by the day the 16 December round falls on; no play-offs (main stage only);
- `calendario_da_primera()` = 0x84e450 (spa_first.cpp, thiscall, ret 0x10): the Primera schedule: 38 rounds (20 clubs), nearly all on Sunday (day 6) with one TV slot (one match on day 5); two tables, by the parity of the year: in odd years (2001/02) from 22 August to 21 May, in even years from 30 August to 6 June; in each, one fixed-date Christmas round (between 20 and 30 December, or 3 January) chosen by the day the mid-December round falls on;
- `cqp7_virada_do_baiano()` = 0x42fe10 (bra_reg_bahia.cpp, thiscall, ret 0): The Bahia turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp7_fase_seguinte_do_baiano()` = 0x42ffc0 (bra_reg_bahia.cpp, thiscall, ret 0): The Bahia next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `cqp7_virada_do_centro_oeste()` = 0x431090 (bra_reg_central.cpp, thiscall, ret 0): The Centre-West turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp7_fase_seguinte_do_centro_oeste()` = 0x431240 (bra_reg_central.cpp, thiscall, ret 0): The Centre-West next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box);
- `cqp7_virada_do_gaucho()` = 0x432370 (bra_reg_gaucho.cpp, thiscall, ret 0): The Rio Grande do Sul turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp7_fase_seguinte_do_gaucho()` = 0x432520 (bra_reg_gaucho.cpp, thiscall, ret 0): The Rio Grande do Sul next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `cqp8_virada_do_goiano()` = 0x4336f0 (bra_reg_goias.cpp, thiscall, ret 0): The Goias turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp8_fase_seguinte_do_goiano()` = 0x4338a0 (bra_reg_goias.cpp, thiscall, ret 0): The Goias next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box);
- `cqp8_virada_do_norte()` = 0x435e80 (bra_reg_north.cpp, thiscall, ret 0): The North turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp8_fase_seguinte_do_norte()` = 0x436030 (bra_reg_north.cpp, thiscall, ret 0): The North next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `cqp8_virada_do_nordeste()` = 0x436e90 (bra_reg_northeast.cpp, thiscall, ret 0): The Northeast turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp8_fase_seguinte_do_nordeste()` = 0x437040 (bra_reg_northeast.cpp, thiscall, ret 0): The Northeast next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `cqp9_virada_do_pernambucano()` = 0x4394d0 (bra_reg_pern.cpp, thiscall, ret 0): The Pernambuco turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp9_fase_seguinte_do_pernambucano()` = 0x439680 (bra_reg_pern.cpp, thiscall, ret 0): The Pernambuco next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `cqp9_virada_do_catarinense()` = 0x43bd10 (bra_reg_santa.cpp, thiscall, ret 0): The Santa Catarina turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp9_fase_seguinte_do_catarinense()` = 0x43bec0 (bra_reg_santa.cpp, thiscall, ret 0): The Santa Catarina next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `cqp9_virada_do_paulista()` = 0x43cd40 (bra_reg_sp.cpp, thiscall, ret 0): The Sao Paulo turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees;
- `cqp9_fase_seguinte_do_paulista()` = 0x43cef0 (bra_reg_sp.cpp, thiscall, ret 0): The Sao Paulo next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box);
- `calendario_da_segunda_espanhola()` = 0x853c90 (spa_second.cpp, thiscall, ret 0x10): the Segunda schedule: 42 rounds (22 clubs), nearly all on Sunday (day 6); in odd years (2001/02) a fixed table from 22 August to 4 June, with one TV slot; in even years, from 30 August to 6 June, with three TV slots and one fixed-date Christmas round chosen by the day the mid-December round falls on;
- `calendario_da_divisao_de_baixo_espanhola()` = 0x851260 (spa_lower.cpp, thiscall, ret 0x10): the Spanish lower division schedule (background): 42 rounds, nearly all on Sunday (day 6); in odd years (2001/02) from 16 August to 16 May, in even years from 30 August to 6 June; in each, one fixed-date Christmas round chosen by the day the mid-December round falls on;
- `construir_primeira_francesa()` = 0x5bb160 (fra_first.cpp, thiscall, ret 8): builds the French first division: discipline 0xc; two placeholder competitions (the CFA and the lower division), with no matches of their own; in a new career, every club whose league in the database is the CFA moves to the generic division with the CFA as reserve competition [inference: the B sides of the professional clubs], and every lower division club moves to the generic division with no reserve; then the rules, the table, the fixtures, the order, the statistics and the predicted positions; at last it builds the CFA (4 groups of 18) and the lower division (2 groups of 20), which leave their matches in the calendar, and frees both objects;
- `cqq7_fase_seguinte_da_serie_a()` = 0x42a950 (bra_nat_first.cpp, thiscall, ret 0): Series A has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0 and the play-off is built; past the last stage it is an error (the box);
- `cqq7_play_off_da_serie_a()` = 0x42a9e0 (bra_nat_first.cpp, thiscall, ret 0): The Series A play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds: the list order) [inference: two-legged semi-finals and final] that decides the Brazilian champion;
- `cqq7_fase_seguinte_da_serie_b()` = 0x42cdd0 (bra_nat_second.cpp, thiscall, ret 0): Series B has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0 and the play-off is built; past the last stage it is an error (the box);
- `cqq7_play_off_da_serie_b()` = 0x42ce60 (bra_nat_second.cpp, thiscall, ret 0): The Series B play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds) that decides the Series B champion [inference: the promotions to Series A come from it];
- `cqq8_virada_da_serie_a()` = 0x42a640 (bra_nat_first.cpp, thiscall, ret 0): The Brazilian turnover, led by Series A: first the promotion and relegation chain; then the Brazilian Cup entrants, the 16 best clubs without a national division into Series C and the swaps between the state leagues and their lower divisions; the division tier of the A and B clubs is capped; Series A is rebuilt (next year, rules, 28 clubs, fixtures); then Series B, Series C and the twelve state leagues turn over (Bahia, Centre-West, Rio Grande do Sul, Goias, Minas Gerais, North, Northeast, Parana, Pernambuco, Rio, Santa Catarina and Sao Paulo), with the board expectations and the referee renewal;
- `cqq8_acesso_e_rebaixamento()` = 0x42ae90 (bra_nat_first.cpp, thiscall, ret 4): Promotion and relegation in Brazil: records the final positions of Series A, Series B, Series C and the seven Series C stages; then swaps the marked clubs between A and B, between B and C and between B and each C stage (who goes up and down comes from the end-of-season marks, with no fixed number here);
- `cqq8_virada_da_serie_b()` = 0x42cc10 (bra_nat_second.cpp, thiscall, ret 0): The Series B turnover: archives the season in the statistics, clears the table and stages, moves to the next year and rebuilds the rules, the table with the 28 Series B clubs, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal;
- `cqq9_fase_seguinte_da_j1()` = 0x6639c0 (jap_j1.cpp, thiscall, ret 0): The J1 has two stages (0 and 1, each a round-robin league) and the final (stage 2): the next stage moves the current stage on and, at the end of the second stage, builds the final; past the last stage it is an error (the box);
- `cqq9_final_da_j1()` = 0x663a50 (jap_j1.cpp, thiscall, ret 0): The J1 championship final: the first-stage winner against the second-stage winner, over two games [inference: home and away], with no seeds; unlike the Brazilian state leagues, it does not check for the same club (the final is built anyway);
- `cqq9_virada_da_j1()` = 0x6636d0 (jap_j1.cpp, thiscall, ret 0): The Japanese turnover, led by the J1: first promotion and relegation with the J2; then the J1 is rebuilt with three stages (the two halves and the final), next year, rules, the table and the fixtures; then the J2 turns over, followed by the predicted positions, the board expectations and the referee renewal;
- `tabela_das_sedes()` = 0x5f9970 (host_country.cpp, thiscall, ret 0): the default host table: 57 entries (World Cup 1998 to 2026 every 4 years, European Championship and Olympics 2000 to 2028, African Cup 2000 to 2014 every 2 years, Copa América 1999 to 2009, Asian Cup 2000 to 2016 and one-off editions), each with up to three bids of two nations, the chosen one (or open/closed voting, or a fixed host) and the decision day (10 July for the World Cup, 20 July or 20 September for the others [inference: the month counts from 0]) plus a random −35 to +34 days;
- `noticia_das_sedes()` = 0x5faec0 (host_country.cpp, thiscall, ret 4): the news of changed hosts: with open voting, the bids (news 4); with a chosen bid, the hosts (news 5); to the managers of the competition's continent or to everyone; except the Olympics, the 2006 World Cup in Germany and the chosen bid of the edition 8 years ahead;
- `carregar_sedes()` = 0x5fb160 (host_country.cpp, thiscall, ret 0): reads HostCountry.tmp (the count and the 0x22-byte entries into a new block); without the file 0 (a new game builds the default table); read errors with the error box;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (31):** 31 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original 0c20b87d (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 279's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 280`

**Fields touched:**
- league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes and reads
- cup round (0x68 bytes); writes (the play-offs)
- league +0x40 season year; value; reads
- league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes and reads
- cup round (0x68 bytes); writes (the play-offs)
- league +0x40 season year; value; reads
- league +0x4c: [inference] season flag; value; writes 0 (turnover)
- league +0xba: auxiliary list; pointer; frees and clears (turnover)
- league +8: the statistics block (Comp_Stats); pointer; reads (turnover)
- league +0xc: the stages (pointer array); pointer; reads and destroys (turnover)
- league +0x2c: the number of stages; value; reads (next stage)
- league +0x30: the current stage (−1 = round robin); value; reads and writes
- league +0x40: the season year; value; reads and writes + 1 (turnover)
- league +4: the competition record in the database; pointer; reads (referees)
- [0xb74340] the officials manager; pointer; reads
- league +0x4c: [inference] season flag; value; writes 0 (turnover)
- league +0xba: auxiliary list; pointer; frees and clears (turnover)
- league +8: the statistics block (Comp_Stats); pointer; reads (turnover)
- league +0xc: the stages (pointer array); pointer; reads and destroys (turnover)
- league +0x2c: the number of stages; value; reads (next stage)
- league +0x30: the current stage (−1 = round robin); value; reads and writes
- league +0x40: the season year; value; reads and writes + 1 (turnover)
- league +4: the competition record in the database; pointer; reads (referees)
- [0xb74340] the officials manager; pointer; reads
- league +0x4c: [inference] season flag; value; writes 0 (turnover)
- league +0xba: auxiliary list; pointer; frees and clears (turnover)
- league +8: the statistics block (Comp_Stats); pointer; reads (turnover)
- league +0xc: the stages (pointer array); pointer; reads and destroys (turnover)
- league +0x2c: the number of stages; value; reads (next stage)
- league +0x30: the current stage (−1 = round robin); value; reads and writes
- league +0x40: the season year; value; reads and writes + 1 (turnover)
- league +4: the competition record in the database; pointer; reads (referees)
- [0xb74340] the officials manager; pointer; reads
- league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes and reads
- cup round (0x68 bytes); writes (the play-offs)
- league +0x40 season year; value; reads
- league round table (0x41 bytes): +0 day of year and +2 relative year (words), +4 code ([inference] the time of day), +5 the 8 slots of 7 bytes (value, day, part, matches; −1 ends), +0x3d value; writes and reads
- cup round (0x68 bytes); writes (the play-offs)
- league +0x40 season year; value; reads
- league +0x40 year, +4 record, +0x50 discipline, +0xee/+0xf2 the CFA and lower division placeholders, +0x44 stage, +0x30 current stage, +0x2c/+0xc stages, +8 statistics; value and pointer; writes
- club +0x57 competition, +0x60 reserve competition; pointer; reads and writes
- placeholder (Comp, 0xa3 bytes): +4 record, +0x42 1, +0x50 0xff; writes
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads
- league +0xc: the stages (pointer array); pointer; writes the first
- league +0x2c: the number of stages; value; reads (next stage)
- league +0x30: the current stage (−1 = round robin); value; reads and writes
- league +0x40: the season year; value; reads
- cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)
- league +0x4c: [inference] season flag; value; writes 0 (turnovers)
- league +0xba: auxiliary list; pointer; frees and clears (turnovers)
- league +8: the statistics block (Comp_Stats); pointer; reads
- league +0xc: the stages (pointer array); pointer; reads and destroys (turnovers), reads the seven of Series C (chain)
- league +0x30: the current stage; value; reads and writes −1 (turnovers)
- league +0x40: the season year; value; reads and writes + 1 (turnovers)
- league +4: the competition record in the database; pointer; reads (referees)
- [0xadadfc] the competition table; pointer; reads
- [0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: Series A, B and C ids; id; reads
- [0x9cf9c0], [0x9cf998], [0x9cf9c8], [0x9cf9b4], [0x9cf9d4], [0x9cf994], [0x9cf9a4], [0x9cf9d0], [0x9cf9bc], [0x9cf9dc], [0x9cf9b0], [0x9cf9e4]: the twelve state league ids; id; reads
- [0xb74340] the officials manager; pointer; reads
- league +0x2c: the number of stages; value; reads (next stage), writes 3 (turnover)
- league +0x30: the current stage; value; reads and writes
- league +0xc: the stages (pointer array); pointer; reads the two halves, writes the final; frees and reallocates (turnover)
- league stage +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads the first row
- league +0x40: the season year; value; reads, writes + 1 (turnover)
- league +0x4c: [inference] season flag; value; writes 0 (turnover)
- league +0xba: auxiliary list; pointer; frees and clears (turnover)
- league +8: the statistics block (Comp_Stats); pointer; reads (turnover)
- [0x9cf768] J2 id; id; reads
- [0xb74340] the officials manager; pointer; reads
- cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)
- hosts [0xb5e134] +0 0x22-byte entries (+0 competition, +4 year, +6/+0xa, +0xe/+0x12, +0x16/+0x1a the three bids, +0x1e chosen/state, +0x1f decision day, +0x21 interval), +4 count (word); writes (tabela_das_sedes, carregar_sedes), reads (noticia_das_sedes)
- [0xae2c92] game year; [0xae23d0] competitions (0x6b bytes, +0x59 continent); reads

**Of interest to:** CMRTE, SGE
