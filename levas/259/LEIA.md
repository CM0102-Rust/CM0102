# 259ª leva — a satisfação do jogador com o contrato, a notícia de que ele quer contrato novo e a do pedido de transferência retirado; a aprovação final da oferta pelos clubes, o fechamento da oferta aprovada e o acerto das cláusulas anteriores; a oferta de empréstimo pelo jogador da lista, a notícia do contrato acabando e a da saída pela lei Bosman; a oferta de contrato acrescentada ou atualizada, a oferta de contrato tirada pela vaga e o menor salário pedido; as notas do mês: as notas da partida, a nota do jogador no mês e o melhor do mês; o prêmio do mês: o melhor das notas, a vaga do vencedor e a reputação do premiado; o físico do dia do jogador: a forma e a condição de jogo até hoje; a faixa de confiança da diretoria na liga e as notas da diretoria; o dia das lesões: as lesões que envelhecem e as lesões no treino; as notícias da volta aos treinos e o jogador a um cartão da suspensão; a fila dos regens: os regens jovens que vencem e os técnicos de reserva refeitos; o veículo da notícia pelo clube e pela pessoa; a tabela da liga: o resultado na tabela, a cópia da tabela e as posições previstas; o jogo entre seleções: as convocações, os jogos e gols pela seleção, o nome do elenco e as rodadas conhecidas; os convocados por clube na data do jogo da seleção, a importância do jogo e a fase da competição

