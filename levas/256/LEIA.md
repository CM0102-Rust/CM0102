# 256ª leva — o ranking da FIFA no mês: os jogos de uma seleção no período, a atualização do dia 18 e a ordem dos resultados por pontos; o ranking da FIFA no jogo: o resultado no histórico da seleção, a faixa de força da seleção e as pontuações do ranking; a nação-chave do jogo: a nação-chave ativa de uma competição ou jogo, se a competição é a Copa do Mundo e se é um torneio internacional; a nação-chave da competição e a ordem do ranking da FIFA: o índice da nação-chave da competição, a ordem das seleções no ranking e a comparação das pontuações; o tempo das cidades: o tempo do dia, a região do weather.cfg e o período da região; os coeficientes das nações (o bônus de dezembro e a história de cinco anos), o dia das aposentadorias e a soma presa de dois bytes; o estádio neutro e o clima inicial: a nota do estádio para o jogo, o fator da distância no público e a leitura do weather.cfg; a força dos treinadores no treino: o fator do treinador por categoria, a comissão de treino do técnico humano e a carga do programa de treino; o progresso do treino nos atributos: aplicar o progresso das cinco categorias, achar o treino de posição do jogador e se ele aprende uma posição depressa; a semana do treino, a reserva de ficha de não jogador e a lesão que volta; a pré-temporada dos amistosos: o dia dos amistosos, o plano da pré-temporada da IA e as janelas da pré-temporada; as excursões da pré-temporada: marcar os jogos da excursão e as duas ordens dos candidatos do amistoso; as posições previstas da copa: as posições pela rodada de saída, ordenar os clubes pela força e gravar a posição do clube na estatística

