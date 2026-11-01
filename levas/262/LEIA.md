# 262ª leva — os play-offs das divisões inglesas: o mata-mata da First, da Second e da Third; as divisões inglesas na tabela: os registros pela situação e a situação do clube nas estatísticas da First e da Third; a tabela da liga: as posições possíveis e a situação de cada clube, e a fase seguinte da First e da Second; a Liga dos Campeões: os registros pela fase alcançada, o sorteio das quartas e a lista dos clubes inscritos; as estatísticas das táticas: o resultado de cada partida na linha da tática do clube do computador, o capitão guardado e o capitão escolhido; o jogador a serviço da seleção: a seleção ou o time das estrelas que o tem na data, o jogador que serve à seleção do técnico e as competições das convocações; os registros de partida e de nome: a partida vista por um clube, a referência ao nome da pessoa e o código do resultado; os jogos remarcados: os jogos vizinhos do clube movidos um dia, os jogos perto da data da seleção e a notícia do jogo remarcado; a forma ruim do jogador e as notícias das multas: o incidente de forma com a multa da IA, a reação à multa e o retorno do treinador; os veículos da imprensa: o comentarista sorteado, o nome do veículo e a notícia da queixa do técnico à federação; a folha salarial e a janela dos ingressos: a folha do clube, as folhas refeitas na criação e os dias até a temporada; o dinheiro das transferências: a venda de jogador, a compra de jogador e as luvas; as receitas e a compensação: o prêmio em dinheiro, os direitos de TV e o salário pago

