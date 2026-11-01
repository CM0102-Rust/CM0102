# Batch 259 — a player's satisfaction with his contract, the news that he wants a new contract and of a withdrawn transfer request; the clubs' final approval of an offer, the completion of an approved offer and the settling of earlier clauses; the loan offer for a shortlisted player, the news of an expiring contract and of a Bosman departure; the contract offer added or updated, the contract offer removed by transfer slot and the lowest wage demand; the month ratings: the match ratings, the player rating of the month and the best of the month; the month award: the best of the ratings, the winner slot and the reputation of the winner; the player's daily fitness: fitness and match condition up to today; the board's league confidence band and the board ratings; the injury day: ageing injuries and training injuries; the return-to-training news and the player one booking away from a ban; the regen queue: youth regens that fall due and rebuilt reserve coaches; the news outlet by club and by person; the league table: adding the result to the table, copying the table and the predicted positions; the international match: call-ups, caps and international goals, the squad label and the known rounds; the call-ups per club on the national side's match date, the match importance and the competition stage

**Routines:**
- `satisfacao_com_o_contrato()` = 0x866620 (staff_contracts.cpp, thiscall): 1 to 3: 100 × the value of the current contract / that of the desired one (with the deserved squad status), with an allowance for recent signings; below 50: 1; below 80: 3; otherwise 2;
- `noticia_quer_contrato_novo()` = 0x4d9f80 (contract_manager.cpp, thiscall): the 0xbc2 news that the player wants a new contract, to the club manager, unless he is transfer listed or in an offer;
- `noticia_pedido_de_transferencia_retirado()` = 0x4d9460 (contract_manager.cpp, thiscall): the 0xbc0 news of a withdrawn transfer request, with the reason, to the club manager;
- `confirmar_oferta_pelo_clube()` = 0x8c3310 (transfer_manager.cpp, thiscall): records one club's approval (states 0xb to 0xe); once both agree, schedules completion for today or, with the deadline still ahead, books a future transfer;
- `tentar_concluir_oferta()` = 0x8c34e0 (transfer_manager.cpp, thiscall): rechecks buyer and seller; a refusal collapses or rejects the offer; with both agreeing, completes the transfer or defers it to the end of the contract;
- `acertar_clausulas_anteriores()` = 0x8c91e0 (transfer_manager.cpp, thiscall): pays and clears the old clauses (next sale percentage, club and international appearance money) of the player and the exchange players;
- `tentar_emprestimo_pelo_jogador()` = 0x842740 (shortlist_manager.cpp, thiscall): with an interest of 3.5, the loan period and a return date within the contract, the window and 3 weeks from today, makes the offer with 0, 50 or 100% of the wage by the reputation gap; otherwise −1;
- `noticia_contrato_acabando()` = 0x4d6790 (contract_manager.cpp, thiscall): the 0xbbd news of an expiring contract to the club manager and, if asked, to interested clubs of active nations;
- `noticia_saida_pela_lei_bosman()` = 0x4d6d70 (contract_manager.cpp, thiscall): the 0xbbe news of a Bosman departure, with the destination club or −1;
- `acrescentar_oferta_de_contrato()` = 0x4cc490 (contract_manager.cpp, thiscall): updates the same club's offer to the person or adds a new one to the list (the offer count goes up); marks the terms as offered on the transfer offer and, if asked, schedules the decision;
- `tirar_oferta_de_contrato_da_vaga()` = 0x4ccb30 (contract_manager.cpp, thiscall): removes the person's contract offer linked to the transfer slot from the list (clearing the agreed-contract flag) and lowers the offer count;
- `menor_salario_pedido()` = 0x4ce450 (contract_manager.cpp, stdcall): the lower of the desired wages for squad status 1 and 2 at the club, at most 10 million;
- `guardar_notas_da_partida()` = 0x767ee0 (month_ratings.cpp, thiscall, ret 8): both sides of the match, 20 slots each: except the empty slot (0xfb) and the substitute who did not come on, each person rating goes to the month ratings;
- `nota_do_jogador_no_mes()` = 0x767fa0 (month_ratings.cpp, thiscall, ret 4): finds or appends the 12-byte record of the person (grows by 300 when full; the club league and the age), adds the rating and counts one match;
- `melhor_do_mes()` = 0x768130 (month_ratings.cpp, thiscall, ret 8): sorts by average and returns the first of the league with 3 matches (then 2, 1, 0), with the age filter and the current club league;
- `premiar_o_melhor_do_mes()` = 0x767150 (month_award.cpp, thiscall, ret 8): the best of the month ratings (0x768130) in the award league and with the requested age becomes the month winner, with his club;
- `gravar_vencedor_do_mes()` = 0x7671c0 (month_award.cpp, thiscall, ret 0x10): finds the month slot, stores the person and the club, raises the winner reputation, sends the news and records the award in the history;
- `reputacao_do_premiado()` = 0x412f00 (award_manager.cpp, cdecl): player: the world, home and current reputations rise by a fraction of the award ones, up to 10000 and at least them; non-player: they rise up to them and the board confidence gains 1000 (between 6500 and 10000);
- `fisico_do_dia()` = 0x62c680 (injury.cpp, thiscall, ret 4): once a day: the days since the last update become free days, recovery days, stopped days (injury, ending it through 0x62e700) or holiday days (club out of season, 0x669bd0); fitness (+8) recovers 1000 − 12 × (+0x1e − natural) per free day, loses 66 per stopped day and 55 per holiday day, capped by natural fitness and with shrunk bands above 7000; match condition (+0xa) rises with free and recovery days; stores today's date;
- `faixa_da_confianca_na_liga()` = 0x690bd0 (manager_manager.cpp, thiscall, ret 4): the board's league confidence (+0x12 of the club's 0x49-byte record) in bands 0 to 6 (−2000, −1000, −500, 500, 1000, 2000);
- `gravar_notas_da_diretoria()` = 0x69f7c0 (manager_manager.cpp, thiscall, ret 8): the four board ratings (+6, +8, +0xc, +0xa of the club's 0x49-byte record) get the value;
- `dia_das_lesoes()` = 0x62b7b0 (injury.cpp, thiscall, ret 0): frees the match knock list, lowers both counters of every injury, on day 180 of the first season the fitness of everyone; on weekdays [0xae2364] / 150 draws (at least 10) of a person: a present, uninjured player who passes the risk draws (player data +0x2d, proneness, recurrence) gets the category, the type (0x62d140), the attribute wear and the new injury (0x62d250); at the end the return news (0x62e010) and the outbreaks (0x62e9e0);
- `noticias_de_volta_aos_treinos()` = 0x62e010 (injury.cpp, thiscall, ret 0): at the human managers' clubs, the light training return news (once, +0x1c) and the daily fitness of every injured player; in the return queue, the ones with date A today and date B today get the news (0x630ee0) and the ones with past date B leave the front;
- `um_cartao_da_suspensao()` = 0x55d360 (discipline.cpp, thiscall, ret 8): with a valid id, the question to the competition's ruling body (+0x50; 0x7e8d60 with the id and the competition's +4); otherwise 0;
- `fila_de_regens()` = 0x7a65d0 (player_regen.cpp, thiscall, ret 0): from the front of the youth regen queue (12 bytes: id and date), while the date is before today, the youth regen (0x7abac0) and the entry leaves; with more than 2 reserve coaches, rebuilds count − 2 of them: the person lending the data (0x7a6e60) gets the non-player record, the job at the AI club and the ratings from reputation (pow and bands) or drawn; the reserve leaves (0x7a82b0);
- `veiculo_pelo_clube()` = 0x757f30 (media.cpp, cdecl): club reputation above 6250: outlet 0 to 6 from random(100); otherwise 2 or 4 from random(4);
- `veiculo_pela_pessoa()` = 0x757fa0 (media.cpp, cdecl): the same from the player's reputation (player data +0xb);
- `somar_resultado_na_liga()` = 0x685d30 (league.cpp, thiscall, ret 8): stamps today's date on the league; a stage fixture goes to its stage; otherwise adds the fixture to the table (0x685f60) and to the competition stats (0x49f570) and returns 1;
- `copiar_tabela_da_liga()` = 0x687710 (league.cpp, thiscall, ret 0): allocates and returns a copy of the table rows, with new head-to-head and form buffers; sets +0xa7 when it is -1;
- `posicoes_previstas_da_liga()` = 0x68a850 (league.cpp, thiscall, ret 0): sorts the table's clubs by strength and stores each one's position in the competition stats;
- `resultado_entre_selecoes()` = 0x76b600 (national_teams.cpp, thiscall, ret 8): between two national sides: counts each nation's match; by competition, marks the general record, swaps the nationality or, in real internationals, scores the FIFA ranking and adds caps and goals to everyone who played; clears the type's +0x44 on its date and sends the news to the clubs;
- `nome_do_elenco_da_selecao()` = 0x775610 (national_teams.cpp, thiscall, ret 0x14): writes the squad name (Main, B, Under 21 or Under 23), in lower case when asked;
- `rodada_com_jogos_conhecidos()` = 0x48d410 (comp.cpp, thiscall, ret 8): the stage is a league, or the round is before the current one, or is the current last round with code 0x8c;
- `registrar_convocados_por_clube()` = 0x7771c0 (national_teams.cpp, thiscall, ret 0xc): sorts the national squad by club and adds, on the match date, how many players each club gives up (the record that stops club fixtures near that date);
- `importancia_do_jogo_na_competicao()` = 0x48e190 (comp.cpp, thiscall, ret 0xc): forwards the match importance query to the fixture's stage (virtual +0x54);
- `fase_da_competicao()` = 0x48d200 (comp.cpp, thiscall, ret 4): returns stage k of the competition (-1 = itself); out of range, the error box and 0;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (35):** 35 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (954d8f4e).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 4a762e34, original 086f15e7 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 258's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 259`

