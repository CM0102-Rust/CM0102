# 284ª leva — a Liga dos Campeões asiática: a fase seguinte, os dois grupos e as semifinais; a Copa Africana de Nações: o dia do sorteio, o sorteio dos 16 e as fases dos grupos B a D; os mata-matas da Copa Africana de Nações e da Recopa asiática: as quartas, a fase seguinte e as finais; a Copa da Itália e a da Série C: a tabela do grupo 1 e a fase seguinte; a Copa da Itália e a da Série C: a fase de cada grupo; a Copa da Itália e a da Série C: o mata-mata depois dos grupos; as eliminatórias da Eurocopa: as regras e a data do sorteio, o tamanho dos grupos, a situação de uma seleção e o grupo 1; a Copa da UEFA: os inscritos; o dia da Copa do Rei e o histórico da temporada da Liga dos Campeões, da Copa da Grécia, da Copa da Liga irlandesa e da Copa da Série C

**Rotinas:**
- `cqt12_asia_fase_seguinte()` = 0x40c070 (asia_club_champ.cpp, thiscall, ret 0): regra: a Liga dos Campeões asiática avança uma fase quando a atual acaba; depois dos mata-matas das duas zonas (ao chegar a 1) monta os dois grupos; depois dos grupos (ao chegar a 3) as semifinais; passar da última fase é erro;
- `cqt12_asia_grupos()` = 0x40c110 (asia_club_champ.cpp, thiscall, ret 0): regra: os 4 vivos do mata-mata da primeira zona formam o grupo 1 e os 4 da segunda zona o grupo 2 (fases de liga 1 e 2, turno único [inferência], as regras e o ano da copa); os técnicos humanos recebem a notícia;
- `cqt12_asia_semifinais()` = 0x40c3d0 (asia_club_champ.cpp, thiscall, ret 0): regra: as semifinais da Liga dos Campeões asiática cruzam os grupos: 1º do grupo 1 × 2º do grupo 2 e 1º do grupo 2 × 2º do grupo 1, numa fase de copa (a fase 3) com a primeira rodada sorteada na hora;
- `cqs13_africana_dia()` = 0x4025a0 (african_nations.cpp, thiscall, ret 8): regra: a Copa Africana é sorteada no dia do começo (+0xb6/+0xb8): o sorteio dos 16, as posições previstas, o grupo A na tabela da própria copa, os jogos e os grupos B a D como fases; antes desse dia a copa nem passa o dia de liga;
- `cqs13_africana_sorteio()` = 0x4022c0 (african_nations.cpp, thiscall, ret 0): regra: a Copa Africana tem 16 seleções: as sedes na frente e as africanas (até 50, na ordem das nações, sem as sedes) ordenadas pela reputação; os potes são os blocos de 4 da lista (as sedes e as melhores no pote 1); cada um dos 4 grupos recebe uma seleção sorteada de cada pote; menos de 16 africanas, não há sorteio;
- `cqs13_africana_grupo()` = 0x401ea0 (african_nations.cpp, thiscall, ret 4): regra: os grupos B a D da Copa Africana são fases de liga com as 4 seleções sorteadas para eles, as regras e o ano da copa e as rodadas da tabela da copa para o grupo; os técnicos humanos recebem a notícia;
- `cqt13_africana_quartas()` = 0x402090 (african_nations.cpp, thiscall, ret 0): regra: as quartas da Copa Africana cruzam os grupos: 1º A × 2º B, 1º C × 2º D, 2º C × 1º D e 1º B × 2º A, numa fase de copa (a fase 3) com a primeira rodada montada na hora;
- `cqt13_recopa_asiatica_fase_seguinte()` = 0x40dfa0 (asia_cup_winner.cpp, thiscall, ret 0): regra: a Recopa asiática avança uma fase quando a atual acaba; depois dos mata-matas das duas zonas (ao chegar a 1) monta as finais; passar da última fase é erro;
- `cqt13_recopa_asiatica_finais()` = 0x40e030 (asia_cup_winner.cpp, thiscall, ret 0): regra: as finais da Recopa asiática têm os 4 vivos dos mata-matas das duas zonas, com o par do meio cruzado (os dois de cada zona não se enfrentam na semifinal), numa fase de copa (a fase 1) com a primeira rodada sorteada na hora;
- `cqr7_copa_da_italia_grupo_um()` = 0x63f250 (ita_cup.cpp, thiscall, ret 0): regra: a Copa da Itália começa por grupos de 4; o grupo 1 é a própria copa (os outros 7 são fases à parte): os 4 primeiros da lista dos inscritos vão às linhas da tabela, saem da lista e cada humano recebe a notícia do grupo;
- `cqr7_copa_da_serie_c_grupo_um()` = 0x63d5e0 (ita_c_cup.cpp, thiscall, ret 0): regra: a Copa da Itália da Série C começa por grupos de 5 (16 grupos); o grupo 1 é a própria copa: os 5 primeiros inscritos vão às linhas da tabela e saem da lista;
- `cqr7_copa_da_italia_fase_seguinte()` = 0x63f5b0 (ita_cup.cpp, thiscall, ret 0): regra: a fase da Copa da Itália avança uma; quando passa da fase 6 (os 8 grupos acabaram) monta o mata-mata (os 8 campeões e os 16 cabeças de chave); na última fase, a caixa de erro;
- `cqr7_copa_da_serie_c_fase_seguinte()` = 0x63d950 (ita_c_cup.cpp, thiscall, ret 0): regra: a fase da Copa da Série C avança uma; quando passa da fase 14 (os 16 grupos acabaram) monta o mata-mata de 32;
- `cqr8_copa_da_italia_grupo()` = 0x63f640 (ita_cup.cpp, thiscall, ret 4): regra: os grupos 2 a 8 da Copa da Itália são ligas à parte (League_Stage) de 4 times: os 4 seguintes da lista dos inscritos, que saem dela, com as rodadas da tabela da copa para aquela fase, em turno único (1) e 2 pontos [inferência: os últimos argumentos do construtor]; cada humano recebe a notícia do grupo;
- `cqr8_copa_da_serie_c_grupo()` = 0x63d9e0 (ita_c_cup.cpp, thiscall, ret 4): regra: os grupos 2 a 16 da Copa da Série C são ligas à parte de 5 times, tirados da lista dos inscritos na ordem, como na Copa da Itália;
- `cqr9_copa_da_italia_mata_mata()` = 0x63f870 (ita_cup.cpp, thiscall, ret 0): regra: depois dos 8 grupos, o mata-mata da Copa da Itália tem 24 times: os 8 campeões dos grupos e os 16 cabeças de chave que entram direto (os primeiros inscritos ainda na lista: os clubes da Série A [inferência]); no sorteio os campeões vão com semente 3 e 2, os cabeças de chave com 2 e 1 (8 vagas por semente) [inferência: o pote de cada vaga]; a primeira rodada é sorteada na hora;
- `cqr9_copa_da_serie_c_mata_mata()` = 0x63dc10 (ita_c_cup.cpp, thiscall, ret 0): regra: depois dos 16 grupos, o mata-mata da Copa da Série C tem 32 times: os 16 campeões, os 6 melhores segundos (os 16 segundos numa tabela, ordenados pelos critérios 1, 2, 3: pontos e desempates; o grupo deles marcado com 2 e a linha com 6) e 10 cabeças de chave que entram direto; sem sementes; a primeira rodada é sorteada na hora;
- `cqs8_eurocopa_regras()` = 0x57e8f0 (euro_champ_qual.cpp, thiscall, ret 0): regra: as eliminatórias da Eurocopa são grupos em ida e volta (2 turnos), 3 pontos pela vitória e 1 pelo empate, desempates 1, 2, 4, 5, o primeiro de cada grupo vai direto e o segundo ao play-off, ninguém cai; o sorteio é em 10 de março do ano da copa, levado à sexta-feira [inferência];
- `cqs8_eurocopa_tamanho_do_grupo()` = 0x57d9d0 (euro_champ_qual.cpp, thiscall, ret 4): regra: no formato 1 os grupos 1, 3 e 6 têm 6 seleções e os outros 5; no formato 2 os grupos 1, 3, 6 e 8 têm 6 e os outros 5;
- `cqs8_eurocopa_situacao()` = 0x580b50 (euro_champ_qual.cpp, thiscall, ret 4): regra: a situação de uma seleção nas eliminatórias: −2 quando a estatística 0x12 dela na copa é menor que 14 (a posição [inferência]), senão −3;
- `cqs8_eurocopa_grupo_um()` = 0x57e7e0 (euro_champ_qual.cpp, thiscall, ret 0): regra: o grupo 1 das eliminatórias é a própria copa, com 5 seleções (as primeiras da lista sorteada), (5 − 1) × 2 rodadas em ida e volta, e a notícia do grupo;
- `cqs9_uefa_inscritos()` = 0x902b90 (uefa_cup.cpp, thiscall, ret 0): regra: entram na Copa da UEFA os clubes que a classificação da UEFA pôs nela, cada um com a classe do seu coeficiente (os de 10 ou mais contam como da fase preliminar); se, descontada a metade dos da preliminar, não chegam a 118, completam os primeiros clubes sem copa europeia de nações sem a marca +0x11c e de coeficiente abaixo de 2000, com a classe 3 (e eles passam a ser da Copa da UEFA); passando de 118, os últimos saem; a lista é ordenada pela classe; com um número par de clubes da preliminar, os de classe 1 vão a uma lista à parte (com a classe 3) e a copa passa a ter os inscritos + 27 vagas (as 27 da Liga dos Campeões e da Intertoto que chegam depois [inferência]); com um número ímpar, a copa fica com 145 vagas e devolve 0;
- `cqt5_copa_do_rei_dia()` = 0x84d740 (spa_cup.cpp, thiscall, ret 8): regra: a Copa do Rei é refeita uma vez por edição, no primeiro dia (com a fase do dia) em que o ano da classificação da UEFA chega ao ano da copa: as tabelas antigas liberadas, os inscritos (os da Primera, da Segunda e da Segunda B e os clubes de baixo, pela regra da nação), as rodadas, a árvore e as posições previstas; todo dia a copa joga as suas rodadas;
- `cqt5_historico_liga_dos_campeoes()` = 0x584ad0 (european_cup.cpp, thiscall, ret 0): regra: o histórico da temporada da Liga dos Campeões é o da sua última fase (a 12: a final);
- `cqt5_historico_copa_da_grecia()` = 0x5e93e0 (gre_cup.cpp, thiscall, ret 0): regra: o histórico da temporada da Copa da Grécia é o da fase 10 (a final);
- `cqt5_historico_copa_da_liga_irlandesa()` = 0x637750 (ire_lge_cup.cpp, thiscall, ret 0): regra: o histórico da temporada da Copa da Liga irlandesa é o da fase 7 (a final);
- `cqt5_historico_copa_da_serie_c()` = 0x63d770 (ita_c_cup.cpp, thiscall, ret 0): regra: o histórico da temporada da Copa da Série C é o do mata-mata (a fase 15);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (27):** 25 acusadas. `cqs8_grupo_um_com_rodada_a_mais` escapou: todo leitor de `games_per_club` (+0xa7) no exe cruza o valor com os jogos disputados da linha (+0x11), que são zero até os jogos do grupo 1 em setembro, e o vetor da forma (malloc de 8 ou 10 bytes) cai no mesmo bloco de 16 do alocador; o comparador roda só a original contra a fórmula fixa (provado pelo código). `cqs9_preliminar_de_11` escapou: o `EURO_SEEDING` de um clube da Copa da UEFA é a fase que o `assign_uefa_cup_places` lhe dá, 1, 2 ou `count_extra` × 10 + 3 com `count_extra` ≥ 2 (23, 33...), nunca 10, então `>= 0xa` e `>= 0xb` contam os mesmos clubes (provado pelo código). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original f3bf21f5 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 283ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 284`

**Campos tocados:**
- copa +0xa7 tabela do mata-mata da zona 1 (6 bytes: clube, +4 classe, +5 situação: 1 = vivo), +0x3e times; ponteiro/valor; lê (grupos)
- copa +0xc fases ([0] o mata-mata da zona 2, [1] e [2] os grupos, [3] as semifinais); ponteiro; lê e grava
- copa +0x2c fases, +0x30 fase atual; valor; lê e grava (fase seguinte, grupos)
- copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)
- copa +0xee lista (50 seleções, depois os 16 dos grupos), +0xf2 a conta (palavra, 16); ponteiro/valor; grava (sorteio), lê (fases), zera (dia)
- nação +0x71 continente (o +0 dele é o id, [0x9cfa08] a África); seleção +0x53 nação; ponteiro/id; lê (sorteio)
- copa +0xc fases, +0x30 fase atual; ponteiro/valor; grava (fase)
- copa +0xb1 tabela do grupo A, +0xc fases ([0..2] os grupos B a D, [3] as quartas); ponteiro; lê e grava (quartas)
- copa +0xa7 tabela do mata-mata da zona 1 (6 bytes: +5 situação, 1 = vivo), +0x3e times, +0xc fases ([0] zona 2, [1] as finais); ponteiro/valor; lê e grava (finais)
- copa +0x2c fases, +0x30 fase atual; valor; lê e grava
- copa +0x3e times; valor; grava 4 ou 5 (grupo 1)
- copa +0xb1 tabela (linhas de 0x3b bytes); ponteiro; grava (grupo 1)
- copa +0xee inscritos que esperam a fase (vetor de clubes); ponteiro; lê e zera (grupo 1)
- copa +0x2c fases, +0x30 fase atual; valor; lê e grava +1 (fase seguinte)
- copa +0x3e times por grupo; valor; lê
- copa +0xee inscritos que esperam a fase (vetor de clubes); ponteiro; lê e zera
- copa +0xc fases; ponteiro; grava a fase g
- copa +0x30 fase atual; valor; grava g
- copa +4, +0x40, +0x51, +0xbe, +0xc2..+0xc5, +0xd9; valores das regras passados à fase; lê
- copa +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +5 e +9 zerados na cópia, +0x37 marca 6 do melhor segundo); ponteiro; lê e grava
- copa +0xc fases (as dos grupos e o mata-mata em 7 ou 15); ponteiro; lê e grava
- copa +0xee inscritos (até +0xf2); ponteiro; lê e zera
- copa +0xbe marca do grupo (2: tem melhor segundo); valor; grava
- copa +0xc2..+0xc8 pontos e desempates, +0x3c turnos, +0x43, +0x49, +0x4a, +0xbe..+0xc1 vagas; valor; grava (regras)
- copa +0xb6 dia do sorteio, +0xb8; valor; grava (regras)
- copa +0xba tabela das rodadas, +0xa9 rodadas, +0x3a id; ponteiro/valor; grava (regras)
- copa +0xf6 formato (byte com sinal); valor; lê (tamanho do grupo)
- copa +0xf7 seleções sorteadas (vetor); ponteiro; lê (grupo 1). +0x3e, +0xa7, +0xb1; grava (grupo 1)
- clube +0x1db copa europeia (id), +0x1df coeficiente (byte com sinal); id/valor; lê, e grava +0x1db em quem completa
- nação +0x11c marca, +0x8e coeficiente (word com sinal); valor; lê
- copa +0x3e vagas, +0xb2 clubes da preliminar, +0x36/+0x38 contas da lista +0x14; valor; grava
- copa +0xa7 inscritos (6 bytes: clube, +4 classe, +5 situação), +0x14 lista dos de classe 1; ponteiro; grava
- [0x9cf6f0] a Copa da UEFA; id; lê
- copa +0x45 já refeita nesta edição; valor; lê e grava 1 (dia)
- [0xde1f58] +0x439 o ano da classificação da UEFA (word); valor; lê
- copa +0xa7, +0xa3, +0xad as tabelas da edição; ponteiro; libera (dia)
- copa +0xc fases; ponteiro; lê a última (históricos)

**Interessa a:** CMRTE
