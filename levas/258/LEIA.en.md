# Batch 258 — the offer passed to the clubs, the news of an accepted offer and of a loan offer; the player's refusal of the terms, the board confidence news and the board expectations news; the offers on hold when the window closes, the FA expectations news and the supporter protest news; the stored negotiation of an offer, the manager job rumour and the player refusing to move; the weekly income and the training ground: merchandising, season tickets and the training facilities works; the weekly finance events: the club events pass, the board paying the debt and the cash injection; staff wages and bonuses: the staff wage ceiling, the bonus payment and the wage ceiling adjustment; the fitness after the match: each player's recovery rate and the knocks that become injuries, the player's knock in the match and the player on holiday; the regen at the club: the squad slot, the value and the youngster's contract; the retirement through injury and the news of the announced retirement; the end of the player's injury, the apology news and the emptied inbox; the injured call-up: withdrawing him from the national squad, the news to his club and the replacement request; can the squad change now?, is the player in the squad and the opponent booked for the friendly; the monthly award in the history (the manager counts it, the player thanks him or renews), the friendly's reputation tier and rest days

**Routines:**
- `passar_a_oferta_aos_clubes()` = 0x8c2f50 (transfer_manager.cpp, thiscall): once every player agrees, cancels the rival offers (rescheduled in 20 days), waits for the work permit (18 to 24 days) or passes the offer to the buyer's and the seller's decision;
- `noticia_da_oferta_aceita()` = 0x8d76d0 (transfer_manager.cpp, thiscall): the 0xfa3 news of an accepted offer to the human clubs involved;
- `noticia_do_emprestimo_oferecido()` = 0x8e1780 (transfer_manager.cpp, thiscall): the 0xfb5 news of a loan offer to the human clubs involved; without a contract, the error box;
- `recusa_dos_termos()` = 0x8c28c0 (transfer_manager.cpp, thiscall): the player refuses the terms: the offer returns in 10 days, the bid markers go, his reputation rises if the buyer is more famous; or he leaves the exchange and his value moves to the fee;
- `noticia_da_confianca_da_diretoria()` = 0x6a4c40 (manager_manager.cpp, thiscall): the 0x1772 board confidence news with the manager's days in the job;
- `noticia_das_expectativas_da_diretoria()` = 0x6a4a20 (manager_manager.cpp, thiscall): the 0x1771 board expectations news to the club manager;
- `ofertas_em_espera_na_janela()` = 0x8c4e10 (transfer_manager.cpp, thiscall): when the window closes, completes the approved offers and puts the nation's offers on hold: news if a human is involved, otherwise withdrawn or rejected; for the computer, status 3 and back the next day;
- `noticia_das_expectativas_da_federacao()` = 0x6a44a0 (manager_manager.cpp, thiscall): the 0x177e news of the FA's expectation for the national team in a competition;
- `noticia_do_protesto_da_torcida()` = 0x6a5d10 (manager_manager.cpp, thiscall): the 0x1782 news of the supporters protesting against the manager;
- `guardar_a_negociacao()` = 0x8be9d0 (transfer_manager.cpp, thiscall): stores the offer terms in a new negotiation (the array grows by 50; the next id) and schedules the offer in 2 days, or in 14 with news for a human buyer;
- `noticia_do_boato_do_tecnico()` = 0x69fd50 (manager_manager.cpp, thiscall): on a one-in-four draw, the 0x1776 news of a rumour linking a manager with another club;
- `noticia_da_recusa_em_se_mudar()` = 0x8d8310 (transfer_manager.cpp, thiscall): the 0xfae news of the player refusing the move, with the reason and both clubs' managers;
- `receita_do_merchandising()` = 0x59b7e0 (finance.cpp, thiscall, ret 4): attendance × reputation × factor (by the stadium of the +0xbf club or by reputation; Japan one step up), × (random(11) + 95)%, added to +0xbc, +0x15c and the balance;
- `ingressos_da_temporada()` = 0x599a20 (finance.cpp, thiscall, ret 4): stadium of at least 1500 places; demand by the manager-table value, attendance and reputation, up to 87.5% of the places; new holders by the step fraction (+0x164) and a random draw; ratio at +0x160; income (with apply) by price, nation type and key clubs; the shop with two random draws by reputation;
- `melhorar_o_centro_de_treino()` = 0x59b630 (finance.cpp, thiscall, ret 4): the training facilities level (+0x7f) rises by reputation (up to 20) and each step charges the works to +0x8c, +0x12c and the balance;
- `eventos_financeiros_da_semana()` = 0x59ed70 (finance.cpp, thiscall, ret 8): by the finance band: ends or opens the crisis (+0x165), moves the squeeze counter (+0x166), tries the boost, the debt, the injection and the board takeover, wears the training ground; with squeeze above 50, removes the heating or closes 10% of the places;
- `diretoria_paga_a_divida()` = 0x59dd40 (finance.cpp, thiscall, ret 8): poor club (band −2/−1, or 0 with a low balance): three random draws; the board pays 450, 300 or 150 × reputation and sends the news;
- `injecao_de_dinheiro()` = 0x59e5a0 (finance.cpp, thiscall, ret 8): club of reputation 4750 outside Wales and the Irelands: draws by the band and the stadium of the +0xbf club; the amount by reputation, up to 10% of the debt and 4 million; news and end of crisis;
- `teto_salarial_do_pessoal()` = 0x597db0 (finance.cpp, thiscall, ret 4): the highest wage offered to staff by job: default by job, nation level (+0x85) / type, key nation adjustments, league weight × 0.2, finance band, club ceiling (+0x14) and job range;
- `pagar_os_bonus()` = 0x599960 (finance.cpp, thiscall, ret 4): adds the amount to the bonus expenses (+0x138 and +0x98) and takes it from the balance;
- `ajustar_o_teto_salarial()` = 0x598340 (finance.cpp, thiscall, ret 4): wage ceiling (+0x14) × (percent + 100)%, truncated;
- `efeitos_fisicos_da_partida()` = 0x62c080 (injury.cpp, thiscall, ret 8): for the 20 slots of each side of the match: the rate (slot level plus (10000 − condition) / 100; half in a match without the +0x40 flag; × 1.25 above 33 years, double below 23, with a random draw) goes to the recovery (0x62bd10); the person's knocks in the match list become injuries (0x62d140, 0x62e700, 0x62d250);
- `batida_do_jogador()` = 0x62bf60 (injury.cpp, thiscall, ret 4): the match knock list (+0x28) item with the person's id, or 0;
- `jogador_de_ferias()` = 0x631550 (injury.cpp, thiscall, ret 4 (this não é lido)): 1 if the person's current injury (0x62ed70 on [0xb63d28]) is a holiday (type 0x51);
- `contratar_o_regen()` = 0x7a6f10 (player_regen.cpp, stdcall, ret 8): puts the regen in the first free slot of the 50 in the squad, with the value (0x4d0c60; at most 5000 at a club with +0x64 = 3), the contract end (below 19 years, two years from now; otherwise random(6) + 1) in the club's season end month on a random day, the contract, the default wage and, outside the human club, an end at most five years ahead; returns 1 (0 without a slot);
- `aposentar_pela_lesao()` = 0x7a7f20 (player_regen.cpp, thiscall, ret 8): removes the person's pending retirement and adds the new one, through injury (reason 8), random(days / 2) + 1 days ahead;
- `noticia_da_aposentadoria_anunciada()` = 0x7aa340 (player_regen.cpp, stdcall, ret 8): builds news 0xbbb (id, names, reason, contract club, age) and sends it to the managers of the interested clubs;
- `fim_da_lesao()` = 0x62e700 (injury.cpp, thiscall, ret 8): ends the person's current injury: the history (except holidays), the return news at the human club (when asked), entry type 0xff and the record's +0xe −1;
- `noticia_do_pedido_de_desculpas()` = 0x72bbf0 (media.cpp, stdcall, ret 0xc): builds news 0xbdb (player, names, club, story and the random variant for some stories) and sends it to the club's manager;
- `esvaziar_caixa_de_entrada()` = 0x787630 (news.cpp, thiscall, ret 4): gives the default replies for the manager's club and national team (0x7874b0) and empties his inbox (the 100 news items with +0xd6 = 0 and +0xda = 1);
- `tirar_convocado_machucado()` = 0x775cd0 (national_teams.cpp, thiscall, ret 8): with 14 or more injury days, removes the call-up from the national tactics and squad; before the squad date, tells his club, opens the slot and calls up a replacement or asks the human manager;
- `noticia_da_saida_do_convocado()` = 0x76f7d0 (national_teams.cpp, thiscall, ret 0xc): builds news item 0x20 (the player left the national squad injured) and delivers it to his club and his contract club when human-managed;
- `noticia_do_pedido_de_substituto()` = 0x7701f0 (national_teams.cpp, thiscall, ret 0x10): builds news item 0x22 (a replacement is needed) with the team, squad count, level and type, and delivers it to the national side's human manager;
- `pode_mudar_a_convocacao()` = 0x76db00 (national_teams.cpp, thiscall, ret 0x18): tells whether the player may join or leave the squad now (retired, other nation, full squad, injured, squad window, date and match day, human manager) and writes the reason;
- `esta_na_convocacao()` = 0x76da80 (national_teams.cpp, stdcall, ret 0xc): is the player in the squad of the type (senior, second or under-21) with the type flag?;
- `marcar_adversario_do_amistoso()` = 0x5c57d0 (friendly.cpp, thiscall, ret 8): stores the opponent in the first empty slot of the club's friendly schedule and marks the request;
- `registrar_premio_do_mes()` = 0x86c8c0 (staff_records.cpp, thiscall, ret 0x14): writes the 0x33 history entry of the winner; a manager counts +1 in his record; a player may get a contract renewal and, by a draw, thanks his manager (news);
- `faixa_de_reputacao_do_amistoso()` = 0x5c52d0 (friendly.cpp, stdcall, ret 4): the club's reputation tier 1..5 (2000, 4000, 6000 and 8000);
- `folga_do_amistoso()` = 0x5c74d0 (friendly.cpp, stdcall, ret 4): the rest days around a friendly by kind: 2 for kinds 1 and 10, 3 for kind 2, else 1;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (36):** 32 caught. `cpv3_status_1` escaped: the offers held at window close (37 times) return to the queue the same with status 1 or 3 before 2002-06-01 [inference]. `cpv3_volta_em_3` escaped: the return date of the held offers falls before the next window in both cases, without changing who negotiates [inference]. `cpv7_sem_clube_grande` escaped: the 5% wage ceiling raise of the big clubs only matters when one of them looks for staff, which decided no signing before 2002-06-01 [inference]. `cpv13_premio_em_dobro` escaped: the manager award count in the points record (+0x9e) is not in the fingerprint, and no manager award came before the comparing confirmation date (the agent foresaw it) [inference]. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (7c7b5d9a).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 9b926687, original 49f5dc41 (recorded; does not block the batch since 31/10/2026).

