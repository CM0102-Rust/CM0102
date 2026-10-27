# Batch 243 — club records: the copies of the rating, man of the match and discipline records; club records: the copy of the count and fans player records and the construction of the tally and count records; club records: the copies of the transfer fee record and the goals in a match record; the copies of the news item and of the past and future transfers; the copy of the regen queue entry, the shortlist lookup and the choice of the non-player data donor; the copy of the ban record, the default contract wage and the player match condition; the staff contract: the days to the end date, clearing all grievances and checking the transfer request withdrawal

**Routines:**
- `copiar_recorde_de_nota()` = 0x448230 (club_records.cpp, thiscall, ret 4): copies the rating record (0x1f bytes): the rating (+0), the holder (+4, through 0x448490), +0x1c and +0x1d;
- `copiar_recorde_de_melhor_em_campo()` = 0x4484d0 (club_records.cpp, thiscall, ret 4): copies the man of the match record (0x1b bytes): the count (+0), the holder (+1) and the year (+0x19);
- `copiar_recorde_de_disciplina()` = 0x448500 (club_records.cpp, thiscall, ret 4): copies the discipline record (0x1c bytes): yellows (+0), reds (+1), the holder (+2) and the year (+0x1a);
- `copiar_recorde_de_contagem()` = 0x448530 (club_records.cpp, thiscall, ret 4): copies the count record (0x1a bytes): the count (+0, word) and the holder (+2, through 0x448490);
- `copiar_recorde_do_torcedor()` = 0x4485b0 (club_records.cpp, thiscall, ret 4): copies the fans player record (0x1a bytes): the holder (+0) and the year (+0x18);
- `construir_recorde_de_ocorrencias()` = 0x450c60 (club_records.cpp, thiscall): constructs the tally record: the holder date at +0x11 (0x54a110);
- `construir_recorde_de_contagem()` = 0x450d30 (club_records.cpp, thiscall): constructs the count record: the holder date at +0x12 (0x54a110);
- `copiar_recorde_de_valor()` = 0x448550 (club_records.cpp, thiscall, ret 4): copies the transfer fee record (0x29 bytes): five dwords, the dates (+0x14, +0x20), +0x1c and the part-exchange flag (+0x28);
- `copiar_recorde_de_gols_na_partida()` = 0x451570 (club_records.cpp, thiscall, ret 4): copies the goals in a match record (0x38 bytes): the goals (+0), the holder (+1) and the match (+0x19);
- `copiar_noticia()` = 0x786010 (news.cpp, thiscall, ret 4): copies the news item (0xdf bytes): +0, +4, the 50 tokens (+5), the date (+0xcd), +0xd5, +0xd6, +0xda and +0xde;
- `copiar_transferencia_passada()` = 0x8cb940 (transfer_manager.cpp, thiscall, ret 4): copies the past transfer (0x25 bytes): seven dwords, the byte +0x1c and the date (+0x1d);
- `copiar_transferencia_futura()` = 0x8cbf40 (transfer_manager.cpp, thiscall, ret 4): copies the future transfer (0xc bytes): the date (+0) and the offer index (+8);
- `copiar_entrada_da_fila_de_regens()` = 0x7a8b70 (player_regen.cpp, thiscall, ret 4): copies the regen queue entry (0xc bytes): the person (+0) and the regen date (+4);
- `esta_na_lista_de_observados()` = 0x842080 (shortlist_manager.cpp, thiscall, ret 8): is the (player, club or −1) pair in the shortlist pairs? bsearch with 0x757ea0;
- `escolher_doador_de_dados()` = 0x7a6e60 (player_regen.cpp, stdcall, ret 4): the non-player with the highest positive score (0x7a7150) of the nation; with none, of any nation; else 0;
- `copiar_registro_de_suspensao()` = 0x556c70 (discipline.cpp, thiscall, ret 4): copies the ban record (0x14 bytes): fields, the date (+4) and bits 0..2 of +0x13;
- `salario_padrao_do_contrato()` = 0x8626e0 (staff_contracts.cpp, thiscall, ret 4): the wage (+0xc) = the default for the person at the club (0x862720) truncated, minimum 55;
- `condicao_de_jogo_do_jogador()` = 0x62c510 (injury.cpp, thiscall, ret 8): updates the day fitness (0x62c680) and returns the match condition (out of 10000, or as a percentage);
- `dias_ate_o_fim_do_contrato()` = 0x85fa00 (staff_contracts.cpp, thiscall): the days from the game date to the contract end date (+0x2d), through 0x54cdb0;
- `limpar_todas_as_queixas()` = 0x867520 (staff_contracts.cpp, thiscall, ret 4): clears the grievances (+0x45), the opinion links (0x549c90) and, when asked, checks the withdrawal (0x867240);
- `conferir_retirada_do_pedido()` = 0x867240 (staff_contracts.cpp, thiscall, ret 4): a player of the contract club with a transfer request, no staff criterion and no agreed contract: withdraws the request (0x4d9460), morale becomes +0x5b + +0x59 + 5, +0x3e = 0, +0x3d at least 15, and clears each grievance checking again;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (22):** 18 caught. `cpg_doador_zero` escaped: escolher_doador_de_dados 0x7a6e60: a score of exactly zero only changes the choice when no other person scores positive; does not happen before 2002-06-01; also escaped the game comparing and the pt load (0 divergences). `cpg_salario_minimo` escaped: salario_padrao_do_contrato 0x8626e0: no default wage falls to 55 or below before 2002-06-01 (the minimum does not act); also escaped the game comparing and the pt load (0 divergences). `cpg_condicao_porcento` escaped: Injury::get_match_condition 0x62c510: disabled routine (DESLIGADAS in motor.rs, 27/10/2026): no hook in any mode, the copy does not run. `cph_queixas_zero` escaped: limpar_todas_as_queixas 0x867520: the three high grievance bytes (+0x46..+0x48) are already zero when the routine runs in the career; also escaped the game comparing and the pt load (0 divergences). (gap in `PENDENCIAS.en.md`)
- **Real game comparing to 2002-01-01:** 0 divergences (963cea289f055ac3).
- **Rust ×2 and original careers to 2002-06-01:** all three equal (b28542870027e432).

