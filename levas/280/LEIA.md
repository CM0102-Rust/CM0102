# 280ª leva — o calendário da primeira divisão inglesa e da Premiership (as rodadas, o Natal e os play-offs); o calendário da Primera espanhola (38 rodadas, tabela do ano par e do ímpar, o Natal); a virada da temporada e a fase seguinte dos estaduais: Baiano, Centro-Oeste e Gaúcho; a virada da temporada e a fase seguinte dos estaduais: Goiano, Norte e Nordeste; a virada da temporada e a fase seguinte dos estaduais: Pernambucano, Catarinense e Paulista; o calendário da Segunda espanhola (42 rodadas, tabela do ano par e do ímpar); o calendário da divisão de baixo espanhola (42 rodadas, tabela do ano par e do ímpar, o Natal); o construtor da primeira divisão francesa (os marcadores, os clubes da CFA e da divisão de baixo e as ligas de fundo); os play-offs das Séries A e B: a fase seguinte e o mata-mata dos 4 primeiros; a virada do futebol brasileiro: a Série A, o acesso e o rebaixamento em cadeia e a Série B; a J1: a fase seguinte, a final entre os campeões dos dois turnos e a virada da temporada; as sedes dos torneios: a tabela padrão, a notícia das candidaturas e a carga do HostCountry.tmp

