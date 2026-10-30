# Batch 250 — news delivery: by scope, to the club manager and into the human manager's inbox; the human manager's inbox: replying to every item, whether an item is pending and the text in an item slot; the media and the inbox: a player's rating for a story, the financial trouble text and the human's inbox items; the retiring player leaves the squads and the choice of the youngster's club: the best scored among all and the drawn one from his nation's list; manager jobs and offer scheduling: opening the missing jobs, the best manager candidate score and scheduling the offer; the club shortlist: the squad status the club gives the person, the human club estimate and releasing the targets that are not the club's; the scout search and the friendlies: whether the search covers the nation, removing the scheduled friendly and the nation pairs that play no friendlies

**Routines:**
- `noticia_pelo_alcance()` = 0x786090 (news.cpp, thiscall, ret 0x18): delivers the news item by scope: 0 the club manager, 1 the division's managers, 2 the country's, 3 the continent's, 4 all, 5 the competition region's; anything else shows the "Error" box;
- `noticia_ao_tecnico_do_clube()` = 0x7861e0 (news.cpp, thiscall, ret 8): marks the scope as club (+0xde = 0) and delivers the item to the club manager (+0xcf) when he is human (among the last 16 people); in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run;
- `entregar_noticia_ao_tecnico()` = 0x785d40 (news.cpp, thiscall, ret 0xc): copies the item into the human's 100-slot ring (created on the first, reallocating the array), gives it an id, skips a repeat of the last one and, with the manager on holiday, applies the automatic reply for the club and the national team; in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original reads: the career left be54a138); the copy stays in the code and does not run;
- `responder_todas_as_noticias()` = 0x7874b0 (news.cpp, thiscall, ret 8): from the newest to the oldest item of the human's inbox, up to the empty slot, applies the automatic reply (0x784100) for the club;
- `tecnico_tem_noticia_pendente()` = 0x787350 (news.cpp, thiscall, ret 8): without "urgent only", whether there is an unread item; with it, whether an item needs a reply (the text returns 2) or is important and unread;
- `gravar_texto_na_noticia()` = 0x7858c0 (news.cpp, thiscall, ret 0xc): copies the text into the item slots from the given slot and returns the first free slot after it (at most 50);
- `nota_do_jogador_para_a_historia()` = 0x736720 (media.cpp, thiscall, ret 0x18, st0): the player's rating for a story kind: stats and the average of ratings against the league average, the weight, the games and the reputation; then, by kind, the contract, age, days at the club and attributes;
- `texto_da_crise_financeira()` = 0x74af60 (media.cpp, thiscall, ret 0xc): the headline or body of the club in financial trouble story, by severity, with the club name linked;
- `noticias_da_caixa()` = 0x786960 (news.cpp, thiscall, ret 0xc): a new array with the human's inbox items, newest first, only those of one category when asked, and the count;
- `aposentar_do_elenco()` = 0x7aace0 (player_regen.cpp, stdcall, ret 4): removes the player from the tactics and the 50 slots of his club (+0x39) and national team (+0x24) and changes the player job to the staff job (12 → 5, 13 → 6, 15 → 8, the others 0x10 retired); without a kept job, clears the club and the national team;
- `melhor_clube_para_o_jovem()` = 0x7ad0e0 (player_regen.cpp, thiscall, ret 4): among the clubs where the youngster is not a foreigner, that are not a B team, that may approach him and with a squad under 50, the best scored one (0x7ad220);
- `clube_da_nacao_para_o_jovem()` = 0x7ad590 (player_regen.cpp, thiscall, ret 4): in the club list of the youngster's nation (some nations mapped to [0x9cf2e4]), the one with the highest draw of the score by reputation against ability, among those that may approach him and with a squad under 50;
- `abrir_vagas_que_faltam()` = 0x69d060 (manager_manager.cpp, thiscall, ret 0): each club without a manager (0x53ca90), manageable (0x544820), not a B team and without an entry in the [this+8] list gets the "Job not found" note and a job vacancy (0x69ba50);
- `melhor_nota_de_candidato_a_tecnico()` = 0x69c540 (manager_manager.cpp, thiscall, ret 8): the highest manager score (0x69d1b0, job 5) among the available staff accepted for the club (nation, continent, reputation, contract, 0x69f810), ignoring the current manager; nobody: −10000;
- `agendar_oferta()` = 0x8c6220 (transfer_manager.cpp, thiscall, ret 0xc): gives the offer today's date + days and the day slice (0 to 2, every 3 slices one more day) and puts it in the bucket (days since the base date + start, modulo 50, × 3 + slice) of the +0x18 lists;
- `estatuto_no_elenco_pelo_clube()` = 0x849f00 (shortlist_manager.cpp, thiscall, ret 0xc): the squad status the club gives the person: from her entry in the club shortlist (bits 1..4 of +0x24, job from +0), from the contract with the club (high nibble of +0x4f), from the human club estimate, or 7;
- `estimar_estatuto_no_elenco()` = 0x849d60 (shortlist_manager.cpp, stdcall, ret 8): the estimate of the status the human club gives: the ratio between the person's reputation seen by the manager (1.5 × when on the human shortlist) and the club's, and the age;
- `liberar_alvos_da_lista_do_clube()` = 0x844660 (shortlist_manager.cpp, thiscall, ret 8): in the club's 40 slots, a target with an invalid id becomes −1 and a target not at the club (contract nor +0x39) and without +0x17 gets shortlist event 3 (0x843e10);
- `busca_cobre_a_nacao()` = 0x7b7490 (player_search.cpp, thiscall, ret 4): whether the scout search (type 4, scout checked, with a club) covers the nation: by the set assignments (club, nation, region, base) or, without one, by the region of the club's nation, the scout's nation and the club's scouts;
- `tirar_amistoso_marcado()` = 0x5c87b0 (friendly.cpp, thiscall, ret 8): removes the match's items from the +0x34d4 list, clears the scheduled slot (with clear_linked) and removes the match from the +0x34c8 list (freeing it);
- `iniciar_pares_proibidos_de_amistoso()` = 0x5cc2d0 (friendly.cpp, thiscall): allocates the 4 nation pairs that play no friendlies with each other (Iran and USA, Iraq and anyone, Vietnam and USA, the Koreas) at +0x3502, count at +0x3506;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (17):** 15 caught. `cpn_responder_todas` escaped: responder_todas_as_noticias 0x7874b0: in the automatic career it only runs when the human goes on holiday at the start (counter: 64 passes, the news already in their inbox); the automatic replies to those news (0x784100) change nothing the career reads (same fingerprint with and without them, and with only the newest); in the game comparing the comparator runs only the original and checks the return value. `cpn_responder_so_a_nova` escaped: responder_todas_as_noticias 0x7874b0: in the automatic career it only runs when the human goes on holiday at the start (counter: 64 passes, the news already in their inbox); the automatic replies to those news (0x784100) change nothing the career reads (same fingerprint with and without them, and with only the newest); in the game comparing the comparator runs only the original and checks the return value. (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (2f65fd6b68d1027b).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (be54a1386f51073a).

**Redo**, on top of batch 249's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 250`

**Fields touched:**
- News +0/+4: the stored news array (0xdf bytes each) and the slot count; pointer and value; reads and writes (entregar_noticia_ao_tecnico)
- News +8: the 16 humans' entries (12 bytes: ring start, next slot, count; −1 without a ring), by id − [0xae2364] + 16; pointer and value; reads and writes (entregar_noticia_ao_tecnico)
- News +0x10: the last news id; value; reads and writes (entregar_noticia_ao_tecnico)
- news item +4, +0xd6, +0xda, +0xde: important, id, free/read, scope (0 club); value; writes (entregar_noticia_ao_tecnico, noticia_ao_tecnico_do_clube)
- club +0xcf: the manager; pointer; reads (noticia_ao_tecnico_do_clube)
- person +0, +0x24, +0x39: id, the national team and the club he manages; id and pointer; reads (entregar_noticia_ao_tecnico)
- [0xae2364] the number of people (humans are the last 16); [0xdd6bec] everyone on holiday; value; reads
- News +0/+8: the stored news array (0xdf bytes) and the humans' entries (ring start, next slot); pointer and value; reads (responder_todas_as_noticias, tecnico_tem_noticia_pendente)
- news item +4, +0xd6, +0xda: important, id (0 empty slot), read; value; reads (tecnico_tem_noticia_pendente, responder_todas_as_noticias)
- news item +5..+0xcc: the 50 4-byte slots (text or value) and the terminator; value; writes (gravar_texto_na_noticia)
- [0xde1f64] the built news text; value; writes through 0x782b70 (tecnico_tem_noticia_pendente)
- [0xb74350] +0x10/+0x18: the league rating sum and count (the average; no games, 6.8); value; reads (nota_do_jogador_para_a_historia)
- person +0x18, +0x3e, +0x61: age, the date (days at the club, 0x54cdb0) and the player record; value and pointer; reads (nota_do_jogador_para_a_historia)
- player record +0xb, +0x25, +0x2f: reputation and two attributes; value; reads (nota_do_jogador_para_a_historia)
- contract +0x4f: the contract type in the high nibble; value; reads (nota_do_jogador_para_a_historia)
- club +0x80, +0x38, +0x52: reputation, name and gender; value; reads (nota_do_jogador_para_a_historia, texto_da_crise_financeira)
- news item, slots 0 and 1: the club and the severity of the trouble; id and value; reads (texto_da_crise_financeira)
- News +0/+8, news item +0xd6: the array, the humans' rings and the id; pointer and value; reads (noticias_da_caixa)
- staff +0x39 / +0x24: the person's club and national team; pointer; reads and writes 0 (aposentar_do_elenco)
- staff +0x3d / +0x28: the job at the club and at the national team (11 player, 12/13/15 player and staff, 16 retired; 5, 6, 8 staff); value; reads and writes (aposentar_do_elenco)
- club +0xd7: the 50 squad slots; pointer; reads and writes 0 (aposentar_do_elenco)
- staff +0x1a / +0x1e: nation and second nation; pointer; reads (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)
- staff +0x61 → player data +5, +7, +0xb: current ability, potential and reputation; value; reads (clube_da_nacao_para_o_jovem)
- club +0x53 / +0x80: nation and reputation; pointer and value; reads (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)
- PLAYER_REGEN +0 → [4 × nation]: the nation's club list (+0 count, +2 head, +0xa cursor); pointer; reads and writes the cursor (clube_da_nacao_para_o_jovem)
- [0x9cf424], [0x9d0120], [0x9d0124], [0x9d0128], [0x9d0134], [0x9d01a0]: the veto nation and clubs; id; reads (melhor_clube_para_o_jovem, clube_da_nacao_para_o_jovem)
- [0x9cf510], [0x9cf2dc], [0x9cf3e4] → [0x9cf2e4]: the nations whose list is another one's; id; reads (clube_da_nacao_para_o_jovem)
- MANAGER_MANAGER +8: the manager vacancy list (+0 count, +2 head, +0xa cursor; entry +0xe the club id); pointer; reads and writes the cursor (abrir_vagas_que_faltam, melhor_nota_de_candidato_a_tecnico)
- club +4: the name; text; reads (abrir_vagas_que_faltam)
- club +0x53 / +0x80 and nation +0x75: nation, reputation and continent; pointer and value; reads (melhor_nota_de_candidato_a_tecnico)
- staff +0x69 → +0xa / +0xc: the staff data reputations; value; reads (melhor_nota_de_candidato_a_tecnico)
- staff +0x5f, +0x1a, +0x39, +0x3d, +0x61: human, nation, club, job and player data; value and pointer; reads (melhor_nota_de_candidato_a_tecnico)
- [0xae2d0c] 79-byte record +0xb: the 0x30 flags; value; reads (melhor_nota_de_candidato_a_tecnico)
- contract +0x49: the flag that bars the candidate; value; reads (melhor_nota_de_candidato_a_tecnico)
- TRANSFER_MANAGER +0x84c / +0x84d / +0x855: the ring start, the base date (year +0x84f) and the base slice; value; reads and writes (agendar_oferta)
- TRANSFER_MANAGER +0x18: the 150 14-byte lists (50 days × 3 slices) of scheduled offers; pointer; writes (agendar_oferta)
- offer +0x12 / +0x1a / +0x20: processing date and slice and the value put in the list; value; writes and reads (agendar_oferta)
- [0xae2c90] / [0xae2c88]: today and today's slice; value; reads (agendar_oferta)
- shortlist slot (40 × 0x25 bytes) +0: the job (type, 11 = non-player); value; reads (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)
- shortlist slot +0xf: the target; person id; reads and writes −1 when above [0xae2364] − 1 (liberar_alvos_da_lista_do_clube)
- shortlist slot +0x17: the second target (−1 empty); id; reads (liberar_alvos_da_lista_do_clube)
- shortlist slot +0x24: bits 1..4 the squad status; value; reads (estatuto_no_elenco_pelo_clube)
- staff +0x18: the age; value; reads (estimar_estatuto_no_elenco)
- staff +0x39: the club; pointer; reads (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)
- staff +0x3d: the job; value; reads (estatuto_no_elenco_pelo_clube)
- staff +0x61/+0x69: the player and non-player data; pointer; reads (estatuto_no_elenco_pelo_clube)
- contract +4: the club; id; reads; +0x35 (6 bits) the type (5 does not count); value; reads (liberar_alvos_da_lista_do_clube); +0x4f high nibble the squad status; value; reads (estatuto_no_elenco_pelo_clube)
- club +0x80: the reputation; value; reads (estimar_estatuto_no_elenco)
- [0xae1080] the last contract looked up; pointer; writes (estatuto_no_elenco_pelo_clube, liberar_alvos_da_lista_do_clube)
- scout search +0xa: the type (4 = scout); value; reads (busca_cobre_a_nacao)
- scout search +0x174..+0x184: the scout's names and date; +0x180 the scout; id; reads (busca_cobre_a_nacao)
- scout search +0x18c nation, +0x190 club, +0x194 region (byte), +0x195 competition, +0x199 base, +0x19d opponent: the assignments; id, value and pointer; reads (busca_cobre_a_nacao)
- nation +0: the id; +0x76 the region; +0x7f the group (2); value; reads (busca_cobre_a_nacao)
- club +0x53: the nation; pointer; reads (busca_cobre_a_nacao)
- staff +0x39: the scout's club; pointer; reads (busca_cobre_a_nacao)
- friendlies +0x34c8: the list of scheduled matches; +0x34d4 the list of items pointing at them (+0 the match); pointer; reads and removes (tirar_amistoso_marcado)
- friendly match +0 day, +2 year, +0x10 opponent, +0x14 club; value and pointer; reads (tirar_amistoso_marcado)
- friendly slot +0x10: the opponent; pointer; writes 0; +0x18 byte writes 0 (tirar_amistoso_marcado)
- friendlies +0x3502: the banned pairs array (8 nation ids, −1 = any); pointer; writes; +0x3506 the count (4); value; writes (iniciar_pares_proibidos_de_amistoso)

**Of interest to:** CMRTE, CMScout, GPF2
