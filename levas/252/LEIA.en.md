# Batch 252 — decision dates and club offers: the date order, deciding the due dates and the club's contract offers; loans and squad placement: signing the loan contract, ending the loan and toggling the squad placement; the board and the manager: the board for a new manager, the manager's departure reason and the manager record; who fits the job and morale after a poor match: the manager candidate, the job a person can take and lowering morale after the match; the clubs' shortlists: the formation slots, rebuilding the club's shortlist and suspending the clubs that target the player; transfer listing the player, putting him on the clubs' shortlists and paying the transfer's appearance money; transfer offers: a club rejects the offer, detaching the offer's players and building the cash offer; illnesses and the close season: adding a club illness outbreak, spreading the outbreaks through the squads and the player in the close season; game creation: the starting squad fitness, the injury records with the starting injuries and the per-nation club lists for youngsters; season end and transfers in the records: the fans' player of the year, archiving the nation's statistics and the transfer fee records; international friendlies: the nation pair that plays no friendlies, the order of nations by reputation and of friendlies by date; national teams and the club calendar: whether the club fixture can be scheduled given the call-ups, the order of call-ups by date and the nation qualified for the finals; cups and club divisions: the order of seeds in the draw (descending and ascending) and the club division tier at the end of the season

**Routines:**
- `comparar_datas_de_decisao()` = 0x4dc910 (contract_manager.cpp, cdecl): the qsort order of the decision dates: by date and, on the same day, by the time slice (signed);
- `decidir_as_datas_vencidas()` = 0x4cd9c0 (contract_manager.cpp, thiscall, ret 8): consumes the decision dates due by the given date and slice: each one becomes −1 and, for type 3, the person decides on the contract offers; in DESLIGADAS since 2026-10-30: the detour alone (without running the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the stack residue a later original reads; the copy stays in the code and does not run;
- `ofertas_de_contrato_do_clube()` = 0x4ca300 (contract_manager.cpp, thiscall, ret 8): copies the club's open contract offers (outside states 5 and 6) into a new array of 111 bytes per offer and returns how many;
- `assinar_contrato_de_emprestimo()` = 0x4d01e0 (contract_manager.cpp, thiscall, ret 4): moves the person to the loan club (lists, tactics, squad as a player), marks the reserve bit in the main contract and stores the loan contract in her slot (the existing one, a free one or at the end of the array); recalculates the wage bills;
- `encerrar_o_emprestimo()` = 0x4d07e0 (contract_manager.cpp, thiscall, ret 4): takes the person out of the loan club (lists, disputes, tactics, squad, shortlist), frees the loan contract, recalculates the wage bills and returns her to the parent club with the squad mark;
- `trocar_lugar_no_elenco()` = 0x4cbd20 (contract_manager.cpp, thiscall, ret 0xc): moves the person between the club's first team and reserves or between the club and its partner (A/B side), taking the loan contract into account; 0 with no room at the destination;
- `diretoria_do_tecnico_novo()` = 0x694c80 (manager_manager.cpp, thiscall, ret 4): recalculates the expectations, moves the manager's reputation towards the club's and draws the board confidence and moods from the reputation; clears the changes and flags, records the manager and stores the wealth band; in DESLIGADAS since 2026-10-30: with the detour the career in Rust mode left the reference be54a138 (bisection of the 39 routines of batch 252): the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run;
- `motivo_da_saida_do_tecnico()` = 0x69b350 (manager_manager.cpp, thiscall, ret 4): the reason (0 to 11) for the manager's departure: unsettled players, league position, debt, board changes and moods (with draws) and time in the job;
- `registro_do_tecnico()` = 0x86f5b0 (staff_records.cpp, thiscall, ret 8): finds (or creates, or takes over) the manager's 210-byte record and updates the nations, the current spell and the longest and shortest spells;
- `serve_para_o_cargo()` = 0x69f810 (manager_manager.cpp, thiscall, ret 0x10): whether the person is a realistic candidate for the club's manager job: vetoes and accepts by reputation, nation, contract and time in the job; otherwise her interest in the job against the minimum score (double);
- `pode_assumir_o_cargo()` = 0x4cb990 (contract_manager.cpp, stdcall, ret 8): whether the person fits the job (6 to 15) by age, current job and the player and non-player records; null person: 0;
- `baixar_moral_pela_partida()` = 0x4cb250 (contract_manager.cpp, thiscall, ret 0xc): lowers the player's contract morale after a poor match (by rating, reputation, time at the club, age and contract band) and, with very low morale, may add the wants-to-leave or lack-of-games grievance; injured, on loan or retiring clears the grievances;
- `montar_vagas_da_formacao()` = 0x83c090 (shortlist_manager.cpp, stdcall, ret 4): sets the role of the 40 slots of the club's entry (25 player slots, 15 staff slots) and writes the position mask of the 25 player slots by formation (+7);
- `refazer_lista_do_clube()` = 0x83a730 (shortlist_manager.cpp, thiscall, ret 4): releases the stale targets of the club's slots, reassesses the squad (0x83cb40) and shares out the transfer budget again (0x83b6e0);
- `suspender_clubes_que_miram_o_jogador()` = 0x842b00 (shortlist_manager.cpp, thiscall, ret 8): sets to 100 the entry state of every club that targets the player (0x842180) and, with clear, removes the bid marker from the slots holding him;
- `por_jogador_nas_listas_dos_clubes()` = 0x842c00 (shortlist_manager.cpp, thiscall, ret 8): with the player's filters (record, marked regen, job, career end, nation, untouchable, arrived less than 200 days ago without being listed), tries to put him on the list of each club in the array (0x842e10) up to 5 clubs, except national teams, the contract's club and his own; in DESLIGADAS since 2026-10-30: the detour alone (without running the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the stack residue a later original reads; the copy stays in the code and does not run;
- `por_jogador_a_venda()` = 0x842320 (shortlist_manager.cpp, thiscall, ret 8): marks the contract as transfer listed; the player may come to dislike the manager (likes the club or loyalty against two draws of 100); with news, puts him on the clubs' lists and posts the news;
- `pagar_premio_por_jogos()` = 0x8cb340 (transfer_manager.cpp, thiscall, ret 0x10): finds the appearance money record (club or national team) and, when the appearances match, posts the news, adds to the totals and the former club receives the amount the current one pays; returns 1 when paid;
- `clube_recusa_a_oferta()` = 0x8c36b0 (transfer_manager.cpp, thiscall, ret 0xc): counts the offer's attempt; by the refusing side, a draw(2 × attempts) rejects or withdraws the offer, otherwise it is rescheduled with the delay news;
- `soltar_jogadores_da_oferta()` = 0x8c6ae0 (transfer_manager.cpp, thiscall, ret 4): takes the offer's player and each exchange player out of other offers and collapses the offers for each exchange player;
- `montar_oferta_em_dinheiro()` = 0x8c8740 (transfer_manager.cpp, thiscall, ret 0x14): allocates and initialises a cash offer for the player at the given fee (0x8e78f0), stores it in the output, appeal 50, and returns 1;
- `acrescentar_surto_de_doenca()` = 0x62e820 (injury.cpp, thiscall, ret 0x10): type 4 never; type 3 with 1 in 15, the others with 3 in 4; outside national teams, stores the outbreak (date, type, club, fixture) in the array (grown by 30) and re-sorts it by date;
- `espalhar_surtos_de_doenca()` = 0x62e9e0 (injury.cpp, thiscall): up to the first outbreak after today: one older than five days is removed; the others make random squad players of the club ill (type 3 only on the day, 1 to 4; the others 0 to 2; up to 20 tries) through the new injury 0x62d250;
- `jogador_fora_da_temporada()` = 0x62ce30 (injury.cpp, stdcall, ret 4): 1 without a person; 1 when the club (without the +0x1db mark) is out of its nation's season with less than 40 days to go and there is no future national call-up; otherwise 0;
- `forma_inicial_dos_elencos()` = 0x62a920 (injury.cpp, thiscall): for each club, the pre-season days (0 in season; up to 40); each squad player's fitness = 10000 − 75 × (days − natural fitness/2 − +0x5b/4, between 1 and 40); non-players leave the slot (and the club, job 0xb);
- `registros_iniciais_de_lesao()` = 0x62cf70 (injury.cpp, thiscall): allocates every person's 31-byte record fully fit; players: 1 in 10 of a recurring injury of the drawn type and 1 in 100 of starting injured (0x62d140, 0x62d250); the totals in 0xde1f64;
- `listas_de_clubes_por_nacao()` = 0x7acfc0 (player_regen.cpp, thiscall): one list per nation; each club with a division joins its nation's (a nation with +0x11c: only with +0x80 up to 2), except national teams and the clubs at 0x9d00a4/0x9d00a8;
- `jogador_do_ano_da_torcida()` = 0x452da0 (club_records.cpp, thiscall, ret 8): from the club's squad, with games above the minimum (factor 0.6 falling by 0.1 down to 0.3), the best score (average + 0.025 × +2 + 1/60 × +6, club and player likes); his reference at record +0x371 and 1, or 0;
- `arquivar_estatisticas_da_nacao()` = 0x7c1350 (player_stats.cpp, thiscall, ret 4): for the nation's clubs (up to the first national team), archives (0x7c5fe0) the statistics of every squad player not on loan elsewhere; 1 (0 and the "Error" box without a nation);
- `recordes_de_valor_de_transferencia()` = 0x448760 (club_records.cpp, thiscall, ret 4): the buyer's paid record and the seller's received record, for the season and all time (fee, reference, other club, date, loan); an all-time one above 1000 × (reputation + 300) goes to the news queue and, on the purchase, the manager comes to favour the player;
- `par_de_nacoes_sem_amistoso()` = 0x5cc3e0 (friendly.cpp, thiscall, ret 8): returns 1 when (a, b), (a, −1) or (−1, b) is among the banned pairs (+0x3502, count +0x3506), else 0;
- `ordem_das_nacoes_pela_reputacao()` = 0x5cc5f0 (friendly.cpp, cdecl): qsort comparator of nation pointers by reputation (+0x8e), highest first;
- `ordem_dos_amistosos_pela_data()` = 0x5cc610 (friendly.cpp, cdecl): qsort comparator of friendly records by the date at +3 (0x456770/0x456740), earliest first;
- `pode_marcar_jogo_do_clube()` = 0x776fc0 (national_teams.cpp, thiscall, ret 4): friendly fixture: 1; otherwise 0 when the home or away club (a plain club) has more than 2 players on call-up from a day before to a day after the fixture date, else 1; in DESLIGADAS since 2026-10-30: the detour alone (without running the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the stack residue a later original reads; the copy stays in the code and does not run;
- `ordem_das_convocacoes_pela_data()` = 0x778e90 (national_teams.cpp, cdecl): qsort comparator of records by the date at +0 (0x456770/0x456740), earliest first;
- `acrescentar_nacao_classificada()` = 0x7779b0 (national_teams.cpp, thiscall, ret 8): with nation and finals both non-null, appends the pair to the new qualifiers list (+0x28);
- `ordem_dos_cabecas_de_chave_decrescente()` = 0x522190 (Cup.cpp, cdecl): qsort comparator of cup entries by level (+4), highest first; on a tie by club reputation (+0x80), lowest first;
- `ordem_dos_cabecas_de_chave_crescente()` = 0x5221c0 (Cup.cpp, cdecl): qsort comparator of cup entries by level (+4), lowest first; on a tie by club reputation (+0x80), lowest first;
- `teto_da_divisao_dos_clubes()` = 0x68a980 (league.cpp, thiscall, ret 0x10): for each table row, moves the club byte +0x64 to the value (down or up) when the row status is not the excluded one and when it is the included one (0xfd = none);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (29):** 25 caught. `cpp_a_venda` escaped: what reads whether the player is listed is bit 1 of contract +0x4e (0x5444a0), which the mutation keeps; the high nibble of +0x4f (7 = listed) is not read by anything the automatic career goes through [inference]. `cpp_soltar` escaped: the branch ran, but the player of the completed offer had no other open offer that changed the career: the remaining ones fall later with the end of the transfer [inference]; comparison mode saw no difference either. `cpp10_compra_grande` escaped: no purchase in the career fell between 1000 × (reputation + 200) and 1000 × (reputation + 300), the band the mutation adds; the comparator uses the same rule in its plan and cannot catch it by design. `cpp10_gostar_forcado` escaped: forcing only matters when the buying manager has no free slot in his favourites list (0x547a10 draws who leaves); with batch 252's DESLIGADAS this did not happen by 2002-06-01 (in the first chain, before them, the mutation was caught on 2002-002). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2f65fd6b68d1027b).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (be54a1386f51073a).

**Redo**, on top of batch 251's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 252`

**Fields touched:**
- CONTRACT_MANAGER +0xc, +0x14, +0x18: the decision date array (14 bytes: +0 person id or −1, +4 date, +0xc slice, +0xd type), the count and the next index; pointer and values; reads and writes −1 and the index (decidir_as_datas_vencidas), reads (comparar_datas_de_decisao)
- CONTRACT_MANAGER +0x34: the contract offer list (111 bytes: +4 club id, +0x35 state in the low 6 bits); pointer; reads (ofertas_de_contrato_do_clube)
- [0xae2d0c] person's 79-byte record +0, +4: the main and loan contracts; index; reads and writes +4 (all three)
- CONTRACT_MANAGER +0, +4, +8: the contract array (0x50 bytes), the capacity and the count; pointer and values; reads and writes (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)
- contract +0x35 bit 0x40: reserve; value; reads and writes (all three)
- [0xae1080] the global contract slot; pointer; writes (assinar_contrato_de_emprestimo, encerrar_o_emprestimo)
- person +0x39 (current club), +0x3e (join date), +0x5e (squad marks), +0x6d (status, 0x10 becomes 4); pointer and values; reads and writes (encerrar_o_emprestimo), reads +0x39/+0x5e and restores +0x3e (assinar_contrato_de_emprestimo)
- MANAGER_MANAGER [this] board row (0x49 bytes per club) +1, +2, +3, +4, +5, +6, +8, +0xa, +0xc, +0x10..+0x16, +0x28, +0x30, +0x38, +0x40; values; writes (diretoria_do_tecnico_novo), reads +2, +6..+0x16, +0x30, +0x38 (motivo_da_saida_do_tecnico)
- non-player record +8, +0xa, +0xc: the home, current and world reputations; value; reads and writes (diretoria_do_tecnico_novo)
- Shortlist_Manager +0xdc (9 bytes per club) +4; value; writes 2 (diretoria_do_tecnico_novo)
- contract +0x45 bit 0x10000000: the complaint; value; reads (motivo_da_saida_do_tecnico)
- Staff_Records +8: the manager records (210 bytes: +0x4a..+0x7e the spells, +0x86 owner, +0xba/+0xbe points, +0xc2/+0xc6 nations); pointer; reads and writes (registro_do_tecnico)
- [0xdd6ec8] the date from which spells count; value; reads (registro_do_tecnico)
- person +0x18 (age), +0x1a (nation), +0x22, +0x24/+0x28 (national team and job there), +0x39 (club), +0x3d (job), +0x3e/+0x40 (join date), +0x57, +0x59, +0x61 (player), +0x65, +0x69 (non-player); values and pointers; reads (all three)
- contract +0x2d/+0x2f (date), +0x3d, +0x3e, +0x3f, +0x44 (morale), +0x45 (grievances), +0x49, +0x4f (band); values; reads and writes +0x44 (baixar_moral_pela_partida), reads (serve_para_o_cargo)
- club +0x53 (nation, +0x85 the weight), +0x57 (league, +0x69 the level), +0x80 (reputation), +0xcf/+0xd3; reads (serve_para_o_cargo, baixar_moral_pela_partida)
- [0xb63d8a] transfer window table (72 bytes per key nation, the dates at +0 and +0xf); reads (baixar_moral_pela_partida)
- [0xae1080] the global contract slot; pointer; writes (baixar_moral_pela_partida)
- club's 9-byte entry in Shortlist_Manager (+0xdc; id × 9) +0: the 40 slots of 0x25 bytes; pointer; reads (all three)
- club entry +7: the formation (1..9; otherwise the default); value; reads (montar_vagas_da_formacao)
- club entry +8: the state (100 = suspended for the player); value; writes 100 (suspender_clubes_que_miram_o_jogador)
- 0x25-byte slot +0: the role (0xb player; 6, 7, 8, 10, 9 staff); value; writes (montar_vagas_da_formacao)
- slot +1: the position mask (16 bits); value; writes (montar_vagas_da_formacao)
- slot +0x24: bits 2..4 = 0 and bit 1 in the staff slots; value; reads and writes (montar_vagas_da_formacao)
- slot +0xf: the target player id; id; reads (suspender_clubes_que_miram_o_jogador)
- slot +0x17: the bid marker (−1 none); value; reads and writes −1 (suspender_clubes_que_miram_o_jogador)
- player +0x61: the position record (+0xb rating, signed 16 bits); pointer; reads (por_jogador_nas_listas_dos_clubes)
- [0xae2d0c] 79-byte record +0xb: bits 0x30 (0x10 mark) and the low nibble (how many clubs already list him); value; reads (por_jogador_nas_listas_dos_clubes)
- player +0x3d, +0x24, +0x28, +0x1a, +0x39, +0x3e/+0x40: job, career end, nation, club, arrival date; value and pointer; reads (por_jogador_nas_listas_dos_clubes)
- [0xae2d1c] and [0xae235c]: the club id array and its count; id; reads (por_jogador_nas_listas_dos_clubes)
- contract +0x4e bit 1 and +0x4f high nibble (7 = listed); value; reads and writes (por_jogador_a_venda), reads (por_jogador_nas_listas_dos_clubes)
- contract +0x44: the loyalty (signed byte); value; reads (por_jogador_a_venda); +0x45; value; reads (por_jogador_nas_listas_dos_clubes)
- [0xae1080] the last contract found; pointer; writes (por_jogador_nas_listas_dos_clubes, pagar_premio_por_jogos)
- TRANSFER_MANAGER +0x88c/+0x894 and +0x898/+0x8a0: the appearance money records (0xd bytes: +0 person, +4 amount, +8 appearances, +9 former club) for club and national team, and the counts; pointer and value; reads (pagar_premio_por_jogos)
- offer (0x8e5680, 0x41 bytes) +0x2c: the state (0x11 refused by the other side, 0x12 by the seller); value; reads and writes (clube_recusa_a_oferta)
- offer +0x2e: the attempts; value; reads and writes (clube_recusa_a_oferta)
- offer +8 and +0xc: the selling club id and the player id; id; reads (clube_recusa_a_oferta, soltar_jogadores_da_oferta)
- offer +4: the exchange player list (items: id); pointer; reads (soltar_jogadores_da_oferta)
- offer +0x1b: the fee; value; written by 0x8e78f0 (montar_oferta_em_dinheiro)
- contract +0x35 (& 0x3f; 5 skips); value; reads (clube_recusa_a_oferta); [0xae1080] the last contract; pointer; writes
- Injury +0x1c, +0x20, +0x24: the outbreak array (17 bytes), its capacity and count; pointer and value; reads and writes (acrescentar_surto_de_doenca, espalhar_surtos_de_doenca)
- outbreak +0, +8, +9, +0xd: the date, the illness type, the club index and the fixture id (−1 none); value and id; writes (acrescentar_surto_de_doenca), reads (espalhar_surtos_de_doenca)
- Injury +0: the 31-byte records per player; +0x1d the current illness; pointer and value; reads (espalhar_surtos_de_doenca)
- club +0xd7: the 50 squad slots; pointer; reads (espalhar_surtos_de_doenca)
- club +0x1db: mark (−1 = none); value; reads (jogador_fora_da_temporada)
- person +0x24, +0x39, +0x5e (bit 3): national team, club and the call-up class; pointer and value; reads (jogador_fora_da_temporada)
- [0xae2c90] the game date; value; reads
- Injury +0: the 31-byte records per person ([0xae2364]); pointer; writes (registros_iniciais_de_lesao), reads (forma_inicial_dos_elencos)
- injury record +0, +8, +0xa, +0xc, +0xe, +0x12, +0x14, +0x18, +0x1d, +0x1e: date, fitness, condition, wear, injury, recurring, history, illness, 50; value; writes (registros_iniciais_de_lesao; +8 forma_inicial_dos_elencos)
- player +0x34: natural fitness; signed value; reads (forma_inicial_dos_elencos)
- person +0x5b, +0x61, +0x39, +0x3d: person attribute, player, club and job; value and pointer; reads (all three), writes +0x39/+0x3d (forma_inicial_dos_elencos)
- club +0xd7: the 50 squad slots; pointer; reads and writes 0 (forma_inicial_dos_elencos)
- club +0x53, +0x57, +0x80: nation, division and reputation; pointer and value; reads (listas_de_clubes_por_nacao)
- nation +0x71, +0x11c: the marks that put clubs on the list; value; reads (listas_de_clubes_por_nacao)
- PLAYER_REGEN +0: the club lists per nation (nation id); pointer; writes (listas_de_clubes_por_nacao)
- Club_Records +0x1c: the 106-byte records per club; +0x10 the games (signed); pointer and value; reads (jogador_do_ano_da_torcida)
- records entry +0x371: the fans' player of the year reference (0x18 bytes); value and id; writes (jogador_do_ano_da_torcida)
- records entry +0x313..+0x33b and +0x33c..+0x364: paid and received record (fee, reference, club, date, loan); value and id; reads and writes (recordes_de_valor_de_transferencia)
- Club_Records +0x24, +0x2c: the record news queue (6 bytes: club id, type) and its count; pointer and value; reads (recordes_de_valor_de_transferencia)
- statistics (0x7c3920, category 6) +0, +2, +6, +0xe: games, +2, +6 and the rating sum; value; reads (jogador_do_ano_da_torcida)
- transfer +4, +8, +0xc, +0x1b: loan, buyer index, person id and fee; value and id; reads (recordes_de_valor_de_transferencia)
- club +0x80, +0xbf, +0xd7: reputation, manager and the 50 slots; value and pointer; reads
- [0xae1080] the last transfer's contract; pointer; writes (recordes_de_valor_de_transferencia)
- friendlies +0x3502: the banned pairs array (2 nation ids per pair, −1 = any); pointer; reads; +0x3506 the count (signed byte); value; reads (par_de_nacoes_sem_amistoso)
- nation +0x8e: the reputation; value; reads (ordem_das_nacoes_pela_reputacao)
- friendly record (20 bytes) +3: the date (day, year); value; reads (ordem_dos_amistosos_pela_data)
- fixture +0x28/+0x2a: the date (year, day); +0x14 the competition; +0x1c, +0x20 home and away clubs; value, id and pointer; reads (pode_marcar_jogo_do_clube)
- NationalTeams +0x20: the call-ups per club (6 bytes by id: +0 9-byte records, +5 the count); pointer; reads (pode_marcar_jogo_do_clube)
- call-up (9 bytes) +0: the date; +8 how many called up; value; reads (pode_marcar_jogo_do_clube, ordem_das_convocacoes_pela_data)
- [0xae235c] clubs, [0xae2350] nations: the limit of plain clubs (without national and under-21 sides); value; reads (pode_marcar_jogo_do_clube)
- NationalTeams +0x28: the list (LLIST) of new qualifiers (nation, finals); pointer; writes (acrescentar_nacao_classificada)
- cup entry +0: the club; pointer; +4 the level; value; reads (both orders)
- club +0x80: the reputation; value; reads (both orders)
- league +0x3e: the table row count; +0xb1 the table (0x3b bytes: +0 the club, +0x37 the end-of-season status); value and pointer; reads (teto_da_divisao_dos_clubes)
- club +0x64: the division tier (signed byte); value; reads and writes (teto_da_divisao_dos_clubes)

**Of interest to:** CMRTE, SGE, GPF2, CMScout
