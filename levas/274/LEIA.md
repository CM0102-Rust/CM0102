# 274ª leva — as regras dos grupos da Libertadores, da Copa Merconorte e do Campeonato de Clubes da Oceania; as regras das eliminatórias da Copa do Mundo na Europa, na América do Sul e na Ásia; as regras dos grupos da Copa América, da Copa Africana de Nações e da eliminatória da Oceania; a Libertadores: os classificados, o destino de cada clube e a fase seguinte; a Copa Africana de Nações: a fase seguinte, a tabela do grupo A e o histórico; as regras das três divisões nacionais do Brasil (Séries A, B e C); as regras dos estaduais do Centro-Oeste, Gaúcho, Nordeste, Paulista, Baiano e Catarinense; as regras dos estaduais Paranaense, Norte, Pernambucano, Mineiro e Carioca; as regras do Goiano e das duas divisões da J-League; as regras da NSL australiana e da MLS, e o histórico da temporada dos estaduais e das Séries A e B; o destino de cada clube no fim das fases da Série A, da Série C e de sete estaduais; o destino de cada clube no fim das fases do Centro-Oeste, Goiano, Mineiro, Paranaense, Carioca e da NSL; o progresso do clube nas estatísticas da competição no Mineiro, Paranaense, Carioca, MLS e Série C

