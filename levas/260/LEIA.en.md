# Batch 260 — the daily absences: the day of the AWOL players, the missed training and the late arrivals, the missed training flag and the fitness incident record; a transfer offer rejected by the selling club, one withdrawn by the buyer and the news of the rejected offer; a player unsettled by a bigger club's bid, the news of a collapsed offer and of appearance money paid; a transfer carried out, with the news of the completed transfer and of the clubs' agreement; a loan from the configuration file, the news of a contract ending and of a transfer delay request; the active club tactic: the tactic of the club team, the human manager tactic slot and his active tactic; the stored club selection: removing the player from the selection, swapping two slots and the undo copy; the manager news: the board considering the manager position, the manager departure and the new manager; the league's end of season: the prize money, the records for the club's status and the status news item; the match statistics and the club records: the match in the players' statistics, the league position records and the record news items

**Routines:**
- `faltas_do_dia()` = 0x4190d0 (awol.cpp, thiscall): ends the first expired AWOL; resolves the first missed training from a week ago (repeat case: draws and a new miss; new case: teammates who ask to leave or lose professionalism, morale and temperament, and the offender who loses professionalism and morale and falls out with the club); resolves the first expired late arrival by the draws;
- `teve_falta_ao_treino()` = 0x418ff0 (awol.cpp, thiscall, ret 4): bit 2 of the person flag; without a contract, clears the miss first;
- `teve_incidente_de_forma()` = 0x419070 (awol.cpp, thiscall, ret 4): true when the person has a record in the late arrival list;
- `marcar_falta_ao_treino()` = 0x41a360 (awol.cpp, thiscall, ret 4): the very unhappy player may go AWOL (0x419840); otherwise marks the miss, posts the news, lowers the condition, joins the missed training list (the case byte, unset in the original, goes 0x90) and the AI manager may fine him;
- `fim_da_ausencia()` = 0x419710 (awol.cpp, thiscall, ret 4): with a contract at a club, the return news; toggles bit 1 of the flag and removes the record from the absence list;
- `marcas_do_contrato()` = 0x867560 (staff_contracts.cpp, thiscall, ret 4): sets the flags at +0x40 and, when the person now wants to leave and nothing prevents it, the transfer request (0x4d8610);
- `recusar_oferta_de_transferencia()` = 0x8bf680 (transfer_manager.cpp, thiscall): rejects the offer: the player may become unsettled, state 2, bids unmarked, contract offers closed, the news, back in 25 days for removal and the person's offer flag;
- `retirar_oferta_de_transferencia()` = 0x8bfcb0 (transfer_manager.cpp, thiscall): withdraws the offer: state 3, held contract offers resubmitted, bids unmarked, back in 10 days; for reason 2, the unsettling; the news and the contract offers;
- `noticia_oferta_recusada()` = 0x8d5170 (transfer_manager.cpp, thiscall): the 0xfa1 news of a rejected or withdrawn offer, with the reason, to the interested human clubs;
- `desassossegar_pelo_lance()` = 0x8c1290 (transfer_manager.cpp, thiscall): a draw against the buyer's reputation flags the player as unsettled; when flagged, a score from his attributes and interest leads to the 0x80 complaint, going missing and grudges;
- `noticia_oferta_desfeita()` = 0x8de790 (transfer_manager.cpp, thiscall): the 0xfb0 news of a collapsed offer, with the club and the reason, to the interested human clubs;
- `noticia_premio_por_jogos()` = 0x8db8a0 (transfer_manager.cpp, thiscall): the 0xfa6 news of appearance money paid, to the paying club and to the contract club;
- `efetivar_transferencia()` = 0x8c9390 (transfer_manager.cpp, thiscall): carries out the transfer: board confidence, nation rules, clauses, payment, records, former clubs' cut, the player and the exchange players moved, squad numbers, state 0x14 and back in 10 days;
- `noticia_transferencia_concluida()` = 0x8dbef0 (transfer_manager.cpp, thiscall): the 0xfa8 news of a completed transfer, with the fee, the exchange players, a draw and the player's strong points;
- `noticia_transferencia_acertada()` = 0x8e05d0 (transfer_manager.cpp, thiscall): the 0xfb3 news of the clubs' agreement, with the fee, the exchange players and the completion date;
- `emprestimo_da_configuracao()` = 0x8bf400 (transfer_manager.cpp, thiscall): a loan from the configuration file: the checks, the contract extended to its end, the loan contract, the squad, the squad numbers and the work permit;
- `noticia_fim_do_contrato()` = 0x4da590 (contract_manager.cpp, thiscall): the 0xbb9 news of a contract, trial or loan ending, to the club, the interested clubs or, for famous players, the nation's managers or everyone;
- `noticia_pedido_de_adiamento()` = 0x8dda90 (transfer_manager.cpp, thiscall): the 0xfab news of a club asking to delay a transfer by a week, to the interested human clubs;
- `tatica_do_clube()` = 0x896660 (tactics.cpp, thiscall, ret 8): for a human club (without +0x9a01 or with holiday option 2), the slot for the team and the manager active block; otherwise, by the index in the club row, the preset block (below the preset count) or the own one;
- `escolher_vaga_de_tatica()` = 0x5fbe20 (human_manager.cpp, thiscall, ret 0xc): the human manager active slot: 4 for the second national team, 3 with reserve in a national team, 2 in a national team, 1 with reserve, 0 for the first team;
- `tatica_ativa_do_humano()` = 0x5fbd00 (human_manager.cpp, thiscall, ret 4): the tactic block (0x1ecd bytes) of the active slot of the human manager record;
- `tirar_da_escalacao()` = 0x896c60 (tactics.cpp, thiscall, ret 0x10): empties (−1) the selection slots holding the person; with roles, also the two extra slots and the set piece takers; the undo copies of the club and team are dropped; 1 when removed from a slot or set piece;
- `trocar_vagas_da_escalacao()` = 0x8973e0 (tactics.cpp, thiscall, ret 0x10): swaps the ids of two selection slots (0..19) and drops the undo copies of the club and team;
- `tem_copia_da_escalacao()` = 0x897250 (tactics.cpp, thiscall, ret 8): 1 when the current human holds the selection undo copy of this club and team;
- `noticia_da_pressao()` = 0x6a0e70 (manager_manager.cpp, thiscall, ret 8): the 0x1777 news with the manager, the club and the chairman; the reason by the board ratings below confidence − 500 (draws of 3 and 5) and the rumour (draw of 10, or 1 when the manager knows the club); delivered to all, to the nation or to the division managers; today in +0x18 of the board row;
- `noticia_da_saida_do_tecnico()` = 0x6a1b00 (manager_manager.cpp, thiscall, ret 0x14): the 6000 news with the manager, the type, the club, the job, the chairman and the reason; a human manager with the names and to all; otherwise by the club (national team or reputation above 7250 to all, otherwise to the nation);
- `noticia_do_tecnico_novo()` = 0x6a2e00 (manager_manager.cpp, thiscall, ret 0x10): only a club of an active nation with an active division, or a national team of an active nation: the 0x1787 news with the manager, the previous club, the club, the job and the previous role; delivered like the departure one;
- `pagar_premios_da_liga()` = 0x682e10 (league.cpp, thiscall, ret 0): divides the prize fund (+0x28) by 1 + 2 + ... + n and pays (n - i) shares to the club in table row i (0x5999a0 on the finance record), with the prize news item above 1000; with +0xe1 = 3 the competition pays through virtual +0x6c; with +0xe1 = 2, nothing;
- `registros_pela_situacao_na_liga()` = 0x686940 (league.cpp, thiscall, ret 0x18): by status: 5 the title (0x868c50), 0 promotion (0x869480, weight 100), 3 relegation (0x86a1c0) in the staff records; returns 0;
- `anunciar_situacao_na_liga()` = 0x686670 (league.cpp, thiscall, ret 0x14): outside friendlies, calls the competition virtual +0x44 and builds news item 0xc (club, competition, stage, status, round, what the virtual returned), delivered by scope or, by status, to the nation, the division (and the promotion or relegation one) or the club manager;
- `registrar_partida_nas_estatisticas()` = 0x7c0d70 (player_stats.cpp, thiscall, ret 8): by each side's category (friendly, reserves, competition type; a draw in a simulated match), adds the rating and the match to the double sums and each player who played to the category records (games, goals, score, rating, minutes...), to the player's games and goals and to contract +0x3b; shifts the form half a byte;
- `recordes_de_posicao_na_liga()` = 0x447430 (club_records.cpp, thiscall, ret 8): sorts the table of each league stage and stores each club's best and worst position for the season and, with the second argument, all time (queueing the news item for a human club or national side);
- `noticias_de_recordes()` = 0x448e30 (club_records.cpp, thiscall, ret 0): delivers the news item of each queued record (human club or national side) and empties the queue (freed at capacity 100 or more);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (28):** 24 caught. `cpx1_sem_desmarcar_comprador` escaped: the buyer's bids left flagged only matter for a new bid by him for the same player, which the career to 2002-06-01 did not make [inference]. `cpx2_sorteio_2000` escaped: no buyer in the career had a reputation between the two limits (2000 and 4250 plus the draw) [inference]. `cpx3_sem_parte_dos_anteriores` escaped: the former clubs' cut only applies with the clause in the contract, which no transfer in the career had [inference]. `cpx10_limite_400` escaped: the limit only changes the reason in the news text, and no board rating fell in the band 400 to 500 below confidence [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (7da97d00).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 2e9d1c64, original 086f15e7 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 259's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 260`

**Fields touched:**
- AWOL +0 absences, +4 missed training, +8 late arrivals (lists with count at +0, first node at +2, cursor at +0xa), +0xc per-person flags (bit 1 AWOL, bit 2 missed training); pointers/values; reads and writes (faltas_do_dia, teve_falta_ao_treino, teve_incidente_de_forma)
- missed training record (13 bytes) +0 person, +4 date, +0xc repeat case; late arrival record +0 person, +8 date, +0x10 flag; id/values; reads (faltas_do_dia)
- staff +0x18 age, +0x57 attribute, +0x5b professionalism, +0x5d temperament; values; reads and writes +0x5b and +0x5d (faltas_do_dia)
- player (staff +0x61) +0xb current reputation, +0x44 morale; values; reads and writes +0x44 (faltas_do_dia)
- club +0xd7 squad (50 slots); contract +0x4f type (high nibble); [0xae1080] last contract; pointers/values; reads, writes [0xae1080] (faltas_do_dia, teve_falta_ao_treino)
- contract +0x40 flags, +0x44, +0x45 bits, +0x49, +0x4e bits 1 and 8; values; reads, writes +0x40 (marcas_do_contrato, marcar_falta_ao_treino)
- [0xae2d0c] 79-byte rows per person, +0xa; value; adds a draw of 10 (marcar_falta_ao_treino); staff +0x3d job, +0x59 attribute; reads
- offer +0: rival offers list; pointer; reads (reject, withdraw)
- offer +4: exchange players list (+4 of each slot: flag); pointer; reads and writes 0 to the flag (reject, withdraw)
- offer +8 buyer, +0xc player; id; reads (all three)
- offer +0x10; value; writes 4 and 0 (withdraw), 0 (reject)
- offer +0x2c: state; value; writes 2 (reject) and 3 (withdraw); reads (the news)
- offer +0x2d; value; writes 1 when flagged (reject, withdraw)
- offer +0x11, +0x1f, +0x20 slot, +0x2e, +0x2f; value; reads
- transfer manager +0x86e: 8-byte record per person, bit 0 = offer in progress; value; reads and clears (reject, withdraw)
- contract +4 club, +0x35 & 0x3f type; value; reads (the news)
- [0xae1080] last contract looked up; pointer; writes (all three)
- offer +8 buyer, +0xc player; id; reads (desassossegar_pelo_lance, noticia_oferta_desfeita)
- offer +0x2d: player unsettled; value; writes 1 and reads (desassossegar_pelo_lance)
- club +0x80: reputation; value; reads (desassossegar_pelo_lance)
- person +0x18 age, +0x57, +0x59, +0x5b, +0x5d: personality attributes; value; reads (desassossegar_pelo_lance)
- player record +0xb: current reputation; value; reads (desassossegar_pelo_lance)
- contract +0x4e & 8: transfer listed; +0x44: request strength; value; reads (desassossegar_pelo_lance)
- transfer manager +0x86e: 8-byte record per person, +1; value; reads (desassossegar_pelo_lance)
- clause +0 person, +4, +9 paying club; id; reads (noticia_premio_por_jogos)
- [0xae1080] last contract looked up; pointer; writes (all three)
- offer +4: exchange players list; pointer; reads (all three)
- offer +8 buyer, +0xc player; id; reads (all three)
- offer +0x1b: transfer fee; value; reads (all three)
- offer +0x1f, +0x11, +0x2f, +0x30; value; reads
- offer +0x2c: state; value; writes 0x14 (efetivar_transferencia)
- transfer manager +0x8ac: the 28 nation rules (+0xa: nation); pointer; reads (efetivar_transferencia)
- transfer manager +0x86e: 8-byte record per person, bit 0 = offer in progress; value; reads and clears (efetivar_transferencia)
- club +0xd7: squad (50 slots); pointer; writes 0 to the first exchange player's slot (efetivar_transferencia)
- person +0x39 club, +0x69, +0x61 player record; pointer; reads
- contract +0x2d/+0x2f: contract end; value; reads (noticia_transferencia_concluida)
- [0xae1080] last contract looked up; pointer; writes (all three)
- person +0x46: date (end of tie); value; reads and writes the loan end (emprestimo_da_configuracao)
- person +0x6d; value; writes 0x10 instead of 2 (emprestimo_da_configuracao)
- contract +0x2d: contract end; value; writes the loan end (emprestimo_da_configuracao)
- local contract +0x25: start; value; writes (emprestimo_da_configuracao)
- person +0x1a nation, +0x39 current club, +0x61 player record; record +0xb/+0xd reputation; value; reads (noticia_fim_do_contrato)
- 79-byte record +0 (general.dat): contract index; value; reads (noticia_fim_do_contrato)
- offer +8 buyer, +0xc player, +0x20 slot, +0x2c state; value; reads (noticia_pedido_de_adiamento)
- [0xae1080] last contract looked up; pointer; writes (the news)
- Tactics +0x5f1 own blocks, +0x5f9 preset blocks (0x18e3 bytes), +0x5fd preset count, +0x601 tactic table (0x91-byte rows per id, +0 tactic index, signed word); pointers/values; reads (tatica_do_clube)
- HUMAN_MANAGER +0 records (0xa6ef bytes, index id − [0xae2364] + 16), +4 count; record +0x9a69 active slot (signed byte), 0x1ecd-byte tactic blocks; values; reads and writes +0x9a69 (escolher_vaga_de_tatica), reads (tatica_ativa_do_humano)
- club +0 id, +0xcf manager; staff +0x39 club; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] current human, [0xb63c98] human manager; pointers/ids; reads
- Tactics +0x601 tactic table (0x91-byte rows: +2 the 20 selection slots, ids, −1 empty; +0x56/+0x5a extra slots; +0x5e set piece takers); ids; reads and writes (tirar_da_escalacao, trocar_vagas_da_escalacao)
- Tactics +0x30 undo copies (16 × 0x5c bytes: +0 club, +4 team, +8 the copy), +0x5f0 count (byte); pointers/values; reads and writes +8 and the count (tirar_da_escalacao, trocar_vagas_da_escalacao), reads (tem_copia_da_escalacao)
- staff +0 id; [0xb74c26 + 0x300 × (i16)[0xb77c7a]] current human; [0xae2364] staff count; ids; reads
- board row (0x49 bytes per id at [MANAGER_MANAGER+0]) +6 confidence, +8/+0xa/+0xc ratings (signed words), +0x18 date of the last pressure news; values; reads, writes +0x18 (noticia_da_pressao)
- club +0 id, +0x53 nation, +0x57 division, +0x80 reputation, +0xbf chairman, +0xcf manager; staff +0 id, +4/+8/+0xc names (+0x33 key), +0x28 and +0x3d job, +0x69 non-player (+0x1b); pointers/ids/values; reads
- [0xdd6ec8] date from which manager news are posted; nation +0x11c active; [0xadadfc] active divisions; [0x9cf764] division without news; dates/pointers/ids; reads
- league +0xe1: the prize mode (2 none, 3 through the competition); value; reads (pagar_premios_da_liga)
- league +0x28: the prize fund; value; reads (pagar_premios_da_liga)
- league +0xb1 / +0x3e: the table (0x3b-byte rows, +0 club) and the count; pointer and value; reads (pagar_premios_da_liga)
- club finance +0 (64 bits), +0xa8, +0x148: the balance and the prize totals; value; writes (pagar_premios_da_liga, through 0x5999a0)
- league +0x3a / +0x44 / +0xc / +0x1c: the round, the stage, the stages and the division above; value and pointer; reads (anunciar_situacao_na_liga)
- competition (object of [0xadadfc]) +0x1c / +0x20: the promotion and relegation divisions; id; reads (anunciar_situacao_na_liga)
- [0xdd7f00] the staff records; pointer; reads (registros_pela_situacao_na_liga)
- PlayerStats +0x10..+0x28: the rating and match sums (double; +0x20/+0x28 with match flag +0x4c); value; writes (registrar_partida_nas_estatisticas)
- category record (0x20 bytes) +0 games, +1 sub appearances, +7/+8 goals for/against, +9/+0xa wins/losses, +0xd man of the match, +0xe rating, +0x1c minutes; value; writes (registrar_partida_nas_estatisticas)
- player record (PlayerStats +0x30, 0x27 bytes) +0x20 games, +0x22 goals, +0x24..+0x26 the form (half a byte per match); value; writes (registrar_partida_nas_estatisticas)
- contract +0x3b: the games for the club; value; writes (registrar_partida_nas_estatisticas)
- match line (0x49 bytes from +0x190 + 0x625 x side) +0 staff, +4 player, +0x21..+0x2d counts, +0x30/+0x32/+0x34 off, on and minutes, +0x35 rating; value; reads (registrar_partida_nas_estatisticas)
- record (season 0x4528d0 / all time 0x452710) +0/+1/+5 best position, competition and year; +7/+8/+0xc the worst; value and id; reads and writes (recordes_de_posicao_na_liga)
- Club_Records +0x24/+0x28/+0x2c: the record news queue (6 bytes: id, type, period), capacity and count; pointer and value; reads and writes (recordes_de_posicao_na_liga, noticias_de_recordes)
- competition ([0xae23d0], 0x6b bytes) +0x69: the reputation; value; reads (recordes_de_posicao_na_liga)

**Of interest to:** CMRTE, SGE, CMScout
