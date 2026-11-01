# Batch 264 — the match income: the match income in both clubs' finances, the gate receipts and the league fine; the rare finance events: the finance boost, the board takeover and the other costs; the international friendly dates: the date table, the dates in the World Cup cycle and the human club tournament; the AI pre-season friendlies: the club friendlies, the added tournament and the human club open invitation; the new injury (the slot, the length by the physio, age and proneness, the return to training, the recurrence and the retirement), the injury news and the AWOL flag; the former clubs' cut of a transfer fee (which the original credits to the buyer), the next-sale percentage paid to the clause's club and its news; the work permit granted (the permit record, potential, wage, youth, caps and the 16-match history), the squad status for a refusal reason and the negotiation slot; the squad reaction to a player story, the warning to position rivals and the media response labels; the international bans served in a national team match, the ban appeal and the ban slot text; the game linked list: removing an item by content, clearing and the cursor item; the cup draws: by seeding and keeping nations apart, and the base cup season start; the competitions key nations: the club one, the stadium one and the table entry by active index; the fixture calendar: the clash of two fixtures on one day and a club's first and last year with fixtures

**Routines:**
- `renda_da_partida()` = 0x59a5d0 (fin_prm.cpp, thiscall, ret 8): the TV money by attendance (1.75 × attendance), the gate receipts (+0x38 of the result), the match costs and the home club expenses (with draws), the competition TV money by stage or type, divided by the nation type; in the home club league, TV 3/4 and 1/4, the net to the home club and the stadium upkeep; outside it, half of each to both;
- `renda_do_estadio()` = 0x59af40 (fin_prm.cpp, thiscall, ret 8): the gate receipts: the price by the home club reputation (/ 500; × 0.5 in friendlies, + 3 or + 4 in competitions) × attendance, less the finance discount in the home club league; at a neutral ground by the competition +0x69; adjusted by nation type, league, key clubs and the nation at 0x9cf314;
- `pagar_multa_da_liga()` = 0x599850 (fin_prm.cpp, thiscall, ret 4): adds the fine to the season (+0x128) and month (+0x88) fines and takes it from the balance;
- `impulso_das_financas()` = 0x59e050 (fin_prm.cpp, thiscall, ret 8): outside seven nations and flags +0x64 2 and 3, the divisor by wealth band and reputation; with the chairman record, three draws (against +0xf, the debt and 750 × +0x20) give (3 × (+0x20)² + 750) × reputation + 1 million plus the balance; without it, two draws give (reputation + 1000) × 1000; a third in three nations; the money, the news by type and the end of the crisis;
- `troca_do_presidente()` = 0x59e940 (fin_prm.cpp, thiscall, ret 8): the divisor by band and the factor by reputation and league; draws by the chairman record and by 15000 / divisor; (reputation + 1000) × 1000 less the balance (or just what clears the balance); with the record, the new chairman (drawn nation, names and record); the news (type 2) and the end of the crisis;
- `pagar_outros_custos()` = 0x5999e0 (fin_prm.cpp, thiscall, ret 4): adds the amount to the season (+0x120) and month (+0x80) other costs and takes it from the balance;
- `tabela_das_datas_internacionais()` = 0x5ca4f0 (friendly.cpp, thiscall): allocates the table (+0x34ec) and writes the 110 international dates of 20 bytes (weekday, day, month, year offset from the World Cup year, confederation and qualifying competition), shrinks the table to 110, resolves the dates (0x5cc510) and marks the last day done (+0x34f4 = 400);
- `resolver_datas_internacionais()` = 0x5cc510 (friendly.cpp, thiscall): gives each entry its date in the World Cup cycle (the first year with (year - 2) a multiple of 4, plus the offset, less 4 when more than 3 years ahead) through 0x54a130 and sorts the table by date;
- `torneio_do_clube_humano()` = 0x5c6830 (friendly.cpp, thiscall): the tournament in use (of 127, 0x69 bytes from +0xb1) that the current human manager club hosts, or 0;
- `amistosos_dos_clubes()` = 0x5c4770 (friendly.cpp, thiscall, ret 0xc): rebuilds the friendly plan (+0x34d8); for each league in pre-season, the AI clubs of the nation (by searching the sorted candidates) get a band by reputation and a draw, and up to draw(3) + 4 friendlies with opponents found (0x5c4cb0) on free dates (0x5c4380), the band falling 20% each time; at the end, the plan sorted by date;
- `acrescentar_torneio()` = 0x5c6e00 (friendly.cpp, thiscall, ret 0x28): fills the first of the 127 free tournaments with the host, the type, the start date, the rule bytes and words, the name and the 4 invited clubs (flagged), or 0 with no free slot;
- `convite_aberto_do_clube_humano()` = 0x5c68b0 (friendly.cpp, thiscall): the friendly in the list (+0x34c8) that the current human manager club requested and that still waits for an opponent, or 0;
- `lesao_nova()` = 0x62d250 (injury.cpp, thiscall, ret 0x20): records the injury: replaces the previous one only for types 0x4d/0x4e/0x4f/0x51, finds or grows the slot, records the match, the length (base + variable × (35 − best physio) / 20 with draws and the age and proneness extras), the return to training (type percentage, at most 60 days before the end), the date, the news, the recurrence, the national team, the relapse, the retirement and the outbreak;
- `noticia_da_lesao()` = 0x62dca0 (injury.cpp, thiscall (this sem uso), ret 0xc): injury news 2000: the person, the type, the date, the days, the birth date and the match; over 100 days, to everyone with potential above 8000 or to the interested clubs; up to 100, to the human club, the contract club and the national team;
- `ausente_sem_licenca()` = 0x418f60 (awol.cpp, thiscall, ret 8): bit 0 of the person's flag; without read-only and without a contract or club, the absence ends first (0x4196a0);
- `parte_dos_clubes_anteriores()` = 0x8e4f20 (transfer_manager.cpp, thiscall, ret 4): player over 22: (offer fee / 20) / sum of weights × weight of each of up to five former clubs goes into the buyer's sales; news 0xfc3 goes to the former club when the cut exceeds 11 × reputation − 10000;
- `percentual_da_proxima_venda_pago()` = 0x8cb0f0 (transfer_manager.cpp, thiscall, ret 8): binary-searches the person's clause with the contract club; the cut (percentage × offer fee plus exchange players, or the person's value) from 1000 up: news, transfer totals, paid into the clause club and out of the contract club;
- `noticia_do_percentual_da_venda()` = 0x8db3a0 (transfer_manager.cpp, thiscall, ret 0x10): news 0xfa7: the person, the percentage, the two clause clubs, the cut and the buyer (or the contract club), to the managers of the clause club and the seller;
- `permissao_concedida()` = 0x8c7950 (transfer_manager.cpp, thiscall, ret 0x14): decides the permit: by the permit record (matches and refusals), non-player by reputation, the 2001 exceptions, potential above 8500, wage above 34999, youth above 190, under 5 caps no, over 20 with a strong nation yes, and the 16-match national team history (over 11 or 70%), which advances with new matches;
- `estatuto_pelo_motivo()` = 0x8c5800 (transfer_manager.cpp, thiscall (this sem uso), ret 8): the squad status for the refusal reason (0x200: 1, 0x400: 2, 0x2000: 5) or the one of the contract with the club (+0x4f >> 4); no contract, 0;
- `vaga_da_negociacao()` = 0x8c6200 (transfer_manager.cpp, thiscall, ret 4): the address of slot i of the negotiation array; out of range, the start of the array;
- `reacao_do_elenco_a_historia()` = 0x755340 (media.cpp, thiscall): after a story about the player (the transfer rumour): the dislike chance from the interested clubs (Old Firm, league fixture, rival, same division), the club rejecting him and, for each squad, staff and board member, a score from squad status, ambition, loyalty and preferences; when drawn, the person comes to dislike the player and his contract gets flag 1 or the 0x800 complaints;
- `aviso_ao_rival_de_posicao()` = 0x758010 (media.cpp, thiscall): with a high-reputation player linked to the club and more than two squad players in his position: the unsettled position rivals (by ambition and loyalty, by a draw) get contract flag 0x800, and the first one gives story 0x25;
- `rotulo_da_resposta_da_midia()` = 0x735160 (media.cpp, stdcall): the label of a media story response option ("Deny rumour", "Defend Player", "No Comment"...), from the 22-case table by story kind;
- `cumprir_suspensoes_internacionais()` = 0x557120 (discipline.cpp, thiscall): in a national team match: the people of its nation in the nation index (binary search by nation) serve one match of their ban under rule 0, one by one from a list of the ids (the count can release the ban and reorder the index);
- `recurso_da_suspensao()` = 0x557000 (discipline.cpp, thiscall): the result of a ban appeal: the ruling body drops the person's pending bans; a match ban loses matches (or stops at the counter) and a timed ban loses days, and it is released when over or when the appeal lifts it;
- `texto_da_vaga_da_suspensao()` = 0x55d3c0 (discipline.cpp, thiscall): the text of the ban in the person's slot, from the ruling body of its rule; with no rule in the slot, 0;
- `tirar_da_lista()` = 0x68d1f0 (manager_manager.cpp, thiscall): the first node whose item has the same bytes as the data leaves the list (head, tail, count and cursor fixed) and is freed with its item; 1 when removed;
- `esvaziar_lista()` = 0x68d080 (manager_manager.cpp, thiscall): frees every node and item of the list and zeroes the count, head, tail and cursor;
- `item_do_cursor()` = 0x68d2e0 (manager_manager.cpp, thiscall): the item of the cursor node, without moving; with no cursor, 0;
- `copa_sortear_pela_semente()` = 0x51d2d0 (Cup.cpp, thiscall): the round draw by seeding: the teams leave the ties, sorted by seed (highest first), and each goes to a drawn tie; on the first pass only empty ties (with both slots free, the side is drawn), then any free slot, with rule 4 keeping clubs of one nation apart until there is no way;
- `copa_sortear_separando_nacoes()` = 0x51d9f0 (Cup.cpp, thiscall): the round draw that keeps nations apart: each team's level becomes how many teams of its nation are in the round (the bigger nations go first) and, from the second pass on, a team does not land in a tie with another of its nation while possible;
- `copa_comeco_da_temporada()` = 0x518780 (Cup.cpp, thiscall): the base cup season start: clears byte +0xb1;
- `nacao_chave_do_clube()` = 0x48dee0 (comp.cpp, cdecl): the active index of the club's nation among the active key nations; otherwise 0 for a national team and 3 for a club (the type used by the fixture search and the date clash check);
- `nacao_chave_do_estadio()` = 0x48df40 (comp.cpp, cdecl): the active index of the stadium city's nation among the active key nations, or 3;
- `entrada_da_nacao_chave_ativa()` = 0x48dcd0 (comp.cpp, cdecl): the first key nation table entry with that active index; none, the error box and 3;
- `conflito_de_jogos_no_mesmo_dia()` = 0x5ae380 (fix_man.cpp, thiscall): the new fixture and the first one on the same day: the higher priority stays (the competition one, +21 with a national team at home; ties by the home club reputation), as long as the other was not played yet; the one that leaves goes to the target weekday from the other date (weekdays 0 and 5 go to 6, 1 and 3 to 2, 2 to 1, 4 and 6 to 5) or, with a clash there, to another date, with the rescheduling news; a fixed fixture with a clash does not move (returns 0);
- `primeiro_ano_com_jogos()` = 0x5aee40 (fix_man.cpp, thiscall): the club's first year with fixtures: 2001, or 2002 by bit 0 of its word in [0xae2a5c];
- `ultimo_ano_com_jogos()` = 0x5aee60 (fix_man.cpp, thiscall): the club's last year with fixtures: its word in [0xae2a5c] without bit 0, or the current year;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (36):** 35 caught. `cqb6_so_acima_de_30` escaped: no sale by 2002-06-01 actually paid the previous clubs' share (the "cqb_6 parte paga" branch never ran in the base), so restricting it to players over 30 changes nothing; the routine-by-routine comparison did not flag it either. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (0bb74e04).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6299f556, original cbf229f9 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 263's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 264`

