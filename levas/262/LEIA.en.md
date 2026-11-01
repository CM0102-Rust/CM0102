# Batch 262 — the English divisions play-offs: the First, Second and Third play-off stages; the English divisions in the table: the records by status and the club progress in the First and Third stats; the league table: the possible positions and each club status, and the next stage of the First and Second; the Champions League: the records by stage reached, the quarter-final draw and the entered club list; the tactic statistics: each match result in the AI club's tactic row, the stored captain and the chosen captain; the player on international duty: the national side or all-star squad holding him on the date, the player eligible for the manager's national side and the duty competitions; the match and name records: the match as seen by one club, the person's name reference and the result code; the rescheduled fixtures: the club's neighbouring fixtures moved one day, the fixtures near the national team date and the fixture moved news; the player's poor fitness and the fine news: the fitness incident with the AI fine, the fine reaction and the coach feedback; the media outlets: the random pundit, the outlet name and the manager's FA complaint news; the wage bill and the season ticket window: the club wage bill, the bills rebuilt at creation and the days to the season start; the transfer money: the player sale, the player purchase and the signing-on fee; the income and the compensation: the prize money, the TV rights and the wages paid

**Routines:**
- `mata_mata_da_first()` = 0x573080 (eng_first.cpp, thiscall, ret 0): builds the play-off stage with the 6th, 3rd, 5th and 4th of the table (ties 6th v 3rd and 5th v 4th), the round table from virtual +0x3c and +0x3a = 0xa0; puts the stage in the first slot (+0xc) and sets up the round (0x51c800); out of memory, the error box (lines 0x2f0 and 0x306);
- `mata_mata_da_second()` = 0x576a70 (eng_second.cpp, thiscall, ret 0): like the First one (6th, 3rd, 5th and 4th), with +0x3a = the extra from virtual +0x3c (lines 0x2b6 and 0x2cc);
- `mata_mata_da_third()` = 0x5783c0 (eng_third.cpp, thiscall, ret 0): like the Second one, with the 7th, 4th, 6th and 5th (ties 7th v 4th and 6th v 5th; lines 0x2be and 0x2d5);
- `registros_pela_situacao_inglesa()` = 0x576c50 (eng_second.cpp, thiscall, ret 0x18): in the league (stage 0xff): title (5) 0x868c50, promotion (0) 0x869480 with weight 0x64, play-off place (1) 0x86bdd0 (0xa0, 0x82, 0x1e), relegation (3) 0x86a1c0; in the play-offs, on the club row: won (1) promotion with 0x32, +0x37 = 0 and removed = 1; went through (0) 0x86bdd0 (0xa0, 0x96, 0xf); out (0xff, 2) +0x37 = 0xfe; returns 0;
- `situacao_na_first()` = 0x573260 (eng_first.cpp, thiscall, ret 0x14): writes stage, best and worst in the club stats entry (+0x73..+0x75): in the league, best 3 to 6 becomes 3 and worst 3 to 6 becomes 6; in the play-offs (stage 0), 1, 2 and 3 become 3, 4 and 5;
- `situacao_na_third()` = 0x578660 (eng_third.cpp, thiscall, ret 0x14): like the First one, with places 4 to 7 (4 and 7; in the play-offs, 4, 5 and 6);
- `posicoes_e_situacao_na_liga()` = 0x684cd0 (league.cpp, thiscall, ret 0): sorts the table (0x6827d0), marks the bands (promotion, play-offs, 2 and relegation) from the top and the bottom, works out the best (+0x39) and worst (+0x3a) possible position from the maximum points and decides the status (+0x37): title (5), promotion (0, 7 or 6), play-offs (1), relegation (3, with the clauses 0x4cd030), 2 and 0xfe, each announced (+0x40); the European place ([0xae2d14]) from the nation places (0x905c60); the stats progress (+0x68);
- `fase_seguinte_da_first()` = 0x572ff0 (eng_first.cpp, thiscall, ret 0): at the last stage (+0x30 >= +0x2c - 1) the error box (line 0x2ce); otherwise +0x30 + 1 and, on reaching 0, the play-offs (0x573080);
- `fase_seguinte_da_second()` = 0x5769e0 (eng_second.cpp, thiscall, ret 0): like the First one (line 0x294; the play-offs 0x576a70);
- `registros_da_liga_dos_campeoes()` = 0x584ae0 (european_cup.cpp, thiscall, ret 0x18): by outcome and stage: in qualifying, the qualified (0x86bdd0) or knocked-out (0x86c000) record weighted by round (0x4bf850); in the group stages, qualified (6) or failed to qualify (0xfe, 0x86c1d0); in the knockout, winners (0x86a800), runners-up (0x86b0b0), qualified or knocked out; whoever leaves gets +0x1db = -1; returns 0;
- `sorteio_das_quartas()` = 0x586330 (european_cup.cpp, thiscall, ret 0): draws the order of the 4 phase 2 groups (0x90cfc0) and pairs the quarter-finals (1st of a v 2nd of b, 2nd of a v 1st of b, same for c and d), resets the 8 clubs bookings (0x7e9180), creates the knockout stage (0x522e00) in slot +0x30, drops the 3rd and 4th of each group (+0x1db = -1), sets up the round (0x51c800) and +0x30 = 12;
- `clubes_da_liga_dos_campeoes()` = 0x586660 (european_cup.cpp, thiscall, ret 8): allocates the list (0x11c bytes) of the entrants (+0x14) still in the cup (+0x1db = its id): kind 1 without the round 7 entries, kind 2 all, kind 3 at stage 11; otherwise the competition one (0x48e1c0); returns the count;
- `registrar_resultado_da_tatica()` = 0x896290 (tactics.cpp, thiscall, ret 4): at the end of a match with no human, adds to each club's tactic row (0x28 bytes) the games, wins, draws, losses, goals and points (3 per win, 1 per draw) and redoes the three double ratios (+0x10/+0x18/+0x20) that the AI formation choice reads;
- `capitao_guardado()` = 0x8976a0 (tactics.cpp, thiscall, ret 8): the captain stored (+0x52) in the club and team row of the tactics table; no row, 0;
- `escolher_capitao()` = 0x897880 (tactics.cpp, thiscall, ret 0xc): writes the captain (+0x52) in the club and team row of the tactics table and returns 1; no row, 0;
- `selecao_que_tem_o_jogador()` = 0x7757d0 (national_teams.cpp, thiscall, ret 0x14): the national side holding the player from the day before to the day after (a national side fixture in the window, against the given opponent if any, or a call-up already started) or the MLS all-star squad on the game date; writes the comment; otherwise 0 (and 0 for a national side or all-star club);
- `jogador_serve_a_selecao()` = 0x76d9b0 (national_teams.cpp, stdcall, ret 8): the player may play for the manager's national side: nationality, under-21 age, current call-up and the second-nationality rule;
- `juntar_competicoes_das_convocacoes()` = 0x775580 (national_teams.cpp, thiscall, ret 4): adds to a list each duty competition it does not yet hold and returns how many;
- `registro_da_partida_pelo_clube()` = 0x7e4750 (record_utils.cpp, cdecl): fills the match record as seen by the club: date, fixture id, competition, venue, club, opponent and goals for and against (the extra-time score, if any);
- `referencia_ao_nome()` = 0x7e4920 (record_utils.cpp, cdecl): stores the three name ids (-1 when missing), the person's id and birth date in a compact reference;
- `codigo_do_resultado()` = 0x7e56d0 (record_utils.cpp, cdecl): the result code of the record: 3 win, 2 score draw, 1 goalless draw, -3 defeat;
- `mover_jogos_vizinhos_do_clube()` = 0x5ae9d0 (fix_man.cpp, thiscall, ret 4): non-friendly fixture: for each club of a flagged nation (+0x11c), its fixture two days before (with +0x43 = 0xff) moves one day earlier, and the one two days after moves one day later, if the new date is free (0x5ad780); removes, adds and posts the news;
- `remarcar_jogos_perto_da_data()` = 0x5abb10 (fix_man.cpp, thiscall, ret 8): the club's fixtures from one day before to one day after the national team date: those removed from the calendar get a new date, are added back and post the news with the old date;
- `noticia_do_jogo_remarcado()` = 0x5aac70 (fix_man.cpp, thiscall, ret 0xc): the news queue at +0x1128 (created the first time): updates the new date of the same fixture's news, or queues news 1 with the fixture, the cause or the old date;
- `incidente_de_forma()` = 0x41add0 (awol.cpp, thiscall, ret 8): the record (player, club, today) in the incident list; human club: a random coach and news 0x138f; AI club with a manager: draw(+0x14) above draw(10) leads to the fine, period by the manager's rating;
- `noticia_reacao_a_multa()` = 0x5a74b0 (Fine.cpp, stdcall, ret 0x14): player under contract at a human club: news 0xbc6 with reason, reaction, draw(2), personality, period, contract type, future transfer and effects;
- `noticia_retorno_do_treinador()` = 0x5a7f20 (Fine.cpp, stdcall, ret 0xc): player and coach with a record and nation, player with a club: news 0x1394 with the outcome and draw(5);
- `sortear_comentarista()` = 0x7dab90 (printouts.cpp, thiscall, ret 0xc): type 1 on Sunday becomes 7; local types (2, 4): the club's pundits; the others: of the type and nation (−1 any); up to 20, draw(n) and the id, or −1;
- `nome_do_veiculo()` = 0x7dad20 (printouts.cpp, thiscall, ret 0x10): the name (+2) and flag (+0xd3) of the pundit with the id, or the default outlet type name and flag 0;
- `noticia_queixa_a_federacao()` = 0x758de0 (media.cpp, thiscall, ret 0x18): club with a nation and manager: news 0x178e (club, manager, type, person, a, b, nation, c) to the nation's managers if the reputation is above 4750, else to the club;
- `folha_salarial_do_clube()` = 0x598370 (finance.cpp, thiscall): sums the squad wages (less the other club's share for loanees) and the staff wages into the wage bill (+0xc) and the highest wage (+0x10); with fewer than 15 players, the missing places at a wage by reputation; plus reputation² × a factor by club type and nation; the wage ceiling (+0x14) by 0x5968d0;
- `refazer_todas_as_folhas()` = 0x59cf50 (finance.cpp, thiscall): at game creation, the wage bill of every club up to the first national team;
- `dias_ate_a_temporada()` = 0x59b4c0 (finance.cpp, thiscall, ret 8 (o ecx não é lido)): the days until the season start of the club's nation, inside the season ticket window (up to two months before); flags whether the season year has come;
- `receber_venda_de_jogador()` = 0x599800 (finance.cpp, thiscall, ret 8): adds the amount (64 bits) to the sales of the month (+0x78) and of the year (+0x118) and to the balance;
- `pagar_compra_de_jogador()` = 0x599890 (finance.cpp, thiscall, ret 8): adds the amount (64 bits) to the purchases of the month (+0x70) and of the year (+0x110) and takes it from the balance;
- `pagar_luvas()` = 0x5998e0 (finance.cpp, thiscall, ret 4): adds the amount to the signing-on fees of the month (+0x9c) and of the year (+0x13c) and takes it from the balance;
- `receber_premio_em_dinheiro()` = 0x5999a0 (finance.cpp, thiscall, ret 4): adds the amount to the prize money of the month (+0xa8) and of the year (+0x148) and to the balance;
- `receber_direitos_de_tv()` = 0x5987c0 (finance.cpp, thiscall, ret 4): adds the amount to the TV income of the month (+0xb8) and of the year (+0x158) and to the balance;
- `pagar_salarios()` = 0x599920 (finance.cpp, thiscall, ret 4): adds the amount to the wages of the month (+0x90) and of the year (+0x130) and takes it from the balance (the free release compensation);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (34):** 31 caught. `cpz2_acesso_pesa_metade` escaped: the weight only goes into the staff records, which live outside the database and which the career only reads for the manager of the month and of the year; no choice changed by 2002-06-01 [inference]. `cpz2_vaga_no_mata_mata_pesa_o_dobro` escaped: the weight only goes into the staff records, which live outside the database and which the career only reads for the manager of the month and of the year; no choice changed by 2002-06-01 [inference]. `cpz4_fase_2_pesa_metade` escaped: the weight of getting through Champions League phase 2 only goes into the staff records, which the career only reads for the manager of the month and of the year [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (81f1a1ed).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6299f556, original cbf229f9 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 261's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 262`

**Fields touched:**
- league +0xb1: the table (0x3b-byte rows, club at +0); pointer; reads (mata_mata_da_first, mata_mata_da_second, mata_mata_da_third)
- league +0xbf: how many go to the play-offs; value; reads (all three)
- league +0xc: the stage slots; pointer; writes the first one (all three)
- league +4 / +0x40: the competition record and the word passed to the stage; pointer and value; reads (all three)
- stage (Cup_Stage) +0xa7 / +0x3e: the club list (6 bytes) and the count; pointer and value; written by the constructor (all three)
- league +0xb1 / +0x3e: the table (0x3b bytes; +0 club, +0x37 status) and the row count; pointer and value; reads, writes +0x37 (registros_pela_situacao_inglesa)
- league +4: the competition record; pointer; reads (registros_pela_situacao_inglesa)
- league +8: the competition stats (Comp_Stats); pointer; reads (situacao_na_first, situacao_na_third)
- club stats entry +0x73 / +0x74 / +0x75: the stage, best and worst progress; value; writes (situacao_na_first, situacao_na_third)
- [0xdd7f00] the staff records; pointer; reads (registros_pela_situacao_inglesa)
- league +0xb1 / +0x3e: the table (0x3b bytes: +0 club, +0x11 games played, +0x19 points, +0x37 status, +0x39 best position, +0x3a worst) and the rows; pointer and value; reads, writes +0x37/+0x39/+0x3a (posicoes_e_situacao_na_liga)
- league +0xa7 / +0x3c / +0x45: the season games (written if -1), the rounds per pair and the schedule; value; reads and writes (posicoes_e_situacao_na_liga)
- league +0xbe / +0xbf / +0xc0 / +0xc1: the promotion, play-off, band 2 and relegation places; value; reads (posicoes_e_situacao_na_liga)
- league +0xc2 / +0xd9 / +0xdd / +0xe6 / +0x44 / +0x1c: points per win, table rules, league halted, tie-break, parent stage, promotion league; value; reads (posicoes_e_situacao_na_liga)
- [0xae2d14] + 6 x club id: next season's European competition; id; reads and writes (posicoes_e_situacao_na_liga)
- league +0x30 / +0x2c: the current stage and the stage count; value; reads and writes (fase_seguinte_da_first, fase_seguinte_da_second)
- club +0x1db: the season's European competition; id; reads (clubes_da_liga_dos_campeoes), writes -1 (registros_da_liga_dos_campeoes, sorteio_das_quartas)
- fixture +0x30 / +0x32 / +0x34: the round words passed to the records; value; reads (registros_da_liga_dos_campeoes)
- Champions League +0xc / +0x30: the stages (+0x20..+0x2c the phase 2 groups, +0x30 the knockout) and the current stage; pointer and value; reads, writes slot +0x30 and stage 12 (sorteio_das_quartas)
- Champions League +0x14 / +0x38: the entrant list (6 bytes: +0 club, +4 entry round) and the count; pointer and value; reads (clubes_da_liga_dos_campeoes)
- Champions League +0x3a / +0xa3 / +0x50: the qualifying stage, the round table and the ruling body index; value and pointer; reads (registros_da_liga_dos_campeoes, sorteio_das_quartas)
- [0xde1d08] +0: the tactic statistics table (0x28 bytes: +0 the tactic, +2 games, +4 wins, +6 draws, +8 losses, +0xa goals for, +0xc against, +0xe points, +0x10/+0x18/+0x20 the double ratios); pointer and value; reads and writes (registrar_resultado_da_tatica)
- [0xde1d08] +0x601: the clubs' tactics table (0x91 bytes a row: +0 the tactic, +0x52 the captain); pointer, value and id; reads (registrar_resultado_da_tatica, capitao_guardado), writes the captain (escolher_capitao)
- fixture +0x1c / +0x20: the home and away clubs; pointer; reads (registrar_resultado_da_tatica)
- match record +0x19 / +0x1a: goals for and against by the club (0x7e4750); value; reads (registrar_resultado_da_tatica)
- staff +0x24: the national side that called him up; pointer; reads (selecao_que_tem_o_jogador, jogador_serve_a_selecao)
- staff +0x5e bit 4: the side in national fixtures and whether call-ups count; value; reads (selecao_que_tem_o_jogador)
- staff +0x39: the club; pointer; reads (selecao_que_tem_o_jogador)
- staff +0x1a / +0x1e / +0x22 / +0x18 / +0x61: nationalities, the single-nationality flag, the age and the player record; pointer and value; reads (jogador_serve_a_selecao)
- [0xae2d0c] person's 79-byte record +0xc bit 2: first nationality only; value; reads (jogador_serve_a_selecao)
- club +0x57: the league; pointer; reads (selecao_que_tem_o_jogador); club +0xd7: the 50 all-star squad names; pointer; reads
- NationalTeams +0: the duty list (+0 the national side, +4 the opponent/competition, +8 the date); pointer; reads (selecao_que_tem_o_jogador, juntar_competicoes_das_convocacoes)
- [0x9d00a4] / [0x9d00a8] the two all-star squads; id; reads. [0x9cf590] the MLS league; id; reads. [0x9cf434] the nation of the second-nationality rule; id; reads
- fixture +0 / +8 / +0xc / +0x10 / +0x28 / +0x2a / +0x30 / +0x32: id, competition, the two opponents, year, day and two words; id, pointer and value; reads (registro_da_partida_pelo_clube)
- fixture +0x43 / +0x44 / +0x47 / +0x48: the normal and the extra-time score (-1 when none); value; reads (registro_da_partida_pelo_clube)
- match record (0x1f bytes): +0 date, +8 fixture, +0xc/+0xe, +0x10 venue, +0x11 club, +0x15 opponent, +0x19 for, +0x1a against, +0x1b competition; value and id; writes (registro_da_partida_pelo_clube), reads the score (codigo_do_resultado)
- staff +4 / +8 / +0xc: the three names (name record +0x33: the id); pointer; reads (referencia_ao_nome). staff +0x10: birth date; value; reads
- fixture (0x4f bytes) +0 id, +0xc, +0x10, +0x14 competition, +0x1c and +0x20 clubs, +0x28 year and +0x2a day, +0x30 and +0x32, +0x40 and +0x41, +0x43; id and value; reads and writes the date in the local copy (mover_jogos_vizinhos_do_clube, noticia_do_jogo_remarcado)
- club +0x53 nation; nation +0x11c rescheduling flag; pointer and value; reads (mover_jogos_vizinhos_do_clube)
- FIXTURE_MANAGER +0x1128: queue of fixture moved news (LLIST of 0xdf-byte items); pointer; creates and writes (noticia_do_jogo_remarcado)
- [0xae2c74] friendlies object; pointer; reads (mover_jogos_vizinhos_do_clube)
- AWOL +8: fitness incident list (0x11-byte record: +0 player, +4 club, +8 date, +0x10 byte); id and value; writes (incidente_de_forma)
- club +0x19f: the 5 coach slots; +0xcf manager; manager +0x69 record (+0x14, +0x19); pointer and value; reads (incidente_de_forma)
- person +0x18 age, +0x1a nation, +0x39 club, +0x59, +0x5b, +0x5d personality, +0x61 record (+0xb reputation); value; reads (the fine news)
- contract +0x4f type (high nibble); [0xae1080] last contract; value and pointer; reads and writes [0xae1080] (noticia_reacao_a_multa)
- [0xae2a3c] the fines object (Fine); pointer; reads (incidente_de_forma)
- PunditList +0 national list, +4 local list (LLIST of 0xd4-byte records: +0 id, +2 name, +0xca type, +0xcb club, +0xcf nation, +0xd3 flag); id and value; reads (sortear_comentarista, nome_do_veiculo)
- club +0x69 stadium (+0x38 the stadium's club), +0x53 nation, +0x80 reputation, +0xcf manager; pointer and value; reads (sortear_comentarista, noticia_queixa_a_federacao)
- [0xae2c90] today (the Sunday); [0xb74430] the pundit list; value and pointer; reads (sortear_comentarista)
- club finance (0x167 bytes) +8 club id, +0xc weekly wage bill, +0x10 highest wage, +0x14 wage ceiling; values; writes (folha_salarial_do_clube); the whole table (refazer_todas_as_folhas)
- club +0xd7 50 squad places, +0x19f/+0x1b3/+0x1cf staff (5, 7, 3), +0x53 nation, +0x64 type, +0x80 reputation; pointers/values; reads (folha_salarial_do_clube)
- contract (0x50 bytes) +0xc weekly wage (−1: default), +0x35 & 0x3f type; values; reads (folha_salarial_do_clube)
- nation +0 id (England 0x9cf2e4, Italy 0x9cf360), +0x7e type; values; reads (folha_salarial_do_clube)
- [0xae1080] last contract looked up; pointer; writes (through the default wage 0x864550)
- nation row (72 bytes at 0xb63d60) +0x17 2002 flag, +0x2a season start date, +0x2c start year, +0x34 start month; values; reads (dias_ate_a_temporada)
- [0xdd6ec8] game start date (year at +2), [0xae2c90] today; values; reads (dias_ate_a_temporada)
- club finance (0x167 bytes) +0 balance (i64), +0x70/+0x110 purchases (i64, month/year), +0x78/+0x118 sales (i64, month/year), +0x9c/+0x13c signing-on fees (month/year); values; reads and writes
- club finance (0x167 bytes) +0 balance (i64), +0x90/+0x130 wages, +0xa8/+0x148 prize money, +0xb8/+0x158 TV (month/year); values; reads and writes

**Of interest to:** CMRTE, SGE
