# Batch 261 — the call-ups for an international match and two news items: picking both sides' squads, the called-up player news item and the cup result news item; the shortlist slots with a bid and the contract's transfer list and loan list flags; a person's status in the clubs' shortlists, the loan list flag and the fee limit by squad status; the transfer budget shared across the shortlist slots, the squad status allowed by age and the scout list names; the day's international friendlies and the human manager's selected scout list; the Champions League entrants: the four-band entry list, the qualifying table and the next stage; the Champions League match: the match prize and coefficient, the club's progress and the competition's date clash; the referee of each match: the referee choice, the daily busy flags and the renewal of the league referees; the referees of a new game: the random referees of a nation, the repair of the referee records and the copy of a record; the year award in the history (the manager counts it, the awarded player thanks or renews) and the win and defeat records; the appearances and absences of work permits and the appearance money after the match, the international appearance money stored and the news of the finalised transfer; the physios' and board's transfer veto, the board ultimatum news and its text; the person released on a free transfer, the human manager's holiday and the news of permission to discuss a contract

**Routines:**
- `convocar_selecoes_para_o_jogo()` = 0x76bab0 (national_teams.cpp, thiscall, ret 4): from two weeks before the match, for each side: a senior, B/Olympic or under-21 squad at most 14 days old and flagged only moves its date up to the match; otherwise asks for a new one (0x76c120); releases the over-age under-21 players and clears duplicates;
- `noticia_do_convocado()` = 0x76fd10 (national_teams.cpp, stdcall, ret 0xc): with a human manager at the player's club, news item 0x21 (the player, the national side and the slot) to the manager of club +0x39 and of the contract club;
- `noticia_do_desfecho_na_copa()` = 0x519690 (Cup.cpp, thiscall, ret 0x14): outside friendlies, the competition virtual +0x44 and news item 0x10 (club, competition, outcome, result category, the match, the next round), delivered by scope, to the competition region (final and semi-final) or to the club manager;
- `contar_vagas_com_lance()` = 0x844540 (shortlist_manager.cpp, thiscall): counts the club's shortlist slots with a target and a bid at the given position (0xff: any); with the list in state 2 for a human club, a slot whose manager arrived over 20 days ago and whose target has no offers loses its target and bid;
- `tirar_da_lista_de_transferencias()` = 0x844280 (shortlist_manager.cpp, stdcall): clears bit 0 (transfer listed) of the contract's +0x4e; 1 with a contract, 0 without;
- `tirar_da_lista_de_emprestimos()` = 0x8442d0 (shortlist_manager.cpp, stdcall): clears bit 1 (loan listed) of the contract's +0x4e; 1 with a contract, 0 without;
- `situacao_na_lista_de_observacao()` = 0x843e10 (shortlist_manager.cpp, thiscall): applies a transfer event for the person to the club's shortlist: for event 4 at his own club, event 1 to every interested club; the (person, club) pair leaves the sorted arrays and the person's shortlist count drops; in the slots holding the person, 1 refills the slot, 2 and 5 block for 120 and 14 days, 3 and 4 release with today − 120;
- `por_na_lista_de_emprestimos()` = 0x8442b0 (shortlist_manager.cpp, stdcall): sets bit 1 (loan listed) of the person's contract +0x4e;
- `teto_de_preco_pelo_estatuto()` = 0x84b1f0 (shortlist_manager.cpp, thiscall): the club's fee limit for the player at the squad status (0x84b230 with his strong positions); −1 without a person or player record;
- `distribuir_verba_de_transferencias()` = 0x83b6e0 (shortlist_manager.cpp, thiscall): shares an AI club's transfer budget across the open shortlist slots (type and status factors), reduces it by held bids and the remainder, fills each slot without a target and gives the remainder to the first own-club player slot the manager would sell;
- `estatuto_permitido()` = 0x84b4d0 (shortlist_manager.cpp, stdcall): statuses 5 and 6 (youth) only below age 24; the others, always;
- `nome_da_lista_do_olheiro()` = 0x7fc150 (scout_manager.cpp, cdecl): the translated scout list name: Short List, Player Search, Staff Search or Scout #n;
- `amistosos_internacionais_do_dia()` = 0x5cbb00 (friendly.cpp, thiscall): on the first day of the year empties the nation lists, sorts the nations by reputation and gives the international dates their year by the World Cup cycle; then arranges the friendlies of the dates equal to today + 12 weeks (0x5cbdf0), once a day;
- `lista_escolhida_do_tecnico()` = 0x7f6e30 (scout_manager.cpp, thiscall): the list selected by the human manager (null: the current one);
- `escolher_lista_do_tecnico()` = 0x7f6f20 (scout_manager.cpp, thiscall): stores the list selected by the human manager (null: the current one);
- `inscritos_da_liga_dos_campeoes()` = 0x584fa0 (european_cup.cpp, thiscall, ret 0): allocates the 71-entry list of 6-byte entries (+0x14) and fills it with the Champions League clubs (+0x1db = [0x9cf6e8]) by band (+0x1df): 1 up to 16 (round 7), 2 up to 18 (8), 3 up to 19 (0xd), 4 up to 18 (0); short of 71, +0x36 = 0 and returns 0;
- `tabela_da_qualificacao_da_liga_dos_campeoes()` = 0x584990 (european_cup.cpp, thiscall, ret 0): allocates the 55 qualifying rows (+0xa7, +0x3e = 55): row i takes the club of entry 70 - i and its entry round (8 -> 2, 0xd -> 1, other -> 0); out of memory, -1;
- `proxima_fase_da_liga_dos_campeoes()` = 0x5851d0 (european_cup.cpp, thiscall, ret 0): moves to the next stage (+0x30) and draws the phase 1 groups (stage 0), the phase 2 groups (8) or the quarter-finals (12); at the last stage, the error box;
- `processar_jogo_da_liga_dos_campeoes()` = 0x583b10 (european_cup.cpp, thiscall, ret 8): by the score (+0x47/+0x48, or +0x43/+0x44), 2 UEFA coefficient points to the winner's nation or 1 each on a draw (0x9058b0); in the group stage (+0x42 from 0 to 11), 200 thousand to the winner or 100 thousand each, with the prize news item (0x48e3e0); then the cup base (0x51a150);
- `progresso_do_clube_na_liga_dos_campeoes()` = 0x586c00 (european_cup.cpp, thiscall, ret 0x14): converts the three progress codes by stage (qualifying, phase 1, phase 2) and writes them to +0x73..+0x75 of the club's competition stats entry;
- `conta_para_conflito_de_datas()` = 0x48e360 (comp.cpp, thiscall, ret 8): 1 when the match round (+0x30) ends in 0 or is above 999, else 0;
- `escolher_arbitro_da_partida()` = 0x79c7b0 (officials_manager.cpp, thiscall, ret 8): the referee range for the competition (the league range in the nation block, the top quarter of a neutral nation of the continent in type 2 and 4 competitions, the home nation for a friendly), a random one in it and the next free one;
- `limpar_arbitros_ocupados()` = 0x79bf40 (officials_manager.cpp, thiscall, ret 0): clears the daily referee busy bytes;
- `renovar_arbitros()` = 0x79cee0 (officials_manager.cpp, thiscall, ret 4): at the league season start, replaces the referees of the range over 45 (or over 40, one in three) with new random ones and clears the stats row of the first one;
- `arbitros_sorteados()` = 0x79c5c0 (officials_manager.cpp, thiscall (this sem uso), ret 8): reallocates the referee table, generates `count` new referees of the nation (names by weight, random ratings and attributes, born in the 1960s) and sorts the table by nation and quality;
- `validar_arbitros()` = 0x79caa0 (officials_manager.cpp, cdecl): repairs each referee record: the nation from the names, out-of-range ratings and attributes drawn again and a too old birth date;
- `copiar_ficha_de_arbitro()` = 0x79c530 (officials_manager.cpp, thiscall, ret 4): copies a 0x2b-byte referee record field by field, the date through the date copy;
- `registrar_premio_do_ano()` = 0x86c390 (staff_records.cpp, thiscall, ret 0x18): the year award entry in the person history; the manager adds the award to his record; the player with a long contract on the club list gets the renewal offer and the winner may thank the manager (draw by loyalty);
- `vitoria_maior()` = 0x7e4a30 (record_utils.cpp, cdecl): the match record is a win by a larger margin than the other one, or the same with more goals scored;
- `derrota_maior()` = 0x7e4b00 (record_utils.cpp, cdecl): the match record is a defeat by a larger margin than the other one, or the same with more goals conceded;
- `jogos_das_permissoes_e_clausulas()` = 0x8c39f0 (transfer_manager.cpp, thiscall): after the match, on both sides: pays the appearance money at 1, 10, 20... 50 games (club by contract, national team by caps since the deal) and removes the paid bonus; in a counting match, adds an appearance to the work permit of who played and an absence to those left out without injury, ban, call-up or loan;
- `guardar_premio_por_jogos_na_selecao()` = 0x8cad90 (transfer_manager.cpp, thiscall): appends the international appearance money (13 bytes) to the array, growing by 50, and sorts it by person;
- `noticia_transferencia_finalizada()` = 0x8da5e0 (transfer_manager.cpp, thiscall): the 0xfa5 news of the agreed transfer, with the completion date, to the human seller and buyer;
- `veto_dos_fisios_e_da_diretoria()` = 0x696b10 (manager_manager.cpp, thiscall): when buying an injured player, the physios draw the veto; at a human club, the board vetoes on budget, asking price or fee and loses confidence;
- `noticia_do_ultimato()` = 0x6ac700 (manager_manager.cpp, thiscall): the 0x178d news of the ultimatum before the match, to the club manager or, by a draw, to those of the division or country;
- `texto_do_ultimato()` = 0x6ac880 (manager_manager.cpp, stdcall): the headline or body of the ultimatum news, with the names and links of the manager and the clubs;
- `liberar_sem_custo()` = 0x8c1af0 (transfer_manager.cpp, thiscall): records the free transfer and, unless record only, pays the club's compensation, removes the person from shortlists, selections, squad, offers and contract, clauses and work permit;
- `ferias_do_humano()` = 0x5fd2f0 (human_manager.cpp, thiscall): turns the human manager's holiday on or off, keeping the count of humans on holiday; when turning on, answers pending news;
- `noticia_conversa_de_contrato()` = 0x8d7cd0 (transfer_manager.cpp, thiscall): the 0xfac news of permission to discuss a contract to the human buyer and, per exchange player, to the human seller;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (37):** 34 caught. `cpy10_sem_sorteio_59` escaped: no award winner in the career had +0x59 above 15, where the thank-you draw applies [inference]. `cpy13_fica_na_escalacao` escaped: the player released on a free who stays in the selection leaves it at the next AI team pick, before playing [inference]. `cpy13_sem_listas` escaped: the released player status in the shortlists only matters for a bid for him, which the career to 2002-06-01 did not make [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (0b8bc0f1).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 65356052, original 8c36a9f9 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 260's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 261`

**Fields touched:**
- nation record (NationalTeams +4, 0x6e bytes) +0x32/+0x46/+0x5a: the today stamp of the senior, B and under-21 squads; value (date); writes (convocar_selecoes_para_o_jogo)
- nation record +0x3a/+0x4e/+0x62: the squad dates; value (date); reads and writes (convocar_selecoes_para_o_jogo)
- nation record +0x43/+0x57/+0x6b: the squad called-up flag; value; reads (convocar_selecoes_para_o_jogo)
- fixture +0x28/+0x2a: the match date; +0x40 the flag; +0x4d/+0x4e the match bits (1 no call-up, 2 no release, 0x100 B, 0x200 under-21); value; reads (convocar_selecoes_para_o_jogo)
- NationalTeams +0x36: the array of nations with called-up players (byte per id); pointer; reads (convocar_selecoes_para_o_jogo)
- [0xae1080] the last contract read; pointer; writes (noticia_do_convocado)
- staff +0x39: the player's national-side club; pointer; reads (noticia_do_convocado)
- cup +0x3c / +0xa3: the rounds and their array (0x68 bytes, +7 the round); value and pointer; reads (noticia_do_desfecho_na_copa)
- Shortlist_Manager +0xdc: 9-byte entry per club (+0 slots, +4 state); pointer; reads (contar_vagas_com_lance)
- shortlist slot (0x25 bytes) +0 position, +0xf target, +0x17 bid; value and id; reads, writes −1 to a stale target and bid (contar_vagas_com_lance)
- club +0xcf: manager (+0x3e arrival date); pointer; reads (contar_vagas_com_lance)
- contract +0x4e: bit 0 transfer listed, bit 1 loan listed; value; writes (tirar_da_lista_de_transferencias, tirar_da_lista_de_emprestimos)
- Shortlist_Manager +0/+0x10 and +4/+0x14: sorted 8-byte (person, club) pair arrays and counts; pointer and value; reads and writes (situacao_na_lista_de_observacao)
- Shortlist_Manager +0xdc: 9-byte entry per club (+0 slots, +8 writes 100); pointer; reads and writes (situacao_na_lista_de_observacao)
- shortlist slot (0x25 bytes) +0 type, +7 and +0xb values, +0xf target, +0x13 blocked person, +0x17 bid, +0x1b date, +0x24 status and flag; reads and writes (situacao_na_lista_de_observacao)
- [0xae2d0c] 79-byte record per person +0xb: low nibble = on how many shortlists; value; writes −1 (situacao_na_lista_de_observacao)
- contract +0x4f: high nibble = squad status; value; writes (situacao_na_lista_de_observacao)
- contract +0x4e: bit 1 loan listed; value; writes (por_na_lista_de_emprestimos)
- person +0x39 club, +0x61 player record; pointer; reads (situacao_na_lista_de_observacao, teto_de_preco_pelo_estatuto)
- Shortlist_Manager +0xdc: 9-byte entry per club (+0 slots, +4 state 1); pointer; reads (distribuir_verba_de_transferencias)
- shortlist slot (0x25 bytes) +0 type, +3, +7 budget, +0xb value, +0xf target, +0x13, +0x17 bid, +0x24 status and flag; reads and writes (distribuir_verba_de_transferencias)
- club +0x64: 1 = rich club (non-player slots get budget); value; reads (distribuir_verba_de_transferencias)
- person +0x52 value, +0x39 club, +0x18 age, +0x61 record; reads (distribuir_verba_de_transferencias, estatuto_permitido)
- contract +0x4f high nibble status, +0x25/+0x27 date; reads (distribuir_verba_de_transferencias)
- [0xae1080] last contract looked up; pointer; writes (distribuir_verba_de_transferencias)
- Friendly +0x34ec/+0x34f0: international dates (20 bytes: +0 weekday, +1 day, +2 month, +3 date, +0xb year offset) and count; reads and writes the date (amistosos_internacionais_do_dia)
- Friendly +0x34f4: last day done (word); value; reads and writes (amistosos_internacionais_do_dia)
- Friendly +0x34f6 nation lists ([0xae2350] of them), +0x34fa/+0x34fe nations and count; reads, empties and sorts (amistosos_internacionais_do_dia)
- [0xdd6ec8] friendlies limit date; value; reads (amistosos_internacionais_do_dia)
- SCOUT_MANAGER +0x4c: selected list per human manager (16 bytes); value; reads and writes (lista_escolhida_do_tecnico, escolher_lista_do_tecnico)
- club +0x1db: the season's European competition; id; reads (inscritos_da_liga_dos_campeoes)
- club +0x1df: the Champions League entry band (1 to 4); value; reads (inscritos_da_liga_dos_campeoes)
- [0x9cf6e8] the Champions League; id; reads (inscritos_da_liga_dos_campeoes)
- Champions League +0x14 / +0x36 / +0x38: the entrant list (6 bytes: +0 club, +4 round, +5 0) and the counts; pointer and value; writes (inscritos_da_liga_dos_campeoes), reads (tabela_da_qualificacao_da_liga_dos_campeoes)
- Champions League +0xa7 / +0x3e: the qualifying table (6 bytes: +0 club, +4 entry round 0/1/2, +5 0) and the row count; pointer and value; writes (tabela_da_qualificacao_da_liga_dos_campeoes)
- Champions League +0x30 / +0x2c: the current stage and the stage count; value; reads and writes the stage (proxima_fase_da_liga_dos_campeoes)
- match +0x47 / +0x48 (or +0x43 / +0x44 when +0x47 = -1): the score; value; reads (processar_jogo_da_liga_dos_campeoes)
- match +0x42: the stage (0 to 11 the group stage); value; reads (processar_jogo_da_liga_dos_campeoes)
- match +0x30: the round; value; reads (conta_para_conflito_de_datas)
- [0xde1f58] +0xf + 18 x index: the nation's UEFA coefficient points (the index is +0x1df of the national team); value; writes through 0x9058b0 (processar_jogo_da_liga_dos_campeoes)
- club finance +0 (64 bits), +0xa8, +0x148: the balance and the prize totals; value; writes through 0x5999a0 (processar_jogo_da_liga_dos_campeoes)
- competition stats, club entry +0x73 / +0x74 / +0x75: the progress codes; value; writes (progresso_do_clube_na_liga_dos_campeoes)
- Champions League +0x30: the current stage; value; read by the comparer (processar_jogo_da_liga_dos_campeoes)
- OfficialsManager +0 stats rows (17 bytes per id: +4 average, +8, +0xa, +0xc words, +0xf cards), +4 count and +8 busy byte array, +0xc per-nation blocks (0x102 bytes: +4 count, +8 first, +0x10 range count, +0x12 16-byte ranges: +0 first, +4 count, +0xc competition id); pointers/values; reads (escolher_arbitro_da_partida, renovar_arbitros), writes the busy bytes (limpar_arbitros_ocupados) and the stats row (renovar_arbitros)
- referee (0x2b bytes at [0xae23e8], [0xae2390] of them) +0 id, +4 and +8 names, +0xc birth day and +0xe year, +0x16 nation, +0x25 rating; ids/pointers/values; reads +0 (escolher_arbitro_da_partida), reads and writes +4, +8, +0xc, +0xe, +0x25 (renovar_arbitros)
- fixture +0 (−2 removed), +0x14 competition, +0x1c and +0x20 clubs, +0x2e referee (word, the output); competition +0 id, +0x59 continent, +0x5d nation; competition object ([0xadadfc]) +0x42 type; club +0x53 nation; nation (0x122 bytes at [0xae23a8]) +0 id, +0x71 continent; ids/pointers; reads (escolher_arbitro_da_partida, renovar_arbitros)
- [0xae2c90] game date (day, year); [0xae2350] nations, [0xae234c] continents; values; reads
- referee (0x2b bytes at [0xae23e8], [0xae2390] of them) +0 id, +4 and +8 names (name record +0x37 nation index), +0xc birth day and +0xe year, +0x16 nation, +0x1a, +0x1e, +0x20 and +0x22 ratings (words; the order uses +0x28 × 50 + +0x22), +0x24..+0x2a attributes (bytes 1..20); pointers/values; writes the new records, the count and the table (arbitros_sorteados), reads and writes (validar_arbitros, copiar_ficha_de_arbitro)
- nation (0x122 bytes at [0xae23a8]) +0x71 continent, +0x8e referee level (byte); [0xae2350] nations; pointers/values; reads
- person history entry (FILE_LLIST [Staff_Records], head and tail at [+0xc] + id × 8) +0 date, +8 club or −1, +0xc type 0x34, +0xd award, +0x11 a4, +0x15 a5, +0x19 credit; values; writes (registrar_premio_do_ano)
- manager points record (0x86f5b0) +0x9e awards (word); value; reads and writes (registrar_premio_do_ano)
- person +0 id, +0x28 and +0x3d jobs, +0x59 and +0x5b character attributes (signed bytes), +0x61 player record; contract (0x4cf5a0/0x4cf5f0) +4 club, +0x25/+0x29 end date, +0x35 type, +0x45 flag; [0xae1080] last contract; values/pointers; reads, writes [0xae1080] (registrar_premio_do_ano)
- match record +0x19 goals for, +0x1a goals against (signed bytes); values; reads (vitoria_maior, derrota_maior)
- fixture +0 (−2), +0x14 competition, +0x1c home, +0x20 away, +0x40/+0x41 side counts; value/pointer; reads (jogos_das_permissoes_e_clausulas)
- match + side × 0x625 + slot × 0x49: +0x190 id, +0x1ad, +0x1c2; id/value; reads
- transfer manager +0x86e: 8-byte record per person (bit 2 club clause, 4 national, 0x20 permit; +7 caps at the deal); value; reads
- work permit (0xe bytes at [+0x93a], count +0x93e) +4 appearances, +6 absences; value; writes +1 (jogos_das_permissoes_e_clausulas)
- person +0x22 caps, +0x61 player record; value/pointer; reads
- contract +0x3b club appearances, +4 club, +0x35 & 0x3f type; value; reads
- transfer manager +0x898 international appearance money (13 bytes), +0x89c capacity, +0x8a0 count; pointer/value; writes (guardar_premio_por_jogos_na_selecao)
- offer +4 exchange, +8 buyer, +0xc person, +0x11, +0x1b fee, +0x20, +0x24, +0x2c state, +0x2f, +0x30; value; reads (noticia_transferencia_finalizada)
- [0xae1080] last contract looked up; pointer; writes
- board (0x49 bytes at [manager manager] + id × 0x49) +6, +0x14, +0x16 confidences; value; reads and writes (veto_dos_fisios_e_da_diretoria)
- club +0x1cf three physios, +0x53 nation (+0x11c), +0x57 division, +0x80 reputation; pointer/value; reads
- injury +0x28 severity; value; reads
- person +0x39 current club, +0x61 player record (+0x2d); pointer/value; reads
- contract +0x35 & 0x3f type, +0x3d, +4 club; value; reads
- [0xad6c30] CRT rand state; value; writes (srand by ids and clock)
- [0x9cf764] lower division key; id; reads
- club +0xcf manager, +4 and +0x38 names, +0x37 and +0x52 genders; pointer/value; reads (noticia_do_ultimato, texto_do_ultimato)
- competition (0x6b bytes at [0xae23d0]) +0x38 name, +0x52 gender; value; reads (texto_do_ultimato)
- [0xae1080] last contract looked up; pointer; writes
- transfer manager +0x862 past transfers (0x25 bytes: +0 person, +4..+0xc keys, +0x10, +0x14 club, +0x18, +0x1c type, +0x1d date), +0x866 capacity, +0x86a count; pointer/value; writes (liberar_sem_custo)
- person +0x39 current club, +0x3e date, +0x52 value, +0x6d, +0x61 player record; pointer/value; reads and writes (liberar_sem_custo)
- contract +0x35 & 0x3f type, +0xc wage, +4 club; value; reads
- humans (0xa6ef bytes at [this]) +0x9a01 holiday; [this+4] count, [this+0xc] on holiday; value; writes (ferias_do_humano)
- [0xb74c26 + 0x300 × [0xb77c7a]] current human manager; pointer; reads
- offer +4 exchange, +8 buyer, +0xc person, +0x20, +0x24; value; reads (noticia_conversa_de_contrato)
- [0xae1080] last contract looked up; pointer; writes

**Of interest to:** CMRTE, CMScout, SGE, GPF2
