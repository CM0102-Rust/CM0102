# O calendário das copas inglesas: as rodadas da Copa da Liga, da Copa da Inglaterra e da Supercopa (o mesmo slot, a
# virtual +0x3c create_round_table; eng_cc_cup.cpp, eng_fa_cup.cpp, eng_charity.cpp). Frente das competições (Agente
# D, 04/11/2026), carreira de 8 ligas.
# - 0x56c570 (EngCcCup::create_round_table, 1394 bytes; 2 chamadas até 2002-06-01 com as 8 ligas: o construtor e a
#   atualização do primeiro dia): as 7 rodadas da Copa da Liga. Uma mutação.
# - 0x56f320 (EngFaCup::create_round_table, 1936 bytes; 2 chamadas): as 9 rodadas da Copa da Inglaterra. Uma mutação.
# - 0x56d640 (EngCharity::create_round_table, 309 bytes; 1 chamada, no construtor): o jogo da Supercopa. Uma mutação.
# Ajudantes comuns (cql10_rodada, cql10_tabela, cql10_gravados, cql10_comparar) com a tabela de cada copa
# (CQL10_COPA_DA_LIGA, CQL10_COPA_DA_INGLATERRA, CQL10_SUPERCOPA); as três têm o mesmo corpo (o byte da fase, as
# saídas, a alocação de 0x68 por rodada, a caixa) e só mudam na tabela, no arquivo/linha da caixa e nos campos
# calculados: a Copa da Liga com +0xb2 (rodadas 0 a 2), a da Inglaterra com +0xb3, +0xb4 e +0x3e (rodadas 2 a 5), sob
# um desvio (+0xb4 = 1). As tabelas foram extraídas da desmontagem (as duas datas e os campos de cada rodada, conferidos
# rodada a rodada: todos gravam o mesmo conjunto de campos; a da Inglaterra grava ainda +0x13 = 0 nas rodadas 0 a 6,
# que a data do jogo já deixou com o mando 0).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte). Dentro: 0x944e46 (push tamanho; add esp, 4 ->
# cdecl, o malloc), 0x521e60 (7 pushes; add esp, 0x1c -> cdecl: tabela, rodada, dia, mês, anos, dia da semana, ano da
# copa movsx de +0x40), 0x521eb0 (9 pushes; add esp, 0x24 -> cdecl: tabela, rodada, dia, mês, anos, dia da semana,
# turno, ano, mando), e a caixa (aviso_em: 0x944cff, 0x90d130, 0x944c9f, 0x5e8290 e [0xb67a34] = 0).
# Comparadores: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais; o
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas (+0..+0x10, +0x13..+0x22 e +0x5c..+0x67 de cada rodada)
# têm de ser iguais; a tabela da cópia é liberada (0x9452ca). Fora da fase −1 só a original (nenhuma grava nada).
# Determinismo: nada lê pilha, heap ou endereço como valor (a tabela nova só é escrita).
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cql10_copa_da_liga_tabela', 0x56c570, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql10_copa_da_inglaterra_tabela', 0x56f320, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql10_supercopa_tabela', 0x56d640, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das copas inglesas: as rodadas da Copa da Liga, da Copa da Inglaterra e da Supercopa',
        'the English cups\' calendar: the rounds of the League Cup, the FA Cup and the Charity Shield',
        [('cql10_copa_da_liga_tabela', 'EngCcCup::create_round_table', '0x56c570', 'eng_cc_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Liga inglesa tem 7 rodadas em jogo único, exceto a semifinal em ida e volta (21 dias entre os jogos): 1ª rodada sorteada em 23/7 e jogada em 22/8; 2ª sorteio 23/8, jogo 20/9; 3ª 28/9 e 28/10; 4ª 29/10 e 11/11; quartas 12/11 e 2/12; semifinal 3/12 e 27/1 do ano seguinte; final sorteio 18/2, jogo em 1/4 no estádio neutro [inferência: mando 4], datas levadas ao dia da semana de cada uma; os clubes nas copas europeias (+0xb2) só entram na 3ª rodada (1ª rodada 2b + 56 times, 2ª (32 − b) × 2, entram (12 − b) × 3 novos; na 3ª entram os b); categoria 4 até a 4ª rodada e 6 depois; sem prêmio por rodada (−1)',
          'rule: the English League Cup has 7 single-leg rounds, except the two-legged semi-final (21 days apart): 1st round drawn on 23/7 and played on 22/8; 2nd draw 23/8, match 20/9; 3rd 28/9 and 28/10; 4th 29/10 and 11/11; quarter-finals 12/11 and 2/12; semi-final 3/12 and 27/1 of the next year; final drawn 18/2, played on 1/4 at a neutral ground [inference: venue 4], dates moved to each one\'s weekday; clubs in Europe (+0xb2) only enter in the 3rd round (1st round 2b + 56 teams, 2nd (32 − b) × 2, (12 − b) × 3 new entrants; the b enter in the 3rd); category 4 up to the 4th round and 6 after; no prize per round (−1)'),
         ('cql10_copa_da_inglaterra_tabela', 'EngFaCup::create_round_table', '0x56f320', 'eng_fa_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Inglaterra tem 9 rodadas em jogo único com replay (+0x20 = 1, 10 dias depois) até as quartas: preliminar (código 0xdc) sorteio 9/10, jogo 16/10, 56 times; 1ª rodada 17/10 e 18/11; 2ª 19/11 e 9/12; 3ª 10/12 e 6/1; 4ª 7/1 e 27/1; 5ª 28/1 e 17/2; quartas 18/2 e 10/3; semifinal 11/3 e 8/4 em estádio próprio (mando 0x2a, sem replay); final 9/4 e 12/5 no estádio neutro; o número de times da 2ª à 5ª rodada depende das isenções (+0xb3) e da escolha de entrar na 5ª rodada (+0xb4 = 1); prêmio de vencer a rodada de 10 mil a 400 mil, e a final 2 milhões ao campeão e 1 milhão ao vice; categoria 4 até a 5ª rodada e 6 das quartas em diante',
          'rule: the FA Cup has 9 single-leg rounds with replays (+0x20 = 1, 10 days later) up to the quarter-finals: preliminary (code 0xdc) drawn 9/10, played 16/10, 56 teams; 1st round 17/10 and 18/11; 2nd 19/11 and 9/12; 3rd 10/12 and 6/1; 4th 7/1 and 27/1; 5th 28/1 and 17/2; quarter-finals 18/2 and 10/3; semi-final 11/3 and 8/4 at its own ground (venue 0x2a, no replay); final 9/4 and 12/5 at a neutral ground; the team counts from the 2nd to the 5th round depend on the exemptions (+0xb3) and on the choice of entering at the 5th round (+0xb4 = 1); prize for winning a round from 10 thousand to 400 thousand, and the final 2 million to the winner and 1 million to the runner-up; category 4 up to the 5th round and 6 from the quarter-finals'),
         ('cql10_supercopa_tabela', 'EngCharity::create_round_table', '0x56d640', 'eng_charity.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa inglesa (Charity Shield) é um jogo único entre 2 clubes, sorteado em 5/7 e jogado em 13/8 (levado ao dia da semana 6) no estádio neutro [inferência: mando 4], decidido nos pênaltis [inferência: desempate 1]; categoria 6; sem prêmio',
          'rule: the Charity Shield is a single match between 2 clubs, drawn on 5/7 and played on 13/8 (moved to weekday 6) at a neutral ground [inference: venue 4], decided on penalties [inference: tiebreak 1]; category 6; no prize')])