**Redo**, on top of batch 242's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 243`

**Fields touched:**
- club records: rating record (0x1f bytes), man of the match (0x1b) and discipline (0x1c), with the 0x18-byte holder inside; writes (copiar_recorde_de_nota, copiar_recorde_de_melhor_em_campo, copiar_recorde_de_disciplina)
- club records: count record (0x1a bytes: word + holder), fans player (0x1a: holder + year) and tally (0x19: byte + holder); writes (copiar_recorde_de_contagem, copiar_recorde_do_torcedor, construir_recorde_de_ocorrencias, construir_recorde_de_contagem)
- club records: transfer fee record (0x29 bytes, dates at +0x14 and +0x20, part-exchange at +0x28) and goals in a match (0x38: goals + holder + match record); writes (copiar_recorde_de_valor, copiar_recorde_de_gols_na_partida)
- news item (0xdf bytes: 50 tokens at +5, date at +0xcd), past transfer (0x25 bytes, date at +0x1d) and future transfer (0xc bytes: date + offer index); writes (copiar_noticia, copiar_transferencia_passada, copiar_transferencia_futura)
- regen queue entry (0xc bytes: person + date); writes (copiar_entrada_da_fila_de_regens); shortlist pairs (8 bytes: player, club or −1), array at [this] and count at +0x10; reads (esta_na_lista_de_observados); non-players ([0xae23bc], 0x6e bytes, +0x1a nation); reads (escolher_doador_de_dados)
- ban record (0x14 bytes: date at +4, flags in bits 0..2 of +0x13); writes (copiar_registro_de_suspensao); contract +0xc wage; writes (salario_padrao_do_contrato); injury record (31 bytes, +0xa match condition); reads (condicao_de_jogo_do_jogador)
- staff contract (0x50 bytes): +0 person, +4 club, +0x2d end date, +0x3d happiness, +0x3e, +0x44 morale, +0x45 grievances (bits), +0x49, +0x4e bit 3 transfer request, +0x4f high nibble; reads and writes (limpar_todas_as_queixas, conferir_retirada_do_pedido); reads (dias_ate_o_fim_do_contrato)

**Of interest to:** SGE, CMRTE