**Redo**, on top of batch 257's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 258`

**Fields touched:**
- offer +0: list of offers for the same player (indices); +4: list of the exchange players; pointer; reads (passar_a_oferta_aos_clubes)
- offer +8: buying club; +0xc: player; id; reads (all three)
- offer +0x10: terms state (2 = agreed); +0x20: index in the array; +0x2f: direct confirmation; value; reads (passar_a_oferta_aos_clubes)
- offer +0x2c: status (0xa waiting for the permit, 0xb with the clubs, 0xf cancelled); +0x2e: attempts (3 on rivals); value; reads and writes (passar_a_oferta_aos_clubes)
- offer +0x30: flags (& 0xf: on hold); value; reads (passar_a_oferta_aos_clubes)
- offer +0x1f, +0x20, +0x24, +4; terms +0xc, +0x2d, +0x2f; value; reads (the news)
- contract +4: club; id; +0x35 & 0x3f: type (5 without club); +0xc: wage; reads (all three)
- [0xae1080] the last contract asked for; pointer; writes (all three)
- offer +0x10: terms state (4 = refused); +0x2c: status (0xf); +0x2e: attempts; value; writes (recusa_dos_termos)
- offer +0x1b: fee; value; reads and writes (recusa_dos_termos)
- offer +4: list of the exchange players (freed when empty); pointer; reads and writes (recusa_dos_termos)
- [this+0x86e] 8-byte record per player: bit 0 of +0 (under negotiation), +2 (offers); value; writes (recusa_dos_termos)
- club +0x80: reputation; value; reads (recusa_dos_termos)
- player record (person +0x61) +0xb: current reputation (up to 10000); value; reads and writes (recusa_dos_termos)
- club +0xcf: manager; pointer; manager +0x3e: arrival date (day, year +0x40); reads (noticia_da_confianca_da_diretoria)
- [0xdd6ec8] date from which the news are posted (day, year +2); value; reads (both news)
- [0xae1080] the last contract asked for; pointer; writes (recusa_dos_termos)
- [this+0x8ac] the 28 transfer rules per nation (+0xa: nation; virtual +0xc: hold the offer); pointer; reads (ofertas_em_espera_na_janela)
- offer +0x2c: status (0xe complete, 0x13 out, 1 on hold, 3 rescheduled); value; reads and writes (ofertas_em_espera_na_janela)
- offer +0x30: flags (bit 0 on hold, bits 1..3 cleared); +0x10: terms (cleared); value; writes (ofertas_em_espera_na_janela)
- offer +4: exchange list (item +4: state, cleared); +0x20: slot; reads and writes (ofertas_em_espera_na_janela)
- club +0x53: nation; pointer; reads (ofertas_em_espera_na_janela)
- person +0x61: player record; pointer; reads (ofertas_em_espera_na_janela)
- club +0xcf: manager; pointer; reads (noticia_do_protesto_da_torcida)
- [0xdd6ec8] date from which the news are posted; value; reads (noticia_do_protesto_da_torcida)
- [0xae1080] the last contract asked for; pointer; writes (ofertas_em_espera_na_janela)
- transfers +0xc: negotiation array (capacity +0x10, count +0x14); +0x85a: next id; pointer and value; reads and writes (guardar_a_negociacao)
- offer +0x28: negotiation slot (−1 none); +0x2c: status (1); +0x2d: marked; value; reads and writes (guardar_a_negociacao)
- negotiation (0x41 bytes): copy of the terms +0..+0x40 (date +0x12; low 4 bits of +0x30 and +0x31), +0x20 of the offer, +0x24 id, +4 new list; writes (guardar_a_negociacao)
- person +0x39: club; +0x3d: job; reads (noticia_do_boato_do_tecnico)
- club +0xcf: manager; pointer; reads (noticia_da_recusa_em_se_mudar)
- [0xdd6ec8] date from which the news are posted; value; reads (noticia_do_boato_do_tecnico)
- [0xae1080] the last contract asked for; pointer; writes (noticia_da_recusa_em_se_mudar)
- club finance (0x167 bytes at [[0xae2a38]]) +0 balance (i64), +0xbc and +0x15c shop income, +0xac and +0x14c ticket income, +0x8c and +0x12c works; values; writes (all three)
- club finance +8 club id, +0x18 season tickets sold, +0x160 sold/places ratio (f32), +0x164 sale step; values; reads and writes (ingressos_da_temporada)
- club +0x53 nation, +0x57 league, +0x69 stadium, +0x6d byte, +0x73 attendance, +0x80 reputation, +0xbf linked club; pointers/values; reads (receita_do_merchandising, ingressos_da_temporada)
- club +0x7f training facilities level; value; reads and writes (melhorar_o_centro_de_treino)
- stadium +0xf level, +0x3c places; values; reads and writes places below 1500 (ingressos_da_temporada)
- nation +0 id (Japan 0x9cf36c, Germany 0x9cf314), +0x7e type; values; reads (receita_do_merchandising, ingressos_da_temporada)
- [[0xb67a44]] + 73·id + 8: club value in the manager table (i16); reads (ingressos_da_temporada)
- club finance (0x167 bytes) +0 balance (i64), +0xb4 and +0x154 money received from the board; values; reads and writes (diretoria_paga_a_divida, injecao_de_dinheiro)
- club finance +0x165 state (0 normal, 1 crisis, 2 squeeze), +0x166 squeeze counter (i8); values; reads and writes (eventos_financeiros_da_semana; the injection writes 0 to the state)
- club +0x53 nation, +0x57 league, +0x69 stadium, +0x6d byte, +0x7f training ground, +0x80 reputation, +0x82 flag, +0xbf linked club; pointers/values; reads, writes +0x7f and +0x82 (eventos_financeiros_da_semana)
- stadium +0xf level, +0x20 level, +0x3c places, +0x40 remainder, +0x44 closed places, +0x4d heating; values; reads and writes (eventos_financeiros_da_semana; reads +0xf/+0x20 in the others)
- player +0x52 value; contract +0x4e bit 4; [0xae1080] last contract; values; writes (eventos_financeiros_da_semana, human club crisis)
- [0xadadfc] active leagues (dword per id); value; reads (eventos_financeiros_da_semana)
- club finance (0x167 bytes) +8 club id, +0x14 wage ceiling; values; reads (teto_salarial_do_pessoal), reads and writes the ceiling (ajustar_o_teto_salarial)
- club finance +0 balance (i64), +0x98 and +0x138 bonus expenses; values; writes (pagar_os_bonus)
- club +0x53 nation, +0x57 league, +0x64 type, +0x80 reputation; pointers/values; reads (teto_salarial_do_pessoal)
- nation +0 id (key nations 0x9cf244..0x9cf4f8), +0x7e type, +0x85 wage level; values; reads (teto_salarial_do_pessoal)
- league +0x69 weight (i16); value; reads (teto_salarial_do_pessoal)
- match +0x1c0 + 0x625 × side + 0x49 × slot: −0x30 person id; −0x11 knock flag; +0, +2, +4 levels (0xff = none); +6 condition (16 bits); id and value; reads (efeitos_fisicos_da_partida)
- match +0x4c: flag (level 90 for the first eleven); +0x4d: default level; value; reads (efeitos_fisicos_da_partida)
- fixture +0x40: home side flag (without it, half the rate); +0: id (−2 means no fixture for the injury); value; reads (efeitos_fisicos_da_partida)
- person +0x18: age; +0x61: player data (+0xf signed rating, 15 or more for a type 7 knock); value; reads (efeitos_fisicos_da_partida)
- Injury +0x28: match knock list (item: +0 id, +4 type, +5 severity; cursor +0xa restarts from +2); pointer, id and value; reads and writes the cursor (efeitos_fisicos_da_partida, batida_do_jogador)
- [0xb63d28] Injury; 0x30-byte injury table (+0 type, 0x51 holiday); pointer and value; reads (jogador_de_ferias)
- club +0xd7: the 50 squad slots (person pointer, 0 free); pointer; reads and writes (contratar_o_regen)
- club +0x64: status (3: regen value at most 5000); value; reads (contratar_o_regen)
- person +0x39: club; +0x4e: 0; +0x52: value; +0x46: contract end (8-byte date); +0x18: age; pointer and value; writes, reads the age (contratar_o_regen)
- contract +0x2d/+0x2f: end (day and year, at most year + 5 outside the human club); +0x3a: 0; +0x35 and +0x4f: human club bits; value; writes (contratar_o_regen)
- PLAYER_REGEN +4: pending retirements (26 bytes: +0 id, +4 club, +8 and +0x10 date, +0x18 reason 8, +0x19 0), +8 count, +0xc capacity; id and value; removes and appends (aposentar_pela_lesao)
- [0xae1080] the last contract looked up; pointer; writes (noticia_da_aposentadoria_anunciada)
- Injury +0: 31-byte record per person (+0xe current injury index, −1 none; +0x14/+0x18 history); +4: 0x30-byte injury table (+0 type, 0xff free, 0x51 holiday); +0xc: history file; index and value; reads and writes (fim_da_lesao)
- person +0x39: club (human: the return news); +0x24: national team; +0x61: player data; pointer; reads (fim_da_lesao, esvaziar_caixa_de_entrada, noticia_do_pedido_de_desculpas)
- News +8: human inboxes (12 bytes: +0 first news item, +4 copy, +8 0); +0: 0xdf-byte news items (+0xd6 0, +0xda 1); index and value; writes (esvaziar_caixa_de_entrada)
- staff +0x24: the call-up's national side club; pointer; reads (tirar_convocado_machucado; cleared by 0x7762f0)
- staff +0x28: the call-up mark (0xb); value; reads (tirar_convocado_machucado)
- staff +0x5e: the squad flags (bit 3 = type 1); value; reads (tirar_convocado_machucado)
- staff +0x39: the player's club; pointer; reads (noticia_da_saida_do_convocado)
- nation record in the national teams ([NationalTeams+4] + 0x6e × id) + 20 × type: +0x3a the squad date; value; reads; +0x45 the open slots (byte); value; reads and writes +1 (tirar_convocado_machucado)
- [NationalTeams+0] the list of clubs with a date (id, ?, date at +8); pointer; reads and moves the cursor +0xa (tirar_convocado_machucado)
- injury (0x62ed70) +0: the injury type (0x51 = illness: just leaves); value; reads (tirar_convocado_machucado, the news)
- club +0xcf: the club's manager; pointer; reads (tirar_convocado_machucado)
- [0xae1080] the last contract found; pointer; writes (noticia_da_saida_do_convocado)
- staff +0x1a: the player's nation; pointer; reads (pode_mudar_a_convocacao)
- staff +0x24: the national side club; pointer; reads (pode_mudar_a_convocacao, esta_na_convocacao)
- staff +0x5e: the squad flags (4 senior and under-21, 8 the second team); value; reads (esta_na_convocacao)
- nation record in the national teams ([NationalTeams+4] + 0x6e × id) + 20 × type: +0x3a the squad date, +0x42 the squad cap, +0x44 the window, +0x45 the open slots; value; reads (pode_mudar_a_convocacao)
- [NationalTeams+0] the list of teams with a date; pointer; reads and moves the cursor +0xa (pode_mudar_a_convocacao)
- friendly schedule +8: the slots (byte); +0x19 + 25·i the opponent; +0x21 + 25·i the request flag; pointer and value; reads and writes (marcar_adversario_do_amistoso)
- staff history (FILE_LLIST [Staff_Records], 0x58e340): the 0x33 entry with the date, club or −1 (+8), award id (+0xd), a4 (+0x11) and a5 (+0x15); value; writes (registrar_premio_do_mes)
- staff +0x3d and +0x28: the job (5 or 0xc = manager); value; reads (registrar_premio_do_mes)
- manager points record (0x86f5b0, 0xd2 bytes) +0x9e: the monthly awards (word); value; reads and writes +1 (registrar_premio_do_mes)
- staff +0x59 and +0x5b: the bytes that allow the thanks draw (above 15 and 10); +0x59 is the draw ceiling; value; reads (registrar_premio_do_mes)
- contract (0x4cf5f0) +0x25/+0x29: the start (over 300 days for the renewal); +0x45 the flag that blocks the thanks; value; reads (registrar_premio_do_mes)
- [0xae1080] the last contract found; pointer; writes (registrar_premio_do_mes)
- club +0x80: the reputation; value; reads (faixa_de_reputacao_do_amistoso)

**Of interest to:** CMRTE, GPF2, SGE
