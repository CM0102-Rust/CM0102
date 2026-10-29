# Batch 248 — offers for a player: whether a club has an offer for him, the club offer and whether he has a pending offer; player offers and contract: whether he is under offer, whether his club follows the Bosman rule and the board veto on the offer; transfer records: storing the past transfer, granting the work permit and storing the first-team appearance money; staff contracts and decision dates: finding the contract slot, giving the new person a contract and scheduling the decision date; staff contracts: building the offer from the current contract, resetting the new contract's morale and the expiry band; the media story log: logging a story, expiring the old ones and whether a story was already told; the media transfer interests: adding an interest and the comparisons of id keys and stories

**Routines:**
- `clube_tem_oferta_pelo_jogador()` = 0x8cdc90 (transfer_manager.cpp, thiscall, ret 8): 1 when the person flagged as under offer (bit 0 of [+0x86e] + id × 8) has, among the open offers (0x8c5cf0), one from the club (offer +8); else 0;
- `oferta_do_clube_pelo_jogador()` = 0x8cdd80 (transfer_manager.cpp, thiscall, ret 0xc): the club offer for the person (with `all`, closed ones too and without requiring the flag), or 0;
- `jogador_com_oferta_pendente()` = 0x8cd150 (transfer_manager.cpp, thiscall, ret 4): 1 when the person is under offer (0x8cde70) and an open offer is in state (+0x2c) 0, 1, 5 or 6; else 0;
- `jogador_sob_oferta()` = 0x8cde70 (transfer_manager.cpp, thiscall, ret 8): without the human view, the "under offer" flag (bit 0 of [+0x86e] + id × 8); with it, 1 when an offer is known (+0x2d), comes from a club of the current human or is for a player of his club, when the person is on his shortlist or when he has an accepted contract offer for the person;
- `clube_do_jogador_com_lei_bosman()` = 0x8d0a70 (transfer_manager.cpp, stdcall, ret 4): 1 when the nation of the club of the person contract has free transfers at contract expiry (0x544670); without contract, club or nation: 1;
- `veto_da_diretoria_na_oferta()` = 0x8cdb80 (transfer_manager.cpp, thiscall, ret 0x18): with an empty exchange part, the deciding club board answers (0x696b10, by the 0x8e7b50 value); 0x2000: reason 0x24 and the player loses 10% of his reputations (+0xb current, +9 home, +0xd world); 0x800: 0x17; 0x200: 0x18; each returns 1;
- `guardar_transferencia_passada()` = 0x8cb810 (transfer_manager.cpp, thiscall, ret 4): appends the past transfer (0x25 bytes, 0x8cb940) to the +0x862 vector (capacity +0x866, count +0x86a), growing by 100; returns the new record;
- `dar_permissao_de_trabalho()` = 0x8c7070 (transfer_manager.cpp, thiscall, ret 0x14): appends the permit (14 bytes: person, year +8, nation +0xa) to the +0x93a vector (growing by 30), sets the person 0x20 flag at +0x86e and, when asked, sorts (0x8e55c0);
- `guardar_premio_por_jogo_no_time()` = 0x8cac40 (transfer_manager.cpp, thiscall, ret 4): appends the appearance money (13 bytes) to the +0x88c vector (growing by 50) and sorts by person (0x8b4e00);
- `vaga_livre_de_contrato()` = 0x4c79a0 (contract_manager.cpp, thiscall, ret 4): the slot in the 0x50-byte contract vector: the first empty one (id −1) or, append only, a new one (grows by 200; without memory, error box and −1);
- `acrescentar_contrato_da_pessoa()` = 0x4c7950 (contract_manager.cpp, thiscall, ret 8): the appended slot for a new person (hired manager, regen): links the person to the slot in [0xae2d0c], stores the id and generates the initial contract from the value (0x85e960);
- `acrescentar_data_de_decisao()` = 0x4cd6d0 (contract_manager.cpp, thiscall, ret 0x10): appends (person, date, time slice, type) to the 14-byte decision date vector (grows by 100); type 1 first adds type 2 reminders from the contract morale;
- `zerar_moral_do_contrato()` = 0x866ee0 (staff_contracts.cpp, thiscall, ret 8): clears the contract morale, grievances and happiness, the person's opinion links (0x549c90), sets +0x4e to 4 and the club nation language level (0x7c6830, up to 10) in +0x4f;
- `contrato_pelo_atual()` = 0x85e590 (staff_contracts.cpp, thiscall, ret 0x18): the offer from the current contract: person, club, wage × percent / 100, today, the end date, flags, morale reset as 0x866ee0 and the two clauses; without contract, error box;
- `faixa_do_vencimento()` = 0x867d30 (staff_contracts.cpp, thiscall): the 0..=14 band of the contract end date against today and the CONTRACT_MANAGER limit dates (0xae1010..0xae1090);
- `registrar_historia_da_midia()` = 0x757a50 (media.cpp, thiscall, ret 0x14): appends the story (today, subject ids or −1, type, subtype, response) and sorts again (0x757ee0); grows by 20;
- `expirar_historias_da_midia()` = 0x757c70 (media.cpp, thiscall, ret 0): removes stories older than 100 days (team ones, 0xbdf, older than 40 + random(40)) moving the last one into the gap; sorts again;
- `historia_ja_registrada()` = 0x757da0 (media.cpp, thiscall, ret 0x14): team story: linear search by second subject id and subtype; the others: binary search by the key and, with a date, whether it is not older;
- `acrescentar_interesse_de_transferencia()` = 0x754740 (media.cpp, thiscall, ret 0x14): if the {player, club} key is missing, appends the interest (today, fee up to 50 million, valuation × 100) and sorts again; grows by 20;
- `ordem_das_chaves_de_ids()` = 0x757ea0 (media.cpp, cdecl): null first; by the +0 id and, when equal, by +4 (−1 matches anything);
- `ordem_das_historias()` = 0x757ee0 (media.cpp, cdecl): null first; by subjects +8 and +0xc, type +0x13 and subtype +0x11;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (19):** 18 caught. `cpl_contrato_valor` escaped: acrescentar_contrato_da_pessoa 0x4c7950: half the value passed to 0x85e960 (the initial contract) does not change the career up to 2002-06-01 (counters in the batch 248 DLL, career to 2002-06-01, same be54a138 fingerprint: 128 or more calls, all with a club and value above 1); in 0x85e960 the value only enters through conditional branches (the type 2 competition via vtables +0x98/+0x9c and the comparison at 0x85ef76), which in the career do not run or end the same; the exact cause was not isolated; also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2f65fd6b68d1027b).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (be54a1386f51073a).

