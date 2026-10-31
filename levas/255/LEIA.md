# 255ª leva — o pedido de transferência do jogador, os cargos com contrato e quem não é da diretoria; as permissões de trabalho do começo do jogo, a contagem da cláusula e o prêmio por jogo do jogador; a fila da transferência livre, a próxima janela aberta e o empréstimo negociado; a contraproposta com cláusula e os motivos da recusa do lance; as relações do elenco no dia a dia: as brigas e preferências entre colegas, o atrito com o técnico e o moral espalhado pela liderança; o dia do jogo: as faltas ao treino, as competições do dia e a confiança da diretoria no clube; os estádios: o dia dos estádios (o estádio nacional fechado e as reservas vencidas), reservar um estádio numa data e se ele está livre; o jogo com jogador observado, a lista do olheiro do humano e o registro da primeira convocação; os gols na carreira do jogador, o registro do empréstimo e a ordem da fila das voltas aos treinos; os gols do jogador pelo clube, o registro da seleção do ano e a ordem dos clubes pelos pontos do técnico; o calendário guardado: achar a cópia guardada de um jogo, guardar um jogo e tirar um jogo; os amistosos marcados: criar os amistosos planejados da semana, acrescentar um amistoso e sortear a data livre do amistoso; o dia a dia das copas e o risco de doença na viagem: a atualização da copa no dia, montar a rodada da copa e o risco de doença dos visitantes