**Rotinas:**
- `calendario_da_primeira_inglesa()` = 0x571980 (eng_first.cpp, thiscall, ret 0x10): o calendário da Division One: as mesmas 46 datas da Division Two, com faixas de TV (em geral um ou dois jogos no dia 6 ou no 4, turno 1 ou 2); o mesmo Natal pelo dia da rodada de 16 de dezembro. Os play-offs de acesso (fase 0): semifinais de ida e volta (4 clubes, sorteio em 7 de maio, ida em 12, 4 dias entre os jogos) e a final em jogo único em 28 (dia da semana 0, turno 2) de maio [inferência: o mando 4 é o estádio neutro];
- `calendario_da_premiership()` = 0x573660 (eng_prm.cpp, thiscall, ret 0x10): o calendário da Premiership: 38 rodadas (20 clubes), de 19 de agosto a 20 de maio; o Natal (quatro rodadas de datas fixas) pelo dia em que cai a rodada de 16 de dezembro; sem play-offs (só a fase principal);
- `calendario_da_primera()` = 0x84e450 (spa_first.cpp, thiscall, ret 0x10): o calendário da Primera: 38 rodadas (20 clubes), quase todas no domingo (dia 6) com uma faixa de TV (um jogo no dia 5); duas tabelas, pela paridade do ano: no ano ímpar (2001/02) de 22 de agosto a 21 de maio, no par de 30 de agosto a 6 de junho; em cada uma, uma rodada de Natal com data fixa (entre 20 e 30 de dezembro, ou 3 de janeiro) escolhida pelo dia em que cai a rodada de meados de dezembro;
- `cqp7_virada_do_baiano()` = 0x42fe10 (bra_reg_bahia.cpp, thiscall, ret 0): A virada do Baiano: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp7_fase_seguinte_do_baiano()` = 0x42ffc0 (bra_reg_bahia.cpp, thiscall, ret 0): A fase seguinte do Baiano: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `cqp7_virada_do_centro_oeste()` = 0x431090 (bra_reg_central.cpp, thiscall, ret 0): A virada do Centro-Oeste: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp7_fase_seguinte_do_centro_oeste()` = 0x431240 (bra_reg_central.cpp, thiscall, ret 0): A fase seguinte do Centro-Oeste: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 8 primeiros vão às quartas de final (1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º); depois da última fase é erro (a caixa);
- `cqp7_virada_do_gaucho()` = 0x432370 (bra_reg_gaucho.cpp, thiscall, ret 0): A virada do Gaúcho: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp7_fase_seguinte_do_gaucho()` = 0x432520 (bra_reg_gaucho.cpp, thiscall, ret 0): A fase seguinte do Gaúcho: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `cqp8_virada_do_goiano()` = 0x4336f0 (bra_reg_goias.cpp, thiscall, ret 0): A virada do Goiano: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp8_fase_seguinte_do_goiano()` = 0x4338a0 (bra_reg_goias.cpp, thiscall, ret 0): A fase seguinte do Goiano: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 8 primeiros vão às quartas de final (1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º); depois da última fase é erro (a caixa);
- `cqp8_virada_do_norte()` = 0x435e80 (bra_reg_north.cpp, thiscall, ret 0): A virada do Norte: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp8_fase_seguinte_do_norte()` = 0x436030 (bra_reg_north.cpp, thiscall, ret 0): A fase seguinte do Norte: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `cqp8_virada_do_nordeste()` = 0x436e90 (bra_reg_northeast.cpp, thiscall, ret 0): A virada do Nordeste: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp8_fase_seguinte_do_nordeste()` = 0x437040 (bra_reg_northeast.cpp, thiscall, ret 0): A fase seguinte do Nordeste: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `cqp9_virada_do_pernambucano()` = 0x4394d0 (bra_reg_pern.cpp, thiscall, ret 0): A virada do Pernambucano: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp9_fase_seguinte_do_pernambucano()` = 0x439680 (bra_reg_pern.cpp, thiscall, ret 0): A fase seguinte do Pernambucano: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `cqp9_virada_do_catarinense()` = 0x43bd10 (bra_reg_santa.cpp, thiscall, ret 0): A virada do Catarinense: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp9_fase_seguinte_do_catarinense()` = 0x43bec0 (bra_reg_santa.cpp, thiscall, ret 0): A fase seguinte do Catarinense: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `cqp9_virada_do_paulista()` = 0x43cd40 (bra_reg_sp.cpp, thiscall, ret 0): A virada do Paulista: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual;
- `cqp9_fase_seguinte_do_paulista()` = 0x43cef0 (bra_reg_sp.cpp, thiscall, ret 0): A fase seguinte do Paulista: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa);
- `calendario_da_segunda_espanhola()` = 0x853c90 (spa_second.cpp, thiscall, ret 0x10): o calendário da Segunda: 42 rodadas (22 clubes), quase todas no domingo (dia 6); no ano ímpar (2001/02) uma tabela fixa de 22 de agosto a 4 de junho, com uma faixa de TV; no ano par, de 30 de agosto a 6 de junho, com três faixas de TV e uma rodada de Natal de data fixa escolhida pelo dia em que cai a rodada de meados de dezembro;
- `calendario_da_divisao_de_baixo_espanhola()` = 0x851260 (spa_lower.cpp, thiscall, ret 0x10): o calendário da divisão de baixo espanhola (de fundo): 42 rodadas, quase todas no domingo (dia 6); no ano ímpar (2001/02) de 16 de agosto a 16 de maio, no par de 30 de agosto a 6 de junho; em cada uma, uma rodada de Natal de data fixa escolhida pelo dia em que cai a rodada de meados de dezembro;
- `construir_primeira_francesa()` = 0x5bb160 (fra_first.cpp, thiscall, ret 8): monta a primeira divisão francesa: disciplina 0xc; duas competições-marcador (a CFA e a divisão de baixo), sem jogos próprios; numa carreira nova, todo clube cujo campeonato no banco é a CFA passa à divisão genérica com a CFA como competição de reserva [inferência: os times B dos clubes profissionais], e todo clube da divisão de baixo passa à genérica sem reserva; depois as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas; por fim monta a CFA (4 grupos de 18) e a divisão de baixo (2 grupos de 20), que deixam os seus jogos no calendário, e solta os dois objetos;
- `cqq7_fase_seguinte_da_serie_a()` = 0x42a950 (bra_nat_first.cpp, thiscall, ret 0): A Série A tem os pontos corridos (a fase −1) e o play-off (a fase 0): quando os pontos corridos acabam, a fase passa a 0 e o play-off é montado; depois da última fase é erro (a caixa);
- `cqq7_play_off_da_serie_a()` = 0x42a9e0 (bra_nat_first.cpp, thiscall, ret 0): O play-off da Série A: os 4 primeiros dos pontos corridos, 1º × 4º e 2º × 3º, num mata-mata (sem sementes: a ordem da lista) [inferência: semifinais e final em ida e volta] que decide o campeão brasileiro;
- `cqq7_fase_seguinte_da_serie_b()` = 0x42cdd0 (bra_nat_second.cpp, thiscall, ret 0): A Série B tem os pontos corridos (a fase −1) e o play-off (a fase 0): quando os pontos corridos acabam, a fase passa a 0 e o play-off é montado; depois da última fase é erro (a caixa);
- `cqq7_play_off_da_serie_b()` = 0x42ce60 (bra_nat_second.cpp, thiscall, ret 0): O play-off da Série B: os 4 primeiros dos pontos corridos, 1º × 4º e 2º × 3º, num mata-mata (sem sementes) que decide o campeão da Série B [inferência: os acessos à Série A saem dele];
- `cqq8_virada_da_serie_a()` = 0x42a640 (bra_nat_first.cpp, thiscall, ret 0): A virada do Brasil, comandada pela Série A: primeiro o acesso e o rebaixamento em cadeia; depois os inscritos da Copa do Brasil, a entrada dos 16 melhores clubes sem divisão nacional na Série C e as trocas dos estaduais com as suas divisões de baixo; o nível de divisão dos clubes da A e da B é limitado; a Série A é refeita (ano seguinte, regras, 28 clubes, jogos); e então viram a Série B, a Série C e os doze estaduais (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste, Paranaense, Pernambucano, Carioca, Catarinense e Paulista), com as expectativas das diretorias e a renovação dos árbitros;
- `cqq8_acesso_e_rebaixamento()` = 0x42ae90 (bra_nat_first.cpp, thiscall, ret 4): O acesso e o rebaixamento no Brasil: grava as posições finais da Série A, da Série B, da Série C e das sete fases da C; depois troca os clubes marcados entre A e B, entre B e C e entre a B e cada fase da C (os que sobem e os que descem saem das marcas do fim da temporada, sem número fixo aqui);
- `cqq8_virada_da_serie_b()` = 0x42cc10 (bra_nat_second.cpp, thiscall, ret 0): A virada da Série B: guarda a temporada nas estatísticas, apaga a tabela e as fases, passa ao ano seguinte e refaz as regras, a tabela com os 28 clubes da Série B, os jogos e a ordem; depois as posições previstas, as expectativas das diretorias e a renovação dos árbitros;
- `cqq9_fase_seguinte_da_j1()` = 0x6639c0 (jap_j1.cpp, thiscall, ret 0): A J1 tem dois turnos (as fases 0 e 1, cada um uma liga de pontos corridos) e a final (fase 2): a fase seguinte sobe a fase atual e, no fim do segundo turno, monta a final; depois da última fase é erro (a caixa);
- `cqq9_final_da_j1()` = 0x663a50 (jap_j1.cpp, thiscall, ret 0): A final da J1: o campeão do primeiro turno contra o do segundo, em dois jogos [inferência: ida e volta], sem sementes; ao contrário dos estaduais brasileiros, não confere se é o mesmo clube (a final é montada mesmo assim);
- `cqq9_virada_da_j1()` = 0x6636d0 (jap_j1.cpp, thiscall, ret 0): A virada do Japão, comandada pela J1: primeiro o acesso e o rebaixamento com a J2; depois a J1 é refeita com três fases (os dois turnos e a final), ano seguinte, regras, a tabela e os jogos; então vira a J2, e vêm as posições previstas, as expectativas das diretorias e a renovação dos árbitros;
- `tabela_das_sedes()` = 0x5f9970 (host_country.cpp, thiscall, ret 0): a tabela padrão das sedes: 57 entradas (Copa do Mundo de 1998 a 2026 de 4 em 4, Eurocopa e Olimpíadas de 2000 a 2028, Copa Africana de 2000 a 2014 de 2 em 2, Copa América de 1999 a 2009, Copa da Ásia de 2000 a 2016 e as edições avulsas), cada uma com até três candidaturas de duas nações, a escolhida (ou votação aberta/fechada, ou sede fixa) e o dia da decisão (10 de julho para a Copa do Mundo, 20 de julho ou 20 de setembro para as outras [inferência: o mês conta de 0]) mais um sorteio de −35 a +34 dias;
- `noticia_das_sedes()` = 0x5faec0 (host_country.cpp, thiscall, ret 4): a notícia das sedes que mudaram: com a votação aberta, as candidaturas (notícia 4); com a escolhida, as sedes (notícia 5); aos técnicos do continente da competição ou a todos; fora as Olimpíadas, a Copa do Mundo de 2006 na Alemanha e a escolhida da edição de daqui a 8 anos;
- `carregar_sedes()` = 0x5fb160 (host_country.cpp, thiscall, ret 0): lê o HostCountry.tmp (a conta e as entradas de 0x22 bytes num bloco novo); sem o arquivo 0 (o jogo novo monta a tabela padrão); erro de leitura com a caixa;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (31):** 31 acusadas.
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original 0c20b87d (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 279ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 280`

**Campos tocados:**
- tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava e lê (+0 da rodada de referência do Natal)
- rodada de copa (0x68 bytes); grava (os play-offs)
- liga +0x40 ano da temporada; valor; lê
- tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava e lê (+0 da rodada de referência do Natal)
- rodada de copa (0x68 bytes); grava (os play-offs)
- liga +0x40 ano da temporada; valor; lê
- liga +0x4c: [inferência] marca da temporada; valor; grava 0 (virada)
- liga +0xba: lista auxiliar; ponteiro; libera e zera (virada)
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê (virada)
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê e destrói (virada)
- liga +0x2c: o número de fases; valor; lê (fase seguinte)
- liga +0x30: a fase atual (−1 = pontos corridos); valor; lê e grava
- liga +0x40: o ano da temporada; valor; lê e grava + 1 (virada)
- liga +4: o registro da competição no banco; ponteiro; lê (árbitros)
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- liga +0x4c: [inferência] marca da temporada; valor; grava 0 (virada)
- liga +0xba: lista auxiliar; ponteiro; libera e zera (virada)
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê (virada)
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê e destrói (virada)
- liga +0x2c: o número de fases; valor; lê (fase seguinte)
- liga +0x30: a fase atual (−1 = pontos corridos); valor; lê e grava
- liga +0x40: o ano da temporada; valor; lê e grava + 1 (virada)
- liga +4: o registro da competição no banco; ponteiro; lê (árbitros)
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- liga +0x4c: [inferência] marca da temporada; valor; grava 0 (virada)
- liga +0xba: lista auxiliar; ponteiro; libera e zera (virada)
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê (virada)
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê e destrói (virada)
- liga +0x2c: o número de fases; valor; lê (fase seguinte)
- liga +0x30: a fase atual (−1 = pontos corridos); valor; lê e grava
- liga +0x40: o ano da temporada; valor; lê e grava + 1 (virada)
- liga +4: o registro da competição no banco; ponteiro; lê (árbitros)
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava e lê (+0 da rodada de referência do Natal)
- rodada de copa (0x68 bytes); grava (os play-offs)
- liga +0x40 ano da temporada; valor; lê
- tabela de rodadas de liga (0x41 bytes): +0 dia do ano e +2 ano relativo (palavras), +4 código ([inferência] o turno), +5 as 8 faixas de 7 bytes (valor, dia, turno, jogos; −1 fecha), +0x3d valor; grava e lê (+0 da rodada de referência do Natal)
- rodada de copa (0x68 bytes); grava (os play-offs)
- liga +0x40 ano da temporada; valor; lê
- liga +0x40 ano, +4 registro, +0x50 disciplina, +0xee/+0xf2 os marcadores da CFA e da divisão de baixo, +0x44 fase, +0x30 fase atual, +0x2c/+0xc fases, +8 estatísticas; valor e ponteiro; grava
- clube +0x57 competição, +0x60 competição de reserva; ponteiro; lê e grava
- marcador (Comp, 0xa3 bytes): +4 registro, +0x42 1, +0x50 0xff; grava
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a primeira
- liga +0x2c: o número de fases; valor; lê (fase seguinte)
- liga +0x30: a fase atual (−1 = pontos corridos); valor; lê e grava
- liga +0x40: o ano da temporada; valor; lê
- fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)
- liga +0x4c: [inferência] marca da temporada; valor; grava 0 (viradas)
- liga +0xba: lista auxiliar; ponteiro; libera e zera (viradas)
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê e destrói (viradas), lê as sete da Série C (cadeia)
- liga +0x30: a fase atual; valor; lê e grava −1 (viradas)
- liga +0x40: o ano da temporada; valor; lê e grava + 1 (viradas)
- liga +4: o registro da competição no banco; ponteiro; lê (árbitros)
- [0xadadfc] a tabela das competições; ponteiro; lê
- [0x9cf9ec] / [0x9cf9e0] / [0x9cf9ac]: ids das Séries A, B e C; id; lê
- [0x9cf9c0], [0x9cf998], [0x9cf9c8], [0x9cf9b4], [0x9cf9d4], [0x9cf994], [0x9cf9a4], [0x9cf9d0], [0x9cf9bc], [0x9cf9dc], [0x9cf9b0], [0x9cf9e4]: ids dos doze estaduais; id; lê
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- liga +0x2c: o número de fases; valor; lê (fase seguinte), grava 3 (virada)
- liga +0x30: a fase atual; valor; lê e grava
- liga +0xc: as fases (vetor de ponteiros); ponteiro; lê os dois turnos, grava a final; libera e realoca (virada)
- fase de liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê a primeira linha
- liga +0x40: o ano da temporada; valor; lê, grava + 1 (virada)
- liga +0x4c: [inferência] marca da temporada; valor; grava 0 (virada)
- liga +0xba: lista auxiliar; ponteiro; libera e zera (virada)
- liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê (virada)
- [0x9cf768] id da J2; id; lê
- [0xb74340] o gerente dos árbitros; ponteiro; lê
- fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)
- sedes [0xb5e134] +0 entradas de 0x22 bytes (+0 competição, +4 ano, +6/+0xa, +0xe/+0x12, +0x16/+0x1a as três candidaturas, +0x1e escolhida/estado, +0x1f dia da decisão, +0x21 intervalo), +4 conta (word); grava (tabela_das_sedes, carregar_sedes), lê (noticia_das_sedes)
- [0xae2c92] ano do jogo; [0xae23d0] competições (0x6b bytes, +0x59 continente); lê

**Interessa a:** CMRTE, SGE