**Rotinas:**
- `mata_mata_da_first()` = 0x573080 (eng_first.cpp, thiscall, ret 0): monta a fase do mata-mata com o 6º, o 3º, o 5º e o 4º da tabela (chaves 6º × 3º e 5º × 4º), a tabela de rodadas da virtual +0x3c e +0x3a = 0xa0; põe a fase na primeira vaga (+0xc) e monta a rodada (0x51c800); sem memória, a caixa (linhas 0x2f0 e 0x306);
- `mata_mata_da_second()` = 0x576a70 (eng_second.cpp, thiscall, ret 0): como a da First (6º, 3º, 5º e 4º), com +0x3a = o extra da virtual +0x3c (linhas 0x2b6 e 0x2cc);
- `mata_mata_da_third()` = 0x5783c0 (eng_third.cpp, thiscall, ret 0): como a da Second, com o 7º, o 4º, o 6º e o 5º (chaves 7º × 4º e 6º × 5º; linhas 0x2be e 0x2d5);
- `registros_pela_situacao_inglesa()` = 0x576c50 (eng_second.cpp, thiscall, ret 0x18): na liga (fase 0xff): título (5) 0x868c50, acesso (0) 0x869480 com peso 0x64, vaga no mata-mata (1) 0x86bdd0 (0xa0, 0x82, 0x1e), rebaixamento (3) 0x86a1c0; no mata-mata, na linha do clube: venceu (1) o acesso com 0x32, +0x37 = 0 e saiu = 1; passou (0) 0x86bdd0 (0xa0, 0x96, 0xf); caiu (0xff, 2) +0x37 = 0xfe; devolve 0;
- `situacao_na_first()` = 0x573260 (eng_first.cpp, thiscall, ret 0x14): grava fase, melhor e pior na entrada do clube nas estatísticas (+0x73..+0x75): na liga, melhor de 3 a 6 vira 3 e pior de 3 a 6 vira 6; no mata-mata (fase 0), 1, 2 e 3 viram 3, 4 e 5;
- `situacao_na_third()` = 0x578660 (eng_third.cpp, thiscall, ret 0x14): como a da First, com as vagas 4 a 7 (4 e 7; no mata-mata, 4, 5 e 6);
- `posicoes_e_situacao_na_liga()` = 0x684cd0 (league.cpp, thiscall, ret 0): ordena a tabela (0x6827d0), marca as faixas (acesso, mata-mata, 2 e rebaixamento) de cima e de baixo, calcula a melhor (+0x39) e a pior (+0x3a) posição possível pelos máximos de pontos e decide a situação (+0x37): título (5), acesso (0, 7 ou 6), mata-mata (1), rebaixamento (3, com as cláusulas 0x4cd030), 2 e 0xfe, cada uma anunciada (+0x40); a vaga europeia ([0xae2d14]) pelas vagas da nação (0x905c60); a situação nas estatísticas (+0x68);
- `fase_seguinte_da_first()` = 0x572ff0 (eng_first.cpp, thiscall, ret 0): na última fase (+0x30 ≥ +0x2c − 1) a caixa (linha 0x2ce); senão +0x30 + 1 e, chegando a 0, o mata-mata (0x573080);
- `fase_seguinte_da_second()` = 0x5769e0 (eng_second.cpp, thiscall, ret 0): como a da First (linha 0x294; o mata-mata 0x576a70);
- `registros_da_liga_dos_campeoes()` = 0x584ae0 (european_cup.cpp, thiscall, ret 0x18): pelo desfecho e pela fase: na qualificação, a classificação (0x86bdd0) ou a eliminação (0x86c000) com o peso pela rodada (0x4bf850); nas fases de grupos, a classificação (6) ou a não classificação (0xfe, 0x86c1d0); no mata-mata, o título (0x86a800), o vice (0x86b0b0), a classificação ou a eliminação; quem sai fica com +0x1db = −1; devolve 0;
- `sorteio_das_quartas()` = 0x586330 (european_cup.cpp, thiscall, ret 0): sorteia a ordem dos 4 grupos da fase 2 (0x90cfc0) e monta as quartas (1º de a × 2º de b, 2º de a × 1º de b, idem c e d), zera os cartões dos 8 (0x7e9180), cria a fase do mata-mata (0x522e00) na vaga +0x30, tira o 3º e o 4º de cada grupo (+0x1db = −1), monta a rodada (0x51c800) e +0x30 = 12;
- `clubes_da_liga_dos_campeoes()` = 0x586660 (european_cup.cpp, thiscall, ret 8): aloca a lista (0x11c bytes) dos inscritos (+0x14) que seguem na copa (+0x1db = o id dela): tipo 1 sem os da rodada 7, tipo 2 todos, tipo 3 na fase 11; noutro caso a da competição (0x48e1c0); devolve a conta;
- `registrar_resultado_da_tatica()` = 0x896290 (tactics.cpp, thiscall, ret 4): no fim da partida sem humano, soma na linha da tática de cada clube (0x28 bytes) jogos, vitórias, empates, derrotas, gols e pontos (3 por vitória, 1 por empate) e refaz as três razões em double (+0x10/+0x18/+0x20) que a escolha da formação do computador lê;
- `capitao_guardado()` = 0x8976a0 (tactics.cpp, thiscall, ret 8): o capitão guardado (+0x52) na linha do clube e do time na tabela tática; sem linha, 0;
- `escolher_capitao()` = 0x897880 (tactics.cpp, thiscall, ret 0xc): grava o capitão (+0x52) na linha do clube e do time na tabela tática e devolve 1; sem linha, 0;
- `selecao_que_tem_o_jogador()` = 0x7757d0 (national_teams.cpp, thiscall, ret 0x14): a seleção que tem o jogador da véspera ao dia seguinte (jogo da seleção dele na janela, contra o adversário se dado, ou convocação já começada) ou o time das estrelas da MLS na data do jogo; põe o comentário; senão 0 (e 0 para clube seleção ou time das estrelas);
- `jogador_serve_a_selecao()` = 0x76d9b0 (national_teams.cpp, stdcall, ret 8): o jogador pode jogar pela seleção do técnico: nacionalidade, idade para a sub-21, convocação atual e a regra da segunda nacionalidade;
- `juntar_competicoes_das_convocacoes()` = 0x775580 (national_teams.cpp, thiscall, ret 4): acrescenta a uma lista cada competição das convocações que ela ainda não tem e devolve quantas;
- `registro_da_partida_pelo_clube()` = 0x7e4750 (record_utils.cpp, cdecl): preenche o registro da partida pelo clube: data, id do jogo, competição, lado, clube, adversário e o placar pró e contra (o da prorrogação, se houver);
- `referencia_ao_nome()` = 0x7e4920 (record_utils.cpp, cdecl): guarda os ids dos três nomes (−1 sem nome), o id e a data de nascimento da pessoa numa referência compacta;
- `codigo_do_resultado()` = 0x7e56d0 (record_utils.cpp, cdecl): o código do resultado do registro: 3 vitória, 2 empate com gols, 1 empate sem gols, −3 derrota;
- `mover_jogos_vizinhos_do_clube()` = 0x5ae9d0 (fix_man.cpp, thiscall, ret 4): jogo não amistoso: para cada clube de nação marcada (+0x11c), o jogo dele dois dias antes (com +0x43 = 0xff) vai um dia antes, e o de dois dias depois vai um dia depois, se a data nova estiver livre (0x5ad780); tira, junta e dá a notícia;
- `remarcar_jogos_perto_da_data()` = 0x5abb10 (fix_man.cpp, thiscall, ret 8): os jogos do clube de um dia antes a um dia depois da data da seleção: os que saem do calendário ganham data nova, voltam e dão a notícia com a data velha;
- `noticia_do_jogo_remarcado()` = 0x5aac70 (fix_man.cpp, thiscall, ret 0xc): a fila de notícias em +0x1128 (criada na primeira vez): troca a data nova da notícia do mesmo jogo, ou põe a notícia 1 com o jogo, a causa ou a data velha;
- `incidente_de_forma()` = 0x41add0 (awol.cpp, thiscall, ret 8): o registro (jogador, clube, hoje) na lista dos incidentes; clube humano: um treinador sorteado e a notícia 0x138f; clube da IA com técnico: sorteio(+0x14) acima de sorteio(10) leva à multa, prazo pela nota do técnico;
- `noticia_reacao_a_multa()` = 0x5a74b0 (Fine.cpp, stdcall, ret 0x14): jogador com contrato num clube humano: a notícia 0xbc6 com motivo, reação, sorteio(2), personalidade, prazo, tipo de contrato, transferência futura e efeitos;
- `noticia_retorno_do_treinador()` = 0x5a7f20 (Fine.cpp, stdcall, ret 0xc): jogador e treinador com ficha e nação, jogador com clube: a notícia 0x1394 com o desfecho e sorteio(5);
- `sortear_comentarista()` = 0x7dab90 (printouts.cpp, thiscall, ret 0xc): tipo 1 no domingo vira 7; tipos locais (2, 4): os comentaristas do clube; os outros: do tipo e da nação (−1 qualquer); até 20, sorteio(n) e o id, ou −1;
- `nome_do_veiculo()` = 0x7dad20 (printouts.cpp, thiscall, ret 0x10): o nome (+2) e a marca (+0xd3) do comentarista do id, ou o nome padrão do tipo de veículo e marca 0;
- `noticia_queixa_a_federacao()` = 0x758de0 (media.cpp, thiscall, ret 0x18): clube com nação e técnico: a notícia 0x178e (clube, técnico, tipo, pessoa, a, b, nação, c) aos técnicos da nação se a reputação passar de 4750, senão ao clube;
- `folha_salarial_do_clube()` = 0x598370 (finance.cpp, thiscall): soma os salários do elenco (menos a parte do outro clube nos emprestados) e do pessoal na folha (+0xc) e o maior salário (+0x10); com menos de 15 jogadores, as vagas que faltam por um salário pela reputação; mais reputação² × fator pelo tipo do clube e pela nação; o teto salarial (+0x14) pela 0x5968d0;
- `refazer_todas_as_folhas()` = 0x59cf50 (finance.cpp, thiscall): na criação do jogo, a folha de cada clube até a primeira seleção;
- `dias_ate_a_temporada()` = 0x59b4c0 (finance.cpp, thiscall, ret 8 (o ecx não é lido)): os dias até o começo da temporada da nação do clube, na janela dos ingressos (até dois meses antes); marca se o ano da temporada já chegou;
- `receber_venda_de_jogador()` = 0x599800 (finance.cpp, thiscall, ret 8): soma o valor (64 bits) às vendas do mês (+0x78) e do ano (+0x118) e ao saldo;
- `pagar_compra_de_jogador()` = 0x599890 (finance.cpp, thiscall, ret 8): soma o valor (64 bits) às compras do mês (+0x70) e do ano (+0x110) e o tira do saldo;
- `pagar_luvas()` = 0x5998e0 (finance.cpp, thiscall, ret 4): soma o valor às luvas do mês (+0x9c) e do ano (+0x13c) e o tira do saldo;
- `receber_premio_em_dinheiro()` = 0x5999a0 (finance.cpp, thiscall, ret 4): soma o valor aos prêmios do mês (+0xa8) e do ano (+0x148) e ao saldo;
- `receber_direitos_de_tv()` = 0x5987c0 (finance.cpp, thiscall, ret 4): soma o valor à TV do mês (+0xb8) e do ano (+0x158) e ao saldo;
- `pagar_salarios()` = 0x599920 (finance.cpp, thiscall, ret 4): soma o valor aos salários do mês (+0x90) e do ano (+0x130) e o tira do saldo (a compensação da liberação sem custo);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (34):** 31 acusadas. `cpz2_acesso_pesa_metade` escapou: o peso só entra nos registros dos técnicos, que ficam fora do banco e que a carreira só lê no técnico do mês e do ano; nenhuma escolha mudou até 2002-06-01 [inferência]. `cpz2_vaga_no_mata_mata_pesa_o_dobro` escapou: o peso só entra nos registros dos técnicos, que ficam fora do banco e que a carreira só lê no técnico do mês e do ano; nenhuma escolha mudou até 2002-06-01 [inferência]. `cpz4_fase_2_pesa_metade` escapou: o peso de passar da fase 2 da Liga dos Campeões só entra nos registros dos técnicos, que a carreira só lê no técnico do mês e do ano [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (81f1a1ed).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6299f556, original cbf229f9 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 261ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 262`

**Campos tocados:**
- liga +0xb1: a tabela (linhas de 0x3b bytes, o clube no +0); ponteiro; lê (mata_mata_da_first, mata_mata_da_second, mata_mata_da_third)
- liga +0xbf: quantos vão ao mata-mata; valor; lê (as três)
- liga +0xc: as vagas das fases; ponteiro; grava a primeira (as três)
- liga +4 / +0x40: o registro da competição e a palavra passada à fase; ponteiro e valor; lê (as três)
- fase (Cup_Stage) +0xa7 / +0x3e: a lista dos clubes (6 bytes) e a conta; ponteiro e valor; grava pelo construtor (as três)
- liga +0xb1 / +0x3e: a tabela (0x3b bytes; +0 o clube, +0x37 a situação) e as linhas; ponteiro e valor; lê, grava +0x37 (registros_pela_situacao_inglesa)
- liga +4: o registro da competição; ponteiro; lê (registros_pela_situacao_inglesa)
- liga +8: as estatísticas da competição (Comp_Stats); ponteiro; lê (situacao_na_first, situacao_na_third)
- entrada do clube nas estatísticas +0x73 / +0x74 / +0x75: a fase, a melhor e a pior situação; valor; grava (situacao_na_first, situacao_na_third)
- [0xdd7f00] os registros de staff; ponteiro; lê (registros_pela_situacao_inglesa)
- liga +0xb1 / +0x3e: a tabela (0x3b bytes: +0 clube, +0x11 jogos feitos, +0x19 pontos, +0x37 situação, +0x39 melhor posição, +0x3a pior) e as linhas; ponteiro e valor; lê, grava +0x37/+0x39/+0x3a (posicoes_e_situacao_na_liga)
- liga +0xa7 / +0x3c / +0x45: os jogos da temporada (grava se −1), os turnos e o calendário; valor; lê e grava (posicoes_e_situacao_na_liga)
- liga +0xbe / +0xbf / +0xc0 / +0xc1: as vagas de acesso, mata-mata, faixa 2 e rebaixamento; valor; lê (posicoes_e_situacao_na_liga)
- liga +0xc2 / +0xd9 / +0xdd / +0xe6 / +0x44 / +0x1c: pontos da vitória, regras da tabela, liga parada, desempate, fase-mãe, liga de acesso; valor; lê (posicoes_e_situacao_na_liga)
- [0xae2d14] + 6 × id do clube: a competição europeia da próxima temporada; id; lê e grava (posicoes_e_situacao_na_liga)
- liga +0x30 / +0x2c: a fase atual e quantas fases; valor; lê e grava (fase_seguinte_da_first, fase_seguinte_da_second)
- clube +0x1db: a competição europeia da temporada; id; lê (clubes_da_liga_dos_campeoes), grava −1 (registros_da_liga_dos_campeoes, sorteio_das_quartas)
- jogo +0x30 / +0x32 / +0x34: as palavras da rodada que vão aos registros; valor; lê (registros_da_liga_dos_campeoes)
- Liga dos Campeões +0xc / +0x30: as fases (+0x20..+0x2c os grupos da fase 2, +0x30 o mata-mata) e a fase atual; ponteiro e valor; lê, grava a vaga +0x30 e a fase 12 (sorteio_das_quartas)
- Liga dos Campeões +0x14 / +0x38: a lista dos inscritos (6 bytes: +0 clube, +4 rodada de entrada) e a conta; ponteiro e valor; lê (clubes_da_liga_dos_campeoes)
- Liga dos Campeões +0x3a / +0xa3 / +0x50: a fase da qualificação, a tabela de rodadas e o índice do órgão; valor e ponteiro; lê (registros_da_liga_dos_campeoes, sorteio_das_quartas)
- [0xde1d08] +0: a tabela de estatísticas das táticas (0x28 bytes: +0 a tática, +2 jogos, +4 vitórias, +6 empates, +8 derrotas, +0xa gols pró, +0xc contra, +0xe pontos, +0x10/+0x18/+0x20 as razões em double); ponteiro e valor; lê e grava (registrar_resultado_da_tatica)
- [0xde1d08] +0x601: a tabela tática dos clubes (0x91 bytes por linha: +0 a tática, +0x52 o capitão); ponteiro, valor e id; lê (registrar_resultado_da_tatica, capitao_guardado), grava o capitão (escolher_capitao)
- fixture +0x1c / +0x20: o mandante e o visitante; ponteiro; lê (registrar_resultado_da_tatica)
- registro da partida +0x19 / +0x1a: gols pró e contra pelo clube (0x7e4750); valor; lê (registrar_resultado_da_tatica)
- staff +0x24: a seleção que o convocou; ponteiro; lê (selecao_que_tem_o_jogador, jogador_serve_a_selecao)
- staff +0x5e bit 4: o lado nos jogos da seleção e se as convocações contam; valor; lê (selecao_que_tem_o_jogador)
- staff +0x39: o clube; ponteiro; lê (selecao_que_tem_o_jogador)
- staff +0x1a / +0x1e / +0x22 / +0x18 / +0x61: nacionalidades, os jogos pela seleção (com jogos, só a primeira nacionalidade vale), a idade e a ficha de jogador; ponteiro e valor; lê (jogador_serve_a_selecao)
- [0xae2d0c] registro de 79 bytes da pessoa +0xc bit 2: só pela primeira nacionalidade; valor; lê (jogador_serve_a_selecao)
- clube +0x57: a liga; ponteiro; lê (selecao_que_tem_o_jogador); clube +0xd7: os 50 nomes do time das estrelas; ponteiro; lê
- NationalTeams +0: a lista das convocações (+0 a seleção, +4 o adversário/competição, +8 a data); ponteiro; lê (selecao_que_tem_o_jogador, juntar_competicoes_das_convocacoes)
- [0x9d00a4] / [0x9d00a8] os dois times das estrelas; id; lê. [0x9cf590] a liga da MLS; id; lê. [0x9cf434] a nação da regra da segunda nacionalidade; id; lê
- fixture +0 / +8 / +0xc / +0x10 / +0x28 / +0x2a / +0x30 / +0x32: id, competição, os dois adversários, ano, dia e duas palavras; id, ponteiro e valor; lê (registro_da_partida_pelo_clube)
- fixture +0x43 / +0x44 / +0x47 / +0x48: o placar normal e o da prorrogação (−1 sem); valor; lê (registro_da_partida_pelo_clube)
- registro da partida (0x1f bytes): +0 data, +8 jogo, +0xc/+0xe, +0x10 lado, +0x11 clube, +0x15 adversário, +0x19 pró, +0x1a contra, +0x1b competição; valor e id; grava (registro_da_partida_pelo_clube), lê o placar (codigo_do_resultado)
- staff +4 / +8 / +0xc: os três nomes (registro de nome +0x33: o id); ponteiro; lê (referencia_ao_nome). staff +0x10: data de nascimento; valor; lê
- jogo (0x4f bytes) +0 id, +0xc, +0x10, +0x14 competição, +0x1c e +0x20 clubes, +0x28 ano e +0x2a dia, +0x30 e +0x32, +0x40 e +0x41, +0x43; id e valor; lê e grava a data na cópia local (mover_jogos_vizinhos_do_clube, noticia_do_jogo_remarcado)
- clube +0x53 nação; nação +0x11c marca das remarcações; ponteiro e valor; lê (mover_jogos_vizinhos_do_clube)
- FIXTURE_MANAGER +0x1128: fila das notícias de jogo remarcado (LLIST de itens de 0xdf bytes); ponteiro; cria e grava (noticia_do_jogo_remarcado)
- [0xae2c74] objeto dos amistosos; ponteiro; lê (mover_jogos_vizinhos_do_clube)
- AWOL +8: lista dos incidentes de forma (registro de 0x11 bytes: +0 jogador, +4 clube, +8 data, +0x10 byte); id e valor; grava (incidente_de_forma)
- clube +0x19f: as 5 vagas de treinador; +0xcf técnico; técnico +0x69 ficha (+0x14, +0x19); ponteiro e valor; lê (incidente_de_forma)
- pessoa +0x18 idade, +0x1a nação, +0x39 clube, +0x59, +0x5b, +0x5d personalidade, +0x61 ficha (+0xb reputação); valor; lê (as notícias das multas)
- contrato +0x4f tipo (nibble alto); [0xae1080] último contrato; valor e ponteiro; lê e grava [0xae1080] (noticia_reacao_a_multa)
- [0xae2a3c] o objeto das multas (Fine); ponteiro; lê (incidente_de_forma)
- PunditList +0 lista nacional, +4 lista local (LLIST de registros de 0xd4 bytes: +0 id, +2 nome, +0xca tipo, +0xcb clube, +0xcf nação, +0xd3 marca); id e valor; lê (sortear_comentarista, nome_do_veiculo)
- clube +0x69 estádio (+0x38 o clube do estádio), +0x53 nação, +0x80 reputação, +0xcf técnico; ponteiro e valor; lê (sortear_comentarista, noticia_queixa_a_federacao)
- [0xae2c90] hoje (o domingo); [0xb74430] a lista de comentaristas; valor e ponteiro; lê (sortear_comentarista)
- finanças do clube (0x167 bytes) +8 id do clube, +0xc folha semanal, +0x10 maior salário, +0x14 teto salarial; valores; grava (folha_salarial_do_clube); toda a tabela (refazer_todas_as_folhas)
- clube +0xd7 50 vagas do elenco, +0x19f/+0x1b3/+0x1cf pessoal (5, 7, 3), +0x53 nação, +0x64 tipo, +0x80 reputação; ponteiros/valores; lê (folha_salarial_do_clube)
- contrato (0x50 bytes) +0xc salário semanal (−1: padrão), +0x35 & 0x3f tipo; valores; lê (folha_salarial_do_clube)
- nação +0 id (Inglaterra 0x9cf2e4, Itália 0x9cf360), +0x7e tipo; valores; lê (folha_salarial_do_clube)
- [0xae1080] último contrato consultado; ponteiro; grava (pelo salário padrão 0x864550)
- linha da nação (72 bytes em 0xb63d60) +0x17 marca de 2002, +0x2a data do começo da temporada, +0x2c ano do começo, +0x34 mês do começo; valores; lê (dias_ate_a_temporada)
- [0xdd6ec8] data do começo do jogo (ano em +2), [0xae2c90] hoje; valores; lê (dias_ate_a_temporada)
- finanças do clube (0x167 bytes) +0 saldo (i64), +0x70/+0x110 compras (i64, mês/ano), +0x78/+0x118 vendas (i64, mês/ano), +0x9c/+0x13c luvas (mês/ano); valores; lê e grava
- finanças do clube (0x167 bytes) +0 saldo (i64), +0x90/+0x130 salários, +0xa8/+0x148 prêmios, +0xb8/+0x158 TV (mês/ano); valores; lê e grava

**Interessa a:** CMRTE, SGE