**Rotinas:**
- `pedido_de_transferencia()` = 0x4d8610 (contract_manager.cpp, thiscall, ret 4): no clube do computador: se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), põe o jogador na lista (+0x4e, moral +12, 0x842c00, 0x4d71b0); senão um sorteio dá a queixa 0x10000, a moral cai 12 e a força do pedido (+0x44) sobe; no clube humano, a notícia (com a original);
- `cargo_com_contrato()` = 0x4cbb20 (contract_manager.cpp, stdcall, ret 4): 1 para os cargos 0, 6 a 0xd e 0xf (os que têm contrato pelo gerente de contratos);
- `fora_da_diretoria()` = 0x4cb960 (contract_manager.cpp, stdcall, ret 4): 0 se o cargo (+0x3d) é de diretoria (1 a 4) num clube (+0x39); senão 1;
- `iniciar_permissoes_de_trabalho()` = 0x8c6e60 (transfer_manager.cpp, thiscall, ret 0): marca 0x3fff/0x1fff pelos jogos na seleção; ao jogador que precisa de permissão (0x8c7560), pelo ano mais antigo na nação do clube (carreira 0x7c3e40): menos de 5 anos, a permissão (0x8c7070); senão a segunda nacionalidade (+0x1e); ordena as permissões;
- `contagem_da_clausula()` = 0x8c3e50 (transfer_manager.cpp, thiscall, ret 8): tipo 4: jogos pela seleção desde o acordo (+0x22 menos o byte +7 do registro da pessoa); senão os jogos pelo clube (+0x3b do contrato);
- `premio_por_jogo_do_jogador()` = 0x8d0ea0 (transfer_manager.cpp, thiscall, ret 4): com o bit 2 da pessoa, busca o prêmio por jogo no vetor +0x88c; sem achar, tira o bit;
- `fila_da_transferencia_livre()` = 0x8c18c0 (transfer_manager.cpp, thiscall, ret 0): para cada jogador da fila (+0x92c): no clube humano a diretoria pode barrar (0x696940); um sorteio põe o técnico nos desafetos (0x547af0); os registros (0x86d800), as ofertas canceladas (0x8df9b0) e a liberação com a compensação (0x8c1af0); limpa a fila;
- `proxima_janela_aberta()` = 0x8cf470 (transfer_manager.cpp, thiscall, ret 8): o primeiro dia, de hoje até um ano depois, com a janela da nação do clube aberta (0x8e8e50; sem regra, hoje); sem achar, a caixa "Error";
- `emprestimo_negociado()` = 0x8c72d0 (transfer_manager.cpp, thiscall, ret 8): o item da lista +0x946 com a pessoa e o clube, ou 0;
- `oferta_com_clausula()` = 0x8c8fd0 (transfer_manager.cpp, thiscall (this sem uso), ret 0x14): com o clube do lance bem acima e um jogador de potencial até 30 anos: uma oferta nova (0x8e5680, 0x8e78f0) com 20% da próxima venda (apelo 80), o prêmio por jogo na seleção (70) ou no time (60), e o valor × 0,8; nenhuma, solta a oferta e 0;
- `motivo_da_recusa_vale()` = 0x8c5560 (transfer_manager.cpp, stdcall, ret 8): 0 quando o motivo não cabe: promessa (0x2000) com 24 anos ou mais, divisão (0x1000) igual, rival (0x800) sem confronto nem parceria; senão 1;
- `motivo_pelo_status_no_elenco()` = 0x8c54c0 (transfer_manager.cpp, stdcall, ret 4): pelo status no elenco do contrato com o clube atual: 1 → 0x200, 2 e 3 → 0x400, 5 → 0x2000; senão 0;
- `relacoes_do_elenco()` = 0x5d6d10 (game.cpp, cdecl): dez pares sorteados de colegas: quem não gostava deixa de não gostar e pode brigar (0x5888a0); senão passa a não gostar, a gostar ou deixa de gostar; a cada 50 dias, clubes sorteados pelo atrito no vestiário; nos outros dias cada jogador, por sorteios dos atributos, pode chegar ao atrito com o técnico;
- `atrito_com_o_tecnico()` = 0x5d7190 (game.cpp, cdecl): cada jogador do elenco com vínculo e preferências: o primeiro colega de quem não gosta; com dia % 14 + id = 0, a queixa contra o técnico, o auxiliar ou outro (0x587c60) e a marca no vínculo (0x867700);
- `moral_pela_lideranca()` = 0x5ce0b0 (game.cpp, cdecl): um clube em sete por dia (não seleção): liderança e liderança × moral do elenco somadas; cada jogador sorteia a partir delas e do próprio moral e o moral sobe ou desce um (0x544710);
- `faltas_ao_treino()` = 0x5d6aa0 (game.cpp, cdecl): n/250 pessoas seguidas a partir de uma sorteada: o jogador presente, com treino e sem lesão grave falta ao treino (0x41a360) por um sorteio da soma de atributos, moral e preferências pelo técnico (mais provável quando chegou há pouco) ou pelas marcas do vínculo;
- `competicoes_do_dia()` = 0x5ce060 (game.cpp, cdecl): cada competição da lista recebe o dia (virtual +0x14 com a data do jogo e a marca); com a marca, o passo seguinte do [0xae2a58] (0x5aaf00);
- `mudar_confianca_da_diretoria()` = 0x5cdde0 (game.cpp, thiscall, ret 8): soma o delta à confiança da diretoria no clube (+6 da linha de 73 bytes do clube), presa em 1..10000;
- `estadios_do_dia()` = 0x85b820 (stadium.cpp, thiscall, ret 4): no dia 0 refaz a lista dos estádios; antes de 2003 fecha o estádio nacional da Inglaterra (id em [0xa835a4], nação e seleção sem estádio) e o reabre em 1/6/2003 com 90000 lugares; a cada 30 dias tira as reservas vencidas e reordena;
- `reservar_estadio()` = 0x85de20 (stadium.cpp, thiscall, ret 8): acrescenta a reserva (id do estádio e data) à lista, que cresce de 30 em 30, e reordena por data e id;
- `estadio_livre_na_data()` = 0x85ddb0 (stadium.cpp, thiscall, ret 8): 1 se não há reserva do estádio na data (busca binária na lista; sem lista, 1);
- `jogo_com_observado()` = 0x7f9720 (scout_manager.cpp, thiscall, ret 4): diz se a partida tem clube da lista vigiada ou jogador com pedido de observação disponível (no clube ou na seleção, não cedido, não lesionado, não suspenso, não impedido e que ainda não jogou);
- `na_lista_do_olheiro()` = 0x7f7490 (scout_manager.cpp, thiscall, ret 8): diz se o jogador está na lista do olheiro do humano (pela edição aberta ou pela primeira busca de tipo 1);
- `registrar_a_primeira_convocacao()` = 0x86cc50 (staff_records.cpp, thiscall, ret 8): o histórico da primeira convocação da pessoa (tipo 6: o clube, o adversário e a idade);
- `gols_na_carreira()` = 0x7c3d20 (player_stats.cpp, stdcall, ret 4): soma os gols de todas as temporadas e clubes da tabela da carreira do jogador (0x7c3e40), em 16 bits;
- `registrar_o_emprestimo()` = 0x86da20 (staff_records.cpp, thiscall, ret 0xc): os históricos do empréstimo: o do técnico do clube que recebe (tipo 0x69) e o da pessoa (tipo 0x6a);
- `ordem_da_volta_aos_treinos()` = 0x632040 (injury.cpp, cdecl): compara a data da chave com a de uma entrada de 0x16 bytes da fila das voltas aos treinos, para o bsearch;
- `gols_pelo_clube()` = 0x7c3bb0 (player_stats.cpp, stdcall, ret 8): soma os gols das temporadas do jogador pelo clube na tabela da carreira (0x7c3e40), em 16 bits;
- `registrar_a_selecao_do_ano()` = 0x86c730 (staff_records.cpp, thiscall, ret 0x14): o histórico da seleção do ano da pessoa (tipo 0x32: o clube, o prêmio e dois valores);
- `ordem_pelos_pontos_do_tecnico()` = 0x870220 (staff_records.cpp, cdecl): ordena dois clubes pelos pontos da temporada do técnico na liga (mais pontos primeiro; sem registro, por último);
- `jogo_guardado_igual()` = 0x5ac7a0 (fix_man.cpp, thiscall, ret 4): devolve a cópia guardada cujos 0x4f bytes são iguais aos do jogo, na lista do dia, fatia e nação-chave dele no bloco do ano; 0 se não há;
- `guardar_jogo_no_calendario()` = 0x5ab230 (fix_man.cpp, thiscall, ret 4): acha (ou cria) o bloco do ano do jogo e acrescenta uma cópia dele à lista do dia, fatia e nação-chave;
- `tirar_jogo_do_calendario()` = 0x5abc30 (fix_man.cpp, thiscall, ret 4): tira da lista do dia, fatia e nação-chave do jogo o primeiro item com os mesmos bytes; devolve 1 se tirou;
- `criar_amistosos_planejados()` = 0x5c2ae0 (friendly.cpp, thiscall, ret 0): anda o plano dos amistosos da IA até hoje + 1 semana: na data, com os clubes livres, cria o amistoso e manda o pedido; plano acabado, libera o vetor;
- `acrescentar_amistoso()` = 0x5c6f00 (friendly.cpp, thiscall, ret 0x10): reaproveita o convite aberto do clube ou monta um amistoso novo; com o estádio de quem joga em casa livre, grava data, clubes e mando e o põe na lista;
- `planejar_amistoso_em_data_livre()` = 0x5c4380 (friendly.cpp, thiscall, ret 0xc): sorteia uma data na janela dos dois clubes e, com os dois livres, marca o amistoso e liga as datas e os adversários dos dois;
- `atualizar_copa_no_dia()` = 0x51f890 (Cup.cpp, thiscall, ret 8): passa as fases completas e encerra a copa, dá o dia a cada fase e, na data, sorteia (fase 0) ou monta (fase ≠ 0) as rodadas até não avançar;
- `montar_rodada_da_copa()` = 0x51c800 (Cup.cpp, thiscall, ret 4): sorteia a rodada se falta, cria os jogos dos confrontos marcados, leva-os às datas da rodada e passa à seguinte; time sem clube adia as rodadas um dia;
- `risco_de_doenca_na_viagem()` = 0x5ab3c0 (fix_man.cpp, thiscall, ret 4): com o estádio num país do terceiro mundo, dá aos clubes de fora o risco de doença na véspera do jogo;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (36):** 29 acusadas. `cps3_sem_registros` escapou: o ramo da liberação roda 8 vezes, e o que a mutação tira são os registros do histórico das pessoas (Staff_Records), que só as telas e o fim de temporada leem [inferência]. `cps4_time_0_9` escapou: o prêmio por jogo no time da contraproposta (32 vezes) só pesa se a contraproposta for aceita, e nenhuma das 32 foi até o contrato antes de 2002-06-01 [inferência]. `cps9_um_gol_a_mais` escapou: os gols na carreira vão para o jogador da partida (+0x43), que só a narração e as notícias de marca usam [inferência]. `cps9_sem_o_primeiro_grupo` escapou: o mesmo valor dos gols na carreira (+0x43), só da narração e das notícias de marca [inferência]. `cps10_todos_os_clubes` escapou: os gols pelo clube vão para o jogador da partida (+0x3d), que só a narração e as notícias de marca usam [inferência]. `cps10_um_gol_a_mais` escapou: o mesmo valor dos gols pelo clube (+0x3d), só da narração e das notícias de marca [inferência]. `cps13_montagem_atrasada` escapou: a montagem da rodada (32 vezes) é chamada de novo no dia seguinte e monta a mesma rodada antes dos jogos [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (2a181435).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 dabca044, original 982f511c (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 254ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 255`

**Campos tocados:**
- contrato +0 / +4: pessoa e clube; id; lê (pedido_de_transferencia)
- contrato +0x44: força do pedido de transferência (com sinal, ±100); valor; lê e grava (pedido_de_transferencia)
- contrato +0x4e: bit 2 tirado e bit 8 (na lista a pedido) ligado; valor; grava (pedido_de_transferencia)
- contrato +0x4f & 0xf0: status no elenco (0x10 = 1); valor; lê (pedido_de_transferencia)
- staff +0x59: ânimo; staff +0x61: ficha de jogador; valor e ponteiro; lê (pedido_de_transferencia)
- staff +0x39 / +0x3d: clube e cargo; ponteiro e valor; lê (fora_da_diretoria)
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa: +0 bit 2 (prêmio por jogo), palavra +3 (0x3fff/0x1fff), byte +7 (jogos na seleção no acordo); valor; lê e grava
- TRANSFER_MANAGER +0x88c/+0x894: prêmios por jogo (13 bytes) e a conta; valor; lê (premio_por_jogo_do_jogador)
- TRANSFER_MANAGER +0x93a/+0x93e: permissões de trabalho e a conta; valor; ordena (iniciar_permissoes_de_trabalho)
- staff +0x22: jogos pela seleção; valor; lê
- staff +0x1e: segunda nacionalidade; ponteiro; grava (iniciar_permissoes_de_trabalho)
- staff +0x39/+0x3d/+0x3e/+0x40: clube, cargo, data e ano de chegada; lê (iniciar_permissoes_de_trabalho)
- contrato +0x3b: jogos pelo clube; valor; lê (contagem_da_clausula)
- [0xae2c92] ano de hoje; [0xdd6ec8] data do começo; valor; lê
- TRANSFER_MANAGER +0x92c: fila da transferência livre (LLIST; item +0 id da pessoa, +4 motivo, +5 compensação); valor; lê e limpa (fila_da_transferencia_livre)
- TRANSFER_MANAGER +0x8ac: as 28 regras das nações (+0xa nação); ponteiro; lê (proxima_janela_aberta)
- TRANSFER_MANAGER +0x946: lista dos empréstimos negociados (item +0 pessoa, +4 clube); ponteiro; lê e move o cursor (emprestimo_negociado)
- clube +0xcf: técnico; clube +0x53: nação; ponteiro; lê
- staff +0x5b: o valor do sorteio do desafeto; valor; lê (fila_da_transferencia_livre)
- [0xae1080] contrato da vez; ponteiro; grava (fila_da_transferencia_livre)
- oferta +0x1b: valor; +0x3a/+0x3e: prêmio por jogo na seleção e os jogos; +0x35/+0x39: prêmio por jogo no time e os jogos; +0x3f: percentual da próxima venda; valor; grava (oferta_com_clausula)
- oferta +8 / +0xc: clube do lance e pessoa; id; lê (motivos)
- clube +0x80: reputação; clube +0x57: divisão; clube +0xb3/+0xb7/+0xbb: parceiros; valor e ponteiro; lê
- ficha de jogador +5/+7: habilidade e potencial; +0xb: reputação; valor; lê (oferta_com_clausula)
- staff +0x18: idade; +0x1a: nação (+0x8e reputação); +0x22: jogos pela seleção; lê
- contrato +0x4f >> 4: status no elenco; valor; lê (motivo_pelo_status_no_elenco)
- [0xae1080] contrato da vez; ponteiro; grava
- clube +0xd7: as 50 vagas do elenco; ponteiros; lê (as três)
- clube +0xcf técnico, +0xd3 auxiliar, +0xff a pessoa do vínculo, +0x80 reputação; ponteiros/valor; lê (relacoes_do_elenco, atrito_com_o_tecnico)
- pessoa +0x18, +0x59..+0x5d atributos mentais; valores; lê (relacoes_do_elenco)
- pessoa +0x39 clube do contrato, +0x61 ficha de jogador, +0x65 preferências; ponteiros; lê (as três)
- jogador (pessoa +0x61) +0x2f liderança, +0xb reputação; valores; lê (moral_pela_lideranca, relacoes_do_elenco)
- vínculo (0x4cf5f0) +0x44 moral, +0x45 marcas; valores; lê (as três)
- [0xae1080] o último contrato achado (0x4cf5a0); ponteiro; grava (relacoes_do_elenco)
- pessoa +0x3d função de origem (11 = jogador), +0x3e data de chegada, +0x18, +0x5b atributos; valores; lê (faltas_ao_treino)
- jogador (pessoa +0x61) +0x2f, +0x44 moral; valores; lê (faltas_ao_treino)
- vínculo (0x4cf5f0) +0x45 marcas 0x8000, 0x30000, 0x7800; valores; lê (faltas_ao_treino)
- clube +0xcf técnico; ponteiro; lê (faltas_ao_treino)
- [0xae1080] o último contrato achado (0x4cf5a0); ponteiro; grava (faltas_ao_treino)
- [0xadadfc]/[0xadadf8] a lista das competições e a conta; ponteiro/valor; lê (competicoes_do_dia)
- MANAGER_MANAGER [0xb67a44] +0: linhas de 73 bytes por clube, +6 confiança da diretoria; valor; lê e grava (mudar_confianca_da_diretoria)
- Stadium [0xdd7ef4] +0 reservas (12 bytes: +0 id do estádio, +4 data), +4 conta, +8 capacidade; ponteiro/valores; lê e grava (estadios_do_dia, reservar_estadio), lê (estadio_livre_na_data)
- nação +0x80 estádio nacional; ponteiro; lê e zera na Inglaterra [0x9cf2e4] (estadios_do_dia)
- clube +0x69 estádio; ponteiro; zera na seleção principal da Inglaterra (estadios_do_dia)
- estádio (0x4e bytes em [0xae23b0]) +0 id, +0x3c/+0x40 lugares; valores; lê o id, grava os lugares na reabertura (estadios_do_dia)
- [0xa835a4] id do estádio nacional fechado (−1 = nenhum); id; grava (estadios_do_dia)
- partida +0, +0xc, +0x10, +0x14, +0x1c, +0x20: competição, dois ids comparados à lista vigiada, competição para a suspensão e os dois clubes; id e ponteiro; lê (jogo_com_observado)
- [0xb74438] lista dos clubes vigiados (LLIST estática, cursor +0xa reposto do +2); valor; lê e grava o cursor (jogo_com_observado)
- SCOUT_MANAGER +0x5c pedidos de observação (0x11 bytes, a pessoa em +4), +0x64 conta; ponteiro e valor; lê (jogo_com_observado)
- SCOUT_MANAGER +0xc + vaga × 4: edição aberta do humano; ponteiro; lê (na_lista_do_olheiro)
- busca do olheiro +0xa: tipo (1 = lista do olheiro); valor; lê (na_lista_do_olheiro)
- [0xb74c26 + 0x300 × (i16)[0xb77c7a]] gerente do humano da vez; ponteiro; lê (na_lista_do_olheiro)
- pessoa +0x39 e +0x24: clube e seleção; ponteiro; lê (jogo_com_observado, registrar_a_primeira_convocacao)
- pessoa +0x18: idade; valor; lê (registrar_a_primeira_convocacao)
- Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_a_primeira_convocacao)
- pessoa +0x61: ficha de jogador; ponteiro; lê (gols_na_carreira)
- tabela da carreira (0x7c3e40): entradas de 0x2b bytes, seis por temporada e clube; +0xdf do grupo: gols; valor; lê (gols_na_carreira)
- jogador da partida +0x43: gols na carreira; valor; grava (MATCH_PLAYER::setup, pela gols_na_carreira)
- clube +0xcf: técnico; ponteiro; lê (registrar_o_emprestimo)
- pessoa +4, +8, +0xc → +0x33: os três ids do nome; id; lê (registrar_o_emprestimo)
- fila das voltas aos treinos (Injury +0x10, conta +0x18; 0x16 bytes, a data em +8); valor; lê (ordem_da_volta_aos_treinos)
- Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_o_emprestimo)
- pessoa +0x61: ficha de jogador; ponteiro; lê (gols_pelo_clube)
- tabela da carreira (0x7c3e40): entradas de 0x2b bytes, seis por temporada e clube; +0xd9 do grupo: id do clube; +0xdf: gols; id e valor; lê (gols_pelo_clube)
- jogador da partida +0x3d: gols pelo clube; valor; grava (MATCH_PLAYER::setup, pela gols_pelo_clube)
- clube +0xcf: técnico; ponteiro; lê (ordem_pelos_pontos_do_tecnico)
- registro do técnico (0x86f5b0 do [0xdd7f00]) +0xba: pontos da temporada na liga; valor; lê (ordem_pelos_pontos_do_tecnico)
- Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (registrar_a_selecao_do_ano)
- jogo (0x4f bytes) +0x14: a competição; +0x28 o ano (palavra); +0x2a o dia (palavra); +0x3f a fatia; ponteiro e valor; lê (as três)
- bloco do ano do calendário (0x1184 bytes) +0: as listas (dia × 3 + fatia) de vetores por nação-chave; +0x112c o bloco seguinte; +0x1130 o ano (palavra); ponteiro e valor; lê e grava o seguinte (as três)
- lista (LLIST) +0: a conta (palavra); +2 o começo; +6 a cauda; +0xa o cursor; nó +0 o dado, +4 o tamanho, +6 o seguinte; valor e ponteiro; lê, anda o cursor, acrescenta e tira (as três)
- Friendly +0x34d8: o plano dos amistosos da IA (20 bytes: +0 a data, +8/+0xc/+0x10 os índices dos clubes A, B e C); +0x34dc o índice; +0x34e0 a conta; +0x34e4 a capacidade; ponteiro e valor; lê e grava (criar_amistosos_planejados)
- Friendly +0x34c8: a lista dos amistosos; +0xa7 o início da temporada (data); ponteiro e valor; lê e cria (acrescentar_amistoso, planejar_amistoso_em_data_livre)
- amistoso (0x19 bytes) +0: a data; +8 zero; +0xc o clube; +0x10 o mandante; +0x14 o visitante; +0x18 a situação (1); ponteiro e valor; grava (acrescentar_amistoso)
- candidato do amistoso +4: a conta (byte); +0xa a lista dos adversários; +0xe/+0x10 a janela (palavras com sinal); +0x12 a lista das datas; valor e ponteiro; lê e grava (planejar_amistoso_em_data_livre)
- clube +0x69: o estádio; ponteiro; lê (acrescentar_amistoso)
- copa +0x3c: as rodadas (palavra); +0x40 o ano-base; +0x44 o pai (0xff sem); +0x45 ativa; +0x4c encerrada; +0x2c/+0x30 as fases e a atual; +0xc as fases; +8 o Comp_Stats; +0xb1 com fases; valor e ponteiro; lê e grava +0x4c (atualizar_copa_no_dia)
- copa +0xa3: as rodadas (0x68 bytes: +0 o dia e +2 o ano do sorteio, +3/+5 a data dos jogos, +7 o nome, 0x8c a final, +0x1a os confrontos); +0xa7 os times (6 bytes, +0 o clube); +0xab a rodada atual; +0xac a do sorteio; +0xad as fases das rodadas (+0x34 pendente; confrontos de 0x35 bytes, +0/+2 os times, +0x34 a marca); lê e grava (as duas da copa)
- [0xadadfc][id da competição]: a competição ativa; [0xae2c88] o modo do dia; valor; lê (as duas da copa)
- jogo +0x18 o estádio (+0x38 a cidade, +0x1f a nação, +0x7e o desenvolvimento); +0x1c/+0x20 os clubes (+0x53 a nação); +0xc/+0x10 os ids; +0x28/+0x2a o ano e o dia; ponteiro e valor; lê (risco_de_doenca_na_viagem)

**Interessa a:** CMRTE, SGE, CMScout, GPF2