**Redo**, on top of batch 247's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 248`

**Fields touched:**
- transfer manager +0x86e: 8-byte table by person id (bit 0 = under offer); value; reads (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)
- transfer manager +0/+4: offer pointer vector and count; pointer; reads (all three)
- offer +8: offering club id; id; reads (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)
- offer +0x2c: offer state (0, 1, 5, 6 = pending); value; reads (jogador_com_oferta_pendente)
- [0xae2364] number of people; value; reads
- transfer manager +0x86e: 8-byte table by person id (bit 0 = under offer); value; reads (jogador_sob_oferta)
- offer +8/+0xc: offering club id and person id; id; reads (jogador_sob_oferta)
- offer +0x2d: offer known to the human; value; reads (jogador_sob_oferta)
- offer +4: the exchange part (word +0 = how many); pointer; reads (veto_da_diretoria_na_oferta)
- contract +4/+0x35: club id and state (& 0x3f = 5: not valid); id and value; reads (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)
- club +0x53: club nation; pointer; reads (clube_do_jogador_com_lei_bosman)
- player (staff +0x61) +9/+0xb/+0xd: home, current and world reputations (signed words); value; reads and writes 90% (veto_da_diretoria_na_oferta)
- [0xae1080] the last contract found; pointer; writes (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)
- current human +0x39: his club; pointer; reads (jogador_sob_oferta)
- transfer manager +0x862/+0x866/+0x86a: 0x25-byte past transfer vector, capacity, count; pointer and values; reads and writes (guardar_transferencia_passada)
- transfer manager +0x93a/+0x942/+0x93e: 14-byte work permit vector (+0 person, +4/+6 zeroed, +8 year, +0xa nation), capacity, count; reads and writes (dar_permissao_de_trabalho)
- transfer manager +0x86e: 8-byte table by person id (bit 0x20 = has work permit); value; writes (dar_permissao_de_trabalho)
- transfer manager +0x88c/+0x890/+0x894: 13-byte first-team appearance money vector, capacity, count; reads and writes (guardar_premio_por_jogo_no_time)
- contract manager +0/+4/+8: 0x50-byte contract vector, capacity, count; pointer and values; reads and writes (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)
- contract +0: person id (−1 empty slot); id; reads and writes (vaga_livre_de_contrato, acrescentar_contrato_da_pessoa)
- [0xae2d0c] + id × 0x4f, +0: index of the person's contract; index; reads and writes (acrescentar_contrato_da_pessoa), reads (acrescentar_data_de_decisao)
- contract +0x44: morale; value; reads (acrescentar_data_de_decisao, the type 1 reminders)
- staff +0x61: player record; pointer; reads (acrescentar_data_de_decisao)
- contract manager +0xc/+0x10/+0x14: 14-byte decision date vector (+0 id, +4 date, +0xc time slice, +0xd type), capacity, count; reads and writes (acrescentar_data_de_decisao)
- [0xae2364] number of staff; value; reads (acrescentar_data_de_decisao)
- contract +0/+4/+8/+0xc: person id, club id, 0, wage; id and value; writes (contrato_pelo_atual)
- contract +0x10/+0x14/+0x18: −1; writes (contrato_pelo_atual)
- contract +0x1c: non-promotion clause; +0x1d: minimum fee clause; +0x1e..+0x24: 0; value; writes (contrato_pelo_atual)
- contract +0x25: start date (today, 0xae2c90); +0x2d: end date; date; writes (contrato_pelo_atual), reads +0x2d (faixa_do_vencimento)
- contract +0x35: flags (& 0x3f type); value; writes & 0xc6 | 6 (contrato_pelo_atual), reads (faixa_do_vencimento)
- contract +0x36: wage (copy); +0x3a, +0x49: 0; +0x3b (16 bits): 0; +0x4a: −1; writes (contrato_pelo_atual)
- contract +0x3e, +0x3f (happiness), +0x40, +0x44 (morale), +0x45 (grievances): 0; +0x4e: 4; +0x4f & 0xf: language level; writes (zerar_moral_do_contrato, contrato_pelo_atual)
- [0xae2d0c] + id × 0x4f, +0x27/+0x37/+0x3b/+0x4b: opinion links; −1; writes through 0x549c90 (zerar_moral_do_contrato, contrato_pelo_atual)
- club +0x53: nation; pointer; reads; nation +0x77: language; value; reads (zerar_moral_do_contrato, contrato_pelo_atual)
- [0xae1010..0xae1090] CONTRACT_MANAGER limit dates; [0xae2c90] today; date; reads (faixa_do_vencimento, contrato_pelo_atual)
- Media +0 story log (23 bytes: +0 date, +8 subject a id or −1, +0xc subject b id or −1, +0x10 response, +0x11 subtype word, +0x13 type), +4 capacity, +8 count; writes (registrar_historia_da_midia, expirar_historias_da_midia), reads (historia_ja_registrada)
- [0xae2c90] current date; value; reads (registrar_historia_da_midia, expirar_historias_da_midia)
- Media +0xc transfer interests (25 bytes: +0 player id, +4 club id, +8 date, +0x10 rumour out, +0x11 fee up to 50,000,000, +0x15 valuation × 100), +0x10 capacity, +0x14 count; writes (acrescentar_interesse_de_transferencia)
- id key (+0 id, +4 id or −1 wildcard); reads (ordem_das_chaves_de_ids)
- Media +0 story log (+8, +0xc subjects, +0x11 subtype, +0x13 type); reads (ordem_das_historias)

**Of interest to:** CMRTE, CMScout, SGE, GPF2
