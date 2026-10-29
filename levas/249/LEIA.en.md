# Batch 249 — the cup: who goes through in the match and in the tie and the seed order in the draw; the league table: setting up the club row, the club array and the final positions; the club division move and the shuffles of the club lists; discipline: counting a served match of a ban, releasing a ban and clearing a person's ban history; injuries: rolling a recurring injury, rewinding the injury history and resetting a player's fitness; club records: adding the transfer fee to the sale and purchase totals, the club's career appearances and goals records and the season rating record; friendlies: whether a competition is current on the date, whether the club already has a friendly near the date and adding the planned friendly

**Routines:**
- `decidir_jogo_da_copa()` = 0x51c020 (Cup.cpp, thiscall, ret 4): who goes through in a cup match (1 home, 2 away, 0 nobody) by the score, extra time, penalties, walkover and points of the rule (+0x38), storing each side's code (+0x3d/+0x3e);
- `decidir_confronto_da_copa()` = 0x51f2f0 (Cup.cpp, thiscall, ret 8): in the first leg marks the winner and returns 3; in the second leg decides by aggregate, walkover, extra time, away goals and penalties (1 or 2 the side through, 4 undecided);
- `ordem_dos_cabecas_de_chave()` = 0x51ebc0 (Cup.cpp, cdecl): the qsort comparison of the 7-byte draw entries: the higher seed (+6) first;
- `iniciar_linha_da_tabela()` = 0x687430 (league.cpp, thiscall, ret 0x10): puts the club in table row i: totals cleared, status 0xff, flag, total rounds (+0xa7) if still −1, position history and, under rule 4, the head-to-head entries (the last row fills every row's opponents);
- `vetor_dos_clubes_da_liga()` = 0x687a20 (league.cpp, thiscall, ret 8): a new array with the table clubs and the count; without memory, error box and count 0;
- `gravar_posicoes_finais_da_liga()` = 0x68aa30 (league.cpp, thiscall): each club still in the league division stores last season's division (+0x5b) and the final position (+0x5f);
- `mudar_clube_de_divisao()` = 0x6830b0 (league.cpp, thiscall, ret 0xc): the current division becomes last season's and the new one is set; clears the division clauses (and the squad numbers when the new one uses them), opens the requested manager vacancy, upgrades the stadium to the league minimum and turns status 3 into 2;
- `embaralhar_ponteiros()` = 0x4afb40 (comp_util.cpp, cdecl): shuffles an array of n pointers with 3 × n swaps of drawn pairs;
- `ordenar_e_embaralhar_os_melhores()` = 0x4afbd0 (comp_util.cpp, cdecl): sorts the clubs by reputation (nulls last) and shuffles the first min(3 × picks, n);
- `contar_jogo_da_suspensao()` = 0x556e90 (discipline.cpp, thiscall, ret 0xc): the person's ban in the rule's ruling body: match bans, from the start date (+4/+6) and when counting, +1 to the counter (+1); on reaching the total (+0), or day bans once today reaches start + days (+2, 0x54c5b0), it goes to the history (0x58e340) and is released (0x556d00);
- `soltar_suspensao()` = 0x556d00 (discipline.cpp, thiscall, ret 8): each ruling body of the ban's 4 rules (+0xc..+0xf) forgets it (0x7e8590); on the fourth, the person's item leaves the nation index (last one moved in, qsort); the ban becomes free (+0 = 0xff, +2 = −1);
- `apagar_historico_de_suspensoes()` = 0x55e1d0 (discipline.cpp, thiscall, ret 4): the last offset of the person's ban history (+0x18 of the 29-byte record) = −1 (0x58f110);
- `sortear_lesao_recorrente()` = 0x62e500 (injury.cpp, thiscall, ret 8): with 1 in 20, a recurring-capable injury becomes recurring in the record (+0x12 type, +0x13 = 0); among the recurring records, the best score (roll 0x65 − 5 × level − proneness, stops at 95): a player at a big club (reputation > 4250) without a human and 1 in 3 gets a new injury 0x4d/0x4e/0x4f (0x62d250); otherwise his mark is cleared; in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run;
- `recomecar_historico_de_lesoes()` = 0x631f90 (injury.cpp, thiscall, ret 4): the last offset of the person's injury history (+0x18 of the 31-byte record) = −1 (0x58f110);
- `reiniciar_fisico_do_jogador()` = 0x62bfa0 (injury.cpp, thiscall, ret 4): removes the id items from list +0x28, ends the injury (0x62e700), rewinds the history (0x631f90) and the 31-byte record (today, condition and form 10000, wear 0, no injury nor recurrence); bit 0 of +0xc of the 79-byte record is cleared;
- `somar_totais_de_transferencia()` = 0x448ad0 (club_records.cpp, thiscall, ret 0xc): adds the fee to the seller's season income and the buyer's season spending, raises the all-time records and queues the record news (0x1f, 0x1e) above (+0x80 + 500) × 1000;
- `recordes_de_carreira_no_clube()` = 0x448cb0 (club_records.cpp, thiscall, ret 0x10): the club's all-time league appearances (+0x247) and goals (+0x261) records, with the player reference (0x7e4920), when the record type applies to the club (0x449590);
- `recorde_de_nota_da_temporada()` = 0x452a90 (club_records.cpp, thiscall, ret 4): the squad player with the best average rating (statistic 0x11) among those who played at least half the games becomes the season rating record;
- `competicao_na_data_do_amistoso()` = 0x5c29b0 (friendly.cpp, thiscall, ret 8): returns 1 for an ordinary competition; for a friendly tournament, only when the date is within 7 days of the slot date;
- `amistoso_perto_da_data()` = 0x5c45a0 (friendly.cpp, thiscall (this sem uso), ret 8): returns 1 when the club already has a planned friendly on the date, the day before or the day after;
- `acrescentar_amistoso_planejado()` = 0x5c4b60 (friendly.cpp, thiscall, ret 0x10): appends the friendly (date and three ids) to the 0x14-byte array, which grows by 100;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (16):** 14 caught. `cpm_confronto_adversario` escaped: iniciar_linha_da_tabela 0x687430: with the club itself in place of the opponent, the head-to-head tiebreak (League::rank_by_head_to_head 0x682b30, which finds the slot by the opponent id) finds no slot and leaves the order to the next rule; the change only applies to tables built during the career (32 or more last rows with rule 4, counters in the batch 249 DLL) and up to 2002-06-01 no tie decided by it changed the career; in the game comparing the comparator runs only the original; the exact cause was not isolated. `cpm_vetor_conta` escaped: vetor_dos_clubes_da_liga 0x687a20: the only caller is the nation manager of the year award (0x768940, nation_awards.cpp, through vtable +0xa0), which sorts the clubs by manager points (0x86fab0) and only goes on with 3 candidates; without the last club of the table (8 or more calls) the candidates are the same and the award does not enter the career. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2f65fd6b68d1027b).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (be54a1386f51073a).

