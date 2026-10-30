# Batch 251 — board confidence: the transfer deal, the confidence after the transfer and the ultimatum; contracts: removing the main contract, resubmitting the held offers and the month of the club's season end; manager points: the turn of the month and the year, the end of the competition season and the manager of the month; leaving clubs: the managers' pending departures, removing the player from the squad and removing the person from the job; the new news item and the supporters' and board's confidence after a media story; the AI manager's response to a media story, marking a news item read and finding a news item by id; the player's news after a media story: gratitude to the manager, request for backing and reaction to criticism; the shortlist position lists: building the lists, sorting them every day and the clubs interested in the player; the shortlist and loans: adding the pair to the shortlist, predicting the selling club's response and the loan period; transfers: the offer response delay, paying the month's instalments and removing the offer; bans on a club move: adding a ban, dropping the slots that do not apply at the new club and restoring the held bans when a loan ends; the player's languages (adding and improving) and resetting a regenerated player's statistics; national teams: the nation's current competition and the clubs playing on a date; discipline: removing a regenerated player

**Routines:**
- `avaliar_o_negocio_da_transferencia()` = 0x696570 (manager_manager.cpp, thiscall, ret 0xc): flags the deal for buyer and seller by the fee/player value ratio (bands for values above or up to 250,000, a fee up to (reputation × 0.2)² is fair), adds and subtracts the value from each one's net spend and flags the club's and chairman's favoured or disliked player;
- `confianca_da_diretoria_na_transferencia()` = 0x6978f0 (manager_manager.cpp, thiscall, ret 4): changes the board confidence (+6, 1..10000) and moods (+0x14, +0x16) of the buyer and the seller from the deal flags and removes the player from the seller chairman's favoured people;
- `resolver_o_ultimato_da_diretoria()` = 0x6ac470 (manager_manager.cpp, thiscall, ret 4): removes the club from the ultimatum list and, when the manager has been in the job over 100 days, queues his departure (reason 2, or 7 without games in the table) and returns 1;
- `tirar_o_contrato_principal()` = 0x4d0b50 (contract_manager.cpp, thiscall, ret 4): frees the person's main contract entry (−1), puts −1 in her slot of the 79-byte record and recalculates the wage bill of the contract's club;
- `reenviar_ofertas_guardadas()` = 0x4cf4a0 (contract_manager.cpp, thiscall, ret 4): resubmits the person's held offers as live offers (with the transfer of the same club, state 0xf becomes 8; without a transfer, with −1), removes each one from the list and returns how many;
- `mes_do_fim_da_temporada_do_clube()` = 0x4d45f0 (contract_manager.cpp, stdcall, ret 4): the month (0..11) of the date ten days after the end of the season of the club's nation;
- `pontos_dos_tecnicos_no_dia()` = 0x86dda0 (staff_records.cpp, thiscall, ret 4): at the turn of the month or year clears every manager's monthly points and stores the month and year; on the first day of the year passes the yearly points to the hall of fame and clears them;
- `pontos_dos_tecnicos_no_fim_da_competicao()` = 0x86d6e0 (staff_records.cpp, thiscall, ret 4): at the end of the competition's season passes its managers' league points (with the cup points of the same competition) and cup points to the hall of fame and clears them;
- `tecnico_do_mes_na_liga()` = 0x86f950 (staff_records.cpp, thiscall, ret 4): the league manager with the most points this month (above 0) among those two weeks in the job; a league without a table: the "Error" box and 0;
- `saidas_pendentes_dos_tecnicos()` = 0x69b650 (manager_manager.cpp, thiscall, ret 0): walks the pending departure list and, at each club with a manager, carries out his departure and, when the club opens a vacancy, the vacancy; removes each item from the list;
- `tirar_jogador_do_elenco_do_clube()` = 0x690730 (manager_manager.cpp, thiscall, ret 8): takes the player out of the club (or the national team): a player-manager job goes back to the staff job (else 0xb and he leaves), out of the lists, the media, the tactics and the 50 squad slots; the loan gets state 4;
- `tirar_pessoa_do_cargo_no_clube()` = 0x690890 (manager_manager.cpp, thiscall, ret 8): removes the contract and the person from the job slot at the club or national team (chairman, director, manager, assistant, coach, scout, physio; the playing jobs become 0xb), from the media and, if human, from the human's lists;
- `construir_noticia()` = 0x77bbe0 (news.cpp, thiscall, ret 8): builds the news item: type (+0), important byte (+4), the 50 data slots cleared, today's date (+0xcd), byte [0xae2c88] at +0xd5, id and +0xda cleared; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run;
- `ajustar_confianca_da_torcida()` = 0x738340 (media.cpp, thiscall, ret 0x14): adds to the supporters' confidence (+8 of the club row in [[0xb67a44]], 1..10000) the target minus the current confidence, × 0.75, damped by a favoured or rejected manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager;
- `ajustar_confianca_da_diretoria()` = 0x7386f0 (media.cpp, thiscall, ret 0x18): adds to the board's confidence (+6 of the club row) (confidence − 6000)/20 + target, × 0.75, damped by a chairman who likes or dislikes the manager and beyond ±1000; beyond ±500, news 0xbe0 to the manager with the subject's flags;
- `resposta_do_tecnico_da_maquina()` = 0x735870 (media.cpp, thiscall, ret 0xc): the AI manager's response 1, 2 or 3 to a story of the given type about the player: by the contract, the player's story rating (× 5), the manager's likes and dislikes and chains of draws per family of types; some types add the player to the manager's rejected list;
- `marcar_noticia_lida()` = 0x787140 (news.cpp, thiscall, ret 0xc): marks read the first unread item with the id in the human's ring unless it needs a reply (button 0x782b70), takes it off the count and, when asked, repeats it for the humans on the same network machine;
- `achar_noticia_pelo_id()` = 0x786b70 (news.cpp, thiscall, ret 0xc): copies the item with the id to the output: in the human manager's 100-slot ring or, without a manager, among all slots;
- `noticia_de_gratidao_do_jogador()` = 0x732c80 (media.cpp, thiscall, ret 0x10): news 0xbda to the manager: the player, the club, the type (0x12/0x21 by a draw for a player on the media list or with a flagged contract, 0xc when injured), the variant, the type's draw (which may raise +0x44 of the player record), the manager and the assistant;
- `noticia_de_pedido_de_apoio()` = 0x743370 (media.cpp, thiscall, ret 0x10): for a human club, news 0xbcf to the manager: the player, the club, the reputation band and the request level by attributes, the type, the fourth argument and whether he is transfer-listed;
- `noticia_de_reacao_a_critica()` = 0x743a50 (media.cpp, thiscall, ret 0x10): news 0xbdc: the player, the club, the reaction (0..4 by reputation, contract category, age, attributes and the response), the response, board confidence below 4000, the topic and the manager, delivered to the managers of the interested clubs (0x8d1da0);
- `montar_listas_de_posicao()` = 0x839e60 (shortlist_manager.cpp, thiscall, ret 4): rebuilds the 32 lists (16 slots × 2) at 12 bytes per entry: the size is the largest number of people fitting a slot (0x844300); each non-human person enters every slot he fits with his id, the nation id (drawn when missing) and the value or the contract +0x3d;
- `ordenar_listas_de_posicao()` = 0x83a780 (shortlist_manager.cpp, thiscall, ret 0): recomputes the entry keys (the value when the contract band is 0, minus 1,500,000 for a listed player of a human club; in the second list −1000, 2000 or the contract +0x3d) and sorts both arrays of each slot (0x945333 with 0x83b670);
- `clubes_interessados_no_jogador()` = 0x842180 (shortlist_manager.cpp, thiscall, ret 0x14): empties the output list and adds the (player, club) pairs: the humans who have the player on the scout shortlist and the clubs in the by-person array that are not suspended, are not his and value him above his value;
- `acrescentar_na_lista_de_observacao()` = 0x8423f0 (shortlist_manager.cpp, thiscall, ret 8): creates both pair arrays (capacity 100) and, when the pair is new: grows by 100, adds and sorts (by person, by club) and, outside a human club, adds 1 to the low nibble of +0xb of the person's 79-byte record;
- `prever_resposta_do_vendedor()` = 0x84af00 (shortlist_manager.cpp, stdcall, ret 0x20): 0 accepts, 1 haggles, 2 rejects: from the contract with the seller, the loan case (listed, reputation, status), the price (human, clause, holiday, asking or 0x4d1600) against the offer;
- `periodo_do_emprestimo()` = 0x8d0590 (transfer_manager.cpp, thiscall, ret 0x1c): the period from the club nation's rules and, with a type 2 contract, moves the end back month by month until it no longer passes the contract end (the start follows); without a contract, expired, too far or before this year: the default year 1972; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run;
- `prazo_da_resposta_a_oferta()` = 0x8cc5b0 (transfer_manager.cpp, thiscall, ret 0xc): from the time until the window closes (the rules of the buyer's and the player club's nations): 0 days, 1 or 2 days with drawn slices, or a value from the player's interest (0x846920) or drawn from the status; days = value / 3, slices = value % 3;
- `pagar_prestacoes_das_transferencias()` = 0x8caee0 (transfer_manager.cpp, thiscall, ret 0): pays each instalment (payer's and receiver's finances), adds the fractions to the instalment figure (+0x1c) in the last 8, removes the finished ones and zeroes the figure of clubs without an instalment in its direction;
- `tirar_oferta_de_transferencia()` = 0x8d0f60 (transfer_manager.cpp, thiscall, ret 4): destroys the linked offer, removes the pair from the loan list and the contract offers of the exchange and of the player, lowers the player's (+0x86e) and the manager's offer counts, the lowest free slot follows and the offer is destroyed;
- `acrescentar_suspensao()` = 0x556940 (discipline.cpp, thiscall, ret 8): when the person already has a ban under one of the 4 rules, extends the old one (matches or days) and flags both; otherwise copies the new ban into the vector's first free slot (or the end, growing by 100 up to 0x7d00), links each rule to it (0x7e8510) and, with the fourth rule, adds {id, nation} to the nation index (qsort);
- `tirar_suspensoes_de_fora_do_clube()` = 0x556270 (discipline.cpp, thiscall, ret 0xc): on a club move, frees each of the person's 4 record slots whose ruling body (nation +0x17 or continent +0x1b) does not cover the new club: releases the ban, frees the slot and the entry; with held bans, shows the "Error" box;
- `restaurar_suspensoes_guardadas()` = 0x556570 (discipline.cpp, thiscall, ret 4): when a loan ends, puts the person's held bans back into their slots (freeing the occupied slot first; the item leaves the list) and frees slots 0..2 not restored whose ruling body does not cover the contract club;
- `acrescentar_idioma()` = 0x7c6a30 (player_stats.cpp, thiscall, ret 0x10): appends the new entry to the language table (6 bytes: id, language, fluency up to 10; starts at 100, grows by 100) and sorts it when asked (qsort);
- `melhorar_idioma()` = 0x7c6ca0 (player_stats.cpp, thiscall, ret 0x10): finds the person's entry for the language (bsearch) and adds the increase to the fluency, up to 10; without one, adds the language with the increase as fluency;
- `zerar_estatisticas_do_jogador()` = 0x7c6650 (player_stats.cpp, thiscall, ret 4): frees the player's 6 category records (+0xe = −1, +1 to the free count), clears the league totals and recent ratings and empties the history; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run;
- `definir_competicao_da_selecao()` = 0x775220 (national_teams.cpp, thiscall, ret 8): makes the competition the nation's current one; with a major finals under way, the new one goes to the queue; when the new one is a major finals, the current one goes to the queue; notifies the manager handler (0x6a3770);
- `marcar_clubes_com_jogo()` = 0x7775c0 (national_teams.cpp, thiscall, ret 4): in the one-byte-per-club table, marks the clubs of the non-friendly fixtures on the date: 1 the club's league game, 2 another, 3 a club of a flagged nation (entry with no nation); in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays in the code and does not run;
- `tirar_jogador_da_disciplina()` = 0x556830 (discipline.cpp, thiscall, ret 4): clears the ban history, releases the bans and frees the 4 slots, drops the held bans from the list and leaves the 29-byte record free (0xff, count 0);

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (33):** 31 caught. `cpo_tecnico_recente` escaped: the branch for a manager hired less than two weeks ago ran in the base, but his points for the month never beat the best in the league, so dropping the rule does not change the pick; the comparator (same return as the original) and the career confirm it. `cpo_tirar_ofertas_pessoa` escaped: the byte +1 of the person's 8-byte record ([+0x86e]) is only a shortcut in the highest offer search (0x8cc930): left above zero, the search walks the vector, finds no offer for the person and returns 0 as the shortcut would; the career does not change. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2f65fd6b68d1027b).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (be54a1386f51073a).

