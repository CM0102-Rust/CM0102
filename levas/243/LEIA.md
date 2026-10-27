# 243ª leva — os recordes do clube: as cópias dos recordes de nota, de melhor em campo e de disciplina; os recordes do clube: a cópia dos recordes de contagem e do jogador da torcida e a construção dos recordes de ocorrências e de contagem; os recordes do clube: as cópias do recorde de valor de transferência e do recorde de gols numa partida; as cópias da notícia e das transferências passada e futura; a cópia da entrada da fila de regens, a busca na lista de observados e a escolha do doador de dados de não jogador; a cópia do registro de suspensão, o salário padrão do contrato e a condição de jogo do jogador; o contrato de pessoal: os dias até o fim, limpar todas as queixas e conferir a retirada do pedido de transferência

**Rotinas:**
- `copiar_recorde_de_nota()` = 0x448230 (club_records.cpp, thiscall, ret 4): copia o recorde de nota (0x1f bytes): a nota (+0), o recordista (+4, pela 0x448490), +0x1c e +0x1d;
- `copiar_recorde_de_melhor_em_campo()` = 0x4484d0 (club_records.cpp, thiscall, ret 4): copia o recorde de melhor em campo (0x1b bytes): as vezes (+0), o recordista (+1) e o ano (+0x19);
- `copiar_recorde_de_disciplina()` = 0x448500 (club_records.cpp, thiscall, ret 4): copia o recorde de disciplina (0x1c bytes): amarelos (+0), vermelhos (+1), o recordista (+2) e o ano (+0x1a);
- `copiar_recorde_de_contagem()` = 0x448530 (club_records.cpp, thiscall, ret 4): copia o recorde de contagem (0x1a bytes): a contagem (+0, palavra) e o recordista (+2, pela 0x448490);
- `copiar_recorde_do_torcedor()` = 0x4485b0 (club_records.cpp, thiscall, ret 4): copia o recorde de jogador da torcida (0x1a bytes): o recordista (+0) e o ano (+0x18);
- `construir_recorde_de_ocorrencias()` = 0x450c60 (club_records.cpp, thiscall): constrói o recorde de ocorrências: a data do recordista em +0x11 (0x54a110);
- `construir_recorde_de_contagem()` = 0x450d30 (club_records.cpp, thiscall): constrói o recorde de contagem: a data do recordista em +0x12 (0x54a110);
- `copiar_recorde_de_valor()` = 0x448550 (club_records.cpp, thiscall, ret 4): copia o recorde de valor de transferência (0x29 bytes): cinco duplas, as datas (+0x14, +0x20), +0x1c e a marca de troca (+0x28);
- `copiar_recorde_de_gols_na_partida()` = 0x451570 (club_records.cpp, thiscall, ret 4): copia o recorde de gols numa partida (0x38 bytes): os gols (+0), o recordista (+1) e a partida (+0x19);
- `copiar_noticia()` = 0x786010 (news.cpp, thiscall, ret 4): copia a notícia (0xdf bytes): +0, +4, as 50 fichas (+5), a data (+0xcd), +0xd5, +0xd6, +0xda e +0xde;
- `copiar_transferencia_passada()` = 0x8cb940 (transfer_manager.cpp, thiscall, ret 4): copia a transferência passada (0x25 bytes): sete duplas, o byte +0x1c e a data (+0x1d);
- `copiar_transferencia_futura()` = 0x8cbf40 (transfer_manager.cpp, thiscall, ret 4): copia a transferência futura (0xc bytes): a data (+0) e o índice da oferta (+8);
- `copiar_entrada_da_fila_de_regens()` = 0x7a8b70 (player_regen.cpp, thiscall, ret 4): copia a entrada da fila de regens (0xc bytes): a pessoa (+0) e a data do regen (+4);
- `esta_na_lista_de_observados()` = 0x842080 (shortlist_manager.cpp, thiscall, ret 8): o par (jogador, clube ou −1) está nos pares observados? bsearch com a 0x757ea0;
- `escolher_doador_de_dados()` = 0x7a6e60 (player_regen.cpp, stdcall, ret 4): a pessoa não jogadora de maior nota positiva (0x7a7150) da nação; sem nenhuma, de qualquer nação; senão 0;
- `copiar_registro_de_suspensao()` = 0x556c70 (discipline.cpp, thiscall, ret 4): copia o registro de suspensão (0x14 bytes): campos, a data (+4) e os bits 0..2 de +0x13;
- `salario_padrao_do_contrato()` = 0x8626e0 (staff_contracts.cpp, thiscall, ret 4): o salário (+0xc) = o padrão da pessoa no clube (0x862720) truncado, mínimo 55;
- `condicao_de_jogo_do_jogador()` = 0x62c510 (injury.cpp, thiscall, ret 8): atualiza o físico do dia (0x62c680) e devolve a condição de jogo (de 10000, ou em porcento);
- `dias_ate_o_fim_do_contrato()` = 0x85fa00 (staff_contracts.cpp, thiscall): os dias da data do jogo até o fim do contrato (+0x2d), pela 0x54cdb0;
- `limpar_todas_as_queixas()` = 0x867520 (staff_contracts.cpp, thiscall, ret 4): zera as queixas (+0x45), tira os vínculos de opinião (0x549c90) e, pedido, confere a retirada (0x867240);
- `conferir_retirada_do_pedido()` = 0x867240 (staff_contracts.cpp, thiscall, ret 4): jogador do clube do contrato com pedido de transferência, sem critério da comissão e sem contrato acertado: retira o pedido (0x4d9460), a moral vira +0x5b + +0x59 + 5, +0x3e = 0, +0x3d ao menos 15, e apaga cada queixa conferindo de novo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (22):** 18 acusadas. `cpg_doador_zero` escapou: escolher_doador_de_dados 0x7a6e60: a nota exatamente zero só muda a escolha quando nenhuma outra pessoa tem nota positiva; não acontece até 2002-06-01; escapou também no jogo comparando e na carga pt (0 divergências). `cpg_salario_minimo` escapou: salario_padrao_do_contrato 0x8626e0: nenhum salário padrão cai em 55 ou abaixo até 2002-06-01 (o mínimo não age); escapou também no jogo comparando e na carga pt (0 divergências). `cpg_condicao_porcento` escapou: Injury::get_match_condition 0x62c510: rotina desligada (DESLIGADAS em motor.rs, 27/10/2026): sem desvio em nenhum modo, a cópia não roda. `cph_queixas_zero` escapou: limpar_todas_as_queixas 0x867520: os três bytes altos das queixas (+0x46..+0x48) já estão zerados quando a rotina roda na carreira; escapou também no jogo comparando e na carga pt (0 divergências). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 242ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 243`

**Campos tocados:**
- recordes do clube: recorde de nota (0x1f bytes), de melhor em campo (0x1b) e de disciplina (0x1c), com o recordista de 0x18 bytes dentro; escreve (copiar_recorde_de_nota, copiar_recorde_de_melhor_em_campo, copiar_recorde_de_disciplina)
- recordes do clube: recorde de contagem (0x1a bytes: palavra + recordista), de jogador da torcida (0x1a: recordista + ano) e de ocorrências (0x19: byte + recordista); escreve (copiar_recorde_de_contagem, copiar_recorde_do_torcedor, construir_recorde_de_ocorrencias, construir_recorde_de_contagem)
- recordes do clube: recorde de valor de transferência (0x29 bytes, datas em +0x14 e +0x20, troca em +0x28) e de gols numa partida (0x38: gols + recordista + registro de partida); escreve (copiar_recorde_de_valor, copiar_recorde_de_gols_na_partida)
- notícia (0xdf bytes: 50 fichas em +5, data em +0xcd), transferência passada (0x25 bytes, data em +0x1d) e futura (0xc bytes: data + índice da oferta); escreve (copiar_noticia, copiar_transferencia_passada, copiar_transferencia_futura)
- entrada da fila de regens (0xc bytes: pessoa + data); escreve (copiar_entrada_da_fila_de_regens); pares observados (8 bytes: jogador, clube ou −1), vetor em [this] e quantos em +0x10; lê (esta_na_lista_de_observados); não jogadores ([0xae23bc], 0x6e bytes, +0x1a nação); lê (escolher_doador_de_dados)
- registro de suspensão (0x14 bytes: data em +4, marcas nos bits 0..2 de +0x13); escreve (copiar_registro_de_suspensao); contrato +0xc salário; escreve (salario_padrao_do_contrato); ficha de lesão (31 bytes, +0xa condição de jogo); lê (condicao_de_jogo_do_jogador)
- contrato de pessoal (0x50 bytes): +0 pessoa, +4 clube, +0x2d data do fim, +0x3d felicidade, +0x3e, +0x44 moral, +0x45 queixas (bits), +0x49, +0x4e bit 3 pedido de transferência, +0x4f nibble alto; lê e grava (limpar_todas_as_queixas, conferir_retirada_do_pedido); lê (dias_ate_o_fim_do_contrato)

**Interessa a:** SGE, CMRTE