**Redo**, on top of batch 248's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 249`

**Fields touched:**
- match +0x38: the rule (0x10 points, 2 extra time, 0x21 penalties, 0x800/0x1000 walkover, 0x2000 −3 codes, 0x4000 and 4 away goals); value; reads (decidir_jogo_da_copa, decidir_confronto_da_copa)
- match +0x3d/+0x3e: each side's result code (1, 2 won, 3 extra time, 4/5 penalties, 6 away goals, 8 walkover); value; reads and writes (decidir_jogo_da_copa, decidir_confronto_da_copa)
- match +0x43/+0x44, +0x47/+0x48, +0x49/+0x4a: score, extra time and penalties; value; reads (decidir_jogo_da_copa)
- tie +0x14..+0x1b and +0x1e..+0x25: score, extra time and penalties of the second and first legs; +0x33 the leg (1 = second); value; reads (decidir_confronto_da_copa)
- cup +0xa7/+0x3e: the 6-byte team entries (+0 club, +4 seed) and the count; pointer and value; reads (decidir_jogo_da_copa)
- draw entry +6: the seed; value; reads (ordem_dos_cabecas_de_chave)
- league +0xa7: total rounds (−1 until the first row); value; reads and writes (iniciar_linha_da_tabela)
- league +0xc5: the 4 tie-break rules (4 = head-to-head); value; reads (iniciar_linha_da_tabela)
- table row (0x3b bytes) +0, +4, +5, +9, +0xd..+0x36, +0x37..+0x3a: club, position, head-to-head, history, totals, status, flag, 1, clubs; pointer and value; writes (iniciar_linha_da_tabela)
- club +0x64: status (3 becomes 2 when entering the table); value; reads and writes (iniciar_linha_da_tabela)
- club +0x57/+0x5b/+0x5f: division, last season's division and final position; pointer and value; reads and writes (gravar_posicoes_finais_da_liga)
- club +0x57/+0x5b: division and last season's division; pointer; reads and writes (mudar_clube_de_divisao)
- club +0x64: status (3 becomes 2); value; reads and writes (mudar_clube_de_divisao)
- club +0xcf, +0x69: the manager and the stadium record; pointer; reads (mudar_clube_de_divisao)
- competition +0x43, +0xe2, +0xe4: the type (2 = league) and the minimum stadium (seats, capacity); value; reads (mudar_clube_de_divisao)
- [0xadadfc] the loaded competitions by id; pointer; reads (mudar_clube_de_divisao)
- club +0x80: reputation (the qsort order); value; reads (ordenar_e_embaralhar_os_melhores, through 0x4b07d0)
- discipline +4: ban vector (20 bytes: +0 matches or 0xff for days, +1 matches served, +2 days, +4 day and +6 year of the start, +0xc..+0xf the 4 rules); pointer; reads and writes (contar_jogo_da_suspensao, soltar_suspensao)
- discipline +0xc/+0x12: nation index (8 bytes: id, nation), count (word); reads and writes (soltar_suspensao)
- discipline +0x14: 29-byte records per person (+0x14 start and +0x18 last offset of the history); reads and writes −1 to +0x18 (apagar_historico_de_suspensoes); +0x18 history file (contar_jogo_da_suspensao)
- ruling body: the person's 14-byte entry (+6 ban index, −1 none); reads (contar_jogo_da_suspensao); writes −1 via 0x7e8590 (soltar_suspensao)
- person +0x1a nation, +0x1e second nation, +0x61 player; [0xae2c90] game date; reads
- fitness (INJURY [this]): 31-byte record per person: +0 date, +8 condition, +0xa form, +0xc wear, +0xe injury index (−1 none), +0x12 recurring type (0xff none), +0x13 level, +0x14/+0x18 history, +0x1d; values; reads and writes (sortear_lesao_recorrente, recomecar_historico_de_lesoes, reiniciar_fisico_do_jogador)
- INJURY +0x28: list of items by id; reads and removes (reiniciar_fisico_do_jogador); +0xc history file (recomecar_historico_de_lesoes)
- [0x9e7ae0 + 13 × type]: the injury can recur; value; reads (sortear_lesao_recorrente)
- person +0x39 club, +0x61 player; player +0x2d injury proneness; club +0x80 reputation; reads (sortear_lesao_recorrente)
- [0xae2d0c] 79-byte record +0xc: bit 0; writes 0 (reiniciar_fisico_do_jogador)
- Club_Records season and all-time record +0x365 transfer spending, +0x369 year, +0x36b income, +0x36f year; value; reads and writes (somar_totais_de_transferencia)
- Club_Records +0x24 record news queue (6 bytes: club id, type), +0x28 capacity, +0x2c count; reads (somar_totais_de_transferencia)
- club +0x80 base of the record news minimum (word); value; reads (somar_totais_de_transferencia)
- Club_Records all-time record +0x247 league appearances record, +0x249 player reference (0x18), +0x261 goals record, +0x263 reference; value; reads and writes (recordes_de_carreira_no_clube)
- Club_Records +0x1c club entry (0x6a bytes) +0x10 season games; value; reads (recorde_de_nota_da_temporada)
- Club_Records season record +0x1f1 rating (float), +0x1f5 player reference, +0x20d games, +0x20e cleared; writes (recorde_de_nota_da_temporada)
- club +0xd7 squad (50 pointers); person +0 id, +0x39 club, +0x61 stats; [0xae2364] first human manager id; [0xb74350] PlayerStats; reads (recorde_de_nota_da_temporada)
- Friendly +4 friendly competition; pointer; +0xaf first id (byte); +0xb1 tournament slots (0x69 bytes: +0 used, +4 date); reads (competicao_na_data_do_amistoso)
- [0xae2378] competition count; value; reads (competicao_na_data_do_amistoso)
- friendly candidate +0x12 planned friendlies list (items: day +0, year +2); reads, writes the cursor +0xa (amistoso_perto_da_data)
- Friendly +0x34d8 planned friendlies (0x14 bytes: date +0, ids +8, +0xc, +0x10), +0x34e0 count, +0x34e4 capacity; writes (acrescentar_amistoso_planejado)

**Of interest to:** CMRTE, SGE, GPF2, CMScout
