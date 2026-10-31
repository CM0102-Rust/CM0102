# Batch 255 — the player's transfer request, the jobs with a contract and who is not a board member; the work permits at game start, the clause count and the player's appearance money; the free transfer queue, the next open window and the negotiated loan; the clause counter offer and the bid rejection reasons; the daily squad relationships: the fights and likes between team-mates, the friction with the manager and the morale spread by leadership; the game day: missed training, the competitions of the day and the board confidence in the club; the stadiums: the stadium day (the closed national stadium and the expired bookings), booking a stadium on a date and whether it is free; the fixture with a scouted player, the human scout list and the first cap record; the player's career goals, the loan record and the order of the return to training queue; the player's goals for a club, the team of the year record and the order of the clubs by manager points; the stored calendar: finding the stored copy of a fixture, storing a fixture and removing a fixture; the arranged friendlies: creating the week's planned friendlies, adding a friendly and drawing the friendly's free date; the cups day by day and the travel illness risk: the cup daily update, setting up the cup round and the visitors' illness risk

**Routines:**
- `pedido_de_transferencia()` = 0x4d8610 (contract_manager.cpp, thiscall, ret 4): at a computer club: when the value (0x4d0c60) × 1.5 beats the asking price (0x4d1600), lists the player (+0x4e, morale +12, 0x842c00, 0x4d71b0); otherwise a draw gives the 0x10000 grievance, morale drops 12 and the request strength (+0x44) grows; at a human club, the news item (by the original);
- `cargo_com_contrato()` = 0x4cbb20 (contract_manager.cpp, stdcall, ret 4): 1 for jobs 0, 6 to 0xd and 0xf (those holding a contract manager contract);
- `fora_da_diretoria()` = 0x4cb960 (contract_manager.cpp, stdcall, ret 4): 0 when the job (+0x3d) is a board job (1 to 4) at a club (+0x39); otherwise 1;
- `iniciar_permissoes_de_trabalho()` = 0x8c6e60 (transfer_manager.cpp, thiscall, ret 0): marks 0x3fff/0x1fff by international caps; for a player needing a permit (0x8c7560), by the earliest year in the club nation (career 0x7c3e40): under 5 years, the permit (0x8c7070); otherwise the second nationality (+0x1e); sorts the permits;
- `contagem_da_clausula()` = 0x8c3e50 (transfer_manager.cpp, thiscall, ret 8): type 4: international caps since the deal (+0x22 minus byte +7 of the person record); otherwise club appearances (+0x3b of the contract);
- `premio_por_jogo_do_jogador()` = 0x8d0ea0 (transfer_manager.cpp, thiscall, ret 4): with the person bit 2, searches the appearance money array +0x88c; when not found, clears the bit;
- `fila_da_transferencia_livre()` = 0x8c18c0 (transfer_manager.cpp, thiscall, ret 0): for each queued player (+0x92c): at a human club the board may block it (0x696940); a draw adds the manager to the disliked staff (0x547af0); the records (0x86d800), the cancelled offers (0x8df9b0) and the release with the compensation (0x8c1af0); clears the queue;
- `proxima_janela_aberta()` = 0x8cf470 (transfer_manager.cpp, thiscall, ret 8): the first day, from today up to a year later, with the club nation window open (0x8e8e50; without a rule, today); when none, the "Error" box;
- `emprestimo_negociado()` = 0x8c72d0 (transfer_manager.cpp, thiscall, ret 8): the +0x946 list item with the person and the club, or 0;
- `oferta_com_clausula()` = 0x8c8fd0 (transfer_manager.cpp, thiscall (this sem uso), ret 0x14): with a much bigger bidding club and a high-potential player up to 30: a new offer (0x8e5680, 0x8e78f0) with a 20% sell-on (appeal 80), international (70) or first-team (60) appearance money, and the fee × 0.8; none, frees the offer and 0;
- `motivo_da_recusa_vale()` = 0x8c5560 (transfer_manager.cpp, stdcall, ret 8): 0 when the reason does not fit: hot prospect (0x2000) aged 24 or more, same division (0x1000), rival (0x800) without a fixture or link; otherwise 1;
- `motivo_pelo_status_no_elenco()` = 0x8c54c0 (transfer_manager.cpp, stdcall, ret 4): by the squad status of the current club contract: 1 → 0x200, 2 and 3 → 0x400, 5 → 0x2000; otherwise 0;
- `relacoes_do_elenco()` = 0x5d6d10 (game.cpp, cdecl): ten random pairs of team-mates: a disliked one stops being disliked and may fight (0x5888a0); otherwise becomes disliked, liked or stops being liked; every 50 days, random clubs go through the dressing room friction; on the other days each player, by attribute draws, may reach the friction with the manager;
- `atrito_com_o_tecnico()` = 0x5d7190 (game.cpp, cdecl): each squad player with a contract and preferences: the first team-mate he dislikes; with day % 14 + id = 0, the complaint against the manager, the assistant or another (0x587c60) and the flag on the contract (0x867700);
- `moral_pela_lideranca()` = 0x5ce0b0 (game.cpp, cdecl): one club in seven each day (not national teams): squad leadership and leadership × morale summed; each player draws from them and his own morale and the morale goes up or down by one (0x544710);
- `faltas_ao_treino()` = 0x5d6aa0 (game.cpp, cdecl): n/250 consecutive people from a random one: a present player, with training and without a serious injury misses training (0x41a360) by a draw on the sum of attributes, morale and likes of the manager (more likely when newly arrived) or by the contract flags;
- `competicoes_do_dia()` = 0x5ce060 (game.cpp, cdecl): each competition in the list gets the day (virtual +0x14 with the game date and the flag); with the flag, the next step of [0xae2a58] (0x5aaf00);
- `mudar_confianca_da_diretoria()` = 0x5cdde0 (game.cpp, thiscall, ret 8): adds the delta to the board confidence in the club (+6 of the 73-byte club row), held to 1..10000;
- `estadios_do_dia()` = 0x85b820 (stadium.cpp, thiscall, ret 4): on day 0 rebuilds the venue list; before 2003 closes the England national stadium (id in [0xa835a4], nation and national team without a stadium) and reopens it on 1/6/2003 with 90000 places; every 30 days removes the expired bookings and re-sorts;
- `reservar_estadio()` = 0x85de20 (stadium.cpp, thiscall, ret 8): appends the booking (stadium id and date) to the list, which grows 30 at a time, and re-sorts by date and id;
- `estadio_livre_na_data()` = 0x85ddb0 (stadium.cpp, thiscall, ret 8): 1 when there is no booking of the stadium on the date (binary search in the list; no list, 1);
- `jogo_com_observado()` = 0x7f9720 (scout_manager.cpp, thiscall, ret 4): tells whether the fixture has a watched club or an available player with a scout request (at the club or national team, not loaned out, injured, suspended or barred, and not yet played);
- `na_lista_do_olheiro()` = 0x7f7490 (scout_manager.cpp, thiscall, ret 8): tells whether the player is on the human scout shortlist (through the open edit session or the first type 1 search);
- `registrar_a_primeira_convocacao()` = 0x86cc50 (staff_records.cpp, thiscall, ret 8): the first cap history of the person (type 6: the club, the opponent and the age);
- `gols_na_carreira()` = 0x7c3d20 (player_stats.cpp, stdcall, ret 4): sums the goals of every season and club of the player's career table (0x7c3e40), in 16 bits;
- `registrar_o_emprestimo()` = 0x86da20 (staff_records.cpp, thiscall, ret 0xc): the loan histories: the one of the receiving club manager (type 0x69) and the one of the person (type 0x6a);
- `ordem_da_volta_aos_treinos()` = 0x632040 (injury.cpp, cdecl): compares the key date with the one of a 0x16-byte return to training queue entry, for bsearch;
- `gols_pelo_clube()` = 0x7c3bb0 (player_stats.cpp, stdcall, ret 8): sums the goals of the player's seasons for the club in the career table (0x7c3e40), in 16 bits;
- `registrar_a_selecao_do_ano()` = 0x86c730 (staff_records.cpp, thiscall, ret 0x14): the team of the year history of the person (type 0x32: the club, the award and two values);
- `ordem_pelos_pontos_do_tecnico()` = 0x870220 (staff_records.cpp, cdecl): orders two clubs by their manager's league season points (highest first; no record last);
- `jogo_guardado_igual()` = 0x5ac7a0 (fix_man.cpp, thiscall, ret 4): returns the stored copy whose 0x4f bytes match the fixture's, in its day, slice and key-nation list of the year block; 0 if none;
- `guardar_jogo_no_calendario()` = 0x5ab230 (fix_man.cpp, thiscall, ret 4): finds (or creates) the fixture's year block and appends a copy of it to the day, slice and key-nation list;
- `tirar_jogo_do_calendario()` = 0x5abc30 (fix_man.cpp, thiscall, ret 4): removes the first item with the same bytes from the fixture's day, slice and key-nation list; returns 1 if removed;
- `criar_amistosos_planejados()` = 0x5c2ae0 (friendly.cpp, thiscall, ret 0): walks the AI friendly plan up to today + 1 week: on the date, with the clubs free, creates the friendly and sends the request; when the plan is used up, frees the array;
- `acrescentar_amistoso()` = 0x5c6f00 (friendly.cpp, thiscall, ret 0x10): reuses the club's open invitation or builds a new friendly; with the home side's stadium free, stores date, clubs and venue and puts it in the list;
- `planejar_amistoso_em_data_livre()` = 0x5c4380 (friendly.cpp, thiscall, ret 0xc): draws a date in both clubs' window and, with both free, books the friendly and links both clubs' dates and opponents;
- `atualizar_copa_no_dia()` = 0x51f890 (Cup.cpp, thiscall, ret 8): moves on completed stages and ends the cup, gives each stage the day and, on the date, draws (phase 0) or sets up (phase ≠ 0) the rounds until none advances;
- `montar_rodada_da_copa()` = 0x51c800 (Cup.cpp, thiscall, ret 4): draws the round if needed, creates the fixtures of the marked ties, moves them to the round dates and moves on; a team without a club postpones the rounds by a day;
- `risco_de_doenca_na_viagem()` = 0x5ab3c0 (fix_man.cpp, thiscall, ret 4): with the stadium in a third-world country, gives the visiting clubs the illness risk on the day before the fixture;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (36):** 29 caught. `cps3_sem_registros` escaped: the release branch runs 8 times, and what the mutation removes are the staff history records (Staff_Records), read only by the screens and the season end [inference]. `cps4_time_0_9` escaped: the first-team appearance money of the counter offer (32 times) only matters if the counter offer is accepted, and none of the 32 reached a contract before 2002-06-01 [inference]. `cps9_um_gol_a_mais` escaped: the career goals go to the match player (+0x43), used only by the commentary and the milestone news [inference]. `cps9_sem_o_primeiro_grupo` escaped: the same career goals value (+0x43), only for the commentary and the milestone news [inference]. `cps10_todos_os_clubes` escaped: the goals for the club go to the match player (+0x3d), used only by the commentary and the milestone news [inference]. `cps10_um_gol_a_mais` escaped: the same goals-for-the-club value (+0x3d), only for the commentary and the milestone news [inference]. `cps13_montagem_atrasada` escaped: the round setup (32 times) is called again on the next day and sets up the same round before the fixtures [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2a181435).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 dabca044, original 982f511c (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 254's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 255`

**Fields touched:**
- contract +0 / +4: person and club; id; reads (pedido_de_transferencia)
- contract +0x44: transfer request strength (signed, ±100); value; reads and writes (pedido_de_transferencia)
- contract +0x4e: bit 2 cleared and bit 8 (listed on request) set; value; writes (pedido_de_transferencia)
- contract +0x4f & 0xf0: squad status (0x10 = 1); value; reads (pedido_de_transferencia)
- staff +0x59: happiness; staff +0x61: player record; value and pointer; reads (pedido_de_transferencia)
- staff +0x39 / +0x3d: club and job; pointer and value; reads (fora_da_diretoria)
- TRANSFER_MANAGER +0x86e: person 8-byte record: +0 bit 2 (appearance money), word +3 (0x3fff/0x1fff), byte +7 (caps at the deal); value; reads and writes
- TRANSFER_MANAGER +0x88c/+0x894: appearance money records (13 bytes) and the count; value; reads (premio_por_jogo_do_jogador)
- TRANSFER_MANAGER +0x93a/+0x93e: work permits and the count; value; sorts (iniciar_permissoes_de_trabalho)
- staff +0x22: international caps; value; reads
- staff +0x1e: second nationality; pointer; writes (iniciar_permissoes_de_trabalho)
- staff +0x39/+0x3d/+0x3e/+0x40: club, job, arrival date and year; reads (iniciar_permissoes_de_trabalho)
- contract +0x3b: club appearances; value; reads (contagem_da_clausula)
- [0xae2c92] current year; [0xdd6ec8] start date; value; reads
- TRANSFER_MANAGER +0x92c: free transfer queue (LLIST; item +0 person id, +4 reason, +5 compensation); value; reads and clears (fila_da_transferencia_livre)
- TRANSFER_MANAGER +0x8ac: the 28 nation rules (+0xa nation); pointer; reads (proxima_janela_aberta)
- TRANSFER_MANAGER +0x946: negotiated loans list (item +0 person, +4 club); pointer; reads and moves the cursor (emprestimo_negociado)
- club +0xcf: manager; club +0x53: nation; pointer; reads
- staff +0x5b: the disliked staff draw value; value; reads (fila_da_transferencia_livre)
- [0xae1080] current contract; pointer; writes (fila_da_transferencia_livre)
- offer +0x1b: fee; +0x3a/+0x3e: international appearance money and caps; +0x35/+0x39: first-team appearance money and games; +0x3f: sell-on percentage; value; writes (oferta_com_clausula)
- offer +8 / +0xc: bidding club and person; id; reads (reasons)
- club +0x80: reputation; club +0x57: division; club +0xb3/+0xb7/+0xbb: linked clubs; value and pointer; reads
- player record +5/+7: current and potential ability; +0xb: reputation; value; reads (oferta_com_clausula)
- staff +0x18: age; +0x1a: nation (+0x8e reputation); +0x22: international caps; reads
- contract +0x4f >> 4: squad status; value; reads (motivo_pelo_status_no_elenco)
- [0xae1080] current contract; pointer; writes
- club +0xd7: the 50 squad slots; pointers; reads (all three)
- club +0xcf manager, +0xd3 assistant, +0xff the contract person, +0x80 reputation; pointers/value; reads (relacoes_do_elenco, atrito_com_o_tecnico)
- staff +0x18, +0x59..+0x5d mental attributes; values; reads (relacoes_do_elenco)
- staff +0x39 contract club, +0x61 player data, +0x65 preferences; pointers; reads (all three)
- player (staff +0x61) +0x2f leadership, +0xb reputation; values; reads (moral_pela_lideranca, relacoes_do_elenco)
- contract (0x4cf5f0) +0x44 morale, +0x45 flags; values; reads (all three)
- [0xae1080] the last contract found (0x4cf5a0); pointer; writes (relacoes_do_elenco)
- staff +0x3d original job (11 = player), +0x3e arrival date, +0x18, +0x5b attributes; values; reads (faltas_ao_treino)
- player (staff +0x61) +0x2f, +0x44 morale; values; reads (faltas_ao_treino)
- contract (0x4cf5f0) +0x45 flags 0x8000, 0x30000, 0x7800; values; reads (faltas_ao_treino)
- club +0xcf manager; pointer; reads (faltas_ao_treino)
- [0xae1080] the last contract found (0x4cf5a0); pointer; writes (faltas_ao_treino)
- [0xadadfc]/[0xadadf8] the competition list and its count; pointer/value; reads (competicoes_do_dia)
- MANAGER_MANAGER [0xb67a44] +0: 73-byte rows per club, +6 board confidence; value; reads and writes (mudar_confianca_da_diretoria)
- Stadium [0xdd7ef4] +0 bookings (12 bytes: +0 stadium id, +4 date), +4 count, +8 capacity; pointer/values; reads and writes (estadios_do_dia, reservar_estadio), reads (estadio_livre_na_data)
- nation +0x80 national stadium; pointer; reads and clears for England [0x9cf2e4] (estadios_do_dia)
- club +0x69 stadium; pointer; cleared for the England national team (estadios_do_dia)
- stadium (0x4e bytes at [0xae23b0]) +0 id, +0x3c/+0x40 places; values; reads the id, writes the places on reopening (estadios_do_dia)
- [0xa835a4] id of the closed national stadium (−1 = none); id; writes (estadios_do_dia)
- fixture +0, +0xc, +0x10, +0x14, +0x1c, +0x20: competition, two ids checked against the watched list, competition for the ban and the two clubs; id and pointer; reads (jogo_com_observado)
- [0xb74438] watched club list (static LLIST, cursor +0xa reset from +2); value; reads and writes the cursor (jogo_com_observado)
- SCOUT_MANAGER +0x5c scout requests (0x11 bytes, person at +4), +0x64 count; pointer and value; reads (jogo_com_observado)
- SCOUT_MANAGER +0xc + slot × 4: human open edit session; pointer; reads (na_lista_do_olheiro)
- scout search +0xa: type (1 = shortlist); value; reads (na_lista_do_olheiro)
- [0xb74c26 + 0x300 × (i16)[0xb77c7a]] manager of the current human; pointer; reads (na_lista_do_olheiro)
- person +0x39 and +0x24: club and national team; pointer; reads (jogo_com_observado, registrar_a_primeira_convocacao)
- person +0x18: age; value; reads (registrar_a_primeira_convocacao)
- Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_a_primeira_convocacao)
- person +0x61: player record; pointer; reads (gols_na_carreira)
- career table (0x7c3e40): 0x2b-byte entries, six per season and club; group +0xdf: goals; value; reads (gols_na_carreira)
- match player +0x43: career goals; value; written (MATCH_PLAYER::setup, by gols_na_carreira)
- club +0xcf: manager; pointer; reads (registrar_o_emprestimo)
- person +4, +8, +0xc → +0x33: the three name ids; id; reads (registrar_o_emprestimo)
- return to training queue (Injury +0x10, count +0x18; 0x16 bytes, date at +8); value; reads (ordem_da_volta_aos_treinos)
- Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_o_emprestimo)
- person +0x61: player record; pointer; reads (gols_pelo_clube)
- career table (0x7c3e40): 0x2b-byte entries, six per season and club; group +0xd9: club id; +0xdf: goals; id and value; reads (gols_pelo_clube)
- match player +0x3d: goals for the club; value; written (MATCH_PLAYER::setup, by gols_pelo_clube)
- club +0xcf: manager; pointer; reads (ordem_pelos_pontos_do_tecnico)
- manager record (0x86f5b0 of [0xdd7f00]) +0xba: league season points; value; reads (ordem_pelos_pontos_do_tecnico)
- Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_a_selecao_do_ano)
- fixture (0x4f bytes) +0x14: the competition; +0x28 the year (word); +0x2a the day (word); +0x3f the slice; pointer and value; reads (all three)
- calendar year block (0x1184 bytes) +0: the lists (day × 3 + slice) of per-key-nation arrays; +0x112c next block; +0x1130 the year (word); pointer and value; reads and writes the next one (all three)
- list (LLIST) +0: the count (word); +2 head; +6 tail; +0xa cursor; node +0 data, +4 size, +6 next; value and pointer; reads, moves the cursor, appends and removes (all three)
- Friendly +0x34d8: the AI friendly plan (20 bytes: +0 the date, +8/+0xc/+0x10 club indices A, B and C); +0x34dc the index; +0x34e0 the count; +0x34e4 the capacity; pointer and value; reads and writes (criar_amistosos_planejados)
- Friendly +0x34c8: the friendly list; +0xa7 the season start (date); pointer and value; reads and creates (acrescentar_amistoso, planejar_amistoso_em_data_livre)
- friendly (0x19 bytes) +0: the date; +8 zero; +0xc the club; +0x10 home club; +0x14 away club; +0x18 the state (1); pointer and value; writes (acrescentar_amistoso)
- friendly candidate +4: the count (byte); +0xa the opponent list; +0xe/+0x10 the window (signed words); +0x12 the date list; value and pointer; reads and writes (planejar_amistoso_em_data_livre)
- club +0x69: the stadium; pointer; reads (acrescentar_amistoso)
- cup +0x3c: rounds (word); +0x40 base year; +0x44 parent (0xff none); +0x45 active; +0x4c finished; +0x2c/+0x30 stages and the current one; +0xc the stages; +8 the Comp_Stats; +0xb1 has stages; value and pointer; reads and writes +0x4c (atualizar_copa_no_dia)
- cup +0xa3: the rounds (0x68 bytes: +0 draw day and +2 year, +3/+5 the fixtures date, +7 the name, 0x8c the final, +0x1a the ties); +0xa7 the teams (6 bytes, +0 the club); +0xab current round; +0xac draw round; +0xad round stages (+0x34 pending; 0x35-byte ties, +0/+2 the teams, +0x34 the mark); reads and writes (both cup ones)
- [0xadadfc][competition id]: the active competition; [0xae2c88] the day mode; value; reads (both cup ones)
- fixture +0x18 the stadium (+0x38 the city, +0x1f the nation, +0x7e state of development); +0x1c/+0x20 the clubs (+0x53 the nation); +0xc/+0x10 the ids; +0x28/+0x2a year and day; pointer and value; reads (risco_de_doenca_na_viagem)

**Of interest to:** CMRTE, SGE, CMScout, GPF2