**Fields touched:**
- club finances (0x167 bytes) +0: the balance (i64); value; adds and subtracts (all three)
- finances +0x158 / +0xb8: season and month TV money; +0x140 / +0xa0: season and month net match income; +0x12c / +0x8c: season and month stadium upkeep; value; adds (renda_da_partida)
- finances +0x128 / +0x88: season and month fines; value; adds (pagar_multa_da_liga)
- finances +0x160: the ticket discount (float); value; reads (renda_do_estadio)
- fixture +0x14 the competition (+0x69 word), +0x18 the stadium (+0x3c capacity), +0x1c / +0x20 the clubs, +0x3a the type, +0x3b the stage, +0x3c neutral ground, +0x40 the cost floor; pointer and value; reads (renda_da_partida, renda_do_estadio)
- result +0x34 the attendance, +0x38 the gate receipts; value; reads (renda_da_partida)
- club +0x53 the nation (+0x7e the type), +0x57 the league, +0x64 the flag (1: costs, expenses and price by bands), +0x69 / +0x6d the own stadium (+0x3c), +0x80 the reputation; pointer and value; reads (renda_da_partida, renda_do_estadio)
- club finances +0 the balance (i64), +0xb4 / +0x154 month and season investment, +0x165 the crisis state; value; reads and writes (impulso_das_financas, troca_do_presidente)
- finances +0x120 / +0x80: season and month other costs; value; adds (pagar_outros_custos)
- club +0x53 the nation, +0x57 the league (+0x69 word), +0x64 the flag, +0x80 the reputation, +0x82 the boost flag (type 7); pointer and value; reads and writes +0x82 (impulso_das_financas, troca_do_presidente)
- club +0xbf the chairman: +0x1a the nation, +4 / +8 / +0xc the names, +0x57 byte, +0x69 the record (+0xf, +0x16, +0x1d, +0x20); pointer and value; reads and writes (impulso_das_financas: +0x20 drops 1; troca_do_presidente: all drawn)
- [0xae2350] / [0xae23a8] the nation count and table (0x122 bytes); value and pointer; reads (troca_do_presidente)
- Friendly +0x34ec / +0x34f0: the international date table (20 bytes: +0 weekday, +1 day, +2 month, +3 the 8-byte date, +0xb year offset, +0xc confederation, +0x10 competition) and the count; pointer and value; allocates, writes and sorts (tabela_das_datas_internacionais, resolver_datas_internacionais)
- Friendly +0x34f4: the last day done of the international friendlies (word); value; writes 400 (tabela_das_datas_internacionais)
- Friendly +0xb1: the 127 tournaments (0x69 bytes: +0 the host, +0x29 in use); pointer; reads (torneio_do_clube_humano)
- [0x9cfa08..0x9cfa1c] the confederations and [0x9cf76c..0x9cf784] the qualifying competitions of the dates; id; reads (tabela_das_datas_internacionais)
- [0xae2c92] the current year; value; reads (resolver_datas_internacionais)
- staff +0x39: the club of the current human manager ([0xb74c26 + 0x300 × [0xb77c7a]]); pointer; reads (torneio_do_clube_humano)
- Friendly +0x34d8 / +0x34dc / +0x34e0 / +0x34e4: the friendly plan (20 bytes), zeroed, the count and the capacity; pointer and value; frees, allocates and writes (amistosos_dos_clubes)
- [0xb63d60] the 35 active leagues (0x48 bytes: +0 the nation, +0x2a the season start, +0x39 the pre-season date); pointer and value; reads (amistosos_dos_clubes)
- friendly candidate +0 the club id, +4 the friendly count (byte), +5 the band, +6 the nation; value; reads (amistosos_dos_clubes)
- club +0x53 the nation, +0x57 the league, +0x80 the reputation, +0x1db the id compared with [0x9cf6f8]; pointer and value; reads (amistosos_dos_clubes)
- Friendly +0xb1: the 127 tournaments (0x69 bytes: +0 host, +4 the date, +0xc type, +0xd/+0xe bytes, +0xf/+0x11/+0x13 words, +0x15 the 4 invited clubs, +0x25 the flags, +0x29/+0x2d/+0x65 zeroed, +0x31 the name); pointer and value; writes (acrescentar_torneio)
- Friendly +0x34c8: the friendly list (+2 the head, +0xa the cursor; friendly +8 still without date, +0xc the club); pointer; reads and writes the cursor (convite_aberto_do_clube_humano)
- fitness record (31 bytes, [Injury+0]) +0xe / +0x12 / +0x13 / +0x14 / +0x18 / +0x1c / +0x1d: the current injury (slot, −1 = none), the recurrence type and count, the history, the flag and the last type; id and value; reads and writes (lesao_nova)
- injury (0x30 bytes, [Injury+4], count [Injury+8]) +0 type (0xff = free slot), +1 match record (+0x1a = 0xff without a match), +0x20 date, +0x28 days, +0x2a return to training, +0x2c/+0x2e the two counters; value; writes (lesao_nova), reads (noticia_da_lesao)
- injury type table (13 bytes per type at 0x9e7adb): +0 base days, +2 variable days, +4 return-to-training percentage, +5 can recur; value; reads (lesao_nova)
- player profile (+0x61) +0x2d injury proneness, +0x34 natural fitness, +0xd potential; value; reads (lesao_nova, noticia_da_lesao)
- 79-byte record ([0xae2d0c]) +0xa: rises with the injury up to 100; value; reads and writes (lesao_nova)
- club +0x1cf: the three physios (with +0x69 the non-player profile); pointer; reads (lesao_nova)
- shortlist [[0xde1f54]+0x856] +0xdc + club id × 9 + 8: the long-injury mark 100; value; writes (lesao_nova)
- AWOL ([0xad9c6c]) +0xc + id: bit 0 absent without leave; value; reads (ausente_sem_licenca)
- [0xae1080]: the last contract looked up; pointer; writes (noticia_da_lesao, ausente_sem_licenca)
- offer +4 / +8 / +0xc / +0x1b: the exchange (LLIST of ids), the buyer, the person and the fee; id and value; reads (all three)
- person +0x18: the age; value; reads (parte_dos_clubes_anteriores)
- person +0x52: the value of the exchange players; value; reads (percentual_da_proxima_venda_pago)
- 79-byte record [0xae2d0c] +0xe + 5k / +0x12 + 5k (k = 0..4): the former clubs (−1 = empty slot) and their weight; id and value; reads (parte_dos_clubes_anteriores)
- club +0x80: the reputation (the floor of the cut news); value; reads (parte_dos_clubes_anteriores)
- club finance (0x167 bytes) +0 balance, +0x78/+0x118 sales, +0x70/+0x110 purchases; values; writes through 0x599800/0x599890 (parte_dos_clubes_anteriores on the buyer, percentual_da_proxima_venda_pago on both clubs)
- [this+0x880] / [this+0x888]: the next-sale percentage clauses (0xd bytes: +0 percentage, +1 selling club, +5 clause club, +9 person) and their count; pointer and value; reads (percentual_da_proxima_venda_pago, noticia_do_percentual_da_venda)
- [0xae1080]: the last contract looked up; pointer; writes (all three)
- [this+0x86e] + id × 8: +0 bit 0x20 has a permit record, +3 the 16-match national team history (u16), +5 the count of national team matches seen (u16); value; reads and writes (permissao_concedida)
- [this+0x93a] / [this+0x93e]: the permit records (14 bytes: +0 id, +4 matches, +6 refusals) and their count; pointer and value; reads (permissao_concedida)
- person +4 / +0x18 / +0x1a / +0x22 / +0x61 / +0x69: the name, the age, the nation, the caps, the profile and the non-player profile; pointer and value; reads (permissao_concedida)
- profile +7 / +0xd: the ability and the potential; value; reads (permissao_concedida)
- non-player profile +0xc: the reputation; value; reads (permissao_concedida)
- nation +0x8e: the reputation; [[0xb74318]+4] + id × 0x6e + 2: the national team matches; value; reads (permissao_concedida)
- contract offer +4 / +0xc / +0x2d: the club, the wage and the date; id and value; reads (permissao_concedida)
- contract +0x4f: the squad status (>> 4); value; reads (estatuto_pelo_motivo)
- [this+0xc] / [this+0x10]: the negotiation array and its count; pointer and value; reads (vaga_da_negociacao)
- [0xae1080]: the last contract looked up; pointer; writes (estatuto_pelo_motivo)
- club +0x9b..+0xb3 favoured and disliked staff; pointer; reads and writes (reacao_do_elenco_a_historia, via 0x547940)
- club +0xb3/+0xb7/+0xbb rivals, +0x57 division, +0xbf, +0xcf manager, +0xd3, +0xd7 squad, +0x19f; pointer; reads
- person +0x18 age, +0x59/+0x5a/+0x5b/+0x5d attributes, +0x61 player record (+0xb reputation), +0x65 preferences (+0x1c..+0x33), +0x69 non-player record (+0x1d); value/pointer; reads (and writes the preferences via 0x547af0)
- contract +0x3d happiness, +0x40 flags, +0x45 complaints, +0x4f type; value; reads and writes (via 0x867560/0x867700)
- [0x9d0118]/[0x9d011c] Celtic and Rangers, [0x9cf764] lower division, [0xadadfc] competitions; id; reads
- [0x7356f4] story kind byte table; value; reads (rotulo_da_resposta_da_midia)
- discipline +0xc nation index (8 bytes: +0 person id, +4 nation id), +0x12 count; pointer/value; reads (cumprir_suspensoes_internacionais)
- discipline +4 bans (20 bytes: +0 matches, +1 counter, +2 days); value; reads and writes (recurso_da_suspensao)
- discipline +0x14 29-byte record per person (5 bytes per slot: the rule); value; reads (texto_da_vaga_da_suspensao)
- ruling body: 14-byte person entry (+6 ban index); value; reads (0x7e82e0)
- club (national team) +0x53 nation; pointer; reads
- list (14 bytes) +0 count (word), +2 head, +6 tail, +0xa cursor; value/pointer; reads and writes (tirar_da_lista, esvaziar_lista)
- node (14 bytes) +0 item, +4 size (signed word), +6 next, +0xa previous; pointer/value; reads and writes the links
- cup +0xa3 rounds (0x68 bytes: +0xb rule 4, +0xd rule 0x400, +0x18 slots, +0x1a ties); pointer/values; reads (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- cup +0xad ties per round (0x35 bytes: +0/+2 teams, +0xe/+0x10 and +0xa/+0xc links, +6/+8/+0x12 the previous tie); values; reads and writes (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- cup +0xa7 teams (6 bytes: +0 club, +4 seed); pointer/value; reads (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- club +0x53 nation (+0 id); pointer; reads (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- [0xae2350] nation count; value; reads (copa_sortear_separando_nacoes)
- cup +0xb1; value; writes 0 (copa_comeco_da_temporada)
- [0xb63d60] table of the 35 key nations (72 bytes: +0 nation, +0x14 active index); pointer/value; reads (all three)
- nation +0x11c: active nation (with loaded leagues); value; reads (nacao_chave_do_clube, nacao_chave_do_estadio)
- club +0x53 nation; pointer; reads (nacao_chave_do_clube)
- stadium +0x38 city, city +0x1f nation; pointers; reads (nacao_chave_do_estadio)
- fixture (0x4f bytes) +0x14 competition, +0x1c home club, +0x28 year, +0x2a day, +0x3a fixed, +0x43 result (0xff unplayed); pointers/values; reads, writes the date in the copy (conflito_de_jogos_no_mesmo_dia)
- competition +0x69 priority; value; reads (conflito_de_jogos_no_mesmo_dia)
- club +0x80 reputation; value; reads (conflito_de_jogos_no_mesmo_dia)
- list (LLIST) +2 head, +0xa cursor; pointers; reads and writes the cursor (conflito_de_jogos_no_mesmo_dia)
- [0xae2a5c] + 2 × club id: years with fixtures (bit 0 the first, the rest the last × 2); value; reads (primeiro_ano_com_jogos, ultimo_ano_com_jogos)
- [0xae2c92] current year; value; reads (ultimo_ano_com_jogos)

**Of interest to:** CMRTE, GPF2, SGE