CAMPOS = [('copa +0x40 ano; valor; lê (as três)', 'cup +0x40 year; value; reads (all three)'),
          ('Copa da Liga +0xb2 clubes nas copas europeias (entram na 3ª rodada); valor; lê (cql10_copa_da_liga_tabela)',
           'League Cup +0xb2 clubs in European cups (enter in the 3rd round); value; reads (cql10_copa_da_liga_tabela)'),
          ('Copa da Inglaterra +0xb3 isenções, +0xb4 a escolha da entrada na 5ª rodada, +0x3e inscritos; valor; lê (cql10_copa_da_inglaterra_tabela)',
           'FA Cup +0xb3 exemptions, +0xb4 the fifth-round entry choice, +0x3e entrants; value; reads (cql10_copa_da_inglaterra_tabela)'),
          ('rodada de copa (0x68 bytes) +0..+6 datas, turno, +7 nome, +9 índice, +0xb/+0xd/+0xf sorteio e desempates, +0x13 mando, +0x17 categoria, +0x18..+0x1e times, jogos, novos, inscritos, +0x20..+0x22 replay, jogos por confronto, dias, +0x5c..+0x64 prêmios; valor; grava (as três)',
           'cup round (0x68 bytes) +0..+6 dates, time slot, +7 name, +9 index, +0xb/+0xd/+0xf draw and tiebreaks, +0x13 venue, +0x17 category, +0x18..+0x1e teams, matches, new, entrants, +0x20..+0x22 replay, legs, days, +0x5c..+0x64 prizes; value; writes (all three)'),
          ('[0xb67a34] grava 0 na caixa de erro', '[0xb67a34] writes 0 on the error box')]
INTERESSA = ['CMRTE']