**Rotinas:**
- `satisfacao_com_o_contrato()` = 0x866620 (staff_contracts.cpp, thiscall): de 1 a 3: 100 × o valor do contrato atual / o do desejado (com a situação merecida), com o desconto de quem chegou há pouco; abaixo de 50: 1; abaixo de 80: 3; senão 2;
- `noticia_quer_contrato_novo()` = 0x4d9f80 (contract_manager.cpp, thiscall): a notícia 0xbc2 de que o jogador quer contrato novo, ao técnico do clube, se ele não estiver na lista de transferências nem em oferta;
- `noticia_pedido_de_transferencia_retirado()` = 0x4d9460 (contract_manager.cpp, thiscall): a notícia 0xbc0 do pedido de transferência retirado, com o motivo, ao técnico do clube;
- `confirmar_oferta_pelo_clube()` = 0x8c3310 (transfer_manager.cpp, thiscall): registra a aprovação de um clube (estados 0xb a 0xe); com os dois de acordo, agenda a conclusão para hoje ou, com a data do prazo ainda por vir, faz a transferência futura;
- `tentar_concluir_oferta()` = 0x8c34e0 (transfer_manager.cpp, thiscall): reconfirma comprador e vendedor; uma recusa desfaz ou recusa a oferta; com os dois de acordo, conclui a transferência ou a adia para o fim do contrato;
- `acertar_clausulas_anteriores()` = 0x8c91e0 (transfer_manager.cpp, thiscall): paga e tira as cláusulas antigas (percentual da próxima venda, prêmios por jogo no time e na seleção) do jogador e das pessoas da troca;
- `tentar_emprestimo_pelo_jogador()` = 0x842740 (shortlist_manager.cpp, thiscall): com interesse de 3,5, o período do empréstimo e a data de volta dentro do contrato, da janela e a 3 semanas de hoje, faz a oferta com 0, 50 ou 100% do salário pela diferença de reputação; senão −1;
- `noticia_contrato_acabando()` = 0x4d6790 (contract_manager.cpp, thiscall): a notícia 0xbbd do contrato acabando ao técnico do clube e, se pedido, aos clubes interessados de nações ativas;
- `noticia_saida_pela_lei_bosman()` = 0x4d6d70 (contract_manager.cpp, thiscall): a notícia 0xbbe da saída pela lei Bosman, com o clube de destino ou −1;
- `acrescentar_oferta_de_contrato()` = 0x4cc490 (contract_manager.cpp, thiscall): atualiza a oferta do mesmo clube à pessoa ou acrescenta uma nova à lista (o contador de ofertas sobe); marca os termos oferecidos na oferta de transferência e, se pedido, agenda a decisão;
- `tirar_oferta_de_contrato_da_vaga()` = 0x4ccb30 (contract_manager.cpp, thiscall): tira da lista a oferta de contrato da pessoa ligada à vaga da transferência (desliga a marca de contrato acertado) e baixa o contador de ofertas;
- `menor_salario_pedido()` = 0x4ce450 (contract_manager.cpp, stdcall): o menor dos salários desejados com a situação 1 e 2 no clube, no máximo 10 milhões;
- `guardar_notas_da_partida()` = 0x767ee0 (month_ratings.cpp, thiscall, ret 8): os dois lados da partida, 20 vagas cada: fora a vaga vazia (0xfb) e a reserva que não entrou, a nota de cada pessoa vai para as notas do mês;
- `nota_do_jogador_no_mes()` = 0x767fa0 (month_ratings.cpp, thiscall, ret 4): acha ou acrescenta o registro de 12 bytes da pessoa (cresce 300 quando cheia; a liga do clube e a idade), soma a nota e conta um jogo;
- `melhor_do_mes()` = 0x768130 (month_ratings.cpp, thiscall, ret 8): ordena pela média e devolve o primeiro da liga com 3 jogos (depois 2, 1, 0), com o filtro de idade e a liga atual do clube;
- `premiar_o_melhor_do_mes()` = 0x767150 (month_award.cpp, thiscall, ret 8): o melhor das notas do mês (0x768130) na liga do prêmio e com a idade pedida vira o vencedor do mês, com o clube dele;
- `gravar_vencedor_do_mes()` = 0x7671c0 (month_award.cpp, thiscall, ret 0x10): acha a vaga do mês, grava a pessoa e o clube, sobe a reputação do premiado, manda a notícia e registra o prêmio no histórico;
- `reputacao_do_premiado()` = 0x412f00 (award_manager.cpp, cdecl): jogador: as reputações mundial, nacional e atual sobem uma fração das do prêmio, até 10000 e pelo menos elas; não jogador: sobem até elas e a confiança da diretoria ganha 1000 (entre 6500 e 10000);
- `fisico_do_dia()` = 0x62c680 (injury.cpp, thiscall, ret 4): uma vez por dia: os dias desde a última atualização viram dias livres, em recuperação, parados (lesão, com o fim dela pela 0x62e700) ou de férias (clube fora da temporada, 0x669bd0); a forma (+8) recupera 1000 − 12 × (+0x1e − natural) por dia livre, perde 66 por dia parado e 55 por dia de férias, com teto pela forma natural e trechos encolhidos acima de 7000; a condição de jogo (+0xa) sobe pelos dias livres e em recuperação; grava a data de hoje;
- `faixa_da_confianca_na_liga()` = 0x690bd0 (manager_manager.cpp, thiscall, ret 4): a confiança da diretoria na liga (+0x12 do registro de 0x49 bytes do clube) em faixas 0 a 6 (−2000, −1000, −500, 500, 1000, 2000);
- `gravar_notas_da_diretoria()` = 0x69f7c0 (manager_manager.cpp, thiscall, ret 8): as quatro notas da diretoria (+6, +8, +0xc, +0xa do registro de 0x49 bytes do clube) recebem o valor;
- `dia_das_lesoes()` = 0x62b7b0 (injury.cpp, thiscall, ret 0): solta a lista das batidas da partida, desce os dois contadores de cada lesão, no dia 180 da primeira temporada o físico de todos; nos dias de semana [0xae2364] / 150 sorteios (pelo menos 10) de uma pessoa: o jogador presente e sem lesão que passa pelos sorteios de risco (ficha +0x2d, propensão, recorrência) ganha a categoria, o tipo (0x62d140), o gasto dos atributos e a lesão nova (0x62d250); no fim as notícias da volta (0x62e010) e os surtos (0x62e9e0);
- `noticias_de_volta_aos_treinos()` = 0x62e010 (injury.cpp, thiscall, ret 0): nos clubes dos técnicos humanos, a notícia da volta ao treino leve (uma vez, +0x1c) e o físico do dia de cada lesionado; na fila das voltas, as de data A hoje e as de data B hoje dão a notícia (0x630ee0) e as de data B passada saem do começo;
- `um_cartao_da_suspensao()` = 0x55d360 (discipline.cpp, thiscall, ret 8): com id válido, a pergunta ao órgão da competição (+0x50; 0x7e8d60 com o id e o +4 da competição); senão 0;
- `fila_de_regens()` = 0x7a65d0 (player_regen.cpp, thiscall, ret 0): da frente da fila dos regens jovens (12 bytes: id e data), enquanto a data for antes de hoje, o regen jovem (0x7abac0) e a entrada sai; com mais de 2 reservas de técnico, refaz conta − 2 delas: a pessoa que empresta os dados (0x7a6e60) ganha o registro de não jogador, o cargo no clube da máquina e as notas pela reputação (pow e faixas) ou sorteadas; a reserva sai (0x7a82b0);
- `veiculo_pelo_clube()` = 0x757f30 (media.cpp, cdecl): reputação do clube acima de 6250: o veículo 0 a 6 pelo sorteio(100); senão 2 ou 4 pelo sorteio(4);
- `veiculo_pela_pessoa()` = 0x757fa0 (media.cpp, cdecl): o mesmo pela reputação do jogador (+0xb da ficha);
- `somar_resultado_na_liga()` = 0x685d30 (league.cpp, thiscall, ret 8): grava a data do dia na liga; partida de uma fase vai para a fase; senão soma a partida na tabela (0x685f60) e na estatística da competição (0x49f570) e devolve 1;
- `copiar_tabela_da_liga()` = 0x687710 (league.cpp, thiscall, ret 0): aloca e devolve uma cópia das linhas da tabela, com vetores novos de confrontos e de forma; acerta o +0xa7 quando é −1;
- `posicoes_previstas_da_liga()` = 0x68a850 (league.cpp, thiscall, ret 0): ordena os clubes da tabela pela força e grava a posição de cada um na estatística da competição;
- `resultado_entre_selecoes()` = 0x76b600 (national_teams.cpp, thiscall, ret 8): entre duas seleções: conta o jogo de cada nação; pela competição, marca o registro geral, troca a nação ou, nos jogos oficiais, soma o ranking da FIFA e os jogos e gols pela seleção de quem jogou; zera o +0x44 do tipo na data dele e manda a notícia aos clubes;
- `nome_do_elenco_da_selecao()` = 0x775610 (national_teams.cpp, thiscall, ret 0x14): escreve o nome do elenco (Main, B, Under 21 ou Under 23), em minúsculas quando pedido;
- `rodada_com_jogos_conhecidos()` = 0x48d410 (comp.cpp, thiscall, ret 8): a fase é liga, ou a rodada é antes da atual, ou é a atual e última com o código 0x8c;
- `registrar_convocados_por_clube()` = 0x7771c0 (national_teams.cpp, thiscall, ret 0xc): ordena o elenco da seleção pelo clube e soma, na data do jogo, quantos convocados cada clube cede (o registro que impede marcar jogo do clube perto da data);
- `importancia_do_jogo_na_competicao()` = 0x48e190 (comp.cpp, thiscall, ret 0xc): repassa a pergunta da importância do jogo à fase dele (virtual +0x54);
- `fase_da_competicao()` = 0x48d200 (comp.cpp, thiscall, ret 4): devolve a fase k da competição (−1 = ela mesma); fora da faixa, a caixa e 0;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (35):** 35 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (954d8f4e).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 4a762e34, original 086f15e7 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 258ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 259`

**Campos tocados:**
- contrato +4: clube; id; lê (as três)
- contrato +0x25 dia, +0x27 ano: início do contrato; valor; lê (satisfacao_com_o_contrato)
- contrato +0x4f >> 4: situação no elenco; valor; lê (satisfacao_com_o_contrato, noticia_quer_contrato_novo)
- contrato +0xc salário, +0x2d/+0x2f fim, +0x3f; valor; lê (as notícias)
- pessoa +0x59: adaptabilidade; valor; lê (satisfacao_com_o_contrato, noticia_quer_contrato_novo)
- pessoa +0x5b; ficha +0xd: reputação; valor; lê (noticia_quer_contrato_novo)
- [0xae2c90] hoje; valor; lê (satisfacao_com_o_contrato)
- oferta +4: lista dos jogadores da troca; ponteiro; lê (as três)
- oferta +8: clube comprador; +0xc: jogador; id; lê (as três)
- oferta +0x2c: estado (0xb a 0xe na aprovação); valor; lê e grava (confirmar_oferta_pelo_clube)
- oferta +0x2f: confirmação; +0x30 & 0xf: tipo de prazo (3 = fim do contrato); valor; lê (confirmar_oferta_pelo_clube, tentar_concluir_oferta)
- contrato +4: clube; id; +0x35 & 0x3f: tipo (5 sem clube); +0x2d: fim; lê (tentar_concluir_oferta)
- [this+0x86e] marcas de 8 bytes por pessoa (bits 2, 4, 8: cláusulas); valor; lê (acertar_clausulas_anteriores)
- [0xae1080] o último contrato pedido; ponteiro; grava (tentar_concluir_oferta)
- contrato +4: clube (dono); id; lê (as três)
- contrato +0x2d: fim do contrato; valor; lê (tentar_emprestimo_pelo_jogador)
- vaga da lista +0x24 >> 1 & 0xf: situação; valor; lê (tentar_emprestimo_pelo_jogador)
- clube +0x53: nação; ponteiro; +0x80: reputação; valor; lê (tentar_emprestimo_pelo_jogador, noticia_contrato_acabando)
- [0xb63d8a] + 72 × nação-chave: o começo da janela; valor; lê (tentar_emprestimo_pelo_jogador)
- pessoa +0x61: ficha; ponteiro; lê (noticia_contrato_acabando)
- [0xae1080] o último contrato pedido; ponteiro; grava (tentar_emprestimo_pelo_jogador)
- [0xae2d0c] + 79 × id +8: contador de ofertas de contrato da pessoa (general.dat); valor; lê e grava (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)
- CONTRACT_MANAGER +0x34: lista das ofertas de contrato (registros de 0x6f bytes); ponteiro; lê e grava (acrescentar_oferta_de_contrato, tirar_oferta_de_contrato_da_vaga)
- oferta de contrato +0..+0x50: os termos (STAFF_CONTRACT); +0x50: 2 acertada; +0x51: data; +0x59: vaga da transferência; +0x5d: valor; +0x61: marca; +0x62: fora do tipo 6; +0x63..+0x6e: o pedido de 0x7a81a0; valor; grava e lê
- oferta de transferência +0xc: jogador; id; lê (acrescentar_oferta_de_contrato)
- [0xae1080] o último contrato pedido; ponteiro; grava (menor_salario_pedido, pela 0x85fc50)
- notas do mês (MONTH_RATINGS) +0 tabela, +4 conta, +8 capacidade, +0xc mês, +0xd ano; ponteiro/valores; lê e grava (nota_do_jogador_no_mes, melhor_do_mes ordena)
- registro da nota (12 bytes) +0 pessoa, +4 liga, +8 soma das notas, +0xa jogos, +0xb idade; id/valores; grava (nota_do_jogador_no_mes), lê (melhor_do_mes)
- vaga da partida (0x49 bytes, 20 por lado a partir de +0x190, lados a 0x625) +0 pessoa, +0x1d marca 0xfb, +0x32 entrou (0xff não), +0x35 nota; id/valores; lê (guardar_notas_da_partida, nota_do_jogador_no_mes)
- pessoa +0x18 idade, +0x39 clube; clube +0x57 liga; valores/ponteiros; lê (nota_do_jogador_no_mes, melhor_do_mes)
- [0xae2364] conta de pessoas; valor; lê (guardar_notas_da_partida)
- prêmio do mês (MONTH_AWARD) +5 prêmio, +9 liga, +0x15 reputação nacional, +0x17 mundial, +0x19 de jogador, 12 vagas de 11 bytes (+0x1d pessoa, +0x21 clube, +0x25 mês, +0x26 ano); ponteiros/ids/valores; lê e grava as vagas (gravar_vencedor_do_mes)
- jogador (pessoa +0x61) +9 reputação nacional, +0xb atual, +0xd mundial; valores; lê e grava (reputacao_do_premiado)
- não jogador (pessoa +0x69) +8, +0xa, +0xc reputações; valores; lê e grava (reputacao_do_premiado)
- pessoa +0x1a nação, +0x39 clube; clube +0x53 nação; ids/ponteiros; lê (reputacao_do_premiado, premiar_o_melhor_do_mes)
- [[0xb67a44]] linhas de 73 bytes da diretoria por clube, +6 confiança; valor; lê e grava (reputacao_do_premiado)
- [[0xde1f54] + 0x856] gerente das listas dos clubes; ponteiro; lê (reputacao_do_premiado)
- Injury +0: registro de 31 bytes por pessoa: +0 data da última atualização (8 bytes); +8 forma; +0xa condição de jogo; +0xc cansaço; +0xe índice da lesão atual (−1 nenhuma); +0x1e propensão; valor e índice; lê e grava (fisico_do_dia)
- Injury +4: tabela de lesões de 0x30 bytes (+0x20 começo, +0x28 fim, +0x2a dias parados); +8 conta; valor; lê (fisico_do_dia)
- [0xae2d0c] registro de 79 bytes da pessoa +0xa: dias de férias (até 50); valor; lê e grava (fisico_do_dia)
- pessoa +0x24 seleção, +0x39 clube, +0x5b, +0x5e bit 3; ficha +0x34 forma natural; ponteiro e valor; lê (fisico_do_dia)
- [0xb63d08..0xb63d34] estáticas do físico: datas, guarda 0xb63d10, clube guardado 0xb63d2c e fora da temporada 0xb63d30; valor; lê e grava (fisico_do_dia)
- MANAGER_MANAGER +0: registro de 0x49 bytes por clube: +6, +8, +0xa, +0xc notas da diretoria; +0x12 confiança na liga; valor; lê (faixa_da_confianca_na_liga) e grava (gravar_notas_da_diretoria)
- Injury +0x28: lista das batidas da partida; ponteiro; solta e grava 0 (dia_das_lesoes)
- Injury +4: tabela de lesões de 0x30 bytes (+0 tipo, +0x20 começo, +0x28 fim, +0x2a dias parados, +0x2c e +0x2e contadores); +8 conta; valor; lê e grava (dia_das_lesoes, noticias_de_volta_aos_treinos)
- Injury +0: registro de 31 bytes (+0xe lesão atual, +0x12 categoria da recorrência, +0x13 peso dela, +0x1c notícia da volta dada, +0x1e propensão); índice e valor; lê, grava o +0x1c (dia_das_lesoes, noticias_de_volta_aos_treinos)
- Injury +0x10: fila das voltas (22 bytes: +0 data A, +8 data B, +0x10 id, +0x14 e +0x15), conta +0x18; valor; lê, tira do começo (noticias_de_volta_aos_treinos)
- ficha +0xf nota, +0x2d, +0x1b, +0x1d, +0x36 (gastos pela lesão); pessoa +0x3d cargo; valor; lê e grava (dia_das_lesoes)
- [0x9e7ad9] tabela de tipos de lesão (13 bytes: categoria); [0x9e7f05] faixas das categorias (3 bytes); valor; lê (dia_das_lesoes)
- competição +0x50 órgão (0xff sem), +4; Discipline +0 órgãos; ponteiro e valor; lê (um_cartao_da_suspensao)
- PLAYER_REGEN +0x10: fila dos regens jovens (12 bytes: +0 id, +4 data), +0x14 conta, +0x40 liga; id e valor; lê, tira da frente (fila_de_regens)
- PLAYER_REGEN +0x1c: reservas de técnico (+0 id, +4 nação), +0x20 conta; id; lê (fila_de_regens)
- pessoa +0x69 registro de não jogador ([0xae23c0] + 0x44 × id: +8, +0xa, +0xc notas, +0x10/+0x11); +0x3d cargo; +0x22; +0x57; ponteiro e valor; grava (fila_de_regens)
- clube +0x19f: as 5 vagas de treinador; +0xd3 auxiliar; +0x80 reputação; ponteiro e valor; lê e grava (fila_de_regens), lê a reputação (veiculo_pelo_clube)
- ficha +0xb reputação, +9, +0xd, +0xf; valor; lê (fila_de_regens, veiculo_pela_pessoa)
- liga +0xc9: a data da última atualização; valor (data de 8 bytes); grava (somar_resultado_na_liga)
- liga +0x44: a fase da liga (0xff com fases, 0xfe sem estatística); valor; lê (somar_resultado_na_liga)
- liga +0xc / +0x30: as fases e a última; ponteiro e valor; lê (somar_resultado_na_liga)
- liga +0xb1 / +0x3e: a tabela (linhas de 0x3b bytes: +0 clube, +5 confrontos, +9 forma) e quantas; ponteiro e valor; lê (as três)
- liga +0xa7: o tamanho da forma (−1 = calcular: (linhas − 1) × +0x3c); valor; lê e grava (copiar_tabela_da_liga)
- liga +8: a estatística da competição; ponteiro; lê (posicoes_previstas_da_liga); registro do clube +0x72: a posição; valor; grava (pela 0x4a2540)
- partida +0x42: a fase da partida; valor; lê (somar_resultado_na_liga)
- [0xadadfc] a tabela das competições; ponteiro; lê (somar_resultado_na_liga)
- pessoa ([0xae23bc] + 0x6e × id) +0x22: os jogos pela seleção; valor; lê e grava +1 (resultado_entre_selecoes)
- pessoa +0x23: os gols pela seleção; valor; lê e grava + os da vaga (+0x21) (resultado_entre_selecoes)
- pessoa +0x1a / +0x1e: a nação e a segunda nação; ponteiro; lê e troca (resultado_entre_selecoes)
- registro geral ([0xae2d0c] + 0x4f × id) +0xc bit 2 e +0xd; valor; grava (resultado_entre_selecoes)
- registro da nação nas seleções ([NationalTeams+4] + 0x6e × id) +2: os jogos da nação; valor; grava +1; +0x3a + 20 × tipo a data do tipo; lê; +0x44 + 20 × tipo; valor; grava 0 (resultado_entre_selecoes)
- resumo do jogo + 0x625 × lado + 0x190 + 0x49 × vaga: +0 id da pessoa, +0x21 gols, +0x32 entrou (reserva); +0x153 a vaga do destaque; valor; lê (resultado_entre_selecoes)
- competição +0x30 / +0xc / +0x43 / +0xab / +0x3c / +0xa3: as fases, o tipo, a rodada atual, as rodadas e os registros de 0x68 bytes (+7 o código); lê (rodada_com_jogos_conhecidos)
- time da seleção +0xd7: o elenco (50 vagas); ponteiro; lê e reordena pelo clube (registrar_convocados_por_clube)
- staff +0x39: o clube; ponteiro; lê; +0x5e bits 2 e 3: o elenco B/principal; valor; lê (registrar_convocados_por_clube)
- [NationalTeams+0x20] + 6 × id do clube: +0 vetor de entradas (data de 8 bytes, +8 convocados), +4 capacidade, +5 quantas; ponteiro e valor; lê e grava (registrar_convocados_por_clube)
- nação +0x11c: a nação marcada; valor; lê (registrar_convocados_por_clube)
- partida +0x28 / +0x2a: o ano e o dia; valor; lê (registrar_convocados_por_clube); +0x42: a fase; valor; lê (importancia_do_jogo_na_competicao)
- competição +0xc / +0x30: as fases e a última; ponteiro e valor; lê (importancia_do_jogo_na_competicao, fase_da_competicao)

**Interessa a:** CMRTE, CMScout, SGE, GPF2
