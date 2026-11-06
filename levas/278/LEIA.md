# 278ª leva — o mata-mata dos estaduais e regionais: Gaúcho, Centro-Oeste e Goiano; o desempate da Série A e da Série B italianas no fim da temporada (o "spareggio"); o play-off de acesso e o play-out da Série C1 italiana; a passagem de fase da Third Division inglesa; os construtores das quatro divisões inglesas; os construtores da Série A, da Série B e dos dois grupos da Série C1 italianas; os construtores da segunda e da terceira divisões francesas e da Primera e da Segunda espanholas; a divisão de baixo espanhola (o construtor e a tabela) e a tabela da Segunda espanhola; as ligas de fundo francesas: a CFA e a divisão de baixo (os construtores, as regras e os destrutores); o calendário real de 2001/02 da Série A e da Série B italianas; o calendário real de 2001/02 dos dois grupos da Série C1 italiana; os tocos que devolvem 1

**Rotinas:**
- `cqn9_mata_mata_do_gaucho()` = 0x4325b0 (bra_reg_gaucho.cpp, thiscall, ret 0): O Gaúcho: depois dos pontos corridos, os 4 primeiros vão às semifinais: 1º × 4º, 2º × 3º (o mais bem colocado é a semente; a ordem dos jogos segue as sementes [inferência: ida e volta, e a decisão em casa do mais bem colocado]);
- `cqn9_mata_mata_do_centro_oeste()` = 0x4312d0 (bra_reg_central.cpp, thiscall, ret 0): O Centro-Oeste: depois dos pontos corridos, os 8 primeiros vão às quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (as sementes são as posições [inferência: ida e volta]);
- `cqn9_mata_mata_do_goiano()` = 0x433930 (bra_reg_goias.cpp, thiscall, ret 0): O Goiano: depois dos pontos corridos, os 8 primeiros vão às quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (as sementes são as posições [inferência: ida e volta]);
- `desempate_da_serie_a()` = 0x642880 (ita_ser_a.cpp, thiscall, ret 8): o dia da Série A; no fim da temporada, dois clubes empatados em pontos em faixas diferentes (o título; as vagas da Liga dos Campeões da Itália; as da Copa da UEFA; as duas seguintes [inferência: a Intertoto]; os rebaixados, os últimos +0xc1 de 18) não se separam pelo saldo: o primeiro par achado de cima para baixo é ordenado pelo confronto direto e vai a um jogo extra (a fase seguinte); sem empate na fronteira, a temporada fecha;
- `desempate_da_serie_b()` = 0x647050 (ita_ser_b.cpp, thiscall, ret 8): o dia da Série B; as faixas fixas são o campeão, mais 3 vagas de acesso (4 sobem) e os 4 rebaixados de 20; cada par empatado numa fronteira é ordenado pelo confronto direto e, havendo algum, o jogo de desempate;
- `ordem_dos_empatados_na_italia()` = 0x6471a0 (ita_ser_b.cpp, thiscall, ret 4): os clubes com os mesmos pontos são ordenados pelo confronto direto entre eles (a minitabela 0x682b30) e as posições da tabela são renumeradas;
- `resultado_do_desempate_da_serie_a()` = 0x641dc0 (ita_ser_a.cpp, thiscall, ret 0x18): na liga, os registros da base; no jogo de desempate, o vencedor (situação 1 [inferência]) fica à frente do outro na tabela (trocam de linha se estava abaixo) e, sem play-off pendente, a temporada fecha;
- `fase_seguinte_da_c1a()` = 0x64b9e0 (ita_ser_c1a.cpp, thiscall, ret 0): depois da liga, a fase atual sobe; na primeira, o play-off e o play-out são criados; além da última, a caixa de erro;
- `fases_do_fim_da_c1a()` = 0x64ba70 (ita_ser_c1a.cpp, thiscall, ret 0): a regra da C1/A (18 clubes): o campeão sobe direto e entra na Supercopa da Série C1; 2º × 5º e 3º × 4º jogam o play-off de acesso (a segunda vaga); 14º × 17º e 15º × 16º jogam o play-out (quem perde cai com o 18º [inferência]); cada confronto é uma fase de copa com o calendário da própria liga;
- `fase_seguinte_da_c1b()` = 0x64fba0 (ita_ser_c1b.cpp, thiscall, ret 0): a mesma da C1/A;
- `fases_do_fim_da_c1b()` = 0x64fc30 (ita_ser_c1b.cpp, thiscall, ret 0): a mesma regra na C1/B;
- `fase_seguinte_da_third()` = 0x578330 (eng_third.cpp, thiscall, ret 0): depois da liga da Third Division, a fase atual sobe e, na primeira, o play-off de acesso é criado (0x5783c0);
- `construir_premier()` = 0x573380 (eng_prm.cpp, thiscall, ret 8): a Premier: a regra de disciplina 9 (a das ligas inglesas), estádio mínimo de 10.000 lugares todos sentados, só a liga (sem play-off); sem estado salvo, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas;
- `construir_first()` = 0x571680 (eng_first.cpp, thiscall, ret 8): a First Division: disciplina 9, sem estádio mínimo, a liga e 1 fase (o play-off de acesso);
- `construir_second()` = 0x5754a0 (eng_second.cpp, thiscall, ret 8): a Second Division: disciplina 9, sem estádio mínimo, a liga e o play-off;
- `construir_third()` = 0x576dd0 (eng_third.cpp, thiscall, ret 8): a Third Division: disciplina 9, estádio mínimo de 6.000 lugares com 1.000 sentados, a liga e o play-off;
- `construir_serie_a()` = 0x6406d0 (ita_ser_a.cpp, thiscall, ret 8): a Série A: a regra de disciplina 0x11 (a das ligas italianas), só a liga (o desempate nasce no fim da temporada); sem estado salvo, as regras, a tabela, o calendário real de 2001/02 pelo nome dos clubes, os jogos, a ordem, as estatísticas e as posições previstas;
- `construir_serie_b()` = 0x645810 (ita_ser_b.cpp, thiscall, ret 8): a Série B: disciplina 0x11, só a liga, o calendário real de 2001/02;
- `construir_c1a()` = 0x64aa70 (ita_ser_c1a.cpp, thiscall, ret 8): a C1/A: disciplina 0x11, a liga e 2 fases (o play-off de acesso e o play-out), o calendário real de 2001/02;
- `construir_c1b()` = 0x64ec20 (ita_ser_c1b.cpp, thiscall, ret 8): a C1/B, como a C1/A;
- `construir_franca_2()` = 0x5be330 (fra_second.cpp, thiscall, ret 8): a segunda divisão francesa: a regra de disciplina 0xc (a francesa), só a liga; sem estado salvo, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas;
- `construir_franca_3()` = 0x5bfb40 (fra_third.cpp, thiscall, ret 8): a terceira divisão francesa (o National): disciplina 0xc, só a liga;
- `construir_segunda()` = 0x8539a0 (spa_second.cpp, thiscall, ret 8): a Segunda espanhola: a regra de disciplina 0x1a (a espanhola), só a liga;
- `construir_primera()` = 0x84e130 (spa_first.cpp, thiscall, ret 8): a Primera: disciplina 0x1a, só a liga; por fim, se a nação não tem o bit 4 de +0x11c [inferência: a divisão de baixo jogável], cria a divisão de baixo espanhola (os times B e os melhores da Segunda B) em +0xee;
- `construir_divisao_de_baixo_espanhola()` = 0x851020 (spa_lower.cpp, thiscall, ret 4): a divisão de baixo espanhola é uma liga de fundo sem disciplina (0xff), na competição de id [0x9cf6b0]; sem estado salvo, as regras, a tabela e os jogos (sem estatísticas nem posições previstas);
- `tabela_da_segunda()` = 0x855b40 (spa_second.cpp, thiscall, ret 0): a Segunda tem 22 clubes (os da competição, na ordem do banco); o time B de um clube da Primera fica marcado como não pode subir (destino −4); o fundo de prêmios parte de 50.000;
- `tabela_da_divisao_de_baixo_espanhola()` = 0x8533d0 (spa_lower.cpp, thiscall, ret 0): a divisão de baixo tem 22 clubes: primeiro os times B espanhóis cujo clube de cima está na Primera, depois os da Segunda (na ordem do banco); o resto vem dos clubes da Segunda B [inferência: as competições [0x9cf62c..0x9cf638]] e da divisão genérica espanhola que não são times B, ordenados pela reputação e com os melhores embaralhados; cada clube posto na tabela guarda a competição em +0x60, mas a passada dos times B da Segunda zera o +0x60 de todo time espanhol que não entra nela, inclusive os da Primera já postos [observado na comparação];
- `regras_da_cfa()` = 0x5ba0a0 (fra_cfa.cpp, thiscall, ret 0): as regras das duas ligas de fundo francesas: turno e returno, 3 pontos por vitória e 1 por empate, os desempates 4, 1, 2 [inferência: a posição, o saldo, os gols], nenhuma vaga de acesso, play-off nem rebaixamento, os bits de regra 0x43 (o campeão, as vagas de cima de acesso e a ordem fixa das rodadas); a liga de cima fica com o id de [0x9cf624] (o mesmo global que a Segunda espanhola usa para a Primera) [inferência: um engano do original, sem efeito porque não há acesso];
- `construir_cfa()` = 0x5b91c0 (fra_cfa.cpp, thiscall, ret 4): a CFA (a quarta divisão francesa, de fundo): até 72 clubes, todos os da primeira e da segunda divisões [inferência: os times reserva] e até 34 da divisão genérica cuja competição de reserva é a CFA, sorteados em 4 grupos de 18; cada grupo ganha a tabela e os jogos (no calendário do jogo) e a tabela é solta;
- `construir_divisao_de_baixo_francesa()` = 0x5bd600 (fra_lower.cpp, thiscall, ret 4): a divisão de baixo francesa (de fundo): até 40 clubes, os do National e até 20 franceses da divisão genérica sem competição de reserva, sorteados em 2 grupos de 20;
- `destruir_cfa()` = 0x5b9460 (fra_cfa.cpp, thiscall, ret 0): solta a tabela, o calendário, as fases e o vetor delas e destrói a liga;
- `destruir_divisao_de_baixo_francesa()` = 0x5bd870 (fra_lower.cpp, thiscall, ret 0): o mesmo para a divisão de baixo;
- `calendario_fixo_da_serie_a()` = 0x642a70 (ita_ser_a.cpp, thiscall, ret 0): a Série A de 2001/02 não sorteia o calendário: os 306 jogos (34 rodadas de 9) vêm fixos, com a data e os dois clubes pelo nome; se algum nome não existe no banco, a lista inteira cai e a liga volta ao calendário sorteado;
- `calendario_fixo_da_serie_b()` = 0x647230 (ita_ser_b.cpp, thiscall, ret 0): a Série B de 2001/02: os 380 jogos (38 rodadas de 10) fixos, com a mesma regra;
- `calendario_fixo_da_c1a()` = 0x64be90 (ita_ser_c1a.cpp, thiscall, ret 0): a C1/A de 2001/02: os 306 jogos (34 rodadas de 9) fixos pelo nome dos clubes; faltando um nome no banco, a lista cai e a liga sorteia o calendário;
- `calendario_fixo_da_c1b()` = 0x650100 (ita_ser_c1b.cpp, thiscall, ret 0): a C1/B de 2001/02, com a mesma regra;
- `toco_um_com_dois()` = 0x5bb540 (fra_first.cpp, thiscall, ret 8): o toco do add_result das competições que não jogam partidas: devolve 1;
- `toco_um_com_um()` = 0x5bb550 (fra_first.cpp, thiscall, ret 4): o padrão do slot 25 das competições e do slot 6 do órgão disciplinar: devolve 1;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (28):** 27 acusadas. `cql4_serie_a_todo_empate` escapou: a mutação só muda algo quando dois vizinhos da tabela final da Série A empatam em pontos dentro da mesma faixa; na Série A de 2001/02 desta carreira não houve esse empate (o ramo rodou: a vizinha `cql4_serie_a_sem_zerar_o_historico` foi acusada em 2002-142), e o modo comparação também deu 0 divergências. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** adiado para a comparação acumulada da auditoria (decisão de 05/11/2026).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 1dedd9dc, original fd53022e (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 277ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 278`

**Campos tocados:**
- liga +0xb1: a tabela (linhas de 0x3b bytes, +0 o clube); ponteiro; lê
- liga +0xc: as fases (vetor de ponteiros); ponteiro; grava a primeira
- liga +0x30: a fase atual; valor; grava 0
- liga +0x40: o ano da temporada; valor; lê
- fase de copa +0xa7: a lista dos times (6 bytes, +0 o clube); ponteiro; lê (comparador)
- liga +0xdd fim da temporada, +0xe6 [inferência: a tabela fechada], +0x2c fases, +0xc vetor das fases, +0x4c marca do histórico; valor e ponteiro; lê e grava (desempate_da_serie_a, desempate_da_serie_b, resultado_do_desempate_da_serie_a)
- liga +0xc1 rebaixados; valor; lê (desempate_da_serie_a)
- linha da tabela +0x19 pontos, +4 posição, +0x37 destino, +0 clube; valor; lê e grava (as quatro)
- registro da competição +0x5d nação; ponteiro; lê (desempate_da_serie_a)
- partida +0x1c e +0x20 os dois clubes; ponteiro; lê (resultado_do_desempate_da_serie_a)
- liga +0x30 fase atual, +0x2c fases, +0xc vetor das fases; valor e ponteiro; lê e grava (as cinco)
- liga +0xbf vagas do play-off, +0x40 ano, +4 registro da competição; valor; lê (fases_do_fim_da_c1a/c1b)
- [[0xadadfc] + [0x9cf938] × 4] a Supercopa da Série C1, +0xa7 inscritos (6 bytes; +0 clube); ponteiro; grava o campeão (fases_do_fim_da_c1a/c1b)
- linha da tabela +0 clube; ponteiro; lê (fases_do_fim_da_c1a/c1b)
- liga +0 vtable, +4 registro da competição, +0x40 ano, +0x50 regra de disciplina, +0xe2/+0xe4 estádio mínimo (capacidade, sentados); valor e ponteiro; grava (os quatro)
- liga +0x44 fase (0xff = a liga), +0x30 fase atual, +0x2c fases, +0xc vetor das fases, +8 Comp_Stats; valor e ponteiro; grava (os quatro)
- liga +0 vtable, +4 registro da competição, +0x40 ano, +0x50 regra de disciplina; valor e ponteiro; grava (os quatro)
- liga +0x44 fase, +0x30 fase atual, +0x2c fases, +0xc vetor das fases, +8 Comp_Stats, +0x4c marca do histórico, +0xee (byte da classe na A e na B) [inferência: o desempate em curso]; valor e ponteiro; grava
- liga +0 vtable, +4 registro da competição, +0x40 ano, +0x50 regra de disciplina; valor e ponteiro; grava (os quatro)
- liga +0x44 fase, +0x30 fase atual, +0x2c fases, +0xc vetor das fases, +8 Comp_Stats; valor e ponteiro; grava
- Primera +0xee a divisão de baixo espanhola; ponteiro; grava (construir_primera)
- nação +0x11c bit 4 [inferência: a divisão de baixo jogável]; valor; lê (construir_primera)
- linha da tabela +0 clube, +0x37 destino (−4 = time B que não sobe); ponteiro e valor; grava (as duas tabelas)
- clube +0x57 competição (id no +0), +0x53 nação; ponteiro; lê e grava (a divisão de baixo muda +0x57 dos seus clubes para a genérica)
- clube +0x60 a competição de reserva [inferência: ClubReserveDivision]; ponteiro; grava (tabela_da_divisao_de_baixo_espanhola)
- liga +0x28 fundo de prêmios, +0xe1 marca; valor; grava (tabela_da_segunda)
- [0xae2350] [inferência: clubes criados depois]; valor; lê (tabela_da_divisao_de_baixo_espanhola)
- liga +0x3c turnos, +0xc2/+0xc3 pontos, +0xc4 categoria das partidas, +0xc5..+0xc7 desempates, +0xbe..+0xc1 vagas, +0x1c/+0x20 ligas de cima e de baixo, +0xd9 bits de regra, +0xba calendário, +0x42 tipo; valor; grava (regras_da_cfa)
- clube +0x57 competição, +0x53 nação, +0x60 competição de reserva; ponteiro; lê (os construtores)
- [0xae2350] [inferência: clubes criados depois]; valor; lê (os construtores)
- liga +0xea a lista dos jogos fixos (16 bytes cada; a conta antes, em −4); +8/+0xc os ids dos clubes, −1 sem achar; ponteiro e valor; grava (as duas)
- liga +0x3e clubes, +0xa9 rodadas; valor; lê (as duas)
- liga +0xea a lista dos jogos fixos (16 bytes cada; a conta antes, em −4); +8/+0xc os ids dos clubes, −1 sem achar; ponteiro e valor; grava (calendario_fixo_da_c1a/c1b)

**Interessa a:** CMRTE, SGE