**Rotinas:**
- `cqj11_libertadores_regras()` = 0x4c3890 (conmebol_liber.cpp, thiscall, ret 0): regra: os grupos da Libertadores são de ida e volta (dois turnos), 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2 e 3 [inferência: saldo, gols pró, vitórias]; dois classificados por grupo; clubes de várias nações; categoria 7 em cada jogo; 7 reservas no banco e 3 substituições; começa no dia 15 do mês 0 do ano da competição [inferência: 15 de janeiro, meses contados de 0]; fundo de prêmios de 2.000.000;
- `cqj11_merconorte_regras()` = 0x4c1ed0 (con_merc_cup.cpp, thiscall, ret 0): regra: os grupos da Copa Merconorte são de ida e volta, 3/1 pontos, desempate 1, 2, 3, um classificado por grupo, categoria 7, começo no dia 1 do mês 7 [inferência: 1º de agosto], prêmios de 2.000.000;
- `cqj11_oceania_clubes_regras()` = 0x7979b0 (oceania_club_champ.cpp, thiscall, ret 0): regra: os grupos do Campeonato de Clubes da Oceania são de turno único, 3/1 pontos, desempate pelos códigos 1, 2 e 4, dois classificados por grupo (os de cima seguem, os outros estão eliminados: bits de regra 0x28), categoria 8 (a das fases decisivas), 3 reservas no banco e 3 substituições; começa no dia 20 do mês 7 levado a um dia da semana 5 [inferência]; sem fundo de prêmios;
- `cqj12_eliminatoria_europa_regras()` = 0x9191e0 (wc_europe_league.cpp, thiscall, ret 0): regra: os grupos europeus são de ida e volta, 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2, 4 e 5 [inferência: saldo, gols pró, confronto direto, gols fora]; o primeiro de cada grupo vai direto; seleções (+0x43 = 2); categoria 10 (seleções) em cada jogo; 7 reservas e 3 substituições; começa em 25 do mês 2 [inferência: 25 de março] levado a um dia da semana 5;
- `cqj12_eliminatoria_sul_americana_regras()` = 0x921040 (wc_south_american_league.cpp, thiscall, ret 0): regra: a eliminatória sul-americana é uma liga única de ida e volta, 3/1 pontos, desempate 1, 2 e 3; as vagas diretas são quatro menos as nações isentas (a campeã do mundo e as anfitriãs sul-americanas, contadas em +0xee e +0xef [inferência]); o quinto vai à repescagem (+0xbf = 1); categoria 10, 7 reservas e 3 substituições; começa em 25 do mês 0 num dia da semana 5;
- `cqj12_eliminatoria_asia_regras()` = 0x912eb0 (wc_asia_league.cpp, thiscall, ret 0): regra: os grupos asiáticos são de ida e volta, 3/1 pontos, desempate 1, 2 e 3, o primeiro de cada grupo segue, categoria 10, começo em 25 do mês 2 num dia da semana 5, +0xb8 = 1 [inferência: a fase começa no ano seguinte];
- `cqk10_copa_america_regras()` = 0x5e12d0 (goldcup.cpp, thiscall, ret 0): regra: os grupos da Copa América são de turno único, 3 pontos a vitória e 1 o empate, desempate pelos códigos 1, 2 e 3 [inferência: saldo, gols pró, vitórias]; dois classificados por grupo; seleções; categoria 10 em cada jogo; 7 reservas e 3 substituições; a data de 20 do mês 2 do ano da competição num dia da semana 5 [inferência: a data do sorteio];
- `cqk10_copa_africana_regras()` = 0x401ab0 (african_nations.cpp, thiscall, ret 0): regra: os grupos da Copa Africana de Nações são de turno único, 3/1 pontos, desempate 1, 2, 3, dois classificados por grupo, categoria 10, 7 reservas e 3 substituições; a data de 5 do mês 9 do ano anterior ao do torneio num dia da semana 5 [inferência: o sorteio no ano anterior] e +0xb8 = −1;
- `cqk10_eliminatoria_oceania_regras()` = 0x91ecf0 (wc_oceania_league.cpp, thiscall, ret 0): regra: os grupos da eliminatória da Oceania são de turno único, 3/1 pontos, desempate 1, 2, 3, um classificado por grupo (para a final da Oceania), categoria 10, 7 reservas e 3 substituições, começo em 15 do mês 0 num dia da semana 5;
- `cqk11_libertadores_classificados()` = 0x4c3f40 (conmebol_liber.cpp, thiscall, ret 0): regra: entram na Libertadores os 32 clubes marcados com ela como competição continental da temporada (a marca vem da classificação nas ligas), na ordem do banco, depois embaralhados (96 trocas sorteadas) para os grupos; com outro número de clubes, erro (a caixa);
- `cqk11_libertadores_registros()` = 0x4c4080 (conmebol_liber.cpp, thiscall, ret 0x18): regra: nos sete grupos (fases 0 a 6), quem passa ganha o registro de classificação (peso 40) e quem não passa o de não classificação (peso 40); no mata-mata (fase 7), quem passa ganha a classificação pela rodada (peso de 50 a 100), quem cai a eliminação; o campeão ganha o registro de título e vai ao Mundial Interclubes, na vaga sul-americana (a 0, se lá já está um clube do continente [0x9cfa1c], senão a 1), com o arquivo da temporada do Mundial e as posições previstas; o vice ganha o registro de vice;
- `cqk11_libertadores_fase_seguinte()` = 0x4c3a80 (conmebol_liber.cpp, thiscall, ret 0): regra: a Libertadores tem sete fases de grupos (0 a 6) e o mata-mata na fase 7 (os dois primeiros de cada grupo, 16 clubes); passar da última fase é erro (a caixa);
- `cqk12_africana_fase_seguinte()` = 0x401e10 (african_nations.cpp, thiscall, ret 0): regra: a Copa Africana tem três fases de grupos (as fases 0 a 2, uma por rodada do grupo [inferência]) e, na fase 3, o mata-mata; passar da última fase é erro (a caixa);
- `cqk12_africana_grupo_a()` = 0x4019c0 (african_nations.cpp, thiscall, ret 0): regra: o grupo A tem quatro seleções, as quatro primeiras da lista do sorteio, na ordem; os técnicos recebem a notícia do sorteio;
- `cqk12_africana_historico()` = 0x401b90 (african_nations.cpp, thiscall, ret 0): regra: o histórico da temporada (campeão, vice, terceiro) é o do mata-mata;
- `cqi7_regras_da_serie_a()` = 0x42a5a0 (bra_nat_first.cpp, thiscall): Série A: turno único; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem liga acima, cai para a Série B; 4 vagas de cima que, pela palavra de regras 8, levam à fase seguinte (os play-offs; sem acesso e sem faixa de campeão: o campeão sai do mata-mata) [inferência: a fase seguinte é a do slot 11]; 4 rebaixados; 7 reservas e 3 substituições; numeração fixa do elenco; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqi7_regras_da_serie_b()` = 0x42cb70 (bra_nat_second.cpp, thiscall): Série B: turno único; vitória 3, empate 1; confronto direto e saldo; sobe para a Série A e cai para a Série C; 4 sobem (palavra de regras 2: as vagas de cima são de acesso) e 4 caem; 7 reservas, 3 substituições, numeração fixa. Depois, o calendário;
- `cqi7_regras_da_serie_c()` = 0x42de50 (bra_nat_third.cpp, thiscall): Série C: dois turnos (grupos de ida e volta [inferência: os grupos de 7 do calendário 0x42d620]); vitória 3, empate 1; desempate pelo saldo, gols pró e gols marcados fora; sobe para a Série B, sem liga abaixo; 2 vagas de cima que levam à fase seguinte (palavra 8) e 2 de baixo [inferência: sem efeito, não há liga abaixo]; 7 reservas, 3 substituições, numeração fixa. Depois, o calendário;
- `cqi8_regras_do_centro_oeste()` = 0x430ff0 (bra_reg_central.cpp, thiscall): Centro-Oeste: turno único; 8 vagas de cima (o mata-mata de 8 [inferência: a fase do slot 11]); 3 rebaixados; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão: o campeão sai da fase final); 7 reservas e 3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqi8_regras_do_gaucho()` = 0x4322d0 (bra_reg_gaucho.cpp, thiscall): Gaúcho, Copa do Nordeste e Paulista (a mesma rotina nas três vtables): turno único; 4 vagas de cima (o mata-mata de 4); 2 rebaixados; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão: o campeão sai da fase final); 7 reservas e 3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqi8_regras_do_catarinense()` = 0x43bc70 (bra_reg_santa.cpp, thiscall): Baiano e Catarinense (a mesma rotina nas duas vtables): dois turnos; 4 vagas de cima (a semifinal de 4); 1 rebaixado; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão: o campeão sai da fase final); 7 reservas e 3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqi9_regras_do_paranaense()` = 0x437fc0 (bra_reg_parana.cpp, thiscall): Paranaense: nenhuma vaga de cima na tabela geral (cada turno é um estágio de liga próprio, criado pelo add_result 0x4386e0, e decide o título [inferência]); 4 rebaixados; dois turnos; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão); 7 reservas e 3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqi9_regras_do_pernambucano()` = 0x439430 (bra_reg_pern.cpp, thiscall): Norte e Pernambucano (a mesma rotina nas duas vtables): 4 vagas de cima (a fase final de 4); 2 rebaixados; dois turnos; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão); 7 reservas e 3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqi9_regras_do_carioca()` = 0x43a600 (bra_reg_rio.cpp, thiscall): Mineiro e Carioca (a mesma rotina nas duas vtables): nenhuma vaga de cima na tabela geral (os turnos têm estágios próprios, criados pelo add_result 0x43af20 / 0x435130 [inferência: decidem o título]); 2 rebaixados; dois turnos; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; sem play-offs na tabela; sem liga acima nem abaixo (os rebaixados saem para o fundo não jogável [inferência]); palavra de regras 8 (as vagas de cima levam à fase seguinte; sem faixa de campeão); 7 reservas e 3 substituições; partidas de categoria 2 (liga). Depois, o calendário da fase principal;
- `cqj7_regras_do_goiano()` = 0x433640 (bra_reg_goias.cpp, thiscall): Goiano: turno único; vitória 3, empate 1; desempate pelo confronto direto e depois pelo saldo; 8 vagas de cima (palavra de regras 8: levam à fase seguinte, o mata-mata [inferência]); 4 rebaixados, sem liga acima nem abaixo; 7 reservas, 3 substituições; partidas de categoria 2. Depois, o calendário;
- `cqj7_regras_da_j1()` = 0x663620 (jap_j1.cpp, thiscall): J1: dois turnos (os dois estágios do ano, criados pelo add_result 0x663940 [inferência]); vitória 3, empate 1; desempate pelo saldo, gols pró e gols marcados fora; nenhuma vaga de cima na tabela geral (palavra 8, sem faixa de campeão: o campeão sai da final entre os vencedores dos estágios [inferência]); 2 caem para a J2; sem liga acima; 5 reservas e 3 substituições; numeração fixa do elenco; a palavra +0xab = 0x82 copiada em cada partida. Depois, o calendário;
- `cqj7_regras_da_j2()` = 0x665140 (jap_j2.cpp, thiscall): J2: quatro turnos; vitória 3, empate 1; desempate pelo saldo, gols pró e gols fora; 2 sobem para a J1 (a palavra de regras fica a da base, 3: campeão e acesso), nenhum rebaixado, sem liga abaixo; 5 reservas, 3 substituições. Depois, o calendário;
- `cqj8_regras_da_nsl()` = 0x411ce0 (aus_nsl.cpp, thiscall): NSL: três turnos; vitória 3, empate 1; desempate pelo saldo, gols pró e gols marcados fora; os 6 primeiros vão para a fase final (palavra de regras 0x28: 0x8 fase seguinte, sem faixa de campeão, e 0x20, os sem chance marcados como eliminados); ninguém sobe nem cai; 4 reservas e 3 substituições; numeração fixa do elenco. Depois, o calendário;
- `cqj8_regras_da_mls()` = 0x908660 (usa_mls.cpp, thiscall): MLS: 32 jogos por time (+0xa7; quatro turnos na conta de +0x3c); vitória 3, empate 1; desempate pelo saldo, gols pró e confronto direto; 1 vaga de cima na tabela geral (palavra 0x48: 0x8 fase seguinte, os play-offs; 0x40 rodadas em ordem fixa); ninguém sobe nem cai; a palavra +0xab = 0xc2 copiada em cada partida. Sem calendário aqui: os jogos são montados à mão (0x9086e0);
- `cqj8_historico_do_estadual()` = 0x432970 (bra_reg_gaucho.cpp, thiscall): fim de um estadual (nove deles): grava o histórico do primeiro estágio (a tabela) e chama a 0x42bba0, que, quando os doze estaduais terminaram, põe os campeões na Copa dos Campeões;
- `cqj8_historico_do_primeiro_estagio()` = 0x408eb0 (arg_second.cpp, thiscall): fim da segunda divisão argentina e das Séries A e B: o histórico vem do primeiro estágio, não da tabela geral;
- `cqj9_destino_na_serie_a()` = 0x42abc0 (bra_nat_first.cpp, thiscall): Série A: no fim dos pontos corridos, os classificados para o mata-mata ganham o registro de classificação (fase 0x41c, peso 0x28) e os rebaixados o de rebaixamento; no mata-mata, o campeão ganha o registro de campeão e o destino 5 na tabela, os eliminados o destino -2;
- `cqj9_destino_na_serie_c()` = 0x42eb00 (bra_nat_third.cpp, thiscall): Série C: nos grupos, os classificados ganham o registro de classificação; na fase 7, quem passa; no quadrangular final (fases 8 e 9), os dois que sobem ganham os registros de acesso (peso 0x32) e de classificação e o destino 0 (acesso) na primeira tabela que os tiver; na final (fase 10), o campeão ganha o registro de campeão;
- `cqj9_destino_no_baiano()` = 0x430210 (bra_reg_bahia.cpp, thiscall): Baiano, Gaúcho, Norte, Nordeste, Pernambucano, Catarinense e Paulista: no fim dos pontos corridos, os classificados para a fase final ganham o registro de classificação (rodada 0x82, peso 0x32) e os rebaixados o de rebaixamento; no mata-mata, o campeão ganha o registro de campeão e o destino 5; os eliminados, o registro de eliminação com peso de 30 a 50 conforme a rodada em que caíram, e o destino -2;
- `cqk7_destino_no_centro_oeste()` = 0x4314e0 (bra_reg_central.cpp, thiscall): Centro-Oeste e Goiano: no fim dos pontos corridos, os classificados para o mata-mata ganham o registro de classificação (rodada 0x82, peso 0x32) e os rebaixados o de rebaixamento; no mata-mata (fase 0), o campeão ganha o registro de campeão e o destino 5; os eliminados, o registro de eliminação com peso de 30 a 50 pela rodada (em 3 rodadas: quartas, semi, final) e o destino -2;
- `cqk7_destino_no_mineiro()` = 0x434d90 (bra_reg_minas_gerais.cpp, thiscall): Mineiro, Paranaense e Carioca: os pontos corridos como no Centro-Oeste; os dois turnos (fases 0 e 1) não gravam nada e devolvem -1; o mata-mata final (fase 2, uma rodada só: peso 40 para o eliminado) dá o campeão (destino 5) e o vice (destino -2);
- `cqk7_destino_na_nsl()` = 0x411980 (aus_nsl.cpp, thiscall): NSL: no fim da temporada regular, os seis classificados para a fase final ganham o registro de classificação (fase 0x435, peso 0x28); na fase final, quem avança ganha o registro (rodada 0x96) e quem cai o de não classificado; o campeão (grande final) ganha o registro de campeão. Não grava destino na tabela;
- `cqk8_progresso_no_mineiro()` = 0x4350d0 (bra_reg_minas_gerais.cpp, thiscall): Mineiro, Paranaense e Carioca: grava os códigos de progresso (posição atual, mínima e máxima) do clube; nos pontos corridos, a mínima abaixo de 3 vira 1 e a máxima abaixo de 3 vira 2 [inferência: os dois turnos têm tabelas próprias e a geral não dá título];
- `cqk8_progresso_na_mls()` = 0x909dd0 (usa_mls.cpp, thiscall): MLS: na temporada regular e nas fases 0 e 1 dos play-offs, a posição atual 1, 2, 3, 4 vira 1, 3, 7, 10, a mínima 1 a 3 vira 1 e 4 vira 10, a máxima 1 a 3 vira 7 e 4 vira 10 (posições em escala da liga inteira [inferência]); na fase 2, cada código vira 2 × código − 1; depois, como vêm;
- `cqk8_progresso_na_serie_c()` = 0x42e100 (bra_nat_third.cpp, thiscall): Série C: nos grupos (fases até 6), a posição no grupo vira a posição na escala das 49 vagas da Série C [inferência]: 1 e 2 → 1 (a máxima → 9), 3 → 17, 4 → 25, 5 → 33, 6 → 41, 7 → 49; nas fases seguintes, os códigos como vêm;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (39):** 33 acusadas. `cqj12_europa_uma_semana_antes` escapou: o dia do começo só vale para as eliminatórias da Copa de 2006, que começam depois de 2002-06-01 (fora da carreira de prova; confirmada com 0 divergências no modo comparação) [inferência]. `cqj12_america_do_sul_uma_semana_antes` escapou: o dia do começo só vale para as eliminatórias da Copa de 2006, que começam depois de 2002-06-01 (fora da carreira de prova; confirmada com 0 divergências no modo comparação) [inferência]. `cqj12_asia_uma_semana_antes` escapou: o dia do começo só vale para as eliminatórias da Copa de 2006, que começam depois de 2002-06-01 (fora da carreira de prova; confirmada com 0 divergências no modo comparação) [inferência]. `cqk10_oceania_uma_semana_depois` escapou: o dia do começo só vale para as eliminatórias da Copa de 2006, que começam depois de 2002-06-01 (fora da carreira de prova; confirmada com 0 divergências no modo comparação) [inferência]. `cqk11_campeao_na_vaga_1` escapou: a vaga do campeão da Libertadores no mundial de clubes só é lida quando o mundial é jogado, depois de 2002-06-01 (0 divergências no modo comparação) [inferência]. `cqk7_classificado_estadual` escapou: o registro de "classificou-se" vai só para a história dos treinadores, que a simulação não lê até a data (0 divergências no modo comparação) [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (d2c61798).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original 4bce14f7 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 273ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 274`

**Campos tocados:**
- liga do grupo +0xc2/+0xc3 pontos, +0xc4 categoria, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas, +0x3c turnos, +0x42 tipo (2 clubes internacionais), +0x43, +0x49 reservas no banco, +0x4a substituições, +0x28 prêmios, +0xd9 bits de regra, +0xb6 a data de começo (a primeira palavra da Date), +0xb8, +0xba calendário; valor/ponteiro; grava (as três)
- competição +0x40 ano; valor; lê (as três)
- liga da eliminatória +0xc2/+0xc3 pontos, +0xc4 categoria, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas, +0x3c turnos, +0x43, +0x49/+0x4a banco e substituições, +0xa7, +0xab, +0xdb, +0xb6 a data de começo, +0xb8, +0xba calendário; valor/ponteiro; grava (as três)
- eliminatória sul-americana +0xee/+0xef nações isentas [inferência]; [0x9cf76c] a competição de onde vem o +0xef; valor/id; lê e grava +0xef (cqj12_eliminatoria_sul_americana_regras)
- liga do grupo +0xc2/+0xc3 pontos, +0xc4 categoria, +0xc5..+0xc8 desempates, +0xbe..+0xc1 vagas, +0x3c turnos, +0x43 (2 seleções), +0x49/+0x4a banco e substituições, +0xb6 a data, +0xb8, +0xba calendário; valor/ponteiro; grava (as três)
- competição +0x40 ano; valor; lê (as três)
- clube +0x1db competição continental da temporada; id; lê (cqk11_libertadores_classificados)
- Libertadores +0xee lista dos classificados (ponteiros), +0xf2 quantos (32); ponteiro/valor; grava (cqk11_libertadores_classificados)
- Mundial Interclubes ([0x9cf6e4]) +0xa7 times (6 bytes; vaga 0 e 1), +8 estatísticas; nação +0x71 continente (+0 id), [0x9cfa1c] continente sul-americano; ponteiro/id; lê e grava a vaga (cqk11_libertadores_registros)
- competição +0x2c fases, +0x30 fase atual; valor; lê e grava +0x30 (cqk11_libertadores_fase_seguinte)
- competição +0x2c fases, +0x30 fase atual, +0xc fases (a 3: o mata-mata); valor/ponteiro; lê e grava +0x30 (cqk12_africana_fase_seguinte, cqk12_africana_historico)
- Copa Africana +0xee lista das seleções sorteadas (ponteiros), +0x3e times, +0xb1 tabela (linhas de 0x3b bytes: +0 seleção, +4 posição); ponteiro/valor; lê e grava (cqk12_africana_grupo_a)
- liga +0x3c: turnos; valor (palavra); grava (as três)
- liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava
- liga +0xc4: categoria copiada em cada partida (2 = liga, vai para o +0x3a da partida); valor; grava
- liga +0x42: tipo da competição (1 = clube nacional); valor; grava
- liga +0xc5..+0xc7: desempates (1 saldo, 2 gols pró, 3 confronto direto, 4 gols fora, 5 vitórias); valor; grava
- liga +0xbe / +0xbf / +0xc0 / +0xc1: vagas de cima, play-off de cima, play-off de baixo, rebaixados; valor; grava
- liga +0x1c / +0x20: liga de cima / de baixo; id (chave fixa, -1 = nenhuma); grava
- liga +0xd9: palavra de bits de regra (2 acesso, 8 fase seguinte); valor (palavra); grava
- liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava
- liga +0x52: marcas da fase (2 = numeração fixa do elenco); valor; grava
- liga +0xa9 / +0x3a: saídas do calendário (rodadas, código do estágio); valor (palavra); grava (pelo slot 16)
- liga +0xba: calendário (rodadas de 0x41 bytes); ponteiro; grava
- [0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: chaves das Séries A, B e C; id; lê
- liga +0x3c: turnos; valor (palavra); grava
- liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava
- liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava
- liga +0x42: tipo da competição (1 = clube nacional); valor; grava
- liga +0xc5..+0xc7: desempates (3 confronto direto, 1 saldo, 0 nenhum); valor; grava
- liga +0xbe..+0xc1: vagas de cima, play-offs de cima e de baixo (0), rebaixados; valor; grava
- liga +0x1c / +0x20: liga de cima / de baixo (-1 = nenhuma); id; grava
- liga +0xd9: palavra de bits de regra (8 = fase seguinte); valor (palavra); grava
- liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava
- liga +0xab / +0xdb: palavras copiadas em cada partida (+0x38 e +0x36 da partida) [inferência: códigos de estágio]; valor; grava 0
- liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (pelo slot 16)
- liga +0x3c: turnos (2); valor (palavra); grava
- liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava
- liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava
- liga +0x42: tipo da competição (1 = clube nacional); valor; grava
- liga +0xc5..+0xc7: desempates (3 confronto direto, 1 saldo, 0 nenhum); valor; grava
- liga +0xbe..+0xc1: vagas de cima, play-offs de cima e de baixo (0), rebaixados; valor; grava
- liga +0x1c / +0x20: liga de cima / de baixo (-1 = nenhuma); id; grava
- liga +0xd9: palavra de bits de regra (8 = fase seguinte); valor (palavra); grava
- liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava
- liga +0xab / +0xdb: palavras copiadas em cada partida (+0x38 e +0x36 da partida) [inferência: códigos de estágio]; valor; grava 0
- liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (pelo slot 16)
- liga +0x3c: turnos; valor (palavra); grava
- liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava
- liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava
- liga +0x42: tipo da competição (1 = clube nacional); valor; grava
- liga +0xc5..+0xc8: desempates (1 saldo, 2 gols pró, 3 confronto direto, 4 gols fora); valor; grava
- liga +0xbe..+0xc1: vagas de cima, play-offs de cima e de baixo, rebaixados; valor; grava
- liga +0x1c / +0x20: liga de cima / de baixo (chave fixa, -1 = nenhuma); id; grava
- liga +0xd9: palavra de bits de regra (8 = fase seguinte); valor (palavra); grava (Goiano e J1)
- liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava
- liga +0x52: marcas da fase (2 = numeração fixa); valor; grava (J1)
- liga +0xab / +0xdb: palavras copiadas em cada partida (+0x38 e +0x36 da partida) [inferência: códigos de estágio]; valor; grava (0x82 e 0 na J-League, 0 e 0 no Goiano)
- liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (pelo slot 16)
- [0x9cf65c] / [0x9cf768]: chaves da J1 e da J2; id; lê
- liga +0x3c: turnos; valor (palavra); grava
- liga +0xa7: jogos por time na temporada; valor (palavra); grava (MLS: 32)
- liga +0xc2 / +0xc3: pontos por vitória / por empate; valor; grava
- liga +0xc4: categoria copiada em cada partida (2 = liga); valor; grava
- liga +0x42: tipo da competição (1 = clube nacional); valor; grava
- liga +0xc5..+0xc8: desempates (1 saldo, 2 gols pró, 3 confronto direto, 4 gols fora); valor; grava
- liga +0xbe..+0xc1: vagas de cima, play-offs, rebaixados; valor; grava
- liga +0x1c / +0x20: liga de cima / de baixo (-1 = nenhuma); id; grava
- liga +0xd9: palavra de bits de regra (0x8 fase seguinte, 0x20 eliminados, 0x40 ordem fixa); valor (palavra); grava
- liga +0x49 / +0x4a: reservas no banco / substituições; valor; grava (NSL)
- liga +0x52: marcas da fase (2 = numeração fixa); valor; grava (NSL)
- liga +0xab: palavra copiada em cada partida (+0x38 da partida) [inferência: código de estágio]; valor; grava (0 na NSL, 0xc2 na MLS)
- liga +0xa9 / +0x3a / +0xba: saídas do calendário e o calendário; valor / ponteiro; grava (NSL, pelo slot 16)
- liga +0xc: vetor de estágios; ponteiro; lê o primeiro (os dois históricos)
- liga +4: registro da competição no banco; ponteiro; lê (vai nos registros de carreira)
- liga +0xb1 / +0x3e: tabela (linhas de 0x3b) e quantos times; ponteiro / valor; lê
- linha da tabela +0: clube; ponteiro; lê
- linha da tabela +0x37: destino (5 campeão, 0 acesso, -2 eliminado); valor; grava
- liga +0xc: vetor de estágios; ponteiro; lê os estágios 0..6 (Série C)
- jogo +0x30 / +0x32 / +0x34: rodada, fase e a rodada da escala; valor (palavra); lê (Baiano)
- [0xdd7f00]: Staff_Records; ponteiro; lê
- liga +4: registro da competição no banco; ponteiro; lê (vai nos registros de carreira)
- liga +0xb1 / +0x3e: tabela (linhas de 0x3b) e quantos times; ponteiro / valor; lê
- linha da tabela +0: clube; ponteiro; lê
- linha da tabela +0x37: destino (5 campeão, -2 eliminado); valor; grava (estaduais)
- jogo +0x30 / +0x32 / +0x34: rodada, fase e a rodada da escala; valor (palavra); lê
- [0xdd7f00]: Staff_Records; ponteiro; lê
- liga +8: Comp_Stats (estatísticas da competição); ponteiro; lê
- entrada do Comp_Stats +0x73 / +0x74 / +0x75: progresso do clube (posição atual, mínima, máxima); valor; grava
- Comp_Stats +0x18: marca de ordem da tabela; valor; lê e grava (pela 0x4a4850)

**Interessa a:** CMRTE
