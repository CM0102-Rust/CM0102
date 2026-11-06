# Batch 277 — the transfer rules of Norway, Northern Ireland and Wales: the transfer windows; the transfer rules of Turkey, Poland and Finland: the transfer windows; the transfer rules of Argentina, Portugal and South Korea: the transfer windows; the transfer rules of Greece, Russia and Belgium: the transfer windows; the registration for the three European cups, the initial FIFA ranking table and the list of neutral venues; the table of the three Brazilian national divisions: the Series A and Series B clubs and the Series C draw; the table of the state leagues: Bahia, Centre-West and Rio Grande do Sul; the table of the state leagues: Goias, Minas Gerais and North; the table of the state leagues: Northeast, Parana and Pernambuco; the table of the state leagues: Rio, Santa Catarina and Sao Paulo; the table of the Japanese and Australian leagues: the J1, J2 and NSL clubs, the J1 prize fund and TV money; the semi-finals of the state and regional leagues: Bahia, North and Northeast; the semi-finals of the state and regional leagues: Pernambuco, Santa Catarina and Sao Paulo

**Routines:**
- `regras_noruega()` = 0x7944f0 (norway_rules.cpp, thiscall, ret 0x14): the Norway rule (17): one window, from 1 September to 31 August;
- `regras_irlanda_do_norte()` = 0x7936d0 (northern_ireland_rules.cpp, thiscall, ret 0x14): the Northern Ireland rule (16): one window, from 6 May (moved to the Saturday [inference]) to 20 March;
- `regras_pais_de_gales()` = 0x90f080 (wales_rules.cpp, thiscall, ret 0x14): the Wales rule (26): one window, from 7 May (moved to the Sunday [inference]) to 20 March;
- `regras_turquia()` = 0x9020a0 (turkey_rules.cpp, thiscall, ret 0x14): the Turkey rule (24): one window, from 1 June to 31 January; +0x13 = 8 (base default 50) [inference: the foreign limit]; +0x14 = 8 (default 30);
- `regras_polonia()` = 0x7cd830 (poland_rules.cpp, thiscall, ret 0x14): the Poland rule (18): one window, from 1 July to 20 February; +0x13 = 5 (base default 50) [inference: the foreign limit]; +0x14 = 5 (default 30);
- `regras_finlandia()` = 0x5a9950 (finland_rules.cpp, thiscall, ret 0x14): the Finland rule (8): one window, from 16 November to 15 August; dword +0x15 = 1;
- `janelas_argentina()` = 0x40a620 (argentina_rules.cpp, thiscall, ret 0): the Argentina rule (1): two windows, from 24 July (moved to the Sunday [inference]) to 6 August (moved to the Wednesday [inference]); from 14 December (moved to the Sunday [inference]) to 4 March (moved to the Wednesday [inference]);
- `janelas_portugal()` = 0x7d3df0 (portugal_rules.cpp, thiscall, ret 0): the Portugal rule (19): two windows, from 1 June to 1 September; from 15 December to 15 January;
- `regras_coreia()` = 0x66e630 (korea_rules.cpp, thiscall, ret 0x14): the South Korea rule (27): one window, from 25 November (moved to the Sunday [inference]) to 25 July; +0x13 = 7 (base default 50) [inference: the foreign limit];
- `janelas_grecia()` = 0x5ecf20 (greece_rules.cpp, thiscall, ret 0): the Greece rule (11): two windows, from 22 May (moved to the Sunday [inference]) to 29 August (moved to the Saturday [inference]); from 1 January to 20 January;
- `janelas_russia()` = 0x7ec7e0 (russia_rules.cpp, thiscall, ret 0): the Russia rule (20): two windows, from 15 December to 24 March (moved to the Thursday [inference]); from 28 June (moved to the Tuesday [inference]) to 4 August;
- `janelas_belgica()` = 0x424c00 (belgium_rules.cpp, thiscall, ret 0): the Belgium rule (3): three windows, from 1 June to 30 June; from 1 June to 31 December; from 1 June to 31 March;
- `inscricao_europeia()` = 0x85a9b0 (squad_manager.cpp, thiscall, ret 4): the rule: at the first stage everyone's registration is cleared; at each stage each club in the Champions League, the UEFA Cup or the third cup [inference: the Cup Winners' Cup] picks its registered players (human-managed clubs get the news and pick themselves); in the career, stage 3 runs in 2002 (not in 2001);
- `ranking_fifa_inicial()` = 0x58dd50 (fifa_rankings.cpp, thiscall, ret 0): the initial FIFA ranking rule: each nation's points are its database strength ÷ 35, repeated as the points of each of the 6 years [inference] (× 1000); the position comes from the points order;
- `lista_de_estadios_neutros()` = 0x85dfc0 (stadium.cpp, thiscall, ret 0): the neutral venue rule (finals and semi-finals): grounds with a city, nation and continent qualify, except the City of Manchester Stadium (under construction) and, in nations with competitions, those under 5000 seats; each nation's national stadium is flagged;
- `cql7_tabela_da_serie_a()` = 0x42a480 (bra_nat_first.cpp, thiscall, ret 0): Series A has 28 clubs: those whose national division is Series A, in database order; when full, TV pays 150,000 to each club (no prize fund);
- `cql7_tabela_da_serie_b()` = 0x42ca70 (bra_nat_second.cpp, thiscall, ret 0): Series B has 28 clubs, those of the division in database order; no TV money nor prize fund;
- `cql7_tabela_da_serie_c()` = 0x42dc90 (bra_nat_third.cpp, thiscall, ret 0): Series C takes up to 56 clubs (those of the division, in database order), which are drawn (3 swaps per club); the first 7 of the draw make the first table, of 7 clubs, and the drawn list is kept for the other groups [inference: 8 groups of 7 in the first stage];
- `cql8_tabela_do_baiano()` = 0x42fd10 (bra_reg_bahia.cpp, thiscall, ret 0): the Bahia league has 10 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cql8_tabela_do_centro_oeste()` = 0x430ef0 (bra_reg_central.cpp, thiscall, ret 0): the Centre-West league has 15 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cql8_tabela_do_gaucho()` = 0x4321d0 (bra_reg_gaucho.cpp, thiscall, ret 0): the Rio Grande do Sul league has 17 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cql9_tabela_do_goiano()` = 0x433540 (bra_reg_goias.cpp, thiscall, ret 0): the Goias league has 14 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cql9_tabela_do_mineiro()` = 0x434810 (bra_reg_minas_gerais.cpp, thiscall, ret 0): the Minas Gerais league has 12 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cql9_tabela_do_norte()` = 0x435d80 (bra_reg_north.cpp, thiscall, ret 0): the North league has 10 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm7_tabela_do_nordeste()` = 0x436d90 (bra_reg_northeast.cpp, thiscall, ret 0): the Northeast league has 16 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm7_tabela_do_paranaense()` = 0x437ec0 (bra_reg_parana.cpp, thiscall, ret 0): the Parana league has 12 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm7_tabela_do_pernambucano()` = 0x439330 (bra_reg_pern.cpp, thiscall, ret 0): the Pernambuco league has 10 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm8_tabela_do_carioca()` = 0x43a500 (bra_reg_rio.cpp, thiscall, ret 0): the Rio league has 12 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm8_tabela_do_catarinense()` = 0x43bb70 (bra_reg_santa.cpp, thiscall, ret 0): the Santa Catarina league has 10 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm8_tabela_do_paulista()` = 0x43cc40 (bra_reg_sp.cpp, thiscall, ret 0): the Sao Paulo league has 16 clubs: those with it as their reserve division in the database (the field that holds the state or regional league), in database order; the table only counts when full; no TV money nor prize fund;
- `cqm9_tabela_da_j1()` = 0x6634e0 (jap_j1.cpp, thiscall, ret 0): J1 has 16 clubs, those of the division in database order; those beyond 16 lose their division (left with no league); when full, the prize fund starts at 100,000 per place and TV pays 3,000,000 to each club;
- `cqm9_tabela_da_j2()` = 0x665030 (jap_j2.cpp, thiscall, ret 0): J2 has 12 clubs, those of the division in database order; those beyond 12 lose their division; no prize fund nor TV money;
- `cqm9_tabela_da_nsl()` = 0x411b70 (aus_nsl.cpp, thiscall, ret 0): the Australian NSL has 13 clubs, those of the division in database order; with fewer, the game shows the error box; no prize fund nor TV money;
- `cqn7_mata_mata_do_baiano()` = 0x430050 (bra_reg_bahia.cpp, thiscall, ret 0): Bahia: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);
- `cqn7_mata_mata_do_norte()` = 0x4360c0 (bra_reg_north.cpp, thiscall, ret 0): North: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);
- `cqn7_mata_mata_do_nordeste()` = 0x4370d0 (bra_reg_northeast.cpp, thiscall, ret 0): Northeast: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);
- `cqn8_mata_mata_do_pernambucano()` = 0x439710 (bra_reg_pern.cpp, thiscall, ret 0): Pernambuco: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);
- `cqn8_mata_mata_do_catarinense()` = 0x43bf50 (bra_reg_santa.cpp, thiscall, ret 0): Santa Catarina: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);
- `cqn8_mata_mata_do_paulista()` = 0x43cf80 (bra_reg_sp.cpp, thiscall, ret 0): Sao Paulo: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club]);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (39):** 39 caught.
- **Real game comparing to 2002-01-01:** deferred to the audit's accumulated comparison (decision of 5 Nov 2026).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 1dedd9dc, original 1e6682cb (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 276's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 277`