**Fields touched:**
- contract +4: club; id; reads (all three)
- contract +0x25 day, +0x27 year: contract start; value; reads (satisfacao_com_o_contrato)
- contract +0x4f >> 4: squad status; value; reads (satisfacao_com_o_contrato, noticia_quer_contrato_novo)
- contract +0xc wage, +0x2d/+0x2f end, +0x3f; value; reads (the news)
- person +0x59: adaptability; value; reads (satisfacao_com_o_contrato, noticia_quer_contrato_novo)
- person +0x5b; player record +0xd: reputation; value; reads (noticia_quer_contrato_novo)
- [0xae2c90] today; value; reads (satisfacao_com_o_contrato)
- offer +4: list of exchange players; pointer; reads (all three)
- offer +8: buying club; +0xc: player; id; reads (all three)
- offer +0x2c: state (0xb to 0xe on approval); value; reads and writes (confirmar_oferta_pelo_clube)
- offer +0x2f: confirmation; +0x30 & 0xf: deadline type (3 = end of contract); value; reads (confirmar_oferta_pelo_clube, tentar_concluir_oferta)
- contract +4: club; id; +0x35 & 0x3f: type (5 without club); +0x2d: end; reads (tentar_concluir_oferta)
- [this+0x86e] 8-byte flags per person (bits 2, 4, 8: clauses); value; reads (acertar_clausulas_anteriores)
- [0xae1080] the last contract asked for; pointer; writes (tentar_concluir_oferta)
- contract +4: club (owner); id; reads (all three)
- contract +0x2d: contract end; value; reads (tentar_emprestimo_pelo_jogador)
- shortlist slot +0x24 >> 1 & 0xf: squad status; value; reads (tentar_emprestimo_pelo_jogador)
- club +0x53: nation; pointer; +0x80: reputation; value; reads (tentar_emprestimo_pelo_jogador, noticia_contrato_acabando)
- [0xb63d8a] + 72 × key nation: window start; value; reads (tentar_emprestimo_pelo_jogador)
- person +0x61: player record; pointer; reads (noticia_contrato_acabando)
- [0xae1080] the last contract asked for; pointer; writes (tentar_emprestimo_pelo_jogador)
- [0xae2d0c] + 79 × id +8: person's contract offer count (general.dat); value; reads and writes (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)
- CONTRACT_MANAGER +0x34: list of contract offers (0x6f-byte records); pointer; reads and writes (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)
- contract offer +0..+0x50: the terms (STAFF_CONTRACT); +0x50: 2 agreed; +0x51: date; +0x59: transfer slot; +0x5d: value; +0x61: flag; +0x62: not type 6; +0x63..+0x6e: the 0x7a81a0 request; value; writes and reads
- transfer offer +0xc: player; id; reads (acrescentar_oferta_de_contrato)
- [0xae1080] the last contract asked for; pointer; writes (menor_salario_pedido, via 0x85fc50)
- month ratings (MONTH_RATINGS) +0 table, +4 count, +8 capacity, +0xc month, +0xd year; pointer/values; reads and writes (nota_do_jogador_no_mes, melhor_do_mes sorts)
- rating record (12 bytes) +0 person, +4 league, +8 rating sum, +0xa matches, +0xb age; id/values; writes (nota_do_jogador_no_mes), reads (melhor_do_mes)
- match slot (0x49 bytes, 20 per side from +0x190, sides 0x625 apart) +0 person, +0x1d 0xfb flag, +0x32 came on (0xff no), +0x35 rating; id/values; reads (guardar_notas_da_partida, nota_do_jogador_no_mes)
- staff +0x18 age, +0x39 club; club +0x57 league; values/pointers; reads (nota_do_jogador_no_mes, melhor_do_mes)
- [0xae2364] staff count; value; reads (guardar_notas_da_partida)
- month award (MONTH_AWARD) +5 award, +9 league, +0x15 home reputation, +0x17 world, +0x19 player flag, 12 slots of 11 bytes (+0x1d person, +0x21 club, +0x25 month, +0x26 year); pointers/ids/values; reads and writes the slots (gravar_vencedor_do_mes)
- player (staff +0x61) +9 home reputation, +0xb current, +0xd world; values; reads and writes (reputacao_do_premiado)
- non-player (staff +0x69) +8, +0xa, +0xc reputations; values; reads and writes (reputacao_do_premiado)
- staff +0x1a nation, +0x39 club; club +0x53 nation; ids/pointers; reads (reputacao_do_premiado, premiar_o_melhor_do_mes)
- [[0xb67a44]] 73-byte board rows per club, +6 confidence; value; reads and writes (reputacao_do_premiado)
- [[0xde1f54] + 0x856] club shortlist manager; pointer; reads (reputacao_do_premiado)
- Injury +0: 31-byte record per person: +0 last update date (8 bytes); +8 fitness; +0xa match condition; +0xc fatigue; +0xe current injury index (−1 none); +0x1e proneness; value and index; reads and writes (fisico_do_dia)
- Injury +4: 0x30-byte injury table (+0x20 start, +0x28 end, +0x2a stopped days); +8 count; value; reads (fisico_do_dia)
- [0xae2d0c] 79-byte person record +0xa: holiday days (at most 50); value; reads and writes (fisico_do_dia)
- person +0x24 national team, +0x39 club, +0x5b, +0x5e bit 3; player data +0x34 natural fitness; pointer and value; reads (fisico_do_dia)
- [0xb63d08..0xb63d34] fitness statics: dates, guard 0xb63d10, cached club 0xb63d2c and out-of-season flag 0xb63d30; value; reads and writes (fisico_do_dia)
- MANAGER_MANAGER +0: 0x49-byte record per club: +6, +8, +0xa, +0xc board ratings; +0x12 league confidence; value; reads (faixa_da_confianca_na_liga) and writes (gravar_notas_da_diretoria)
- Injury +0x28: match knock list; pointer; frees and writes 0 (dia_das_lesoes)
- Injury +4: 0x30-byte injury table (+0 type, +0x20 start, +0x28 end, +0x2a stopped days, +0x2c and +0x2e counters); +8 count; value; reads and writes (dia_das_lesoes, noticias_de_volta_aos_treinos)
- Injury +0: 31-byte record (+0xe current injury, +0x12 recurrence category, +0x13 its weight, +0x1c return news given, +0x1e proneness); index and value; reads, writes +0x1c (dia_das_lesoes, noticias_de_volta_aos_treinos)
- Injury +0x10: return queue (22 bytes: +0 date A, +8 date B, +0x10 id, +0x14 and +0x15), count +0x18; value; reads, removes from the front (noticias_de_volta_aos_treinos)
- player data +0xf rating, +0x2d, +0x1b, +0x1d, +0x36 (worn by the injury); person +0x3d job; value; reads and writes (dia_das_lesoes)
- [0x9e7ad9] injury type table (13 bytes: category); [0x9e7f05] category ranges (3 bytes); value; reads (dia_das_lesoes)
- competition +0x50 ruling body (0xff none), +4; Discipline +0 ruling bodies; pointer and value; reads (um_cartao_da_suspensao)
- PLAYER_REGEN +0x10: youth regen queue (12 bytes: +0 id, +4 date), +0x14 count, +0x40 switch; id and value; reads, removes from the front (fila_de_regens)
- PLAYER_REGEN +0x1c: reserve coaches (+0 id, +4 nation), +0x20 count; id; reads (fila_de_regens)
- person +0x69 non-player record ([0xae23c0] + 0x44 × id: +8, +0xa, +0xc ratings, +0x10/+0x11); +0x3d job; +0x22; +0x57; pointer and value; writes (fila_de_regens)
- club +0x19f: the 5 coach slots; +0xd3 assistant; +0x80 reputation; pointer and value; reads and writes (fila_de_regens), reads reputation (veiculo_pelo_clube)
- player data +0xb reputation, +9, +0xd, +0xf; value; reads (fila_de_regens, veiculo_pela_pessoa)
- league +0xc9: the last update date; value (8-byte date); writes (somar_resultado_na_liga)
- league +0x44: the league stage (0xff with stages, 0xfe without stats); value; reads (somar_resultado_na_liga)
- league +0xc / +0x30: the stages and the last one; pointer and value; reads (somar_resultado_na_liga)
- league +0xb1 / +0x3e: the table (0x3b-byte rows: +0 club, +5 head to head, +9 form) and the count; pointer and value; reads (all three)
- league +0xa7: the form size (-1 = compute: (rows - 1) x +0x3c); value; reads and writes (copiar_tabela_da_liga)
- league +8: the competition stats; pointer; reads (posicoes_previstas_da_liga); club entry +0x72: the position; value; writes (via 0x4a2540)
- fixture +0x42: the fixture's stage; value; reads (somar_resultado_na_liga)
- [0xadadfc] the competition table; pointer; reads (somar_resultado_na_liga)
- person ([0xae23bc] + 0x6e x id) +0x22: international caps; value; reads and writes +1 (resultado_entre_selecoes)
- person +0x23: international goals; value; reads and adds the slot goals (+0x21) (resultado_entre_selecoes)
- person +0x1a / +0x1e: nationality and second nationality; pointer; reads and swaps (resultado_entre_selecoes)
- general record ([0xae2d0c] + 0x4f x id) +0xc bit 2 and +0xd; value; writes (resultado_entre_selecoes)
- nation record in the national teams ([NationalTeams+4] + 0x6e x id) +2: the nation's match count; value; writes +1; +0x3a + 20 x type the type date; reads; +0x44 + 20 x type; value; writes 0 (resultado_entre_selecoes)
- match summary + 0x625 x side + 0x190 + 0x49 x slot: +0 person id, +0x21 goals, +0x32 came on (sub); +0x153 the star slot; value; reads (resultado_entre_selecoes)
- competition +0x30 / +0xc / +0x43 / +0xab / +0x3c / +0xa3: stages, type, current round, rounds and 0x68-byte round records (+7 the code); reads (rodada_com_jogos_conhecidos)
- national side +0xd7: the squad (50 slots); pointer; reads and reorders by club (registrar_convocados_por_clube)
- staff +0x39: the club; pointer; reads; +0x5e bits 2 and 3: B/main squad; value; reads (registrar_convocados_por_clube)
- [NationalTeams+0x20] + 6 x club id: +0 entry array (8-byte date, +8 call-ups), +4 capacity, +5 count; pointer and value; reads and writes (registrar_convocados_por_clube)
- nation +0x11c: the nation flag; value; reads (registrar_convocados_por_clube)
- fixture +0x28 / +0x2a: year and day; value; reads (registrar_convocados_por_clube); +0x42: the stage; value; reads (importancia_do_jogo_na_competicao)
- competition +0xc / +0x30: the stages and the last one; pointer and value; reads (importancia_do_jogo_na_competicao, fase_da_competicao)

**Of interest to:** CMRTE, CMScout, SGE, GPF2