**Redo**, on top of batch 250's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 251`

**Fields touched:**
- MANAGER_MANAGER +0: the per-club board table (0x49 bytes by id); pointer; reads (all three)
- board +2: flags (bit 2 = ultimatum carried out); value; writes (resolver_o_ultimato_da_diretoria)
- board +6: confidence (1..10000); value; reads and writes (confianca_da_diretoria_na_transferencia)
- board +0x14, +0x16: moods (+0x16 not below −10000); value; reads and writes (confianca_da_diretoria_na_transferencia)
- board +0x38: net transfer spend; double; reads and writes (avaliar_o_negocio_da_transferencia)
- MANAGER_MANAGER +4, +0x10: the pending departure (6 bytes) and ultimatum lists; pointer; reads and writes (resolver_o_ultimato_da_diretoria)
- transfer +8, +0xc, +0x1f: buying club, player, already assessed; id and value; reads
- person +0x39, +0x61, +0x3e/+0x42: club, player record, job start date; pointer and value; reads
- club +0x80, +0xbf, +0xcf, +0x57: reputation, chairman, manager, division; value and pointer; reads
- finances +0x165: club without board confidence (1); value; reads (confianca_da_diretoria_na_transferencia)
- [0xae2d0c] person's 79-byte record +0: the main contract index (−1 none); value; reads and writes −1 (tirar_o_contrato_principal)
- CONTRACT_MANAGER +0: the contract array (0x50 bytes; +0 −1 free, +4 club id); pointer; reads and writes (tirar_o_contrato_principal)
- CONTRACT_MANAGER +0x38: the held offer list (+0 person id, +0x59 transfer, +0x5d club, +0x61 byte); pointer; reads and writes (reenviar_ofertas_guardadas)
- transfer (0x8c61e0) +0x24, +0x2c: club and state (0xf becomes 8); id and value; reads and writes (reenviar_ofertas_guardadas)
- club: the season end date by nation (0x669bd0); value; reads (mes_do_fim_da_temporada_do_clube)
- Staff_Records +8: the points records (0xd2 bytes, [0xae2368] + 16); pointer; reads (all three)
- Staff_Records +0x14, +0x16: the stored month and year; value; reads and writes (pontos_dos_tecnicos_no_dia)
- record +0x86: person id (−1 empty); id; reads
- record +0xb6: yearly points; value; reads and writes 0 (pontos_dos_tecnicos_no_dia)
- record +0xba/+0xbe and +0xc2/+0xc6: league and cup points and their competitions; value and id; reads and writes (pontos_dos_tecnicos_no_fim_da_competicao)
- record +0xca/+0xce: monthly points; value; reads and writes 0 (pontos_dos_tecnicos_no_dia, tecnico_do_mes_na_liga)
- person +0x39, +0x3e/+0x40: club and job start date (signed day, unsigned year); pointer and value; reads (tecnico_do_mes_na_liga)
- league +0x43, +0x3e, +0xb1: type (2 = table), count and table array (0x3b bytes); value and pointer; reads (tecnico_do_mes_na_liga)
- competition +0x11c: kept in the hall of fame; value; reads (pontos_dos_tecnicos_no_fim_da_competicao)
- MANAGER_MANAGER +4: the pending departure list (6 bytes: two bytes and the club id at +2); pointer; reads and writes (saidas_pendentes_dos_tecnicos)
- person +0x39, +0x3d: club and club job; pointer and value; reads and writes (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)
- person +0x24, +0x28: national team and its job; pointer and value; reads and writes (tirar_jogador_do_elenco_do_clube, tirar_pessoa_do_cargo_no_clube)
- club +0xbf, +0xc3 (3), +0xcf, +0xd3: chairman, directors, manager, assistant; pointer; reads and writes 0 (tirar_pessoa_do_cargo_no_clube)
- club +0x19f (5), +0x1b3 (7), +0x1cf (3): coaches, scouts, physios; pointer; reads and writes 0 (tirar_pessoa_do_cargo_no_clube)
- club +0xd7 (50): the squad slots; pointer; reads and writes 0 (tirar_jogador_do_elenco_do_clube)
- loan contract (0x4cf5a0) +0x4e: state; value; writes 4 (tirar_jogador_do_elenco_do_clube)
- club row in [[0xb67a44]] (id × 0x49) +6: board confidence (1..10000); value; reads and writes (ajustar_confianca_da_diretoria)
- club row in [[0xb67a44]] +8: supporters' confidence (1..10000); value; reads and writes (ajustar_confianca_da_torcida)
- club row in [[0xb67a44]] +0xe: word carried in the board news; value; reads (ajustar_confianca_da_diretoria)
- club +0, +0xbf, +0xcf: id, the chairman and the manager; id and pointer; reads (both adjustments)
- person +0x18, +0x3e/+0x42, +0x59, +0x5b, +0x61: age, arrival date, two attributes and the player record (+5 reputation); value and pointer; reads (ajustar_confianca_da_diretoria)
- news item +0, +4, +5..+0xcc, +0xcd, +0xd5, +0xd6, +0xda: type, important, the 50 data slots, the date, the byte from [0xae2c88], id, read; value; writes (construir_noticia)
- [0xae2c90] today's date; [0xb74350] +0x10/+0x18 the league ratings; value; reads
- person (manager) +0x24, +0x39, +0x57, +0x59..+0x5d, +0x65, +0x69: national team, club, attributes, preferences and the non-player record (+0x14, +0x19, +0x1b, +0x1d, +0x1f); pointer and value; reads, and writes +0x65 +0x28..+0x30 through 0x547af0 (resposta_do_tecnico_da_maquina)
- person (player) +0, +0x5a, +0x5b, +0x61 (+0xb reputation, +0x25, +0x44, +0x45): id, attributes and the player record; value; reads (resposta_do_tecnico_da_maquina)
- contract +0x3d, +0x3f, +0x49, +0x4f: values and the type (high nibble); value; reads (resposta_do_tecnico_da_maquina)
- [0xae2d0c] the person's 0x4f-byte record +0xa, +0xb (bit 7); value; reads (resposta_do_tecnico_da_maquina)
- News +0/+4/+8: the news array, the slot count and the 16 humans' entries (ring start, next, count); pointer and value; reads and writes the count (marcar_noticia_lida, achar_noticia_pelo_id)
- news item +0xd6, +0xda: id and read; value; reads and writes +0xda (marcar_noticia_lida)
- person +0x5f: active human (1); value; reads (marcar_noticia_lida)
- player record (person +0x61) +0x44: raised by 1 (up to 20) in the gratitude news when the draw(5) gives 0 or 1; value; reads and writes (noticia_de_gratidao_do_jogador)
- player record +0xb: reputation; value; reads (backing request, criticism reaction)
- person +0, +4/+8/+0xc (+0x33), +0x18, +0x3e/+0x42, +0x57, +0x59, +0x5b, +0x5d: id, keys, age, arrival and attributes; value; reads (all three)
- contract +0x45: flags 0x2000, 8 and 0x181; value; reads (gratitude, criticism reaction)
- Media +0/+8: the media list (0x17 bytes: +8 person id, +0x10 flag 2) and its count; pointer and value; reads (noticia_de_gratidao_do_jogador)
- club +0, +0x80, +0xcf, +0xd3: id, reputation, manager and assistant; row in [[0xb67a44]] +6 (board confidence); value and pointer; reads
- Shortlist_Manager +0x18/+0x58: the 16 + 16 position list arrays (12 bytes: person id, nation id, key); pointer and value; writes (montar_listas_de_posicao, ordenar_listas_de_posicao)
- Shortlist_Manager +0x98: the 16 counts; +0xd8: the array size; value; writes (montar_listas_de_posicao), reads (ordenar_listas_de_posicao)
- Shortlist_Manager +0/+0x10: the (person, club) pair array sorted by person and its count; +0xdc: the 9-byte club entries (+0 the 40 slots, +8 the state: 100 suspended); pointer and value; reads (clubes_interessados_no_jogador)
- club shortlist slot (0x25 bytes) +7: what the club values the player at; +0xf: the player; value and id; reads (clubes_interessados_no_jogador)
- staff +0x1a: the nation; pointer; reads and writes a drawn one when null (montar_listas_de_posicao)
- staff +0x39, +0x52, +0x5f, +0x61: the club, the value, the active human, the player data; pointer and value; reads
- contract +4: the club; +0x3d: the status; +0x4e: bits 0..3; value; reads (montar_listas_de_posicao, ordenar_listas_de_posicao)
- club +0x57: the division (flag in [0xadadfc]); +0x80: the reputation; pointer and value; reads (ordenar_listas_de_posicao)
- [0xae1080] the last contract looked up; pointer; writes
- Shortlist_Manager +0/+4: the (person id, club id) pair arrays by person and by club; +8/+0xc the capacities; +0x10/+0x14 the counts; pointer and value; reads and writes (acrescentar_na_lista_de_observacao)
- person 79-byte record ([0xae2d0c]) +0xb: low nibble, on how many AI shortlists she is; value; reads and writes (acrescentar_na_lista_de_observacao)
- contract +0x2d/+0x2f: the end (day of year, year); +0x35: the type; +0x3d, +0x49, +0x4f: status and flags; value; reads (prever_resposta_do_vendedor, periodo_do_emprestimo)
- club +0x53: the nation (transfer rules); +0x80: the reputation; +0xcf: the manager; +0x1db; pointer and value; reads
- player data (staff +0x61) +0xb, +0xf; staff +0x52: the value; value; reads (prever_resposta_do_vendedor)
- TRANSFER_MANAGER +0x8ac: the 28 transfer rules (+0xa the nation; method +0x28 the loan period); pointer; reads (periodo_do_emprestimo)
- [0xae2c90] today's date (day of year, year); value; reads (periodo_do_emprestimo)
- transfer offer +4: the exchange player list; +8: the buying club; +0xc: the player; +0x11: the job; +0x1f: loan; +0x20: the slot; +0x28: the linked offer; pointer, id and value; reads (prazo_da_resposta_a_oferta, tirar_oferta_de_transferencia)
- TRANSFER_MANAGER +0/+8: the offers and the count; +0xc/+0x14: the linked ones and the count; +0x85e: the lowest free slot; +0x86e: the 8-byte per-person record (+1 offers, +2 exchange); +0x946: the loan list; pointer and value; reads and writes (tirar_oferta_de_transferencia)
- TRANSFER_MANAGER +0x872: the instalment list (+0 amount, +4 paying club, +8 receiving club, +0xc how many remain); pointer and value; reads and writes (pagar_prestacoes_das_transferencias)
- club finances (0x5a0590) +0x1c: the instalment figure; value; reads and writes (pagar_prestacoes_das_transferencias)
- TRANSFER_MANAGER +0x8ac: the 28 rules (+0xa the nation); +0x856: the shortlist manager; pointer; reads (prazo_da_resposta_a_oferta)
- staff +0x18: the age; player data +7: the ability; 79-byte record +0xb (low nibble): on how many shortlists; contract +0x3d: the status; club +0x80: the reputation; value; reads (prazo_da_resposta_a_oferta)
- [0xae1080] the last contract looked up; pointer; writes
- discipline +4/+8/+0xa: ban vector (20 bytes), capacity and count (words); pointer and value; reads and writes (acrescentar_suspensao)
- ban +0 matches (0xff for days), +2 days, +0xc..+0xf the 4 rules, +0x13 bit 0 extended; value; reads and writes (acrescentar_suspensao)
- discipline +0xc/+0x10/+0x12: nation index (8 bytes: id, nation), capacity and count (words); pointer and value; reads and writes (acrescentar_suspensao)
- discipline +0x14: 29-byte record per person (4 slots of 5 bytes: rule and entry index; +0x1c held count, signed); reads and writes (tirar_suspensoes_de_fora_do_clube, restaurar_suspensoes_guardadas)
- discipline +0x1c: list of held bans (item: id, +4 slot, +5 the slot's 5 bytes); reads and removes (restaurar_suspensoes_guardadas)
- ruling body +0x17 nation, +0x1b continent (−1 none); id; reads; the person's 14-byte entry (+0 = −1 free, +6 ban index); writes −1 to +0
- club +0x53 nation, nation +0x71 continent; pointer; reads. person +0x1a nation; reads. [0xae1080] the contract; writes (restaurar_suspensoes_guardadas)
- stats +0x3c/+0x40/+0x44: the language table (6 bytes: +0 person id, +4 language, +5 fluency 0..10), capacity and count; pointer and value; reads and writes (acrescentar_idioma, melhorar_idioma)
- stats +0x30: 39-byte record per person (+0..+0x14 the 6 category indices, +0x18/+0x1c history, +0x20/+0x22 league totals, +0x24..+0x26 recent ratings); reads and writes (zerar_estatisticas_do_jogador)
- stats +4: 32-byte category records (+0xe = −1 free); +0x38 free count; writes (zerar_estatisticas_do_jogador)
- person +0 id, +0x61 player; reads
- national teams +4: 110-byte records per nation (+4 the current competition, −1 none); id; reads and writes (definir_competicao_da_selecao)
- national teams +8: the queued competition per nation (4 bytes); id; writes (definir_competicao_da_selecao)
- [0xae23d0] competitions (107 bytes, +0 id); [0x9cf78c..0x9cf964] the major finals; id; reads
- national teams +0x24: one byte per club (0, 1 league game, 2 other, 3 flagged nation; n = [0xae235c] − 2 × [0xae2350]); pointer and value; writes (marcar_clubes_com_jogo)
- 0xb63df0: 0x21 entries of 0x48 bytes (+0 nation, +0x14 list group); nation +0x11c flagged; fixture +0x14 competition, +0x1c/+0x20 clubs; club +0x53 nation, +0x57 league, +0x60; reads (marcar_clubes_com_jogo)
- discipline +0x14: the person's 29-byte record (4 slots, +0x1c held count); writes 0xff and count 0; +0x1c held-ban list; removes (tirar_jogador_da_disciplina)

**Of interest to:** CMRTE, SGE, GPF2, CMScout
