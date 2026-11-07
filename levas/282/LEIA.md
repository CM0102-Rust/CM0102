# 282ª leva — os turnos do Carioca, do Mineiro, do Paranaense e da J1: a fase de pontos corridos de cada turno; a virada da temporada do Carioca, do Mineiro e do Paranaense; a NSL australiana: a fase seguinte, a série final dos seis primeiros e a grande final; a MLS: a fase seguinte, o play-off dos oito e a final da MLS Cup; a virada das copas: a Copa do Brasil, a Copa do Imperador e a Copa da Liga japonesa; a virada da Supercopa japonesa, da Copa dos EUA e do Jogo das Estrelas da MLS, e os dois times das estrelas; a Libertadores: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos; a Mercosul: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos; a Merconorte: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos; a Copa América: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros dois grupos; as três divisões da MLS: a tabela da primeira e as fases da segunda e da terceira; a MLS: a virada da temporada, as posições previstas e a lista dos doze clubes; a Série C: os sete grupos da primeira fase, a segunda fase dos dezesseis e os dois grupos da terceira

**Rotinas:**
- `cqr1_turno_do_carioca()` = 0x43afa0 (bra_reg_rio.cpp, thiscall, ret 4): Um turno do Carioca: quando o primeiro jogo de um turno é processado, monta a fase de pontos corridos dele (uma liga de pontos corridos com todos os doze clubes da tabela, os mesmos pontos por vitória e por empate e os mesmos critérios de desempate da liga, e uma vaga de cima, a do campeão do turno); o primeiro turno vai para a fase 0 e o segundo (a partir da 12ª rodada, pelo +0x34 do jogo) para a fase 1; o campeão de cada turno (o primeiro da tabela da fase) vai à final;
- `cqr1_turno_do_mineiro()` = 0x4351b0 (bra_reg_minas_gerais.cpp, thiscall, ret 4): Um turno do Mineiro: quando o primeiro jogo de um turno é processado, monta a fase de pontos corridos dele (uma liga de pontos corridos com todos os doze clubes da tabela, os mesmos pontos por vitória e por empate e os mesmos critérios de desempate da liga, e uma vaga de cima, a do campeão do turno); o primeiro turno vai para a fase 0 e o segundo (a partir da 12ª rodada, pelo +0x34 do jogo) para a fase 1; o campeão de cada turno (o primeiro da tabela da fase) vai à final;
- `cqr1_turno_do_paranaense()` = 0x438760 (bra_reg_parana.cpp, thiscall, ret 4): Um turno do Paranaense: quando o primeiro jogo de um turno é processado, monta a fase de pontos corridos dele (uma liga de pontos corridos com todos os doze clubes da tabela, os mesmos pontos por vitória e por empate e os mesmos critérios de desempate da liga, e uma vaga de cima, a do campeão do turno); o primeiro turno vai para a fase 0 e o segundo (a partir da 12ª rodada, pelo +0x34 do jogo) para a fase 1; o campeão de cada turno (o primeiro da tabela da fase) vai à final;
- `cqr1_turno_da_j1()` = 0x663c20 (jap_j1.cpp, thiscall, ret 4): Um turno da J1: quando o primeiro jogo de um turno é processado, monta a fase de pontos corridos dele (uma liga de pontos corridos com todos os [inferência: dezesseis] clubes da tabela, os mesmos pontos por vitória e por empate e os mesmos critérios de desempate da liga, e uma vaga de cima, a do campeão do turno); o primeiro turno vai para a fase 0 e o segundo (a partir da 16ª rodada, pelo +0x34 do jogo) para a fase 1; o campeão de cada turno (o primeiro da tabela da fase) vai à final;
- `cqr2_virada_do_carioca()` = 0x43a6a0 (bra_reg_rio.cpp, thiscall, ret 0): A virada do Carioca: guarda a temporada nas estatísticas, apaga a tabela, o calendário e as fases (os dois turnos e a final) e prepara três vagas novas de fase; passa ao ano seguinte e refaz as regras, a tabela com os doze clubes do estadual, os jogos e a ordem; depois as posições previstas e a renovação dos árbitros. Os turnos só são montados no primeiro jogo de cada um (cqr_1);
- `cqr2_virada_do_mineiro()` = 0x434910 (bra_reg_minas_gerais.cpp, thiscall, ret 0): A virada do Mineiro: guarda a temporada nas estatísticas, apaga a tabela, o calendário e as fases (os dois turnos e a final) e prepara três vagas novas de fase; passa ao ano seguinte e refaz as regras, a tabela com os doze clubes do estadual, os jogos e a ordem; depois as posições previstas e a renovação dos árbitros. Os turnos só são montados no primeiro jogo de cada um (cqr_1);
- `cqr2_virada_do_paranaense()` = 0x438060 (bra_reg_parana.cpp, thiscall, ret 0): A virada do Paranaense: guarda a temporada nas estatísticas, apaga a tabela, o calendário e as fases (os dois turnos e a final) e prepara três vagas novas de fase; passa ao ano seguinte e refaz as regras, a tabela com os doze clubes do estadual, os jogos e a ordem; depois as posições previstas e a renovação dos árbitros. Os turnos só são montados no primeiro jogo de cada um (cqr_1);
- `cqr3_fase_seguinte_da_nsl()` = 0x411f70 (aus_nsl.cpp, thiscall, ret 0): A NSL tem a temporada regular (pontos corridos), a série final (fase 0) e a grande final (fase 1): a fase seguinte sobe a fase atual e monta a série final no fim da temporada regular e a grande final no fim da série final; depois da última fase é erro (a caixa);
- `cqr3_serie_final_da_nsl()` = 0x412010 (aus_nsl.cpp, thiscall, ret 0): A série final da NSL: os seis primeiros da temporada regular jogam uma fase de pontos corridos própria (com os pontos e os critérios de desempate da liga) e os dois primeiros dela vão à grande final; manda a notícia do sorteio. [inferência] Na NSL real era um mata-mata com dupla chance; aqui é uma liga curta dos seis;
- `cqr3_grande_final_da_nsl()` = 0x4121c0 (aus_nsl.cpp, thiscall, ret 0): A grande final da NSL: o primeiro contra o segundo da série final, numa fase de copa de dois clubes [inferência: jogo único, o primeiro como mandante]; o vencedor é o campeão australiano;
- `cqr4_fase_seguinte_da_mls()` = 0x909400 (usa_mls.cpp, thiscall, ret 0): A MLS tem três divisões de quatro (a liga e as fases 0 e 1, que correm juntas na temporada regular), o play-off (fase 2) e a final (fase 3): a fase seguinte vai da fase 1 ao play-off e do play-off à final, e monta cada um; depois da última fase é erro (a caixa);
- `cqr4_play_off_da_mls()` = 0x9094c0 (usa_mls.cpp, thiscall, ret 0): O play-off da MLS: oito clubes. Os três campeões de divisão, ordenados pelos pontos, e os cinco melhores dos outros (do segundo ao quarto de cada divisão, pelos pontos; empate sem critério próprio). Os pares: o melhor campeão contra o quinto melhor dos outros, o segundo campeão contra o quarto, o terceiro campeão contra o terceiro e o melhor dos outros contra o segundo [inferência: os pares seguem a lista]; os vencedores vão à final;
- `cqr4_final_da_mls()` = 0x9097d0 (usa_mls.cpp, thiscall, ret 0): A final da MLS Cup: os dois vencedores do play-off (os dois primeiros com a marca de vencedor, na ordem da lista do play-off) numa fase de copa de dois [inferência: jogo único]; o vencedor é o campeão da MLS;
- `cqr5_virada_copa_do_brasil()` = 0x427d10 (bra_cup.cpp, thiscall, ret 0): regra: na virada da temporada a Copa do Brasil é refeita do zero: as tabelas da edição anterior são liberadas, o histórico da temporada é arquivado (Comp_Stats), o ano sobe um e a copa monta de novo os seus inscritos (a tabela dos times), as rodadas e a árvore dos confrontos, e as posições previstas;
- `cqr5_virada_copa_do_imperador()` = 0x662950 (jap_emp_cup.cpp, thiscall, ret 0): regra: a mesma virada na Copa do Imperador (Japão): as tabelas liberadas, o histórico arquivado, o ano +1, os inscritos (JapEmpCup::collect_entrants), as rodadas, a árvore e as posições previstas;
- `cqr5_virada_copa_da_liga_japonesa()` = 0x665bc0 (jap_j_cup.cpp, thiscall, ret 0): regra: a mesma virada na Copa da Liga japonesa (J-League Cup): as tabelas liberadas, o histórico arquivado, o ano +1, os 28 inscritos (JapJCup::collect_entrants), as rodadas, a árvore e as posições previstas;
- `cqr6_virada_supercopa_japonesa()` = 0x6664e0 (jap_super.cpp, thiscall, ret 0): regra: na virada da temporada a Supercopa japonesa é refeita: as tabelas da edição anterior liberadas, o histórico arquivado, o ano +1, os dois clubes (JapSuper::select_teams: o campeão da J1 e o da Copa do Imperador [inferência]), as rodadas, a árvore e as posições previstas;
- `cqr6_virada_copa_dos_eua()` = 0x90b790 (usa_open_cup.cpp, thiscall, ret 0): regra: a mesma virada na Copa dos EUA (US Open Cup): as tabelas liberadas, o histórico arquivado, o ano +1, os inscritos (UsaOpenCup::collect_entrants), as rodadas, a árvore e as posições previstas;
- `cqr6_virada_jogo_das_estrelas()` = 0x90a760 (usa_mls_all_stars.cpp, thiscall, ret 0): regra: a virada do Jogo das Estrelas da MLS: as tabelas liberadas, o histórico arquivado, o ano +1, os dois times das estrelas, as rodadas e a árvore (sem posições previstas);
- `cqr6_times_do_jogo_das_estrelas()` = 0x90a540 (usa_mls_all_stars.cpp, thiscall, ret 0): regra: o Jogo das Estrelas da MLS opõe as duas seleções das conferências ([0x9d00a4] e [0x9d00a8], clubes da tabela); no ano ímpar a de [0x9d00a4] manda o jogo, no par a de [0x9d00a8]; o estádio é o de um clube da MLS sorteado (1 em 6, pela ordem da tabela) entre os da conferência do mandante (0x413 no ano ímpar, 0x414 no par); os dois times ficam com a nação dos EUA [inferência];
- `cqr10_libertadores_dia()` = 0x4c4410 (conmebol_liber.cpp, thiscall, ret 8): regra: a Libertadores tem 32 clubes (os que as regras sul-americanas põem nela, embaralhados) em 8 grupos de 4, em dois turnos (3 pontos a vitória, 1 o empate; dois classificados por grupo, para o mata-mata de 16); o sorteio é no dia do começo (15 de janeiro do ano da copa): o grupo 1 fica na tabela da própria copa (os sorteados 0 a 3, na ordem) e os grupos 2 a 8 são fases (os sorteados 4g + 4 a 4g + 7); no dia do sorteio: os clubes (0x4c3f40 (os 32 clubes com a Libertadores como competição da temporada, embaralhados)), a tabela do grupo 1, os jogos, a tabela ordenada, as fases dos outros grupos e as posições previstas; depois o dia de liga;
- `cqr10_libertadores_primeiro_grupo()` = 0x4c37a0 (conmebol_liber.cpp, thiscall, ret 0): regra: o grupo 1 da tem 4 clubes, os 4 primeiros sorteados, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo Libertadores;
- `cqr10_libertadores_grupo()` = 0x4c3b10 (conmebol_liber.cpp, thiscall, ret 4): regra: cada um dos outros grupos da é uma fase de liga com os 4 sorteados seguintes (4g + 4 a 4g + 7), as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia Libertadores;
- `cqr11_mercosul_dia()` = 0x4c5c90 (conmebol_merc.cpp, thiscall, ret 8): regra: a Mercosul tem 20 convidados (os clubes que as regras sul-americanas convidam, embaralhados) em 5 grupos de 4 [inferência: seis rodadas às terças, de 29 de julho a 14 de outubro]; a temporada vira em 1º de junho; no dia do sorteio o grupo 1 fica na tabela da própria copa (os sorteados 0 a 3) e os grupos 2 a 5 são fases (4g + 4 a 4g + 7); os 5 primeiros e os 3 melhores segundos vão às quartas; no dia do sorteio: os clubes (0x4c5850 (os 20 convidados, embaralhados)), a tabela do grupo 1, os jogos, a tabela ordenada, as fases dos outros grupos e as posições previstas; depois o dia de liga;
- `cqr11_mercosul_primeiro_grupo()` = 0x4c4e60 (conmebol_merc.cpp, thiscall, ret 0): regra: o grupo 1 da tem 4 clubes, os 4 primeiros sorteados, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo Mercosul;
- `cqr11_mercosul_grupo()` = 0x4c51d0 (conmebol_merc.cpp, thiscall, ret 4): regra: cada um dos outros grupos da é uma fase de liga com os 4 sorteados seguintes (4g + 4 a 4g + 7), as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia Mercosul;
- `cqr12_merconorte_dia()` = 0x4c2c90 (con_merc_cup.cpp, thiscall, ret 8): regra: a Merconorte tem 16 convidados (os melhores de quatro nações e dois clubes da liga americana [inferência], embaralhados) em 4 grupos de 4, em dois turnos (3/1 pontos, um classificado por grupo, para as semifinais); começa em 1º de agosto; no dia do sorteio o grupo 1 fica na tabela da própria copa (os sorteados 0 a 3) e os grupos 2 a 4 são fases (4g + 4 a 4g + 7, sempre 4 times); no dia do sorteio: os clubes (0x4c2560 (os 16 convidados, embaralhados)), a tabela do grupo 1, os jogos, a tabela ordenada, as fases dos outros grupos e as posições previstas; depois o dia de liga;
- `cqr12_merconorte_primeiro_grupo()` = 0x4c1de0 (con_merc_cup.cpp, thiscall, ret 0): regra: o grupo 1 da tem 4 clubes, os 4 primeiros sorteados, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo Merconorte;
- `cqr12_merconorte_grupo()` = 0x4c2140 (con_merc_cup.cpp, thiscall, ret 4): regra: cada um dos outros grupos da é uma fase de liga com os 4 sorteados seguintes (4g + 4 a 4g + 7), as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia Merconorte;
- `cqr13_copa_america_dia()` = 0x5e2120 (goldcup.cpp, thiscall, ret 8): regra: a Copa América tem 12 seleções em 3 grupos de 4 (turno único, 3/1 pontos, dois classificados por grupo e os dois melhores terceiros para as quartas); o sorteio é no dia do começo (20 de março do ano da copa, levado ao dia da semana 5) e só na chamada sem `completo`: a lista das 12, as posições previstas, o grupo A na tabela da própria copa, os jogos e os grupos B e C como fases; antes desse dia a copa nem passa o dia de liga;
- `cqr13_copa_america_primeiro_grupo()` = 0x5e11e0 (goldcup.cpp, thiscall, ret 0): regra: o grupo A da Copa América tem 4 seleções, os sorteados 0, 3, 6 e 9 (os grupos são intercalados na lista), cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo;
- `cqr13_copa_america_grupo()` = 0x5e16b0 (goldcup.cpp, thiscall, ret 4): regra: os grupos B e C da Copa América são fases de liga com os sorteados g + 1, g + 4, g + 7 e g + 10, as regras e o ano da copa e as rodadas da tabela da copa para o grupo (6, 10 e 13 de julho); os técnicos humanos recebem a notícia;
- `cqs1_tabela_da_mls()` = 0x908560 (usa_mls.cpp, thiscall, ret 0): A tabela da MLS é a da primeira divisão: os quatro clubes da competição com o código de divisão 0x413, na ordem do banco; dá certo se achou os quatro;
- `cqs1_segunda_divisao_da_mls()` = 0x908fc0 (usa_mls.cpp, thiscall, ret 0): A segunda divisão da MLS (código 0x47b): uma fase de pontos corridos com os quatro clubes dela (na ordem do banco), os mesmos turnos, pontos, critérios de desempate e vagas da liga, na fase 0; os três campeões de divisão e os cinco melhores dos outros vão ao play-off;
- `cqs1_terceira_divisao_da_mls()` = 0x9091e0 (usa_mls.cpp, thiscall, ret 0): A terceira divisão da MLS (código 0x414): o mesmo molde da segunda, na fase 1; a fase atual fica 1, de onde a fase seguinte passa ao play-off;
- `cqs2_virada_da_mls()` = 0x908e00 (usa_mls.cpp, thiscall, ret 0): A virada da MLS: apaga a tabela, o calendário e as fases (as outras duas divisões, o play-off e a final), guarda a temporada nas estatísticas, passa ao ano seguinte e refaz as regras, a primeira divisão, os jogos da temporada (com as fases das outras duas divisões) e a ordem; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros;
- `cqs2_posicoes_previstas_da_mls()` = 0x909bb0 (usa_mls.cpp, thiscall, ret 0): As posições previstas da MLS: em cada divisão os quatro clubes são ordenados pela reputação e ganham as posições 1, 4, 7 e 10 (o favorito de cada divisão é previsto em 1º, o segundo em 4º, o terceiro em 7º e o último em 10º), uma escala comum às três divisões;
- `cqs2_clubes_da_mls()` = 0x909a40 (usa_mls.cpp, thiscall, ret 8): A lista dos doze clubes da MLS: os quatro da primeira divisão, depois os quatro da segunda e os quatro da terceira, na ordem das tabelas [inferência: usada pelo jogo das estrelas da MLS];
- `cqs3_grupo_da_serie_c()` = 0x42ee80 (bra_nat_third.cpp, thiscall, ret 4): Um grupo da primeira fase da Série C: sete clubes da lista dos 56 (as sete primeiras vagas são o grupo da própria liga; o grupo g leva as vagas 7(g+1) a 7(g+1)+6), em pontos corridos de ida e volta, com os pontos, os critérios de desempate e as vagas da liga; os dois primeiros de cada um dos oito grupos vão à segunda fase;
- `cqs3_segunda_fase_da_serie_c()` = 0x42e430 (bra_nat_third.cpp, thiscall, ret 0): A segunda fase da Série C: os dezesseis (os dois primeiros de cada grupo) semeados pelos pontos da primeira fase (no empate, o que vem antes: a liga, depois os grupos 0 a 6, o primeiro antes do segundo), num mata-mata em que o melhor enfrenta o 16º, o 2º o 15º e assim por diante [inferência: os pares seguem a lista];
- `cqs3_terceira_fase_da_serie_c()` = 0x42e700 (bra_nat_third.cpp, thiscall, ret 0): A terceira fase da Série C: os oito vencedores da segunda fase, sorteados (embaralhados) em dois grupos de quatro, de ida e volta, com 3 pontos por vitória, 1 por empate, os critérios de desempate {1, 2, 4} e duas vagas de cima em cada grupo [inferência: os dois primeiros de cada grupo sobem à Série B e os campeões dos grupos fazem a final];

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (39):** 39 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original a6f0e662 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 281ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 282`

**Campos tocados:**
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê
- liga +0x3e: a quantidade de clubes; valor; lê
- liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; lê
- liga +0xc4: a categoria dos jogos; valor; lê
- liga +0xc5: os critérios de desempate; ponteiro (o endereço); lê
- liga +0x40: o ano da temporada; valor; lê
- liga +4: o registro da competição no banco; ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a vaga k
- liga +0x30: a fase atual; valor; grava k
- liga +0x4c: [inferência] marca da temporada; valor; grava 0
- liga +0xba: o calendário (rodadas de 0x41 bytes); ponteiro; libera e zera
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; destrói as fases, libera e grava um vetor novo de 3
- liga +0x2c: a quantidade de fases; valor; grava 3
- liga +0x30: a fase atual; valor; lê e grava −1
- liga +0x40: o ano da temporada; valor; lê e grava + 1
- liga +4: o registro da competição no banco; ponteiro; lê (árbitros)
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- liga +0x30 / +0x2c: a fase atual / a quantidade de fases; valor; lê e grava (fase seguinte, série final)
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê os seis primeiros
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava as vagas 0 e 1, lê a tabela da fase 0
- liga +0xc2 / +0xc3 / +0xc4 / +0xc5: pontos por vitória e empate, categoria dos jogos, critérios de desempate; valor; lê
- liga +0x40 / +0x51 / +0xd9 / +4: o ano, a importância, os bits de regra, o registro da competição; valor; lê
- fase de copa +0xa7 / +0x3e: os times (6 bytes) / quantos; lê (comparador)
- liga +0x30 / +0x2c: a fase atual / a quantidade de fases; valor; lê e grava (fase seguinte)
- liga +0xb1: a tabela (linhas de 0x3b bytes: +0 o clube, +0x19 os pontos); ponteiro; lê as linhas 0 a 3 da liga e das fases 0 e 1
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê as vagas 0, 1 e 2, grava a 2 e a 3
- fase de copa +0xa7: os times (registros de 6 bytes: +0 o clube, +5 a marca de vencedor); ponteiro; lê (final)
- fase de copa +0x3e: a quantidade de times; valor; lê
- liga +0x40 / +4: o ano / o registro da competição; valor; lê
- copa +0x40 ano (word); valor; lê e grava +1 (virada)
- copa +0x4c, +0xab, +0xb1 (byte); valor; grava 0 (virada)
- copa +0xa7 inscritos, +0xa3 rodadas, +0xad confrontos por rodada (+0x3c rodadas); ponteiro; libera e grava 0 (virada)
- copa +8 estatísticas da competição (Comp_Stats); ponteiro; lê (virada)
- copa +0x40 ano (word); valor; lê e grava +1 (viradas), lê (times das estrelas: o mandante pelo ano par ou ímpar)
- copa +0x4c, +0xab, +0xb1 (byte); valor; grava 0 (viradas)
- copa +0xa7 inscritos (6 bytes: clube, +4, +5), +0xa3 rodadas, +0xad confrontos por rodada; ponteiro; libera e grava (viradas, times das estrelas)
- [0x9d00a4], [0x9d00a8] os dois clubes das estrelas; índice na tabela dos clubes; lê
- clube +0x57 divisão (competição); ponteiro; lê. clube +0x69 estádio; ponteiro; lê e grava (o mandante das estrelas); clube +0x6d byte; grava 0; clube +0x53 nação; ponteiro; grava
- [0x9cf590] a MLS; id; lê. [0x9cf4f8] os EUA [inferência]; índice na tabela das nações [0xae23a8] (0x122 bytes); lê
- copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)
- copa +0xee lista dos sorteados (ponteiros de clube); ponteiro; lê (tabela e fases) e libera e zera (dia)
- copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)
- copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)
- copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)
- copa +0xee lista dos sorteados (ponteiros de clube); ponteiro; lê (tabela e fases) e libera e zera (dia)
- copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)
- copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)
- copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)
- copa +0xee lista dos sorteados (ponteiros de clube); ponteiro; lê (tabela e fases) e libera e zera (dia)
- copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)
- copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)
- copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)
- copa +0xee lista dos sorteados (ponteiros de clube da seleção), +0xf2 a conta; ponteiro/valor; lê (tabela e fases), libera e zera (dia)
- copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (tabela)
- copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)
- clube +0x57: a competição do clube (o registro); ponteiro; lê
- [0xae23b4] / [0xae235c]: os clubes do banco (0x245 bytes) / quantos; lê
- liga +0x3e: a quantidade de clubes; valor; grava 4 (tabela), lê
- liga +0xb1: a tabela; ponteiro; grava (tabela)
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava as vagas 0 e 1
- liga +0x30: a fase atual; valor; grava 0 e 1
- liga +0x3c, +0xa7, +0xab, +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: turnos, jogos por time, vagas, pontos, categoria, desempate, bits de regra, ano, importância, registro; valor; lê
- fase +0x3a: o código da fase principal; valor; grava o código da divisão
- liga +0x4c: [inferência] marca da temporada; valor; grava 0
- liga +0xba: o calendário; ponteiro; libera e zera
- liga +0xc: as fases (vetor de ponteiros); ponteiro; destrói as fases (virada), lê as vagas 0 e 1
- liga +0x30: a fase atual; valor; lê e grava −1
- liga +0x40: o ano da temporada; valor; lê e grava + 1
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê
- estatística, registro do clube +0x72: a posição prevista; valor; grava (posições previstas)
- liga +0xb1 / +0x3e: a tabela / quantos clubes; lê
- clube +reputação: lida pela 0x4b07d0 (qsort); lê
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- liga +0xee: a lista dos 56 clubes da Série C (só no construtor e na virada); ponteiro; lê
- liga +0xb1: a tabela (linhas de 0x3b bytes: +0 o clube, +0x19 os pontos); ponteiro; lê as duas primeiras linhas da liga e dos grupos
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava as vagas 0 a 9
- liga +0x30: a fase atual; valor; grava g, 7 e 9
- fase de copa +0xa7: os times (registros de 6 bytes: +0 o clube, +5 a marca de vencedor); ponteiro; lê (terceira fase)
- liga +0xbe, +0xc2..+0xc5, +0xd9, +0x40, +0x51, +4: vagas, pontos, categoria, desempate, bits de regra, ano, importância, registro; valor; lê

**Interessa a:** CMRTE
