# Batch 263 — the cup round: the round prize money, the home teams of the round and the team progress; club friendlies: the reply to a friendly request, the sent request and the nation list for international friendlies; the cup round draw, the stage and round name and the club in the higher division; the cup by outcome and at season end: the records by outcome, the season honours and the semi-final losers for UEFA; the squad room for an offer (the 50-player limit and the staff slots, releasing the cheapest player), the loan offer rejected by the club and the next-sale percentage clause found; the human manager's shortlist: the player in the search, the scout lists matched with the club's scouts and the player's slot in the scouted club record; the player's value for a club (the rating for the shortlist slot and the fee the club would pay), the player's entry in the scouted club search and the international appearance fee clause found; contracts after the match (bonuses, happiness, morale and the development of those left out), the record morale and its text; the player value, the morale comparison of two players and the squad flags of the club screen; the board and the players after the match: the players it names (with the confidence), the match rating verdict and its text; the managers' records: each match result, the final league position and the supporters' player of the year; the cup and manager change records: the runner-up, the third place and the manager's spell at the club; the squads' reactions to a transfer: both squads' contracts, the signing celebration and the team mate unsettled by the sale

**Routines:**
- `copa_premio_da_rodada()` = 0x522240 (Cup.cpp, thiscall, ret 0x10): pays the round prize money (+0x60 to the winner, +0x64 to the loser; with -1, from the fund +0x28 over the remaining rounds and the ties, 3/4 and 1/4 when both are -1): the winner with the news item (0x48e3e0) and the money (0x5999a0), the loser only the money; the fund loses both;
- `copa_mandantes_da_rodada()` = 0x51e870 (Cup.cpp, thiscall, ret 8): swaps home and away in one tie or in every tie of the round by the round rule: the club in the higher division (virtual +0x90), the team level or the cup stadium;
- `copa_progresso_do_time()` = 0x521b30 (Cup.cpp, thiscall, ret 8): how far the team went in the cup, by outcome and round (with or without the third place round), handed to virtual +0x68 (set_club_progress) with competition +0x44;
- `amistoso_responder_pedido()` = 0x5c5f40 (friendly.cpp, thiscall, ret 8): the invited club (without a human manager, or when forced) accepts or refuses by reputation, with the nation, region, continent and stadium city distance factor; tells the requester (news 7); an accepted one goes to the fixture list (0x5aa680) and marks the friendly slot; with 3 or 4 the request is removed (0x5c87b0); otherwise it only leaves the reply list;
- `amistoso_enviar_pedido()` = 0x5c5570 (friendly.cpp, thiscall, ret 4): stores the request on the reply list (+0x34d4, created when missing); an invited club with a human manager: status 2 and news 6;
- `amistoso_lista_das_nacoes()` = 0x5ca3e0 (friendly.cpp, thiscall, ret 0): allocates the list (+0x34fa) of nations with a continent, counts it in +0x34fe and sorts it by reputation (0x5cc5f0);
- `copa_sortear_rodada()` = 0x51c410 (Cup.cpp, thiscall, ret 4): with some team still open it puts the draw off by 1 day (and the following rounds to at least that date); otherwise draws by the rule (seeded 0x51d2d0, nations apart 0x51d9f0 or open 0x51e220), sets the home teams (0x51e870) and the shared stadium (0x522610), marks the drawn ties and sends the draw news to the human managed teams; returns 1;
- `nome_da_fase_e_rodada()` = 0x4bffc0 (comp_util.cpp, cdecl): writes the stage and round names (with their genders, through template 0x9a7cfc), or just one of them; without an output or under 200 bytes, the error box and 0;
- `clube_da_divisao_mais_alta()` = 0x522360 (Cup.cpp, thiscall, ret 8): returns the club whose division level (+0x57 -> +0x69) is lower, or 0 when level;
- `copa_registros_pelo_desfecho()` = 0x519890 (Cup.cpp, thiscall, ret 0x18): by the club outcome: the winners record (and the UEFA Cup place through the English League Cup or a European nation main cup), runners-up, third place, qualified for the next round or knocked out, weighted by round; returns 0;
- `copa_honras_da_temporada()` = 0x519a90 (Cup.cpp, thiscall, ret 0): finds the winner, runner-up, third (in the Welsh cup, through the statistics) and the host national team, writes the history row and, in the nation main cup, passes the semi-final losers to UEFA;
- `copa_semifinalistas_para_a_uefa()` = 0x519fc0 (Cup.cpp, thiscall, ret 0): finds the semi-finals (0x82) and the final (0x96) and sends the two semi-finalists not in the final to the UEFA seeding;
- `recusar_oferta_de_emprestimo()` = 0x8bfa30 (transfer_manager.cpp, thiscall, ret 0xc): the club rejects the loan offer: state 2, the bids unmarked, the news, the offer comes back in 10 days to be removed, the person's offer mark cleared with no open rival, the contract offers closed and the buyer's shortlist status;
- `vaga_no_elenco_para_a_oferta()` = 0x8cd450 (transfer_manager.cpp, thiscall, ret 0x10): checks that the offer keeps the club (with its partner) within 50 players and the coach (5), physio (3), scout (7) and assistant slots; a computer buyer without exchange releases for free the cheapest player who can leave; otherwise reason 4 or 0x22 (plus 1 for the seller) and 0;
- `percentual_da_proxima_venda_achado()` = 0x8d0dd0 (transfer_manager.cpp, thiscall, ret 4): with the person's flag 8, binary-searches the next-sale percentage clause of the person with the contract club; when not found, clears the flag;
- `jogador_na_busca()` = 0x7af600 (player_search.cpp, thiscall, ret 4): looks the player up in the search results (binary search by id) and returns 1 when found and not marked as removed;
- `sincronizar_listas_dos_olheiros()` = 0x7f7270 (scout_manager.cpp, thiscall, ret 4): removes from the manager's scout lists the scouts no longer at the club and gives each club scout without a list the first free list; with the edit session open, it refreshes the scouts;
- `vaga_do_jogador_observado()` = 0x7bd700 (player_search.cpp, thiscall, ret 8): the player's slot among the 50 of the scouted club record; when claiming, the first free one gets the id and its counters are zeroed;
- `valor_do_jogador_para_o_clube()` = 0x840d80 (shortlist_manager.cpp, thiscall, ret 0x18): the player's rating for the slot (through the club's best observer, the scout list at the human club, the fog of war) and the fee the club would pay: the base value, age, the young players' seen ability, consistency, the caps by position, reputation, record, age, division and the club's nation;
- `entrada_do_jogador_na_busca_do_clube()` = 0x7f8610 (scout_manager.cpp, thiscall, ret 4): the player's slot in the record of the human's search that scouts his club, or 0;
- `premio_de_jogos_na_selecao_achado()` = 0x8d0f00 (transfer_manager.cpp, thiscall, ret 4): with the person's flag 4, binary-searches the international appearance fee clause; when not found, clears the flag;
- `contratos_depois_do_jogo()` = 0x4ca6d0 (contract_manager.cpp, thiscall): after each match: contract bonuses, happiness by minutes played, morale by a draw and by the result, and the development and happiness of those left out;
- `moral_da_ficha()` = 0x4cc2d0 (contract_manager.cpp, stdcall): the morale of the player record (+0x45), for the screens;
- `texto_da_moral()` = 0x4cc2e0 (contract_manager.cpp, stdcall): the morale word (from "Very Low" to "Superb") into a text, for the screens;
- `valor_do_jogador()` = 0x4d0c60 (contract_manager.cpp, thiscall): the player value by peak age and age, record ratings, ability as seen by the club manager, club nation, contract days left and release clause;
- `diferenca_de_moral_em_texto()` = 0x4cc3a0 (contract_manager.cpp, stdcall): the morale difference of two players as a word ("Same", "Marginal", "Better", "Superior"), for the comparison screen;
- `marcas_de_elenco_para_a_tela()` = 0x4cc130 (contract_manager.cpp, thiscall): the squad flags the club screen shows, by the contract reserve bit when the player is on loan elsewhere;
- `jogadores_que_a_diretoria_cita()` = 0x6aaee0 (manager_manager.cpp, thiscall): after the match, by a draw, the league position for the news and the manager's favoured or rejected players who played badly, with the board confidence rising or falling by them;
- `veredito_da_nota_na_partida()` = 0x6ab670 (manager_manager.cpp, thiscall): finds the player among the starters of the match record: −1 for a rating under 7, 1 above, else 0;
- `texto_da_diretoria_sobre_os_jogadores()` = 0x6ab2d0 (manager_manager.cpp, stdcall): the board sentence praising or blaming the named player, by variant;
- `resultado_nos_registros_dos_tecnicos()` = 0x868830 (staff_records.cpp, thiscall): in both managers' records: the match, the goals, the win, draw or loss and the match rating (importance × opponent reputation × weight × points) in the month, the year, the club's league and the club or national season; the away manager gets the home rating in the season total (original bug);
- `posicoes_no_fim_da_liga()` = 0x86a070 (staff_records.cpp, thiscall): at the end of the league, each club's final position in its manager's history;
- `jogador_do_ano_da_torcida_no_registro()` = 0x86dc30 (staff_records.cpp, thiscall): the supporters' player of the year in his history and, by a draw, the favourite link between him and the club;
- `vice_da_copa_nos_registros()` = 0x86b0b0 (staff_records.cpp, thiscall): the cup runner-up: the club reputation rises up to weight × 500 (at most 350 at once), its side's three match ratings by the weight, and the manager, chairman, line-up and club histories;
- `terceiro_da_copa_nos_registros()` = 0x86b710 (staff_records.cpp, thiscall): the cup third place: the same with weight × 200 (at most 200) and smaller ratings; ends the call-ups in the stored competitions;
- `troca_de_tecnico_nos_registros()` = 0x869aa0 (staff_records.cpp, thiscall): closes the manager's current spell (shortest and longest), opens the new one when hired and counts the spells; the manager, chairman and club histories;
- `reacoes_dos_elencos_a_transferencia()` = 0x758200 (media.cpp, thiscall): after the transfer of a player aged 19 or more with reputation 4250: in the new squad, the contract flags and grievances by position and squad strength, the supporters (+500) and the celebration of a star signing; in the old squad, the grievances at the team mate leaving, morale −25, cleared flags and the unsettled team mate news;
- `noticia_da_comemoracao_da_contratacao()` = 0x75b6e0 (media.cpp, thiscall): news 0xbe5 of the fans or a squad spokesman celebrating the signing, to the club manager;
- `noticia_do_companheiro_desassossegado()` = 0x75cfc0 (media.cpp, thiscall): news 0xbe4 of the team mate unhappy at the sale, to the old club's manager;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (33):** 31 caught. `cqa2_sem_o_fator_da_distancia` escaped: the distance factor only weighs on the invitation score, and in the 256 invitations with a stadium city the decision (accept or refuse) came out the same; the routine-by-routine comparison did not flag it either [inference]. `cqa3_sorteio_adiado_dois_dias` escaped: the postponement only runs with an earlier tie still open (4 times, FA Cup replays), and drawing two days later changed neither the ties nor the random draws used by 2002-06-01 [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (4307c8ab).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6299f556, original da4329bf (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 262's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 263`

**Fields touched:**
- Cup +0x28: the prize fund; value; reads and writes (copa_premio_da_rodada)
- Cup +0x3c / +0xa3: the rounds and their table (0x68 bytes: +7 type, +0xb rule, +0x1a ties, +0x5b progress, +0x60/+0x64 prizes); value and pointer; reads (all three)
- Cup +0xa7: the teams (6 bytes: +0 club, +4 level, +5 outcome); pointer; reads (all three)
- Cup +0xad: the ties of each round (0x35 bytes: +0/+2 teams, +0xa/+0xc, +0xe/+0x10); pointer; reads and writes (copa_mandantes_da_rodada)
- Cup +0x24 / +0x44: the cup stadium (the id, compared with the club stadium +0x69) and the competition passed to +0x68; id; reads (copa_mandantes_da_rodada, copa_progresso_do_time)
- fixture +0x30: the fixture round (0x136 turns outcome 1 into 3 in the news); value; reads (copa_premio_da_rodada)
- club finances (0x5a0590): balance, +0xa8, +0x148; value; adds through 0x5999a0 (copa_premio_da_rodada)
- friendly request (0x19 bytes): +0/+2 day and year, +0xc requester, +0x10/+0x14 the match clubs, +0x18 status (1 pending, 2 with the human, 3 accepted, 4 refused); reads, writes +0x18 (amistoso_responder_pedido, amistoso_enviar_pedido)
- club +0x53 / +0x69 / +0x77 / +0x80: nation, stadium (+0x38 the city, +0x23/+0x2b the coordinates), the +0x77 integer and reputation; pointer and value; reads (amistoso_responder_pedido)
- nation +0x71 / +0x76: continent and region; pointer and value; reads (amistoso_responder_pedido, amistoso_lista_das_nacoes)
- Friendly +0x34d4: the friendly reply list; pointer; reads and writes (amistoso_responder_pedido, amistoso_enviar_pedido)
- Friendly +0x34fa / +0x34fe: the nation list and its count; pointer and value; writes (amistoso_lista_das_nacoes)
- scheduled friendly slot (0x5c6740) +0x10 / +0x18: opponent and mark; pointer and value; writes (amistoso_responder_pedido)
- [0xae2c92] the current year; [0xae2350] / [0xae23a8] the nation count and table (0x122 bytes); value and pointer; reads
- Cup +0x45 / +0x3c / +0x40 / +0x3a / +0x44 / +0xab / +0xac: ready flag, rounds, base year, id, competition, current round and draw news count; value; reads, writes +0xac (copa_sortear_rodada)
- cup round (0x68 bytes) +0 / +2 / +7 / +0xb / +0xd / +0x1a / +0x1c / +0x21: draw date (day and year), type, rules, ties, entries and stadium; value; reads, writes the date (copa_sortear_rodada)
- cup tie (0x35 bytes) +0 / +2 / +0x34: teams and drawn flag; value; reads, writes +0x34 (copa_sortear_rodada)
- club +0x57 -> division +0x69: the division level; pointer and value; reads (clube_da_divisao_mais_alta)
- Cup +0x2c / +0x44 / +0x3c / +0x3e / +0x40: stages, competition, rounds, teams and year; value; reads (all three)
- Cup +0xa3 / +0xa7 / +0xad: rounds (+7 type, +0x1a ties), teams (+5 outcome) and ties; pointer; reads (all three)
- club state [0xae2d14] + id × 6: the season European competition; id; reads and writes the UEFA Cup key (copa_registros_pelo_desfecho)
- fixture +0x30 / +0x32 / +0x34 / +0x3a: the round words and the flag; value; reads (copa_registros_pelo_desfecho)
- competition history [0xae23d4] (26 bytes per row): +4 competition, +8 year, +0xa..+0x16 the four clubs; written through 0x4afce0 (copa_honras_da_temporada)
- UEFA seeding [0xde1f58] +0x440: 50 slots of 12 bytes (competition, the two clubs); written through 0x905790 (copa_semifinalistas_para_a_uefa)
- offer +4 / +8 / +0xc / +0x11: the exchange (LLIST), the buyer, the person and the offer type; id and value; reads (vaga_no_elenco_para_a_oferta, recusar_oferta_de_emprestimo)
- offer +0x2c / +0x2d / +0x10: the state (2), the flag and the stage; value; writes (recusar_oferta_de_emprestimo)
- club +0xd7: the 50 squad slots; pointer; reads (vaga_no_elenco_para_a_oferta)
- club +0x19f / +0x1b3 / +0x1cf / +0xd3: coaches (5), scouts (7), physios (3) and assistant; pointer; reads (vaga_no_elenco_para_a_oferta)
- person +0x3d / +0x52: the type and the value; value; reads (vaga_no_elenco_para_a_oferta)
- contract +0x3d / +0x35: the squad status (signed) and the type (5 = loan); value; reads (vaga_no_elenco_para_a_oferta)
- [this+0x86e] + id × 8: the person's flags (bit 0 offer in progress, bit 3 next-sale percentage); value; reads and writes (recusar_oferta_de_emprestimo, percentual_da_proxima_venda_achado)
- [this+0x880] / [this+0x888]: the next-sale percentage clauses (0xd bytes: +1 club, +9 person) and their count; pointer and value; reads (percentual_da_proxima_venda_achado)
- the free transfer queue (LLIST of ids); id; reads (vaga_no_elenco_para_a_oferta)
- [0xae1080]: the last contract looked up; pointer; writes (all three)
- search +0x1ee / +0x1f6: the results (0x2d bytes: +0xc the id, +0x20 bit 0 removed) and the count; pointer and value; reads (jogador_na_busca)
- search +0xa: the list type (4 = scout); value; reads (sincronizar_listas_dos_olheiros)
- search +0x174..+0x195: the scout (ids, index +0x180, date) and the assignments; id and value; writes through 0x7ae8a0 (sincronizar_listas_dos_olheiros)
- person +0x39 / +0x3d: the club and the type (9 = scout); pointer and value; reads (sincronizar_listas_dos_olheiros)
- [scout manager + slot × 4 + 0xc]: the manager's open edit session; pointer; reads (sincronizar_listas_dos_olheiros)
- scouted club record +4: 50 slots of 0x1a2 bytes (+0 id, −1 free; +4 word, +0x1a0, +0x1a1 counters); id and value; reads and writes (vaga_do_jogador_observado)
- shortlist slot +1 / +7 / +0xb / +0xf / +0x24: the position, the fee cap, the reference value, the person and the type (bit 0 loan, bits 1..4 the type); value and id; reads (valor_do_jogador_para_o_clube)
- club +0xcf / +0x1b3 / +0x80 / +0x53 / +0x57: the manager, the 7 scouts, the reputation, the nation and the division (+0x69); pointer and value; reads (valor_do_jogador_para_o_clube)
- person +0x69: the observer's knowledge record; pointer; reads, and writes a random one when missing (valor_do_jogador_para_o_clube)
- person +0x18 / +0x39 / +0x61: the age, the club and the player record (+0xb, +0xd, +0x2d); pointer and value; reads (valor_do_jogador_para_o_clube)
- [0xae2d0c] + id × 0x4f + 0xb: the person's 4 fog-of-war bits; value; reads (valor_do_jogador_para_o_clube)
- [this+0xdc] + club id × 9 + 5: the club's reputation in the shortlist table; value; reads (valor_do_jogador_para_o_clube)
- [0x9cf434] / [0x9cf300]: the two nations with their own fee rule; id; reads (valor_do_jogador_para_o_clube)
- search +0x170 / +0x190: the scouted record and club; pointer and id; reads (entrada_do_jogador_na_busca_do_clube)
- [this+0x86e] + id × 8 bit 2 and [this+0x898] / [this+0x8a0]: the flag and the international appearance fee clauses (0xd bytes, +0 the person); value and pointer; reads and writes the flag (premio_de_jogos_na_selecao_achado)
- match +0x40, +0x41 flags; +0x43/+0x44 and +0x47/+0x48 score; +0x14 competition; +0x1c/+0x20 clubs; +0x3a reputation check; value/pointer; reads (contratos_depois_do_jogo)
- match summary + side × 0x625: +0x190 0x49-byte slots (+0/+4 ids, +0x1f, +0x20, +0x21, +0x22, +0x27, +0x30, +0x32 minutes), +0x154 words; value; reads
- contract +0x10, +0x14, +0x18 bonuses; +0x35 & 0x3f type; value; reads
- contract +0x3d happiness, +0x3e and +0x3f match counters, +0x44 morale; value; reads and writes
- 79-byte record at [0xae2d0c] +9 (national team) and +4 (second contract); value/index; reads and writes (+9)
- club +0xd7 squad (50), +0x80 reputation; pointer/value; reads
- person +0x57, +0x61 player record; value/pointer; reads
- player record +0x45 morale; value; reads (moral_da_ficha, texto_da_moral)
- [0xb74350] +0x10/+0x18 league ratings; value; reads
- player record +0xf, +0x12..+0x15 positions; +0x27, +0x28, +0x2d, +0x2f, +0x36, +0x43 ratings; +9 reputation; +0xb current; +0xd potential; value; reads (valor_do_jogador)
- person +0x18 age, +0x22 and +0x1a (+0x8e) experience, +0x61 player record, +0x5e squad flags; value/pointer; reads
- club +0xcf manager, +0x53 nation (+0x7e, +0x85), +0x57 division (+0x69), +0x64, +0x80 reputation; pointer/value; reads
- contract +0x2d/+0x2f end, +0x35 type and 0x40 bit, +0x3d happiness, +0x21 clause, +4 club; value; reads
- [0xae1080] contract looked up; pointer; writes
- [0x9a7f30] 20-byte table by age; value; reads
- player record +0x45 morale; value; reads (diferenca_de_moral_em_texto)
- board (0x49 bytes at [manager manager] + id × 0x49) +6 confidence; value; reads and writes (jogadores_que_a_diretoria_cita); +0x2c; value; reads
- club +0xbf manager (+0x57, +0x69 record +0x16), +0x57 division, +0x80 reputation; pointer/value; reads
- league table row +4 games, +0x19, +0x2d, +0x73, +0x74; value; reads
- person +0x18 age, +0x3d job, +0x61 player record (+0xb); value/pointer; reads
- match record (0xd7a bytes): 0x625-byte sides at +0x11f, slots at +0x71 (0x49 bytes: +0 id, +0x35 rating); value; reads (veredito_da_nota_na_partida)
- [0xae1080] contract looked up; pointer; writes
- manager record (0xd2 bytes at [0xdd7f00]+8) +0x8e games, +0x90 wins, +0x92 draws, +0x94 losses, +0x96 goals for, +0x9a goals against; values; writes (resultado_nos_registros_dos_tecnicos)
- manager record +0xb6 year points, +0xca points, +0xce month points (club league), +0xba club season, +0xbe national season; values; writes (resultado_nos_registros_dos_tecnicos)
- fixture +0x14 competition, +0x1c/+0x20 clubs, +0x3a not played, +0x43/+0x44 and +0x47/+0x48 scores; pointers/values; reads (resultado_nos_registros_dos_tecnicos)
- match data +0x11f/+0x127/+0x128/+0x129 and +0x744/+0x74c/+0x74d/+0x74e counts for each side; values; reads (resultado_nos_registros_dos_tecnicos)
- club +0xcf manager, +0x80 reputation, +0x57/+0x60 leagues, +0x9b..+0xb3 favoured and disliked people; pointers/values; reads (resultado_nos_registros_dos_tecnicos, posicoes_no_fim_da_liga), writes the favourites (jogador_do_ano_da_torcida_no_registro)
- person preferences (+0x65) +4..+0x18 favoured and disliked clubs; pointers; writes (jogador_do_ano_da_torcida_no_registro)
- league +4 competition, +0x3a year, +0x3e table rows; values; reads (posicoes_no_fim_da_liga)
- club +0x80 reputation; value; reads and writes (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)
- club +0xcf manager, +0xbf chairman, +0x53 nation; pointers; reads (all three)
- match data +0x188/+0x18a/+0x18c side ratings (side × 0x625), +0x190 line-up (20 × 0x49 bytes, id or −1); values/ids; writes the ratings, reads the line-up (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)
- competition +0x69 weight, +0x42 stage; values; reads (vice_da_copa_nos_registros, terceiro_da_copa_nos_registros)
- manager record +0x4a current club, +0x4e start, +0x56 date, +0x5e/+0x62/+0x6a longest spell, +0x72/+0x76/+0x7e shortest, +0xb4 spells, +0xb5 national jobs; ids/dates/values; writes (troca_de_tecnico_nos_registros)
- [0xdd6ec8] game start, [0xae2c90] today, [0xae2470] static date of 0x54cdb0; dates; reads, writes the static (troca_de_tecnico_nos_registros)
- contract (0x50 bytes) +0x3d, +0x40 event flags, +0x44 morale, +0x45 grievances; values; reads and writes (reacoes_dos_elencos_a_transferencia)
- [[0xb67a44]] + 0x49 × club id, +8: the supporters (1..10000); value; writes +500 (reacoes_dos_elencos_a_transferencia)
- person +0x18 age, +0x3e joining date, +0x57/+0x58/+0x59/+0x5a/+0x5b attributes, +0x61 player data (+5, +0xb reputation, +0xd potential); values; reads (reacoes_dos_elencos_a_transferencia)
- club +0xd7 squad (50), +0x80 reputation, +0x53 nation, +0x57 league; pointers/values; reads (reacoes_dos_elencos_a_transferencia)
- Media +0 story queue (+8 person, +0x10 kind, +0x11, +0x13 news), +8 count; values; reads (reacoes_dos_elencos_a_transferencia)

**Of interest to:** CMRTE, SGE, CMScout
