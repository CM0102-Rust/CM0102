# Batch 253 — the board and the manager: considering the sacking, issuing the ultimatum and removing the manager; the FA and the board: reviewing the national team in a competition, the club in financial trouble and the board confidence band; career records: promotion, relegation and qualifying for the next round; career records: the transfer, the knock-out and the failure to qualify; the transfer dates: the offer's completion date, the first day the rules allow and the fixed transfer window; the transfer money: the compensation for the person, paying the fee and recording the clauses, and whether the selling club must accept; the transfer contracts: the work permit on a club move, completing an accepted contract offer and offering a contract to the offer's player; the transfer offers: collapsing an offer, making a loan offer and completing the day's future transfers; regens: the non-player data donor score, the retirement order and the regen queue order; player statistics and the pending retirement: the category record, creating the record and whether the person is on the retirement list; the cup: the draw of the round teams, the ties at a shared stadium and the scale by round; the fixture side, the nation coefficient order and the scout request; the stored selection: clearing a slot and the set piece taker

**Routines:**
- `pensar_em_demitir_o_tecnico()` = 0x69aba0 (manager_manager.cpp, thiscall, ret 8): with low board confidence and the manager in the job for 150 days: the limit from the division table, the runs and the manager mark; chairman draws hold back, issue the ultimatum, let the manager leave or sack him (the departure queued, reason 0x69b350), or look for a better candidate with the pressure story;
- `dar_o_ultimato_da_diretoria()` = 0x6ac560 (manager_manager.cpp, thiscall, ret 4): the club's first match in the next 8 weeks (not today, still to play): the ultimatum story with the opponent and the club in the ultimatum list; returns 1, or 0 without a match;
- `tirar_o_tecnico_do_cargo()` = 0x69b790 (manager_manager.cpp, thiscall, ret 0x10): the manager leaves: the departure date, the squad and the staff, the contract (outside national teams), the story (outside type 3), confidence at 3500 (unless kept), the job change record, the fines and the squad contracts' grievances and flags;
- `avaliar_a_selecao_na_competicao()` = 0x6a5610 (manager_manager.cpp, thiscall, ret 8): the FA confidence from the manager grade in the competition (−3..3, virtual +0x74), the assessment story and, by time in the job, reputation and draws, the manager departure or the FA considering the sacking (0x69aba0);
- `clube_em_apuro_financeiro()` = 0x69eb50 (manager_manager.cpp, thiscall, ret 4): manageable club, not a national team, with board confidence at row +8 under 3000 and a negative balance;
- `faixa_da_confianca_da_diretoria()` = 0x690b70 (manager_manager.cpp, thiscall, ret 4): the 0..6 band of the board confidence (limits 3000, 3500, 4275, 6500, 8500, 9500);
- `registrar_o_acesso()` = 0x869480 (staff_records.cpp, thiscall, ret 0xc): club reputation +500 (up to 10000) and +0x7b; the manager ratings by the competition importance (× 10/61, with a minimum); the manager, chairman and club histories (type 8); the manager points; squad morale +15 and bond +75 and renewal of the expiring contracts;
- `registrar_o_rebaixamento()` = 0x86a1c0 (staff_records.cpp, thiscall, ret 8): club reputation −500 (minimum 750); the manager ratings drop by the competition importance (1..10000); the manager, chairman and club histories (type 9); squad morale −15, reputations × 0.9 (contracts 1 and 2, renewing the expiring ones) or × 0.95 (3 to 5);
- `registrar_a_classificacao_na_fase()` = 0x86bdd0 (staff_records.cpp, thiscall, ret 0x14): the manager history (type 3, with the round and the base round) and his points: importance² × weight in the year, the month and the cup (national team) or the league (club);
- `registrar_a_transferencia()` = 0x86cd70 (staff_records.cpp, thiscall, ret 4): in the buying and selling managers' records: the biggest signing or sale, the counts, the 64-bit totals and the exchange players; the buy (0x64), sale (0x65) and transfer (0x66) histories of the managers, the person and the exchange players;
- `registrar_a_eliminacao()` = 0x86c000 (staff_records.cpp, thiscall, ret 0x14): the manager history (type 4, with the round and the base round) and the end of the national team duty in the stored competition;
- `registrar_a_nao_classificacao()` = 0x86c1d0 (staff_records.cpp, thiscall, ret 0x10): the manager history (type 7, with the round) and the end of the national team duty in the stored competition;
- `data_de_conclusao_da_oferta()` = 0x8cee70 (transfer_manager.cpp, thiscall, ret 8): by the offer's timing type (+0x30): today, the first allowed day (0x8cf0e0), the seller's season end plus 7 days, the day after the contract ends (never before today) or the start of the fixed window; returns the reason;
- `primeiro_dia_permitido_da_oferta()` = 0x8cf0e0 (transfer_manager.cpp, thiscall, ret 8): steps day by day, from today (or the contract end, in type 4) up to a year, until the rules of the buyer's and seller's nations allow it (method +0xc); with no day, returns the first reason;
- `janela_fixa_de_transferencias()` = 0x8d0bb0 (transfer_manager.cpp, stdcall, ret 8): the start (today, when inside) and the end of the current or next window: 1 to 31 July and 15 December to 15 January;
- `compensacao_pela_pessoa()` = 0x8d0780 (transfer_manager.cpp, stdcall, ret 0xc): staff on a type 2 contract: what is left of the contract (1000 to 1000000); new rules: the average with the player value; between clubs of nation [0x9cf4a8] or, under 24, between British nations: half the value (0x4d0c60);
- `pagar_transferencia_e_clausulas()` = 0x8ca8a0 (transfer_manager.cpp, thiscall, ret 4): adds the fee to the totals, the buyer pays and the seller receives the first instalment, stores the remaining instalments (list +0x872) and the sell-on and appearance clauses, with the person's flags;
- `clube_vendedor_deve_aceitar()` = 0x8cd240 (transfer_manager.cpp, stdcall, ret 0xc): 1 when the fee covers an active release clause (reason 0x1b) or the seller in financial trouble gets far more than the value (reason 0x1c); without a seller, 1;
- `permissao_de_trabalho_na_mudanca()` = 0x8c7e20 (transfer_manager.cpp, thiscall, ret 8): when the new club needs a permit: updates the existing one (nation, year) or grants a new one (0x8c7070); otherwise: removes any permit and re-sorts the array;
- `concluir_oferta_de_contrato_aceita()` = 0x8c38b0 (transfer_manager.cpp, thiscall, ret 4): without the work permit, the news of the collapsed offer or the refused permit and 0; otherwise signs (0x4cf720), the club pays the signing-on fee and 1;
- `oferecer_contrato_ao_jogador_da_oferta()` = 0x8c1e40 (transfer_manager.cpp, thiscall, ret 0xc): builds the club's contract (0x4cda70, factor 1) and adds the contract offer; without one, withdraws the offer (main player) or drops the player from the exchange adding his value;
- `desfazer_oferta()` = 0x8c44e0 (transfer_manager.cpp, thiscall, ret 0x10): state 0x10, closes the proposals, releases the bids, posts the news, schedules the removal, clears the flags, the contract offer and the exchange contracts;
- `fazer_oferta_de_emprestimo()` = 0x8bed60 (transfer_manager.cpp, thiscall, ret 0x18): builds the loan offer and the contract from the current one (wage percentage), adds both and returns the offer slot; in DESLIGADAS since 2026-10-30: with the detour the career in Rust mode left the reference be54a138 (bisection of the 38 routines of batch 253); the original builds the offer and the contract in locals the constructors (0x8e5680, 0x85e410) do not fully fill, and the stack residue of those bytes goes into the stored offer, where the copy puts zero; the copy stays in the code and does not run;
- `concluir_transferencias_futuras()` = 0x8cbbe0 (transfer_manager.cpp, thiscall): the future transfers whose date has come: collapses those the restrictions block, completes the others (0x8c9390) and removes them from the array;
- `nota_do_doador_de_dados()` = 0x7a7150 (player_regen.cpp, stdcall, ret 4): −10000 when under 31 or flagged; else a draw by age, ability over the club reputation, a draw by the cube of +0x2f, the attributes and the squares of +0x5b and +0x57, and the final draw (+75 without club +0xcf; without a club, + (+0x22)² and doubled against a 10000 draw);
- `ordem_das_aposentadorias()` = 0x7ae2a0 (player_regen.cpp, cdecl): qsort comparator of the retirement list: by the effective date (the announcement +8 if not past yet, else the retirement +0x10), year and day; on a tie by id;
- `ordem_da_fila_de_regens()` = 0x7ae240 (player_regen.cpp, cdecl): qsort comparator of the regen queue: by year (+6), day (+4) and id (+0);
- `registro_da_categoria()` = 0x7c3920 (player_stats.cpp, thiscall, ret 8): the 0x20-byte record of the player's category (1..6), through the index in the 39-byte entry of his id; 0 without an index;
- `criar_estatisticas()` = 0x7c5d80 (player_stats.cpp, thiscall, ret 8): the player's category record; without an index, the first free record (+0xe = 0xffff) or a new one at the end (grows by 100), zeroed;
- `aposentadoria_marcada()` = 0x7ae350 (player_regen.cpp, thiscall, ret 4): 1 when the person's id is on the retirement list, else 0;
- `copa_sortear_times_da_rodada()` = 0x51e220 (Cup.cpp, thiscall, ret 4): takes the teams out of the round ties, shuffles them and puts each one in a drawn tie (empty ones first), drawing the side when both slots are free, and sets the link to the next round;
- `copa_separar_estadio_compartilhado()` = 0x522610 (Cup.cpp, thiscall, ret 4): swaps home and away in the later tie whose team shares a stadium with a team of an earlier tie of the round, and sets the side in the linked ties;
- `escala_pela_rodada()` = 0x4bf850 (comp_util.cpp, cdecl): the round value between two limits: the average with a single round, otherwise (high − low) × round / (rounds − 1); outside 0..100, the error box;
- `lado_do_jogo_para_o_clube()` = 0x4afa80 (comp_util.cpp, cdecl): the club side in the fixture: 1 home, 2 away, 3 neutral, by the fixture stadium and the two clubs' stadiums (national sides: by the stadium nation);
- `ordem_dos_coeficientes_das_nacoes()` = 0x4afba0 (comp_util.cpp, cdecl): the qsort order of the nation coefficients: highest first;
- `pedido_de_observacao()` = 0x7f8d70 (scout_manager.cpp, thiscall, ret 0xc): whether the club (or the current human's) asked to scout the player, with fewer than 3 reports and, if asked, with the date already due;
- `limpar_vaga_da_escalacao()` = 0x896e90 (tactics.cpp, thiscall, ret 0xc): the club selection slot in the tactics table becomes empty (−1) and the stored slots of the same club and team are released;
- `marcar_batedor_de_bola_parada()` = 0x897bb0 (tactics.cpp, thiscall, ret 0x14): stores the taker of a set piece (first or second choice); without a person, clears it and the second choice moves up to the first;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (30):** 28 caught. `cpq1_tecnico_com_contrato` escaped: 0x690890 (leave the staff), called just before, already removes the manager's contract (call 0x4d0b50 at 0x69089e, unconditional): the second call finds no contract. `cpq2_apuro_sem_saldo` escaped: no club in the career reached the question with board confidence below 3000 and a positive balance [inference]; the comparator checks the value and did not diverge in comparison mode either. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2f65fd6b68d1027b).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (be54a1386f51073a).

**Redo**, on top of batch 252's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 253`

**Fields touched:**
- board row [this] + id×73: +2 bit 2 (departure flagged), +6 confidence, +0x18 date of the last story; values; reads and writes (pensar_em_demitir_o_tecnico), writes +6 = 3500 (tirar_o_tecnico_do_cargo)
- MANAGER_MANAGER +4, +0x10: the departure list (6 bytes) and the ultimatum list (4 bytes, the club id); pointer; appends (pensar_em_demitir_o_tecnico, dar_o_ultimato_da_diretoria)
- manager non-player (+0x69) +0xa: job security; value; reads and writes −250 with a floor (pensar_em_demitir_o_tecnico)
- chairman non-player (+0xbf, +0x69) +0x1d: patience; value; reads (pensar_em_demitir_o_tecnico)
- division table row (0x4a4850) +4 position, +0x14..+0x18 last results, +0x19/+0x41/+0x55/+0x2d runs, +0x74; values; reads (pensar_em_demitir_o_tecnico)
- staff +0x29 / +0x3e: national team / club job date; value; writes today (tirar_o_tecnico_do_cargo)
- contract +0x40 (flags 0x80, 0x100) and +0x45 (grievances 0x8000000, 8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000, 0x10000000); values; clears (tirar_o_tecnico_do_cargo)
- [0xdd6ec8] date from which boards consider sackings; value; reads (pensar_em_demitir_o_tecnico)
- board row [this] + id×73: +6 confidence (the FA one for national teams), +8 financial confidence, +0xe; values; reads and writes +6 (avaliar_a_selecao_na_competicao), reads +6 (faixa_da_confianca_da_diretoria) and +8 (clube_em_apuro_financeiro)
- MANAGER_MANAGER +4: the departure list (6 bytes: 1, reason 6 or 0, id); pointer; appends (avaliar_a_selecao_na_competicao)
- club +0x80: reputation; value; reads (avaliar_a_selecao_na_competicao)
- finance record (0x5a0590 of [0xae2a38]) +0: 64-bit balance; value; reads (clube_em_apuro_financeiro)
- club +0x80: reputation; value; reads and writes ±500 with limits (registrar_o_acesso, registrar_o_rebaixamento)
- club +0x7b, +0x73; values; reads, writes +0x7b (registrar_o_acesso)
- manager non-player (+0xcf, +0x69) +8, +0xa, +0xc: ratings; values; reads and writes (registrar_o_acesso, registrar_o_rebaixamento)
- competition +0x69: importance; value; reads (all three); object in [0xadadfc] +0x1c (club competition record, 0x6b bytes in [0xae23d0]) and +0x42; reads (registrar_o_acesso)
- player record (+0x61) +0x45 morale (0x544710), +9/+0xb/+0xd reputations; values; reads and writes (registrar_o_acesso: morale; registrar_o_rebaixamento: morale and reputations)
- contract (0x4cf5f0) +0x44 bond, +0x4f type (>> 4), +0x25 date; values; reads, writes +0x44 (registrar_o_acesso), reads (registrar_o_rebaixamento)
- manager points record (0x86f5b0, 210 bytes) +0xb6 year, +0xba league, +0xbe cup, +0xca month; values; adds (registrar_o_acesso, registrar_a_classificacao_na_fase)
- Staff_Records +0 and +0xc (staff history file and lists), +4 and +0x10 (club ones): entries of an 8-byte date + club, type, competition; appends (all three)
- manager points record (0x86f5b0, 210 bytes): +0..+0x24 biggest signing (person, three +0x33 ids, date, value +0x18, exchange +0x1c, seller, buyer), +0x25..+0x49 biggest sale (value +0x3d), +0xa0/+0xa2 counts, +0xa4/+0xac 64-bit totals; values; reads and writes (registrar_a_transferencia)
- transfer +4 exchange list (LLIST: cursor +0xa reset), +8 buyer, +0xc person, +0x11 type; ids and pointer; reads (registrar_a_transferencia)
- [0xae1080] the current contract; pointer; writes (registrar_a_transferencia)
- person +0x52: value; reads (registrar_a_transferencia); +0x39 contracted club, +0x3d job; reads
- Staff_Records +0 and +0xc: staff history file and lists; appends (all three)
- transfer offer +0x30: the timing type (& 0xf: 0 today, 1 and 4 by the rules, 2 season end, 3 contract end, 5 fixed window); value; reads (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)
- offer +4: the list of part-exchange players (only whether the seller is consulted); pointer; reads (primeiro_dia_permitido_da_oferta)
- offer +8 / +0xc: the buying club and the person; id; reads (the first two)
- contract (0x4cf5a0) +4: the club; +0x2d: the end (day, year); +0x35 & 0x3f: the type (5 does not count); id and value; reads
- [0xae1080] the last contract found; pointer; writes (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)
- staff +0x61: the player data (without it, today); pointer; reads (primeiro_dia_permitido_da_oferta)
- club +0x53: the nation (transfer rules among the 28 at TRANSFER_MANAGER +0x8ac, +0xa the nation); pointer; reads
- [0xae2c90] today's date (day of year, year); value; reads (all three)
- transfer offer +8 / +0xc: the buyer and the person; id; reads (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)
- offer +0x1b: the fee; +0x40: the instalments; +4: the part exchange; value and pointer; reads
- offer +0x3f: the sell-on percentage; +0x35/+0x39 and +0x3a/+0x3e: the club and international appearance money; value; reads (pagar_transferencia_e_clausulas)
- TRANSFER_MANAGER +0x872: the instalment list (13 bytes); +0x86e: the person's 8-byte record (flags 8, 2, 4); pointer and value; writes (pagar_transferencia_e_clausulas)
- club finances (0x5a0590) +0x1c: adds ± 5/21 of the instalment; +0x165: the financial trouble flag (1); value; writes and reads
- contract (0x4cf5a0) +4 the club; +0xc the wage; +0x21 the release fee; +0x2d the end; +0x35 the type; id and value; reads
- staff +0x61 / +0x69: the player and non-player data; pointer; reads (compensacao_pela_pessoa)
- club +0x53: the nation; +0x80: the reputation; pointer and value; reads
- [0xae233c] the new transfer rules (1); value; reads (compensacao_pela_pessoa)
- [0xae1080] the last contract found; pointer; writes (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)
- TRANSFER_MANAGER +0x93a / +0x93e: the work permits (14 bytes: +0 person, +4 and +6, +8 year, +0xa nation) and the count; pointer and value; reads and writes (permissao_de_trabalho_na_mudanca)
- TRANSFER_MANAGER +0x86e: person's 8-byte record, bit 0x20 (has a permit); value; reads and writes (permissao_de_trabalho_na_mudanca)
- nation (0x122-byte records at [0xae23a8]) +0x76: the 0xe marker; value; reads (permissao_de_trabalho_na_mudanca)
- contract offer +0 / +4: the person and the club; +8: the signing-on fee; +0x61: the job; id and value; reads (concluir_oferta_de_contrato_aceita)
- transfer offer +0xc: the main player; +0x11: the type (0xb); +0x1b: the fee; +0x20 / +0x24: the contract offer slot and value; id and value; reads and writes +0x1b (oferecer_contrato_ao_jogador_da_oferta)
- staff +0x52: the player value; value; reads (oferecer_contrato_ao_jogador_da_oferta)
- club +0x53: the nation; pointer; reads
- [0xae1080] the last contract found; pointer; writes (concluir_oferta_de_contrato_aceita)
- transfer offer +0x2c: the state (0x13, 0x10, 0x14...); +0x2e: the step; value; reads and writes (desfazer_oferta, concluir_transferencias_futuras)
- offer +0 / +4: the lists of rival offers and of exchange players (LLIST, cursor +0xa); pointer; reads (desfazer_oferta)
- offer +0x10: cleared on collapse; +0x20 / +0x24: the contract offer slot and value; value; writes and reads
- TRANSFER_MANAGER +0x86e: person's 8-byte record, bits 0x10 and 1; value; writes (desfazer_oferta)
- TRANSFER_MANAGER +0x920 / +0x928: the future transfers (12 bytes: date, offer index) and the count; pointer and value; reads and writes (concluir_transferencias_futuras)
- TRANSFER_MANAGER +0: the offers (pointers by index); +0x856: the Shortlist_Manager; pointer; reads
- [0xae1080] the last contract found; pointer; writes (desfazer_oferta, concluir_transferencias_futuras)
- person +0x18, +0x22, +0x56..+0x5d, +0x69: age, an unsigned byte, attributes (signed bytes) and the excluding flag; value; reads (nota_do_doador_de_dados)
- person +0x39, +0x61: club and player; pointer; reads (nota_do_doador_de_dados)
- player +0xb, +0xd, +0x2f: current and potential ability (signed) and a signed byte; value; reads (nota_do_doador_de_dados)
- club +0x80, +0xcf: reputation and a pointer that gives +75 when missing; value and pointer; reads (nota_do_doador_de_dados)
- [0xae2d0c] 79-byte record per person, +0xb bits 0x30; value; reads (nota_do_doador_de_dados)
- retirement (0x1a bytes) +0, +8, +0x10: id, announcement date and retirement date (signed day, unsigned year); id and value; reads (ordem_das_aposentadorias)
- [0xae2c90], [0xae2c92]: game day and year; value; reads (ordem_das_aposentadorias)
- regen queue (0xc bytes) +0, +4, +6: id, day and year; id and value; reads (ordem_da_fila_de_regens)
- PlayerStats +0x30: the per-player index (39 bytes per id; +0..+0x14 the indexes of the 6 categories, −1 without a record); pointer and value; reads (registro_da_categoria) and writes (criar_estatisticas)
- PlayerStats +4, +8, +0xc, +0x38: the 0x20-byte records, capacity, count and free ones; pointer and value; reads and writes (criar_estatisticas)
- statistics record +0xe: 0xffff in a free record; value; reads (criar_estatisticas)
- PLAYER_REGEN +4, +8: the retirement list (0x1a bytes, +0 the id) and its count; pointer and value; reads (aposentadoria_marcada)
- Cup +0xa3: the rounds (0x68 bytes: +0x18 team slots, +0x1a ties); reads (the three cup ones)
- Cup +0xad: the ties per round (0x35 bytes: +0/+2 teams, −1 no team, −2 free slot; +0xe/+0xa and +0x10/+0xc the linked tie in the next round; +6 source tie, +8 side, +0x12 source round); reads and writes (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)
- Cup +0xa7: the teams (6 bytes: +0 club, +4 level); reads (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)
- club +0x69: the stadium; pointer; reads (copa_separar_estadio_compartilhado: two clubs with the same non-null stadium)
- fixture +0x18: the fixture stadium (0 without one); +0x1c/+0x20 home and away clubs; pointers; reads (lado_do_jogo_para_o_clube)
- club +0x69: the stadium; club +0x53: the nation; stadium +0x38 → nation +0x1f; pointers; reads (lado_do_jogo_para_o_clube)
- 0x5d5f20 temporary array (12 bytes: +0 nation, +4 coefficient as a double); reads (ordem_dos_coeficientes_das_nacoes)
- SCOUT_MANAGER +0x5c, +0x64: the scout requests (17 bytes: +0 club id, +4 player id, +8 date, +0x10 reports) and the count; reads (pedido_de_observacao)
- [0xb74c26 + 0x300 × [0xb77c7a]] +0x39: the current human's club; [0xae2c90] today; reads (pedido_de_observacao)
- Tactics +0x601: the tactics table (0x91-byte rows, row from 0x8982b0): +2 the 20 selection slots (person id or −1), +0x5e the takers (5 set pieces × 2 choices); id; reads and writes (limpar_vaga_da_escalacao, marcar_batedor_de_bola_parada)
- Tactics +0x30: 16 stored slots of 0x5c bytes (+0 club, +4 team, +8 used) and +0x5f0 the count; reads and writes 0 / −1 (limpar_vaga_da_escalacao)

**Of interest to:** CMRTE, SGE, GPF2, CMScout
