# Batch 254 — the selection limits: the fixture restriction table, the matchday squad limit and who needs a work permit; who may negotiate: whether the club may approach the player, the terms sent to the exchange player and the club's free transfer queue; the future transfers: the player's one, removing the offer's one and adding the agreed offer's one; the open and collapsed offers: the offers for the club's players, collapsing the offers for a player and storing the next sale percentage; the club boards at the start of the career, the ban appeals and the fixture the human may watch; retirements: who retires at the start of the game, who retires by age and the initial list; the search for new retirements, leaving the jobs on retirement and the free release record; the player injury status, the offer decision date and the manager of the year candidates; the league projection for the board: the club's projected positions and the two projection orders; the fixture calendar: finding the fixtures of a club or competition between two dates, the competition fixtures on a day and whether the competition has a fixture on a date; the club free for a friendly, the club in the cup and the calendar partner nation

**Routines:**
- `restricoes_da_partida()` = 0x8cf6b0 (transfer_manager.cpp, thiscall, ret 0xc): fills the 42-byte table with 0xff and, by the fixture rules scope (0x8d2fa0), lets the nation or continent rule fill it (virtual +0x1c); when asked, keeps only the first limit (+5/+3, +2/+0 or +0x16/+0x17);
- `limite_do_banco_do_dia()` = 0x8cf880 (transfer_manager.cpp, thiscall (this em ecx), ret 0x18): counts, over the matchday squad slots (given players or the tactics 0x8968f0), the non-EU, foreign or over-age players of the fixture limit (0x8cf7e0); when over, 1 and the message;
- `precisa_de_permissao_de_trabalho()` = 0x8c7560 (transfer_manager.cpp, thiscall, ret 0xc): 1 when the club nation uses work permits (+0x76 = 0xe), the player is not EU and holds no granted permit (type 2 contract, 0x20 flag, search in the +0x93a array);
- `clube_pode_chegar_ao_jogador()` = 0x8ce990 (transfer_manager.cpp, thiscall, ret 0x10): cannot (0 and the reason): nation that keeps its players (0x27), agreed contract (0x21), open exchanges (7), free queue (1), offer in negotiation (0x12, flag +0x2d) or future transfer (0x20), the rules of the Basque club (0x19), of nation [0x9cf4dc] (0x28) and of clubs [0x9d0128]/[0x9d01a0] (0x1a); otherwise 1;
- `termos_ao_jogador_da_troca()` = 0x8c2060 (transfer_manager.cpp, thiscall, ret 8): flags the exchange player sent terms; when none is flagged, the offer moves to state 8 (from 7) or 6;
- `fila_livre_do_clube()` = 0x8c1780 (transfer_manager.cpp, thiscall, ret 4): counts the free transfer queue people of the club or of its partner club (0x540a50);
- `transferencia_futura_do_jogador()` = 0x8cca10 (transfer_manager.cpp, thiscall, ret 8): with the 0x10 flag, searches the offers for the player (0x8c5cf0) and the exchanges offering him (0x8c5e40) for a state 0x13 one; found, the +0x920 array entry and 1; otherwise clears the flag and 0;
- `tirar_transferencia_futura()` = 0x8cbf70 (transfer_manager.cpp, thiscall, ret 4): clears the 0x10 flag of the player and the exchange players and removes the offer's entry from the +0x920 array (the next ones move down, 0x8cbf40);
- `acrescentar_transferencia_futura()` = 0x8cb9a0 (transfer_manager.cpp, thiscall, ret 8): adds (date, offer) to the +0x920 array and sorts it; the offer moves to state 0x13; suspends the shortlists for the player and the exchange players (0x842b00) and sets their 0x10 flag; a flagged contract keeps the offer;
- `ofertas_abertas_pelos_jogadores_do_clube()` = 0x8c6100 (transfer_manager.cpp, thiscall, ret 8): counts (and lists the indices of) the open offers for players contracted to the club, except states 2, 3, 4, 0x10, 0x14, collapsed ones and loans;
- `desfazer_ofertas_pelo_jogador()` = 0x8c47d0 (transfer_manager.cpp, thiscall, ret 4): each offer for the player loses its future transfer (0x8cbf70), is collapsed (0xf, +0x2e = 3), clears the buyer's and seller's bid markers (0x84aeb0) and is scheduled (0x8c6220, 10 days);
- `guardar_percentual_da_proxima_venda()` = 0x8caaf0 (transfer_manager.cpp, thiscall, ret 4): appends the 13-byte next sale percentage to the +0x880 array (growing by 50) and sorts it (0x8e54d0);
- `iniciar_diretorias_dos_clubes()` = 0x68f130 (manager_manager.cpp, thiscall, ret 0): fixes the jobs of staff with a club (manager at +0xcf) and builds each club board row: dates, reputation squared, the four confidences drawn from the expectation (board rating or reputation) and the reputation, held to 5750/5500..10000, or 5000 without a manager; the season expectations (0x6a3770) and, without a manager, the vacancy (0x69ba50);
- `apelacoes_de_suspensao()` = 0x6abef0 (manager_manager.cpp, thiscall, ret 0): for each appeal (+0x14) whose date has come: the result on up to 4 discipline rule sets (0x557000), the news (0x6ac060) and the appeal leaves the list;
- `pode_ver_a_partida()` = 0x5fe700 (human_manager.cpp, thiscall, ret 8): 1 when the fixture has a negative +0x43 and neither it nor the fixtures of that day (each key nation group) have a club of the human; else 0;
- `aposentadoria_no_inicio()` = 0x7a79c0 (player_regen.cpp, thiscall, ret 8): at a new game, does the person with a club retire? Non-players by the job age limit; players by the profile limit, the ability, the club competition and the contract status; the output (id, club, reason) and the announcement and retirement dates by the key nation;
- `aposentadoria_por_idade()` = 0x7a7310 (player_regen.cpp, thiscall, ret 8): in the search for new retirements, does the person retire? Like the start one, with the limit adjusted by contract and wage, unemployment, contract end and potential; without a key nation, dates drawn near today;
- `lista_inicial_de_aposentadorias()` = 0x7a58f0 (player_regen.cpp, thiscall): at a new game, tests every eligible person (0x7a79c0), adds those who retire and sorts the list;
- `novas_aposentadorias()` = 0x7a5790 (player_regen.cpp, thiscall): tests every eligible person (except the young ones in game competitions, the flagged, those already listed and those with a flagged contract) by age (0x7a7310) or unemployment (0x7a8050), adds those who retire and sorts the list;
- `sair_dos_cargos()` = 0x7aaeb0 (player_regen.cpp, stdcall, ret 4): removes the person from the job slots at the club and national team (chairman, directors, manager with the vacancy opened, assistant, coaches, scouts, physios) and clears the club, the national team and the job types;
- `registrar_a_liberacao()` = 0x86d800 (staff_records.cpp, thiscall, ret 4): the free release histories: the club manager one (type 0x68) and the person one (type 0x67);
- `estado_da_lesao()` = 0x62c400 (injury.cpp, thiscall, ret 4): updates the daily fitness (0x62c680) and returns 2 while the injury lasts, 1 while recovering match fitness and 0 when fit, by the gap from the injury date to the game date;
- `marcar_decisao_da_oferta()` = 0x4cd230 (contract_manager.cpp, thiscall, ret 8): finds or creates the person's type 3 decision entry and sets the day and time slice of the answer: by the offer deadline, or without an offer by today and the contract end; at most 14 days ahead;
- `candidatos_a_tecnico_do_ano()` = 0x86fab0 (staff_records.cpp, thiscall, ret 8): sorts the clubs and keeps up to three whose manager has been in the job 28 weeks and has games in the record;
- `projetar_posicoes_da_liga()` = 0x68ac40 (league.cpp, thiscall, ret 4): projects each table club's end-of-season points (in the first third, by a reputation scale), sorts them and hands the club's positions in normal, better (× 1.5) and worse (× 0.444) form to virtual +0x68;
- `ordem_da_projecao_pela_reputacao()` = 0x68afb0 (league.cpp, cdecl): qsort comparator of projection entries by club reputation (+0x80), highest first;
- `ordem_da_projecao_pelos_pontos()` = 0x68afd0 (league.cpp, cdecl): qsort comparator of projection entries by projected points (+0xa), highest first;
- `achar_jogos_do_calendario()` = 0x5acbe0 (fix_man.cpp, thiscall, ret 0x24): counts (and copies into the list, if given) the fixtures between two dates in the key-nation lists of the club or competition, with type, club side and competition filters;
- `jogos_da_competicao_no_dia()` = 0x5abd90 (fix_man.cpp, thiscall, ret 0x1c): copies into the list the fixtures of one day and slice for the requested competition, round and type and returns how many;
- `competicao_tem_jogo_na_data()` = 0x5ae6d0 (fix_man.cpp, stdcall, ret 8): tells whether the competition has a fixture on the date in any of the three calendar slices;
- `clube_livre_na_data()` = 0x5c6b70 (friendly.cpp, stdcall, ret 0xc): tells whether the club is free for a friendly on the date: no fixture from the day before to the kind rest days and, outside national sides, outside the key-nation season;
- `copa_tem_o_clube()` = 0x522910 (Cup.cpp, thiscall, ret 4): tells whether the club is in the cup's team table;
- `nacao_parceira_do_clube()` = 0x5af5b0 (fix_man.cpp, cdecl): for an English or Welsh club, the active key-nation index of the other nation; otherwise 0xff;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (27):** 24 caught. `cpr9_vaga_nao_abre` escaped: the manager branch ran twice in the career, but the vacancy only opens when 0x544820 lets the club look for a manager [inference: it did not in either case]; comparison mode did not diverge either. `cpr12_qualquer_rodada` escaped: on the days and competitions asked with a fixed round (moving cup rounds to their fixed dates) every fixture in the list is already of that round, so the filter removes none [inference]; comparison mode did not diverge either. `cpr13_copa_sem_o_clube` escaped: the branch ran 32 times (board projection, 0x4ae300), but the cup projection changes nothing the career to 2002-06-01 uses [inference]; comparison mode did not diverge either. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (8c95873456684fd1).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (7f4ecded7ef233a0).

