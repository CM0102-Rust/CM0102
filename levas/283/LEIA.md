# 283ª leva — a Série C: a final, a virada da temporada e as posições previstas; o Troféu da Liga inglesa: os 56 clubes das chaves Norte e Sul, a fase das chaves e a fase seguinte; o Troféu da Liga inglesa: a final entre os campeões das chaves e a classificação final; a fase de mata-mata das copas (Cup_Stage): o construtor e o destrutor; os mata-matas da Libertadores e da Merconorte: os classificados dos grupos e a fase seguinte; os convidados da Merconorte e a fase seguinte da Mercosul e da Copa América; a Copa América: as 12 seleções, as posições previstas e a temporada nova; os construtores dos estaduais: Baiano, Centro-Oeste e Gaúcho; os construtores dos estaduais: Goiano, Mineiro e Norte; os construtores dos estaduais: Nordeste, Paranaense e Pernambucano; os construtores dos estaduais: Carioca, Catarinense e Paulista; e a média de público dos clubes no fim da temporada; o Campeonato de Clubes da Oceania: o dia do sorteio e as tabelas dos grupos A e B; o Campeonato de Clubes da Oceania: os 8 clubes, a fase seguinte e as semifinais

**Rotinas:**
- `cqs4_final_da_serie_c()` = 0x42e980 (bra_nat_third.cpp, thiscall, ret 0): A final da Série C: o campeão do grupo A da terceira fase contra o do grupo B, numa fase de copa de dois [inferência: ida e volta]; o vencedor é o campeão da Série C;
- `cqs4_virada_da_serie_c()` = 0x42def0 (bra_nat_third.cpp, thiscall, ret 0): A virada da Série C: guarda a temporada, apaga a tabela, o calendário e as fases, passa ao ano seguinte e refaz as regras, a lista dos 56 clubes e a tabela, os jogos e a ordem; depois as expectativas das diretorias, a renovação dos árbitros, os sete grupos da primeira fase e as posições previstas; a lista dos 56 é descartada no fim;
- `cqs4_posicoes_previstas_da_serie_c()` = 0x42f0a0 (bra_nat_third.cpp, thiscall, ret 0): As posições previstas da Série C: os 56 ordenados pela reputação; o favorito em 1º, o segundo em 2º, os dois seguintes em 3º (as semifinais), dois em 5º e dois em 7º, oito em 9º (a segunda fase) e os outros quarenta em faixas de oito: 16º, 24º, 32º, 40º e 48º;
- `cqs5_trofeu_clubes()` = 0x56b730 (eng_auto_cup.cpp, thiscall, ret 0): regra: o Troféu da Liga (LDV Vans) tem 56 clubes: todos os da segunda e da terceira divisões, o rebaixado da terceira para a Conference e, para completar, clubes da Conference (com a regra da nação ligada, os que já estavam nela pela categoria, 6 ou menos primeiro; senão até 30 da Conference ou outros ingleses fora da divisão genérica, ordenados pela reputação e com os melhores embaralhados); a lista é dividida em Norte e Sul pela latitude da cidade do estádio e cada metade vai da menor reputação para a maior;
- `cqs5_trofeu_fase_das_chaves()` = 0x56bab0 (eng_auto_cup.cpp, thiscall, ret 0): regra: a fase das chaves do Troféu é um mata-mata (Cup_Stage) com os 56 na ordem da lista (Norte, depois Sul), com as rodadas da fase 0 da tabela da copa, sem cabeças de chave;
- `cqs5_trofeu_fase_seguinte()` = 0x56bc50 (eng_auto_cup.cpp, thiscall, ret 0): regra: a fase do Troféu avança uma; quando as chaves acabam (fase 1), monta a final entre o campeão do Norte e o do Sul; na última fase, a caixa de erro;
- `cqs6_trofeu_final()` = 0x56bce0 (eng_auto_cup.cpp, thiscall, ret 0): regra: quando as chaves acabam, a final do Troféu é um jogo (Cup_Stage de 2, fase 1) entre o que segue vivo na lista da copa (situação 1) e o que segue vivo na fase das chaves; a rodada é sorteada na hora (o mando);
- `cqs6_trofeu_classificacao()` = 0x56c010 (eng_auto_cup.cpp, thiscall, ret 0): regra: a classificação final do Troféu nas estatísticas: os dois de maior reputação de cada metade disputam o 1º e o 2º (o de reputação maior fica em 1º); os outros, rodada a rodada da última para a primeira, ficam com a mesma posição os que caíram na mesma rodada (3, depois 3 + 2 × quantos caíram antes...);
- `cqs7_fase_de_copa()` = 0x522e00 (cup_stage.cpp, thiscall, ret 0x3c): regra: uma fase de mata-mata de uma copa leva a tabela das rodadas copiada do modelo da copa (datas, sorteios), o ano, a fase, o id e as regras do pai (+0x49, +0x4a); cada time entra com a situação 0 e a semente do pote (dada pela copa, ou a posição na lista quando a copa pede a ordem, ou 0); a árvore dos confrontos é montada na hora: a da fase (Cup_Stage::map_fixture_tree, que liga os vencedores e põe as sementes na primeira rodada) ou a da copa (Cup::map_fixture_tree, que liga ao acaso);
- `cqs7_fim_da_fase_de_copa()` = 0x523300 (cup_stage.cpp, thiscall, ret 0): regra: o fim de uma fase de mata-mata: a lista dos times, a tabela das rodadas e os confrontos de cada rodada liberados, e a base destruída;
- `cqs10_libertadores_mata_mata()` = 0x4c3d10 (conmebol_liber.cpp, thiscall, ret 0): regra: o mata-mata da Libertadores tem 16 clubes: os 2 primeiros (+0xbe) de cada um dos 8 grupos (o grupo 1 na ordem da tabela, os outros pela posição na tabela de cada fase), numa fase de copa (a fase 7) com a tabela de rodadas da copa e o ano dela; a primeira rodada é sorteada na hora;
- `cqs10_merconorte_semifinais()` = 0x4c2330 (con_merc_cup.cpp, thiscall, ret 0): regra: as semifinais da Merconorte têm 4 clubes: o primeiro (+0xbe = 1) de cada um dos 4 grupos, numa fase de copa (a fase 3); a primeira rodada é sorteada na hora;
- `cqs10_merconorte_fase_seguinte()` = 0x4c20b0 (con_merc_cup.cpp, thiscall, ret 0): regra: a Merconorte avança uma fase quando a atual acaba; depois das 3 fases de grupo (ao chegar a 3) monta as semifinais; passar da última fase é erro;
- `cqs11_merconorte_convidados()` = 0x4c2560 (con_merc_cup.cpp, thiscall, ret 0): regra: a Merconorte convida 16: os 4 melhores do México, os 5 da Colômbia, os 3 do Peru e os 2 da Bolívia (cada lista ordenada pela reputação, com a faixa de cima embaralhada: 3 vezes as vagas) e 2 da liga americana (com a competição dela: o campeão do último registro da MLS e o da outra competição dela, e o vice da MLS se forem o mesmo; sem ela, os 2 melhores clubes da MLS); a lista final é embaralhada (3 × 16 trocas) antes dos grupos;
- `cqs11_mercosul_fase_seguinte()` = 0x4c5140 (conmebol_merc.cpp, thiscall, ret 0): regra: a Mercosul avança uma fase quando a atual acaba; depois das 4 fases de grupo (ao chegar a 4) monta as quartas; passar da última fase é erro;
- `cqs11_copa_america_fase_seguinte()` = 0x5e1620 (goldcup.cpp, thiscall, ret 0): regra: a Copa América avança uma fase quando a atual acaba; depois das 2 fases de grupo (ao chegar a 2) monta o mata-mata; passar da última fase é erro;
- `cqs12_copa_america_selecoes()` = 0x5e1cd0 (goldcup.cpp, thiscall, ret 0): regra: a Copa América tem 12 seleções; em 2001 uma lista fixa (os grupos intercalados nas vagas: A = Colômbia, Chile, Equador e Venezuela; B = Brasil, México, Peru e Paraguai; C = Costa Rica, Honduras, Uruguai e Bolívia); nos outros anos as sedes na frente, depois as sul-americanas e as convidadas (o Japão em 1999 [inferência: a nação de 0x9cf36c], nos outros anos a de 0x9cf280, e o México sempre), sem repetir as sedes, até 12, ordenadas pela reputação; sem 12, erro;
- `cqs12_copa_america_previstas()` = 0x5e2340 (goldcup.cpp, thiscall, ret 0): regra: as posições previstas da Copa América pela reputação: 1, 2, 3, 3, quatro 5, 9 e três 10;
- `cqs12_copa_america_temporada_nova()` = 0x5e1530 (goldcup.cpp, thiscall, ret 0): regra: a Copa América é a cada 2 anos: ao acabar, a tabela e as fases são desfeitas, a temporada vai para o histórico da estatística, o ano sobe 2 e as regras são montadas de novo;
- `cqt1_construir_baiano()` = 0x42f2d0 (bra_reg_bahia.cpp, thiscall, ret 8): O Baiano na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt1_construir_centro_oeste()` = 0x4304b0 (bra_reg_central.cpp, thiscall, ret 8): O Centro-Oeste na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt1_construir_gaucho()` = 0x4317c0 (bra_reg_gaucho.cpp, thiscall, ret 8): O Gaúcho na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt2_construir_goiano()` = 0x432b50 (bra_reg_goias.cpp, thiscall, ret 8): O Goiano na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt2_construir_mineiro()` = 0x433d50 (bra_reg_minas_gerais.cpp, thiscall, ret 8): O Mineiro na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, dois turnos e a final (três fases além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt2_construir_norte()` = 0x435340 (bra_reg_north.cpp, thiscall, ret 8): O Norte na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt3_construir_nordeste()` = 0x4363f0 (bra_reg_northeast.cpp, thiscall, ret 8): O Nordeste na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt3_construir_paranaense()` = 0x437400 (bra_reg_parana.cpp, thiscall, ret 8): O Paranaense na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, dois turnos e a final (três fases além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt3_construir_pernambucano()` = 0x4388f0 (bra_reg_pern.cpp, thiscall, ret 8): O Pernambucano na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt4_construir_carioca()` = 0x439a40 (bra_reg_rio.cpp, thiscall, ret 8): O Carioca na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, dois turnos e a final (três fases além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt4_construir_catarinense()` = 0x43b130 (bra_reg_santa.cpp, thiscall, ret 8): O Catarinense na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt4_construir_paulista()` = 0x43c280 (bra_reg_sp.cpp, thiscall, ret 8): O Paulista na criação da carreira: o ano e a competição do banco, a regra de disciplina 7 e, sem estado salvo, a importância 15, pontos corridos e o mata-mata (uma fase além da liga), as regras, a tabela com os clubes do estadual, os jogos, a ordem, as estatísticas e as posições previstas;
- `cqt4_atualizar_media_de_publico()` = 0x5d6280 (game.cpp, cdecl): A média de público dos clubes no fim da temporada de cada liga: anda com a posição final contra a prevista e com a reputação da competição, com um peso por nação (0,1, 0,25 ou 0,5 em algumas, 1 nas outras): o campeão e o vice sem acesso ganham (reputação + posições acima da prevista) × 200/100 ou 125/65 (dono do estádio ou não); quem sobe ganha pela reputação do clube e pela diferença de reputação para a liga de cima, com o piso da liga de cima; quem cai perde pela diferença para a de baixo, com o teto dela; os do play-off de acesso e os do meio da tabela ganham ou perdem por sorteio, com mistura com o público real da temporada (1/5 do público médio em casa + 4/5 da média) e os pisos e tetos pela reputação (2000, 3000 ou 5000 vezes); no fim, no mínimo 500 e entre o mínimo e o recorde do clube;
- `cqt10_oceania_dia()` = 0x797b40 (oceania_club_champ.cpp, thiscall, ret 8): regra: o Campeonato de Clubes da Oceania é sorteado no dia do começo (+0xb6/+0xb8) e só na chamada sem `completo`: os 8 clubes, as posições previstas, o grupo A na tabela da própria copa, os jogos e o grupo B como fase; antes desse dia a copa nem passa o dia de liga; um erro na montagem devolve 1 sem o dia de liga;
- `cqt10_oceania_grupo_a()` = 0x7978c0 (oceania_club_champ.cpp, thiscall, ret 0): regra: o grupo A do Campeonato de Clubes da Oceania tem os 4 primeiros escolhidos, cada um na linha da sua ordem; os técnicos humanos recebem a notícia do sorteio do grupo;
- `cqt10_oceania_grupo_b()` = 0x797e80 (oceania_club_champ.cpp, thiscall, ret 0): regra: o grupo B do Campeonato de Clubes da Oceania é uma fase de liga com os escolhidos 4 a 7, as regras e o ano da copa e as rodadas da tabela da copa para a fase 0 (10, 12 e 14 de janeiro); os técnicos humanos recebem a notícia;
- `cqt11_oceania_clubes()` = 0x798220 (oceania_club_champ.cpp, thiscall, ret 0): regra: o Campeonato de Clubes da Oceania tem 8 clubes em 4 pares, cada par dividido entre os grupos A e B por sorteio: o campeão da liga australiana (sem ele, o melhor australiano) e o melhor neozelandês; o melhor de Fiji e o do Taiti; dois melanésios de nações diferentes; dois polinésios de nações diferentes (cada lista ordenada pela reputação, com a faixa de cima embaralhada);
- `cqt11_oceania_fase_seguinte()` = 0x797df0 (oceania_club_champ.cpp, thiscall, ret 0): regra: o Campeonato de Clubes da Oceania avança uma fase quando a atual acaba; depois dos grupos (ao chegar a 1) monta as semifinais; passar da última fase é erro;
- `cqt11_oceania_semifinais()` = 0x798050 (oceania_club_champ.cpp, thiscall, ret 0): regra: as semifinais do Campeonato de Clubes da Oceania cruzam os grupos: 1º do A × 2º do B e 2º do A × 1º do B, numa fase de copa (a fase 1) com a primeira rodada montada na hora;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (40):** 40 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original 5b041fc1 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 282ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 283`

**Campos tocados:**
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê as vagas 8 e 9, grava a 10 (final); destrói, libera e realoca (virada)
- liga +0x30: a fase atual; valor; grava 10 (final), −1 (virada)
- liga +0x40: o ano da temporada; valor; lê e grava + 1
- liga +0xee: a lista dos 56 clubes; ponteiro; ordena (posições previstas), libera e zera (virada)
- liga +0x2c: a quantidade de fases; valor; lê
- liga +0x4c, +0xba: [inferência] marca da temporada, o calendário; grava 0, libera
- estatística, registro do clube +0x72: a posição prevista; valor; grava
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- copa +0xb2 lista dos clubes, +0xb6 quantos (56); ponteiro/valor; grava (clubes) e lê (fase das chaves)
- clube +0x57 divisão, +0x5b divisão da temporada anterior [inferência], +0x53 nação; ponteiro; lê
- clube +0x5f categoria (byte com sinal) [inferência]; valor; lê
- nação +0x11c bit 4 (a regra da Conference por categoria) [inferência]; valor; lê
- [0x9cf5c4], [0x9cf5c8], [0x9cf69c], [0x9cf764], [0x9cf2e4]: a segunda e a terceira divisões, a Conference, a divisão genérica e a Inglaterra [inferência]; id; lê
- copa +0xc fases, +0x2c fases, +0x30 fase atual; ponteiro/valor; grava
- copa +0xa7 lista dos inscritos (6 bytes: clube, +5 situação); ponteiro; lê
- copa +0xc fases ([0] as chaves, [1] a final); ponteiro; lê e grava
- copa +0xa3 tabela das rodadas (0x68 bytes por rodada, +0x1a os clubes da rodada), +0x3c rodadas; lê
- copa +8 estatísticas (Comp_Stats); ponteiro; grava a posição final de cada clube
- clube +0x80 reputação (word); valor; lê
- fase +4, +0x28, +0x3a id, +0x3c rodadas, +0x3e times, +0x40 ano, +0x42, +0x43, +0x44 fase, +0x49, +0x4a, +0x51, +0x2c, +0x30, +0xab, +0xac, +0xb1; valor; grava (construtor)
- fase +0xa3 tabela das rodadas (0x68 bytes por rodada), +0xa7 times (6 bytes: clube, +4 semente, +5 situação), +0xad confrontos por rodada; ponteiro; grava (construtor) e libera (destrutor)
- copa +0xbe classificados por grupo (byte com sinal), +0xb1 tabela do grupo 1; valor/ponteiro; lê (mata-mata)
- copa +0xc fases (League_Stage dos grupos; o Cup_Stage do mata-mata gravado em [+0xc][7] ou [3]); ponteiro; lê e grava (mata-mata)
- copa +0x2c fases, +0x30 fase atual; valor; lê e grava (fase seguinte)
- clube +0x53 nação (o +0 dela é o id), +0x57 liga (o +0 dela é o id da competição); ponteiro; lê (convidados)
- [0x9cf3d0] México, [0x9cf29c] Colômbia, [0x9cf428] Peru, [0x9cf254] Bolívia, [0x9cf590] a MLS, [0x9cf728] a competição da liga americana; id; lê
- copa +0xee lista dos convidados, +0xf2 a conta (16); ponteiro/valor; grava (convidados)
- copa +0x2c fases, +0x30 fase atual; valor; lê e grava (fases seguintes)
- copa +0x40 ano (2001 = 0x7d1), +0xee lista das seleções, +0xf2 a conta (12); valor/ponteiro; lê e grava (as 12)
- nação +0x71 continente (o +0 dele é o id), +0 id; seleção +0x53 nação; ponteiro/id; lê (as 12)
- estatística (+8) posição prevista da entrada do clube (+0x72); valor; grava (previsões)
- copa +0x4c, +0xba tabela das rodadas, +0x45, +0x30; valor/ponteiro; grava (temporada nova)
- liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7
- liga +0x51: [inferência] a importância; valor; grava 0xf
- liga +0x40: o ano da temporada; valor; grava
- liga +4: o registro da competição no banco; ponteiro; grava
- liga +0x44: [inferência] marca da liga nova; valor; grava 0xff
- liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava
- liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7
- liga +0x51: [inferência] a importância; valor; grava 0xf
- liga +0x40: o ano da temporada; valor; grava
- liga +4: o registro da competição no banco; ponteiro; grava
- liga +0x44: [inferência] marca da liga nova; valor; grava 0xff
- liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava
- liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7
- liga +0x51: [inferência] a importância; valor; grava 0xf
- liga +0x40: o ano da temporada; valor; grava
- liga +4: o registro da competição no banco; ponteiro; grava
- liga +0x44: [inferência] marca da liga nova; valor; grava 0xff
- liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava
- liga +0x50: a regra de disciplina (índice na tabela dos órgãos disciplinares); valor; grava 7
- liga +0x51: [inferência] a importância; valor; grava 0xf
- liga +0x40: o ano da temporada; valor; grava
- liga +4: o registro da competição no banco; ponteiro; grava
- liga +0x44: [inferência] marca da liga nova; valor; grava 0xff
- liga +0x2c / +0x30 / +0xc: as fases / a atual / o vetor; grava
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; grava
- clube +0x73 / +0x77 / +0x7b: a média de público (Attendance) / o mínimo / o recorde; valor; lê e grava (média de público)
- clube +0x80: a reputação (word); +0x6d: dono do estádio; lê
- liga +0x1c / +0x20: a liga de cima / de baixo (id); +0xbe, +0xbf, +0xc1: vagas de acesso, play-off e rebaixamento; lê
- competição +0x69: a reputação (word); lê
- copa +0x45 sorteio feito, +0xb6/+0xb8 dia e ano relativo do sorteio, +0x40 ano; valor; lê e grava +0x45 (dia)
- copa +0xee os 8 escolhidos (ponteiros de clube), +0xf2 a conta; ponteiro/valor; lê (grupos), libera e zera (dia)
- copa +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 clube, +4 posição); valor/ponteiro; grava (grupo A)
- copa +0xc fases ([0] o grupo B), +0x30 fase atual; ponteiro/valor; grava (grupo B)
- clube +0x53 nação (+0 o id, +0x71 o continente; [0x9cfa18] a Oceania); ponteiro/id; lê (os 8)
- [0x9cf3f4] Nova Zelândia, [0x9cf224] Austrália, [0x9cf4b4] Taiti, [0x9cf2f8] Fiji, [0x9cf89c] a liga australiana; id; lê
- copa +0xee os 8 (o grupo A nas vagas 0..3, o B nas 4..7), +0xf2 a conta; ponteiro/valor; grava (os 8)
- copa +0x2c fases, +0x30 fase atual, +0xc fases ([1] as semifinais); valor/ponteiro; lê e grava (fase seguinte, semifinais)

**Interessa a:** CMRTE