**Fields touched:**
- transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)
- transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)
- transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)
- transfer rules +0 vtable, +4 windows (6-byte dates, two per window), +8 number of windows, +9 the rule, +0xa nation, +0xe continent, +0x12, +0x13 (default 50) [inference: foreign limit], +0x14 (default 30), +0x15; pointer and value; writes (all)
- person (0x6e-byte records at [0xae23bc]) +0x6d European registration mark; value; writes 0 at stage 1 (inscricao_europeia)
- club +0x1db European competition; id; reads (inscricao_europeia); SQUAD_MANAGER +0 registration year; value; writes (inscricao_europeia)
- nation (0x122 bytes at [0xae23a8]) +0xa8 strength (double); value; reads (ranking_fifa_inicial); FIFA_RANKINGS +0 the table (0x2c bytes per nation: +0 index, +0xc float points, +0x10 position, +0x12 previous position, +0x14..+0x28 6 dwords of points); pointer and value; writes (ranking_fifa_inicial)
- stadium (0x4e bytes at [0xae23b0], count [0xae2358]) +4 name, +0x38 city, +0x3c capacity; city +0x1f nation; nation +0x71 continent, +0x80 national stadium, +0x11c has competitions; value and pointer; reads (lista_de_estadios_neutros)
- Stadium +0xc neutral venue list (16 bytes: nation, continent, national, stadium), +0x10 count, +0x14 capacity, +0x18 ready; pointer and value; writes (lista_de_estadios_neutros)
- club +0x57: national division (competition record); pointer; reads (all three)
- [0x9cf9ec] / [0x9cf9e0]: Series A / Series B id; id; reads
- league +4: the competition own record; pointer; reads (Series C)
- league +0x3e: number of teams in the table; value; writes (28, 28, 7)
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes
- Series C +0xee: the drawn list of entrants (56 pointers); pointer; frees and writes
- club +0x60: reserve division (ReserveDivision; in Brazil, the state league); pointer; reads
- [0x9cf9c0] / [0x9cf998] / [0x9cf9c8]: Bahia, Centre-West, Rio Grande do Sul ids; id; reads
- league +0x3e: number of teams in the table; value; writes
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes
- club +0x60: reserve division (ReserveDivision; in Brazil, the state league); pointer; reads
- [0x9cf9b4] / [0x9cf9d4] / [0x9cf994]: Goias, Minas Gerais, North ids; id; reads
- league +0x3e: number of teams in the table; value; writes
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes
- club +0x60: reserve division (ReserveDivision; in Brazil, the state league); pointer; reads
- [0x9cf9a4] / [0x9cf9d0] / [0x9cf9bc]: Northeast, Parana, Pernambuco ids; id; reads
- league +0x3e: number of teams in the table; value; writes
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes
- club +0x60: reserve division (ReserveDivision; in Brazil, the state league); pointer; reads
- [0x9cf9dc] / [0x9cf9b0] / [0x9cf9e4]: Rio, Santa Catarina, Sao Paulo ids; id; reads
- league +0x3e: number of teams in the table; value; writes
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes
- club +0x57: division (competition record); pointer; reads (all three) and writes 0 to the surplus ones (J1, J2)
- [0x9cf65c] / [0x9cf768] / [0x9cf89c]: J1, J2 and NSL ids; id; reads
- league +0x3e: number of teams in the table; value; writes (16, 12, 13)
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; writes
- league +0x28: prize fund; value; writes (J1)
- league +0xe1: table ready flag; value; writes 1 (J1)
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads
- league +0xc: the stages (pointer array); pointer; writes the first
- league +0x30: the current stage; value; writes 0
- league +0x40: the season year; value; reads
- cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)
- league +0xb1: the table (0x3b-byte rows, +0 the club); pointer; reads
- league +0xc: the stages (pointer array); pointer; writes the first
- league +0x30: the current stage; value; writes 0
- league +0x40: the season year; value; reads
- cup stage +0xa7: the team list (6 bytes, +0 the club); pointer; reads (comparator)

**Of interest to:** CMRTE, SGE