**Redo**, on top of batch 253's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 254`

**Fields touched:**
- TRANSFER_MANAGER +0x8ac: the 28 national transfer rules (+0xa nation, +0xe continent, +0x12 type, virtual +0x1c fills the table); pointer; reads (restricoes_da_partida)
- restriction table (42 bytes): +0/+2 foreign, +3/+5 non-EU, +0x16 age and +0x17 count; value; writes (restricoes_da_partida)
- fixture +0x1c/+0x20: home and away; +0x40/+0x41: computer side; +0x4d & 0xf: substitutes beyond the 11; pointer and value; reads (limite_do_banco_do_dia)
- selection limit (6 bytes): +0 type, +1 age, +2 starters, +3 substitutes, +4 total, +5 count; value; writes (limite_do_banco_do_dia)
- staff +0x18: age; value; reads (limite_do_banco_do_dia)
- club +0x53: the nation; nation +0x76: the 0xe marker (work permit); pointer and value; reads
- contract +0x35 & 0x3f: the type (2); value; reads (precisa_de_permissao_de_trabalho)
- TRANSFER_MANAGER +0x86e: person's 8-byte record, bit 0x20; +0x93a/+0x93e: the permits (14 bytes) and the count; value; reads (precisa_de_permissao_de_trabalho)
- staff +0x1a / +0x1e: the nationalities; +0x39: the contract club; +0x61: the player record; pointer; reads (clube_pode_chegar_ao_jogador, fila_livre_do_clube)
- TRANSFER_MANAGER +0x86e: person's 8-byte record, bit 0x40 (agreed contract); value; reads and writes (clube_pode_chegar_ao_jogador)
- TRANSFER_MANAGER +0x92c: the free transfer queue (LLIST of ids; cursor +0x936); id; reads (clube_pode_chegar_ao_jogador, fila_livre_do_clube)
- transfer offer +4: the exchange list (items: +0 id, +4 terms sent); +0x2c: the state; +0x2d: the flag; value; reads and writes (clube_pode_chegar_ao_jogador, termos_ao_jogador_da_troca)
- club +0x53 / +0x57: the nation and the region; pointer; reads (clube_pode_chegar_ao_jogador)
- [0x9cf458], [0x9cf424], [0x9cf4dc], [0x9cf8d8], [0x9cf488], [0x9d0128], [0x9d01a0]: the nation and club ids of the rules; id; reads (clube_pode_chegar_ao_jogador)
- TRANSFER_MANAGER +0x920 / +0x924 / +0x928: the future transfers (12 bytes: date +0, offer +8), the capacity and the count; pointer, value and id; reads and writes
- TRANSFER_MANAGER +0x86e: person's 8-byte record, bit 0x10 (future transfer); value; reads and writes
- transfer offer +4: the exchange list; +0xc: the player; +0x20: the offer number; +0x2c: the state (0x13); +0x2d: the flag; id and value; reads and writes +0x2c/+0x2d (acrescentar_transferencia_futura)
- contract +0x49 / +0x4a: the flag and the future transfer offer; value and id; reads and writes (acrescentar_transferencia_futura)
- TRANSFER_MANAGER +0x856: the club shortlists (Shortlist_Manager); pointer; reads (acrescentar_transferencia_futura)
- transfer offer +4: the exchange list; +8: the buyer; +0xc: the player; +0x2c: the state; +0x2e: the end reason (3 = collapsed); id and value; reads and writes +0x2c/+0x2e (desfazer_ofertas_pelo_jogador)
- contract +4: the club; +0x35 & 0x3f: the type (5 = loan); id and value; reads (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)
- [0xae1080] the last contract found; pointer; writes (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)
- TRANSFER_MANAGER +0x86e: person's 8-byte record, byte +1 (offers for the person); value; reads (desfazer_ofertas_pelo_jogador)
- TRANSFER_MANAGER +0x880 / +0x884 / +0x888: the next sale percentages (13 bytes), the capacity and the count; pointer and value; writes (guardar_percentual_da_proxima_venda)
- staff +0x24 club, +0x28 job, +0x3d original job, +0x39 contract, +0x61 player data; pointer/value; reads and writes +0x24, +0x28, +0x3d (iniciar_diretorias_dos_clubes)
- club +0xcf manager; pointer; writes the staff with job 5/12, cleared for second national teams (iniciar_diretorias_dos_clubes)
- club +0x80 reputation; value; reads, capped at 200 × nation +0x8e for second national teams (iniciar_diretorias_dos_clubes)
- club +0xbf, +0xc3..+0xcf, +0xd3, +0x19f..+0x1db; pointers; cleared for second national teams (iniciar_diretorias_dos_clubes)
- board row [this] + 73 × i: +0, +2..+5 flags, +6/+8/+0xa/+0xc confidences, +0xe, +0x10..+0x16, +0x18 date (today), +0x20 date, +0x28, +0x2c, +0x30 reputation squared × 0.25 (double), +0x38, +0x3c, +0x40, +0x41 date; values; writes (iniciar_diretorias_dos_clubes)
- manager preferences (staff +0x69) +0xa board rating; value; reads and lowers to 0.75 × reputation (iniciar_diretorias_dos_clubes)
- MANAGER_MANAGER +0x14: the appeal list (+0 staff id, +4 result, +5 date, +0xd..+0x10 rule sets, +0x11..+0x18 result fields); pointer; reads and removes (apelacoes_de_suspensao)
- fixture +0x1c/+0x20 clubs, +0x28/+0x2a date, +0x3f slice, +0x40/+0x41 flags, +0x43; values; reads (pode_ver_a_partida)
- [0xb64738] key nation groups; value; reads (pode_ver_a_partida)
- person +0x18, +0x3d, +0x28, +0x57, +0x59: age, club and national job types, two signed bytes; value; reads (both decisions)
- person +0x1a, +0x24, +0x39, +0x61, +0x69: nation, national team, club, player profile and staff profile; pointer; reads (both decisions)
- person +0x3e: date since unemployed; value; reads (aposentadoria_por_idade)
- player +5, +7, +0xb, +0xf, +0x14, +0x15, +0x3c: current and potential ability, the ability used in the sums, profile bytes; value; reads (both decisions)
- club contract +0xc, +0x2f, +0x3d, +0x3e, +0x45, +0x4f: wage, end year, status, a byte, a field and the high nibble; value; reads (both decisions)
- contract +0x49: flag that excludes from the initial list; value; reads (lista_inicial_de_aposentadorias)
- club +0x57 and [[0xadadfc] + id × 4]: competition and whether it runs in the game; pointer and value; reads; nation +0x11c; value; reads
- 0xb63d60 + k × 72: +0x15 season end day, +0x1d one more year; value; reads (both decisions)
- output (26 bytes) +0 id, +4 club, +8 announcement, +0x10 retirement, +0x18 reason, +0x19 zero; id and value; writes (both decisions)
- [0xae1080] the current contract; pointer; writes (both decisions)
- [0xae2d0c] 79-byte record per person, +0xb bits 0x30; value; reads (lista_inicial_de_aposentadorias)
- PLAYER_REGEN +4 retirement list (26 bytes), +8 count; pointer and value; appends and sorts (lista_inicial_de_aposentadorias)
- person +0x18: age; value; reads (novas_aposentadorias)
- person +0x39 and +0x24: club and national team; pointer; reads and writes 0 (sair_dos_cargos); reads (novas_aposentadorias)
- person +0x3d and +0x28: club and national job types; value; reads and writes 0 (sair_dos_cargos)
- club +0xbf, +0xc3 (3), +0xcf, +0xd3, +0x19f (5), +0x1b3 (7), +0x1cf (3): chairman, directors, manager, assistant, coaches, scouts, physios; pointer; reads and writes 0 (sair_dos_cargos)
- club +0x57 and [[0xadadfc] + id × 4]: competition and whether it runs in the game; pointer and value; reads (novas_aposentadorias)
- contract +0x49: flag that excludes from the search; value; reads (novas_aposentadorias)
- [0xae2d0c] 79-byte record per person, +0xb bits 0x30; value; reads (novas_aposentadorias)
- PLAYER_REGEN +4 retirement list (26 bytes, id at +0), +8 count; pointer and value; reads, appends and sorts (novas_aposentadorias)
- [0xae1080] the current contract; pointer; writes (registrar_a_liberacao)
- Staff_Records +0 and +0xc: staff history file and lists; appends (registrar_a_liberacao)
- Injury +0: current injury index per person (+0xe of the 31-byte record, −1 if none); +4: 48-byte injuries (+0x20 date, +0x28 and +0x2a recovery and injury days); id and value; reads (estado_da_lesao)
- CONTRACT_MANAGER +0xc 14-byte decisions (+0 id, +4 date, +0xc slice, +0xd type), +0x14 count, +0x18 start, +0x1c counter; value; reads and writes (marcar_decisao_da_oferta)
- contract (80 bytes at CONTRACT_MANAGER +0) +0x2d: end date; value; reads (marcar_decisao_da_oferta)
- offer +0x2f: short deadline flag; value; reads (marcar_decisao_da_oferta)
- [0xae2c88] time slice of the day; value; reads (marcar_decisao_da_oferta)
- club +0xcf: manager; pointer; reads; manager +0x3e: arrival date; value; reads; manager record +0x8e: games; value; reads (candidatos_a_tecnico_do_ano)
- league +0x3e: the table row count; +0xb1 the table (0x3b bytes: +0 the club, +0x11 games played, +0x19 points); value and pointer; reads (projetar_posicoes_da_liga)
- league +0xa7: season games; +0xc2 the scale points per game (byte); +0x44 the competition; value; reads (projetar_posicoes_da_liga)
- league +8: the Comp_Stats; pointer; reads (projetar_posicoes_da_liga, comparator)
- club +0x80: the reputation; value; reads (ordem_da_projecao_pela_reputacao)
- club Comp_Stats record +0x73..+0x75: the projected positions (normal, better, worse); value; writes (through virtual +0x68 of projetar_posicoes_da_liga)
- fixture (0x4f bytes) +0: the competition; +0xc home club; +0x10 away club; +0x34 the round (word); +0x40/+0x41 home and away side; +0x42 the type; id and value; reads (all three)
- list (LLIST) +0: the count (word); +2 head; +6 tail; +0xa cursor; value and pointer; reads, moves the cursor and appends (all three)
- [0xae23b4] the clubs (0x245 bytes); [0xae23d0] the competitions (0x6b bytes); [0xae2a58] the calendar; pointer; reads (achar_jogos_do_calendario, competicao_tem_jogo_na_data)
- club +0: the id; +0x53 the nation; id and pointer; reads (clube_livre_na_data, nacao_parceira_do_clube)
- nation +0: the id; +0x11c active (byte); id and value; reads (clube_livre_na_data, nacao_parceira_do_clube)
- key nation (72 bytes at 0xb63d60) +0: the nation; +0x14 the index; +0x2a season start; +0x39 season end; pointer, value and date; reads (clube_livre_na_data, nacao_parceira_do_clube)
- [0x9cf2e4] England; [0x9cf510] Wales; [0xae23a8] the nations (0x122 bytes); id and pointer; reads (nacao_parceira_do_clube)
- cup +0x3e: the team count; +0xa7 the table (6 bytes: +0 the club); value and pointer; reads (copa_tem_o_clube)

**Of interest to:** CMRTE, SGE, GPF2, CMScout