**Rotinas:**
- `jogos_da_selecao_no_periodo()` = 0x58d710 (fifa_rankings.cpp, thiscall, ret 0x14): conta os resultados do histórico da seleção entre dois meses (ano × 12 + mês), do fim para o começo, para a permissão de trabalho;
- `atualizar_ranking_da_fifa()` = 0x58d460 (fifa_rankings.cpp, thiscall, ret 4): no dia 18: tira os resultados de mais de um ano, ordena os históricos, gira os totais dos anos e refaz pontuações e posições;
- `ordem_dos_resultados_por_pontos()` = 0x58e220 (fifa_rankings.cpp, cdecl): ordem do qsort dos históricos: os pontos de b menos os de a (do maior para o menor);
- `resultado_no_historico()` = 0x58dea0 (fifa_rankings.cpp, thiscall, ret 0xc): acrescenta ao histórico da seleção os pontos, o mês e o ano do jogo, crescendo o vetor de 5 em 5 entradas;
- `faixa_da_selecao()` = 0x58e190 (fifa_rankings.cpp, thiscall, ret 4): a faixa de força de 1 a 4 da seleção, pela reputação e pela posição no ranking;
- `pontuacoes_do_ranking()` = 0x58dfc0 (fifa_rankings.cpp, thiscall): o total do ano pelos oito melhores resultados e a pontuação pelos totais do ano e dos cinco anteriores;
- `nacao_chave_do_jogo()` = 0x48dd70 (comp.cpp, cdecl): a chave ativa da nação-chave dona do jogo: pela nação da competição, pelas nações dos clubes do amistoso, 0 na Copa do Mundo, 1 nos torneios de seleções;
- `e_copa_do_mundo()` = 0x669a10 (key_nation.cpp, cdecl): 1 se o id da competição é a Copa do Mundo ou uma das seis eliminatórias;
- `e_torneio_internacional()` = 0x669ae0 (key_nation.cpp, cdecl): 1 se o id da competição é um dos dez torneios de seleções fora da Copa do Mundo;
- `indice_da_nacao_chave()` = 0x669470 (key_nation.cpp, cdecl): a linha da nação-chave da competição: a da nação, 0 na Copa do Mundo, 1 nos torneios de seleções, 2 nos mundiais de clubes, a do continente ou 3;
- `ordem_do_ranking()` = 0x58d610 (fifa_rankings.cpp, thiscall, ret 4): um vetor novo com as entradas do ranking das nações com seleção, ordenadas pela pontuação, e a conta;
- `ordem_das_pontuacoes()` = 0x58e240 (fifa_rankings.cpp, cdecl): ordem do qsort do ranking: a maior pontuação primeiro;
- `tempo_das_cidades()` = 0x922ee0 (weather.cpp, thiscall): cada cidade: as três medidas viram (medida + sorteio(100)) × 0,5, presas em 0..100; uma vez em dez (sorteio(10) zerado), o vento novo = sorteio(3);
- `ler_regiao_do_clima()` = 0x922c40 (weather.cpp, thiscall, ret 8): uma linha do weather.cfg com o id e o nome da região ("%ld %s") e os quatro períodos dela (0x922d20) em +0x1e, +0x31, +0x44 e +0x57;
- `ler_periodo_do_clima()` = 0x922d20 (weather.cpp, thiscall, ret 8): uma linha com o nome e 18 inteiros (o dia, cinco W, cinco P, sete H): o período recebe o dia (palavra) e os 17 bytes; arquivo nulo é a caixa "Error";
- `coeficientes_das_nacoes()` = 0x5d5f20 (game.cpp, cdecl): na primeira vez, a história de cada nação recebe o coeficiente de +0xa8; no dia do mês da tabela, a soma ponderada da história num vetor ordenado e solto; no dia de dezembro, o ano corrente ganha reputação × 0,05 × (12 − marca) × 3 (marca < 12), a história anda um ano e o ano corrente e a marca zeram;
- `dia_das_aposentadorias()` = 0x5d6250 (game.cpp, cdecl): 1 nos dias 0, 80, 173 e 250 do ano (os da busca das aposentadorias), senão 0;
- `soma_presa()` = 0x5d61d0 (game.cpp, cdecl): a soma de dois bytes com sinal presa entre o mínimo e o máximo dados;
- `nota_do_estadio_neutro()` = 0x85da00 (stadium.cpp, thiscall, ret 0x14): o estádio nacional inglês antes de 2004 vale 0; a nota parte de 100000, muda pelas distâncias entre as cidades (marca 8) ou pelos lugares contra o público esperado (marca 0x20: seleção 60000, senão a soma dos públicos dos clubes) ou vira sorteio(lugares) (marca 4), e sai × (sorteio(100) × 0,004 + 0,8);
- `fator_da_distancia()` = 0x85d8a0 (stadium.cpp, thiscall, ret 0x14): 1 − distância × k (ao quadrado sem +0x3c), dividido por (reputação + 1)² longe e com reputação; com mínimos pela base, pelo tipo 10 (0,2), pelos clubes da mesma nação (0,05) e um geral;
- `ler_o_clima()` = 0x922810 (weather.cpp, thiscall): lê o CM3_DATA\weather.cfg: quantas regiões, as regiões (0x922c40) e a tabela das cidades, com as três medidas ((sorteio(100) + sorteio(100)) × 0,5) e o vento (sorteio(3)) sorteados; erro de arquivo ou memória para o jogo;
- `fator_do_treinador()` = 0x8b41d0 (training_manager.cpp, thiscall, ret 8): o fator dos treinadores numa categoria de treino: 100 sem técnico ou com técnico do computador; no clube humano, a média das notas da comissão (dividida pelo divisor da quantidade) × 40/7, no mínimo 50;
- `comissao_do_humano()` = 0x8b4cd0 (training_manager.cpp, thiscall, ret 4): a comissão de treino (seis vagas de 9 bytes) de um dos 16 técnicos humanos; fora deles, a caixa de erro e 0;
- `carga_do_programa()` = 0x8b4f60 (training_manager.cpp, thiscall): a carga do programa de treino pelos cinco níveis das categorias, com pesos fixos, até 100;
- `aplicar_o_progresso()` = 0x8b2be0 (training_manager.cpp, thiscall, ret 8): soma o progresso de cada categoria ao meio byte guardado do jogador; a cada dois passos os atributos da categoria andam (os de 1 a 20 um, os de −125 a 125 seis); depois a posição ou o lado do treino de posição vai a 20 quando aprendida, ou volta a 1 quando o treino para;
- `achar_o_treino_de_posicao()` = 0x8b4b20 (training_manager.cpp, thiscall, ret 8): acha na lista ordenada do treino de posição o primeiro item do jogador e devolve quantos são dele (0 sem a marca de treino de posição);
- `aprende_depressa()` = 0x8b4db0 (training_manager.cpp, thiscall, ret 4): 1 quando algum treino de posição do jogador tem aprendizado acima de 50;
- `avancar_a_semana()` = 0x8b1730 (training_manager.cpp, thiscall, ret 4): no dia do ano múltiplo de 7, a semana do treino sobe 1 (o treino de cada jogador é posto em dia pelas semanas que faltam); devolve o dia / 7;
- `tirar_a_ficha_reservada()` = 0x7a6e30 (player_regen.cpp, thiscall, ret 0xc): copia o primeiro item da reserva de fichas de não jogador (id, nação, segunda nação) para as saídas e tira-o da reserva;
- `lesao_que_volta()` = 0x62ed30 (injury.cpp, thiscall, ret 4): o tipo da lesão que volta do jogador, ou 0xff sem ela ou quando a pessoa não é jogador;
- `atualizar_amistosos_do_dia()` = 0x5c2460 (friendly.cpp, thiscall, ret 8): nos dias da pré-temporada planeja os amistosos da IA e cancela torneios velhos; todo dia cria os amistosos planejados, responde os pedidos e vence torneios, pedidos e convites;
- `planejar_pre_temporada_da_ia()` = 0x5c27a0 (friendly.cpp, thiscall, ret 0): monta um candidato por clube com nação (faixa = reputação / 500), ordena pela faixa e pela nação, põe as janelas e planeja torneios, excursões e amistosos;
- `janelas_da_pre_temporada()` = 0x5c2d40 (friendly.cpp, thiscall, ret 8): para cada nação em uso, grava nos candidatos dela a janela até o início da temporada − 1 semana (18 dias antes, no mínimo 3) e dá lista de datas ao primeiro e ao último da liga;
- `marcar_jogos_da_excursao()` = 0x5c4040 (friendly.cpp, thiscall, ret 0xc): corta a janela pelo primeiro adversário, sorteia o começo (até 5 tentativas) e marca um amistoso por adversário, de 2 em 2 dias, guardando as datas na lista dos dois;
- `ordem_pela_faixa()` = 0x5c5270 (friendly.cpp, cdecl): comparador dos candidatos pela faixa (+5), a maior primeiro;
- `ordem_pela_nacao_e_faixa()` = 0x5c5290 (friendly.cpp, cdecl): comparador dos ponteiros para candidatos pela nação (+6) e, na mesma nação, pela faixa (salvo 0xff);
- `posicoes_previstas_da_copa()` = 0x5223a0 (Cup.cpp, thiscall, ret 0): ordena os clubes da copa pela força e dá a cada um a posição prevista pela rodada em que sai, na estatística da copa;
- `ordenar_clubes_pela_forca()` = 0x4c08f0 (comp_util.cpp, cdecl): ordena um vetor de clubes pela força (0x4c0720), a maior primeiro;
- `gravar_posicao_do_clube()` = 0x4a2540 (comp_stats.cpp, thiscall, ret 8): acha ou acrescenta o registro do clube na estatística da competição e grava a posição em +0x72;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (39):** 33 acusadas. `cpt1_um_jogo_a_mais` escapou: a contagem dos jogos pela seleção só pesa na permissão de trabalho de jogador de fora da UE que chega a um clube inglês, e nenhuma permissão virou até 2002-06-01 [inferência]. `cpt1_sem_o_mes_final` escapou: a mesma contagem da permissão de trabalho; sem permissão que vire até 2002-06-01 [inferência]. `cpt1_no_maximo_oito` escapou: a mesma contagem da permissão de trabalho; sem permissão que vire até 2002-06-01 [inferência]. `cpt2_ano_seguinte` escapou: o resultado no histórico da seleção (256 vezes) só alimenta a contagem da permissão de trabalho (cpt_1), que não vira nenhuma permissão até 2002-06-01 [inferência]. `cpt2_mes_seguinte` escapou: o mesmo histórico da seleção, só da contagem da permissão de trabalho [inferência]. `cpt4_sem_a_eliminatoria_europeia` escapou: com os jogos das eliminatórias europeias nas regras do continente, nenhuma regra barrou jogador de seleção até 2002-06-01 (o agente já previa a escapada) [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (bc5827d1).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 345e9799, original 30385bae (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 255ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 256`

**Campos tocados:**
- [0xb74318]+0x1c: o ranking da FIFA (FIFA_RANKINGS); ponteiro; lê (permissão de trabalho 0x8c7950)
- FIFA_RANKINGS +0: tabela das nações (0x2c bytes por id de nação); ponteiro; lê (as três)
- FIFA_RANKINGS +4: mês da virada dos totais; valor; lê (atualizar_ranking_da_fifa)
- FIFA_RANKINGS +5: data da última atualização (8 bytes); valor; grava (atualizar_ranking_da_fifa)
- entrada do ranking +4/+8/+0xa: histórico (entradas de 5 bytes: pontos +0, mês +2, ano +3), capacidade e conta; lê e grava (atualizar_ranking_da_fifa), lê (jogos_da_selecao_no_periodo)
- entrada do ranking +0x10/+0x12: posição e posição anterior; valor; grava (atualizar_ranking_da_fifa)
- entrada do ranking +0x14..+0x28: totais do ano e dos cinco anteriores; valor; grava (atualizar_ranking_da_fifa)
- [0xae2350] número de nações; valor; lê (atualizar_ranking_da_fifa)
- entrada do ranking +4/+8/+0xa: histórico, capacidade e conta; lê e grava (resultado_no_historico), lê (pontuacoes_do_ranking)
- entrada do histórico (5 bytes): pontos +0, mês +2, ano +3; valor; grava (resultado_no_historico)
- entrada do ranking +0xc: pontuação; f32; grava (pontuacoes_do_ranking)
- entrada do ranking +0x10: posição; valor; lê (faixa_da_selecao)
- entrada do ranking +0x14: total do ano; valor; grava (pontuacoes_do_ranking); +0x18..+0x28 os cinco anteriores; lê
- nação +0x8e: reputação; valor; lê (faixa_da_selecao)
- partida +0x28/+0x2a: ano e dia do ano; valor; lê (resultado_no_historico)
- [0xdd6ec8] data de referência do ranking; valor; lê (pontuacoes_do_ranking)
- tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, chave ativa +0x14; lê (nacao_chave_do_jogo)
- nação +0x11c: nação escolhida para jogar; valor; lê (nacao_chave_do_jogo)
- competição +0 e +0x5d: id e nação; lê (as três)
- partida +0x14, +0x1c, +0x20: competição, mandante e visitante; ponteiro; lê (nacao_chave_do_jogo)
- clube +0x53: nação; ponteiro; lê (nacao_chave_do_jogo)
- [0x9cf684] competição de reservas; [0x9cf76c..0x9cf7ac], [0x9cf8a0], [0x9cf964] ids das competições de seleções; id; lê
- tabela das nações-chave 0xb63d60 (35 × 72 bytes): nação +0, continente +4; lê (indice_da_nacao_chave)
- competição +0, +0x59, +0x5d: id, continente e nação; lê (indice_da_nacao_chave)
- nação +0x71 (registros de 0x122 bytes em [0xae23a8]): a seleção; ponteiro; lê (ordem_do_ranking)
- entrada do ranking +0xc: pontuação; f32; lê (ordem_das_pontuacoes)
- [0x9cf6e4] e [0x9cf7bc] Copa do Mundo de Clubes e Mundial de Clubes da FIFA; id; lê (indice_da_nacao_chave)
- Weather [0xde2f74] +4 tabela das cidades (4 bytes por cidade: +0..+2 medidas 0..100, +3 vento 0..2); ponteiro/valores; lê e grava (tempo_das_cidades)
- [0xae2354] quantas cidades; valor; lê (tempo_das_cidades)
- região do clima (0x6e bytes) +0 id, +4 nome, +0x1e/+0x31/+0x44/+0x57 os quatro períodos; valores; grava (ler_regiao_do_clima)
- período do clima (0x13 bytes) +0 dia (palavra), +2..+6 W, +7..+0xb P, +0xc..+0x12 H; valores; grava (ler_periodo_do_clima)
- nação (0x122 bytes em [0xae23a8], conta [0xae2350]) +0xa8 coeficiente; double; lê (coeficientes_das_nacoes)
- nação +0xb0..+0xd0 história de cinco anos, +0xd8 ano corrente; doubles; lê e grava (coeficientes_das_nacoes)
- nação +0x8e reputação; valor; lê (coeficientes_das_nacoes)
- nação +0x121 marca do bônus de dezembro (< 12 ganha); valor; lê e zera (coeficientes_das_nacoes)
- [0x9cd200] dia do mês da atualização (12 palavras), [0x9cd216] o de dezembro, [0x9cd218] história por iniciar; valores; lê, zera [0x9cd218] (coeficientes_das_nacoes)
- estádio (0x4e bytes em [0xae23b0]) +0x38 cidade, +0x3c lugares; ponteiro/valor; lê (nota_do_estadio_neutro)
- clube +0x69 estádio, +0x73 público médio; ponteiro/valor; lê (nota_do_estadio_neutro)
- cidade +0x23/+0x2b coordenadas; doubles; lê (nota_do_estadio_neutro, fator_da_distancia)
- nação +0x80 estádio nacional; ponteiro; lê na Inglaterra [0x9cf2e4] (nota_do_estadio_neutro)
- jogo +0x1c/+0x20 clubes, +0x30 marcas (0xa0), +0x3a tipo, +0x3c marca; ponteiros/valores; lê (nota_do_estadio_neutro, fator_da_distancia)
- nação do clube (+0x53) +0x7e reputação; valor; lê (fator_da_distancia)
- Weather [0xde2f74] +0 regiões (0x6e bytes), +4 tabela das cidades, +8 quantas regiões; ponteiros/valor; grava (ler_o_clima)
- clube +0xcf: técnico; ponteiro; lê (fator_do_treinador)
- pessoa +0: id (os 16 últimos de [0xae2364] são os técnicos humanos); id; lê (fator_do_treinador, comissao_do_humano)
- TRAINING_MANAGER +0x652d + 54 × humano: comissão de treino, seis vagas de 9 bytes (+0 id do treinador, −1 vazia; +4 + categoria: nota de 1 a 20); id e valor; lê (fator_do_treinador, comissao_do_humano)
- TRAINING_SCHEDULE +4..+8: níveis das cinco categorias (0, 10, 25, 50); valor; lê (carga_do_programa)
- pessoa +0 e +0x61: id e ficha de jogador; id e ponteiro; lê (aplicar_o_progresso, achar_o_treino_de_posicao)
- ficha +0x1b, +0x1d, +0x22, +0x29, +0x2e, +0x34, +0x36, +0x3c, +0x3d, +0x3f, +0x40: atributos de 1 a 20; valor; lê e grava (aplicar_o_progresso)
- ficha +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x35, +0x37, +0x38, +0x39, +0x3a, +0x3e, +0x41: atributos de −125 a 125; valor; lê e grava (aplicar_o_progresso)
- ficha +0xf..+0x15 e +0x17..+0x19: posições e lados (1 a 20); valor; lê e grava 20 ou 1 (aplicar_o_progresso)
- TRAINING_MANAGER +0x651d: entradas de treino de 6 bytes por ficha (+3 e +4 meio bytes do progresso das categorias 0, 1, 2 e 3; +5 baixo: categoria 4; +5 bit 0x10: tem treino de posição); valor; lê e grava (aplicar_o_progresso), lê (achar_o_treino_de_posicao)
- TRAINING_MANAGER +0x6521 lista do treino de posição (itens de 8 bytes: +0 id da pessoa, +4 máscara da posição ou lado, +6 estado, +7 aprendizado), +0x6525 conta; ponteiro e valor; lê (as três)
- [0xae2c90] data do jogo (dia do ano, 16 bits); valor; lê (avancar_a_semana)
- TRAINING_MANAGER +0x688d: semana do treino (16 bits); valor; lê e grava (avancar_a_semana)
- PLAYER_REGEN +0x1c: reserva de fichas de não jogador (itens de 12 bytes: id, nação, segunda nação); id; lê e tira o primeiro (tirar_a_ficha_reservada)
- Injury +0 fichas de forma (31 bytes por pessoa; +0x12 tipo da lesão que volta, 0xff sem ela); valor; lê (lesao_que_volta)
- pessoa +0 e +0x61: id e ficha de jogador; id e ponteiro; lê (lesao_que_volta)
- Friendly +0xa7: o início da temporada (data); +0xb1 os 127 torneios (0x69 bytes: +0 em uso, +4 a data, +0x65 a lista); +0x34c8 os pedidos (+0x10/+0x14 os clubes, +0 a data); +0x34cc os convites (10 de 0x19 bytes, a data em +9, a marca em +0x15); +0x34d0/+0x34d4 as respostas; +0x34e8/+0x34ea os dias da pré-temporada; valor e ponteiro; lê e grava (atualizar_amistosos_do_dia)
- candidato do amistoso (0x16 bytes) +0: o id do clube; +5 a faixa; +6 o id da nação; +0xa/+0x12 as listas; +0xe/+0x10 a janela; valor e ponteiro; grava (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)
- clube +0x53: a nação; +0x80 a reputação (palavra); ponteiro e valor; lê (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)
- [0xb63d60] as 35 temporadas das nações (0x48 bytes: +0 a nação, +0x26 a liga, +0x2a o início); [0xdd6ec8] o início do jogo; valor e ponteiro; lê
- candidato do amistoso (0x16 bytes) +0: o id do clube; +5 a faixa (byte com sinal; 0xff = qualquer); +6 o id da nação; +0xe/+0x10 a janela (palavras com sinal); +0x12 a lista das datas (8 bytes cada); valor e ponteiro; lê e grava a lista (marcar_jogos_da_excursao, ordem_pela_faixa, ordem_pela_nacao_e_faixa)
- Friendly +0xa7: o início da temporada (data); valor; lê (marcar_jogos_da_excursao)
- Cup +8: a estatística (Comp_Stats); +0x3c a conta das rodadas (palavra); +0x3e a conta dos clubes (palavra); +0xa3 as rodadas (0x68 bytes, +0x1a quantos saem nela); +0xa7 os clubes (6 bytes, +0 o clube); ponteiro e valor; lê (posicoes_previstas_da_copa)
- registro do clube na estatística da competição +0x72: a posição prevista; valor (byte); grava (gravar_posicao_do_clube)

**Interessa a:** CMRTE, GPF2, CMScout, SGE
