# Batch 267 — the bid accepted by the selling club (the refusal to move on loans, exchange players too expensive and the rescheduled offer), the news of the offer made and whether a loan is possible; the league: the daily update (stages, season end, TV, registration deadline), the creation of the season fixtures and promotion and relegation; the fixture manager: the daily update (reserves, fixture news, yesterday's friendlies, round labels), the fixture list of a day and the loading of the year's fixtures; the cup and the stages: the cup fixture tree (the slots in the next round and the round teams, drawn), the first stage of a group and the current stage; player training brought up to date: the weekly progress, the player schedule and the category level; the stored training progress: remove and restore the progress, how much training adds to an attribute and the aptitude for a position; the daily retirements: the announced and the due ones, the retirement news and the youth promotion news; the end of the missed training and of the absence without leave and the missed training news; the compensation deal, the order of the shortlist slots and the player under offer; the league and cup champions records and the cleared human manager record

**Routines:**
- `aceitar_lance()` = 0x8c0080 (transfer_manager.cpp, thiscall, ret 8): the seller accepts the bid: status 5; on loans, target interest below 0.75 gives the refusal to move; exchange players wanting more than 1.5 times the seller wage cap leave, with their value added to the fee; the offer goes to the next slice;
- `noticia_da_oferta_feita()` = 0x8d3090 (transfer_manager.cpp, thiscall, ret 4): news 0xfa0 of an offer made (target, clubs, values, exchange, contract, window) to the interested human managers;
- `pode_emprestar()` = 0x8d0040 (transfer_manager.cpp, thiscall, ret 0xc): whether the club can take the player on loan: current club, contract record, playing role, whether the club can reach him, the rule of the club nation and of the current club month by month over the loan period; the reason text in the message;
- `liga_dia()` = 0x6847c0 (league.cpp, thiscall, ret 8): the league day: the fixture list news on its day, each stage's day, the +0xc9 date event, the season end (next stage, history, records), the TV fixtures four weeks ahead, the registration deadline 5/20/14 days before the first date and the +0xd1 date;
- `liga_criar_tabela()` = 0x6835c0 (league.cpp, thiscall, ret 0): the season fixtures: the round robin grid (with the club order drawn) or the year's own list, home and away by the parity of the half and the year, the stadium, date and slot of each round, and the calendar; then the TV dates;
- `liga_promover_e_rebaixar()` = 0x689c80 (league.cpp, thiscall (ecx repassado), ret 0x18): the non-promotion clauses, the relegated clubs go down (relegation clauses, human manager sacked in a competition with no active record) and the promoted clubs go up, in pairs or all; the box if the counts differ from the expected ones;
- `calendario_dia()` = 0x5aaf00 (fix_man.cpp, thiscall, ret 0): on days 2 and 3 the reserve fixtures 3 days ahead (and tomorrow on day 3); the queued fixture news to both clubs' managers; on days 5 and 6 yesterday's friendlies between clubs without a human manager leave the lists; the cup round labels from the leg check list;
- `calendario_lista()` = 0x5abf10 (fix_man.cpp, thiscall, ret 0x10): the fixture list of a year, day, slice and key nation, switching the other year block to the requested year or creating it;
- `calendario_carregar_ano()` = 0x5ac3b0 (fix_man.cpp, thiscall, ret 0): reads fixtures_<year>.tmp: each 0x4f-byte fixture gets its club, competition and stadium pointers and goes into the list of its day, slice and key nation;
- `copa_arvore_dos_confrontos()` = 0x518790 (Cup.cpp, thiscall, ret 0): allocates the ties of each round, clears them, links each tie to a slot in the next round (by pair or drawn, the side by a coin; with no slot, two rounds ahead), puts each round's teams into its ties (drawn, the cup nation's club at home) and numbers the first tie of each round;
- `competicao_primeira_fase_do_grupo()` = 0x48d2b0 (comp.cpp, thiscall, ret 4): from the given stage backwards, the index of the first stage of the same group;
- `competicao_fase_atual()` = 0x48cec0 (comp.cpp, thiscall, ret 0): the first stage not finished (in a cup not yet started, the start of its group); all finished, the start of the last one's group;
- `atualizar_o_treino_do_jogador()` = 0x8b1750 (training_manager.cpp, thiscall): brings the player training weeks up to date (up to 30): the human club schedule, the progress rolls of each category by the schedule and coach weight, the position training (the rating, the end and the player complaint) and the progress on the attributes;
- `programa_de_treino_do_jogador()` = 0x8b39f0 (training_manager.cpp, thiscall): the player training schedule: the club human manager one by the entry slot (the rest one when injured) or the default schedule of the slot;
- `nivel_da_categoria_do_programa()` = 0x8b4f50 (training_manager.cpp, thiscall): the level of one of the five training schedule categories;
- `tirar_ou_devolver_o_progresso()` = 0x8b3640 (training_manager.cpp, thiscall): takes the progress stored in the training entry off the attributes (keeping the five nibbles) or puts it back, around the player attribute normalisations;
- `mudanca_do_atributo_pelo_treino()` = 0x8b37c0 (training_manager.cpp, thiscall): how much the training progress adds to an attribute: the attribute minus the same attribute without the progress;
- `aptidao_para_a_posicao()` = 0x8b4770 (training_manager.cpp, thiscall): the player aptitude to learn a position or side: the average of the adaptability and the attributes the position needs;
- `aposentadorias_do_dia()` = 0x7a59e0 (player_regen.cpp, thiscall): the pending retirements of the day: drops those of young players with a club, of players with a pending contract, under offer or recently moved; retires (news, contract, national team, squad), may turn the player into a coach (attributes from reputation and record), stores the record in the regen bank or queues the regen slot, and announces the future ones;
- `noticia_da_aposentadoria()` = 0x7a92e0 (player_regen.cpp, stdcall): news item 0xbba about the retirement for the managers (all, the country, the division, the club or the interested clubs, by fame) and the club insurance payout when injury retires a valuable player;
- `noticia_do_jovem_promovido()` = 0x7a8ba0 (player_regen.cpp, stdcall): news item 0xbce about the youth player joining the club, with the assistant manager or a coach verdict on his ability;
- `limpar_ausencia()` = 0x4196a0 (awol.cpp, thiscall): the absence ends: with bit 1 of the person's flag, the bit flips and her record leaves the absence list;
- `limpar_falta_ao_treino()` = 0x4197c0 (awol.cpp, thiscall): the missed training ends: with bit 2 of the person's flag, the bit flips and her record leaves the missed training list;
- `noticia_da_falta_ao_treino()` = 0x41a630 (awol.cpp, thiscall): the 0xbc9 missed training news: the player, the club, a draw of 3, whether the club is human-run and who reports it (the AI club's manager, or by draw one of the 5 coaches or the assistant); to the player's club if it is human-run, else to the nation when the player's reputation is above 7250;
- `troca_com_compensacao()` = 0x84b6f0 (shortlist_manager.cpp, thiscall): the club takes the player paying the compensation: outside the FIFA protected period, if the fee (0x8d0780) is below his value; inside it, only with the Bosman rule and the fee up to the value, by the contract expiry band (0 no; 1-8: the slot's +3 × 0.5 to 0.9 above the price; 9-14 yes);
- `ordem_das_vagas()` = 0x83b6c0 (shortlist_manager.cpp, cdecl): the qsort criterion of the 40 shortlist slots: the difference of the indices (+0x23); equal, 1;
- `marcar_sob_oferta()` = 0x84b580 (shortlist_manager.cpp, thiscall): the player under offer: his slot in the interested clubs' lists (except human clubs, the buyer or the clubs of the exchange offers) gets bit 0x20, which doubles the AI bid price;
- `campeao_da_liga_nos_registros()` = 0x868c50 (staff_records.cpp, thiscall): the league champion: the club reputation (up to weight × 500), the +0x7b and the +0x69 block, the manager's ratings and record (weight × 5000), the histories (type 5), the squad morale by the player's average (above the threshold: +20 and +100 on the contract with the renewal; else +10 and +50) and the media praise;
- `campeao_da_copa_nos_registros()` = 0x86a800 (staff_records.cpp, thiscall): the cup winner: the club reputation, its side's ratings in the match, the manager's record (weight^4 × 0.5), the histories (type 0), the squad morale (+15 and +100 on the contract, with the renewal) while the reputation stays up to weight × 400, the end of the call-ups and the media praise;
- `limpar_registro_do_humano()` = 0x86fc70 (staff_records.cpp, thiscall): the human manager record (one of the 16; else the person itself, as in the original) cleared, with −1 in the slots;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (29):** 28 caught. `cqd10_fica_com_o_contrato` escaped: the retired player leaves the squad anyway (0x7aace0) and his slot goes to the regen; the leftover contract belongs to someone out of the game, and no routine in the career to 2002-06-01 decided anything from it (about 18 actual retirements); the routine-by-routine comparison did not flag it either [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (32ca2db6).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 f8972df1, original e3b7b045 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 266's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 267`

**Fields touched:**
- offer +0 list, +4 exchange (nodes with +5 status), +8 buyer, +0xc target, +0x11 type, +0x1b fee, +0x20, +0x24, +0x2c status, +0x2d, +0x2e, +0x2f, +0x30, +0x31; id and value; reads and writes (aceitar_lance), reads (noticia_da_oferta_feita)
- transfer manager +0x86e 8-byte records (bit 0 of +0, count +2), +0x856 the shortlist manager; value; writes (aceitar_lance)
- shortlist entry +0 role, +0x17 offer index; value; reads and writes (aceitar_lance)
- nation rules (the 28 at +0x8ac) +0xa nation, virtuals +0x20 and +0x24; person +0x3d role; club +0x38 name, +0x52, +0x53 nation; pointer and value; reads (pode_emprestar)
- contract +0x2d/+0x2f end, +0x35 type; [0xae1080] the last contract; value and pointer; reads (noticia_da_oferta_feita), writes [0xae1080] (both)
- league +0x2c stages, +0x30 current stage, +0xc the stages, +0x3c rounds per pair, +0x3e teams, +0x40 year, +0x44 stage (0xff first), +0x45 clubs, +0x4c end done, +0x52 bit 2 registration, +0xa7 rounds, +0xa9 dates, +0xad, +0xb1 table (0x3b bytes: +0 club, +0x37 status, +0x38 flag; +0x11 in progress), +0xb5 stopped, +0xba date table (0x41 bytes), +0xc9 event date, +0xd1 last date, +0xdd season over, +0xe6, +0xea own list; value and pointer; reads and writes (liga_dia, liga_criar_tabela, liga_promover_e_rebaixar)
- fixture (0x4f bytes) +0 competition, +8, +0xc/+0x10 ids, +0x14 competition, +0x1c/+0x20 clubs, +0x24/+0x26 last date, +0x28 year, +0x2a day, +0x2c league year, +0x34 round, +0x3f slot, +0x40/+0x41 flags, +0x42 stage, +0x4e bit 3 last round; value and pointer; writes (liga_criar_tabela)
- club +0x57 competition, +0x5b previous competition, +0x69 nation; pointer; reads and writes (liga_promover_e_rebaixar), reads (liga_criar_tabela)
- fixture manager (year block, 0x1184 bytes) +0..+0x1127 the lists of 366 days × 3 slices (vector by key nation), +0x1128 the fixture news queue, +0x112c the other year block, +0x1130 the year, +0x1133 the file name; pointer and value; reads and writes (calendario_dia, calendario_lista, calendario_carregar_ano)
- fixture (0x4f bytes) +0 competition (id → +0x14), +4 stadium (id → +0x18), +0xc/+0x10 clubs (ids → +0x1c/+0x20), +0x2a day, +0x3a type, +0x3f slot; id, pointer and value; writes the pointers (calendario_carregar_ano), reads (calendario_dia)
- [0xae2a48] the leg check list (+4 stage, +5 round, +7/+0xb the clubs); value; reads and empties (calendario_dia)
- [0x9cf684] the friendlies competition; id; reads (calendario_dia)
- cup +0x3c rounds, +0x3e teams, +0x45 clubs, +0xa3 rounds (0x68 bytes: +7 type 0x8c, +0xd bits 0x200 by pair and 0x400 fully free slot, +0x18, +0x1a ties, +0x1c entering teams, +0x1e first team, +0x5b first tie), +0xa7 teams (6 bytes: +0 club, +5 flag), +0xab/+0xac, +0xad tie arrays, +0xb1 no ties, +0x24 nation; value and pointer; reads and writes (copa_arvore_dos_confrontos)
- tie (0x35 bytes) +0 home, +2 away, +6 slot in the target round, +8 side (1 home), +0xa/+0xc source ties, +0xe/+0x10 source rounds, +0x12 target round, +0x14..+0x31, +0x32..+0x34; value; writes (copa_arvore_dos_confrontos)
- competition +0xc stages, +0x30 last stage, +0x3a id (group by 0x4b01d0), +0x43 cup, +0xab started; value and pointer; reads (competicao_primeira_fase_do_grupo, competicao_fase_atual)
- training (TRAINING_MANAGER [0xde1f44]) +0x651d 6-byte entries per player record (+0 slot, +1 week, +3..+5 progress nibbles, +5 flags 0x10/0x20); pointer/value; reads and writes (atualizar_o_treino_do_jogador)
- training +0x688d training week; value; reads; +0x6521/+0x6525 position training list (8 bytes: +0 id, +4 mask, +6 rating, +7 rate); pointer/value; reads and writes
- training + 0x9b × slot schedules (+4..+8 levels, +0x15 type); the human ones at +0x4d8 + 0x60e × index; value; reads (programa_de_treino_do_jogador, nivel_da_categoria_do_programa)
- person +0x39 club, +0x58 and +0x5b (progress limits), +0x61 record (+0xf goalkeeper, +0x1b..+0x41 attributes through 0x8b2be0); value/pointer; reads
- club contract +0x44 happiness, +0x45 flag 0x8000; value; reads and writes (the position training complaint)
- [0xae1080] last contract looked up; pointer; writes; [[0xb63d28]] + 31 × id + 0x1e training load; value; writes
- training (TRAINING_MANAGER [0xde1f44]) +0x651d 6-byte entries per record (+3..+5 progress nibbles); pointer/value; reads (and writes through 0x8b2be0)
- 0xde1f3c..0xde1f40 the removed progress (categories 3, 2, 4, 0, 1); value; writes and reads (tirar_ou_devolver_o_progresso)
- person +0x56 adaptability; value; reads; record +0x27, +0x2a, +0x30, +0x32, +0x33, +0x35..+0x37, +0x39, +0x3b, +0x3e..+0x40 attributes; value; reads (aptidao_para_a_posicao)
- regen (PLAYER_REGEN [0xb7434c]) +4/+8 pending retirements (26 bytes: +0 person, +4 club, +8/+0xa announcement, +0x10/+0x12 date, +0x18 reason, +0x19 announced); pointer/value; reads and writes (aposentadorias_do_dia)
- regen +0x10/+0x14 regen slots (12 bytes: person, day, year, leap flag 1); +0x1c/+0x20 spare non-player record pool; pointer/value; reads and writes
- [0xae2d0c] + 79 × id +0xb retirement flags (0x10 announced, 0x20 slot, 0xf); value; reads and writes
- person +0x18 age, +0x1a/+0x1e nations, +0x22 reputation, +0x39 club, +0x3d job, +0x3e/+0x42 joined date, +0x52, +0x57, +0x5b, +0x5f, +0x61 player record (+9, +0xb, +0xd, +0xf), +0x69 non-player record (+8, +0xa, +0xc, +0x10, +0x11); value/pointer; reads and writes
- club +0x53 nation, +0x57 division, +0x7f strength, +0x80 reputation, +0xd3 assistant, +0x19f coaches; pointer/value; reads (the news)
- [[0xde1f54] + 0x856] (+0xdc 9-byte rows per club, +8 = 100); pointer/value; writes
- AWOL +0xc flag table (one byte per person: bit 1 absent, bit 2 missed training); value; reads and writes (limpar_ausencia, limpar_falta_ao_treino)
- AWOL +0 absence list, +4 missed training list (LLIST: +0 count, +2 head, +0xa cursor; record +0 person id); pointer; reads and writes (limpar_ausencia, limpar_falta_ao_treino)
- person +0 id, +4/+8/+0xc keys (+0x33), +0x39 club, +0x61 player record (+0xb reputation); id/pointer/value; reads (noticia_da_falta_ao_treino)
- club +0xcf manager, +0xd3 assistant, +0x19f the 5 coaches, +0x53 nation; pointer; reads (noticia_da_falta_ao_treino)
- shortlist slot (0x25 bytes) +3 budget, +0xb price, +0x23 index, +0x24 flags (bit 0x20 under offer); value; reads (troca_com_compensacao, ordem_das_vagas) and writes the bit (marcar_sob_oferta)
- contract +0 person id, +0x49 byte that skips the check; id/value; reads (troca_com_compensacao)
- person ([0xae23bc] + 110 × id) +0x52 value; value; reads (troca_com_compensacao)
- Shortlist_Manager +0xdc club table (9 bytes per club; +0 the 40 slots); pointer; reads (marcar_sob_oferta)
- club +0x80 reputation; value; reads and writes (campeao_da_liga_nos_registros, campeao_da_copa_nos_registros)
- club +0x57, +0x73, +0x7b and the +0x69 block (+0x44); value; reads and writes (campeao_da_liga_nos_registros)
- fixture +0x188/+0x18a/+0x18c side ratings (0x625 per side), +0x190 lineup (20 × 0x49 bytes); value; reads and writes (campeao_da_copa_nos_registros)
- manager's non-player record +8/+0xa/+0xc ratings; value; writes (campeao_da_liga_nos_registros)
- manager record (0xd2 bytes, Staff_Records +8) +0x8a/+0x8c titles, +0xb6/+0xba/+0xbe/+0xca points; value; writes (both) and clears (limpar_registro_do_humano)
- contract +0x44 morale, +0x4f status (>> 4), +0x25 date; value; reads and writes (both)
- competition +0x69 weight; object [0xadadfc] + 4 × id: +0x3c/+0x3e (threshold), +0x42 phase; value; reads (both)

**Of interest to:** CMRTE, GPF2, SGE, CMScout
