# O calendário das copas do Japão e dos EUA: as rodadas da Copa da Liga japonesa, da Copa do Imperador e da Copa dos EUA
# (o mesmo slot, a virtual +0x3c create_round_table; jap_j_cup.cpp, jap_emp_cup.cpp, usa_open_cup.cpp). Frente das
# competições (Agente D, 04/11/2026), carreira de 8 ligas (Japão e EUA jogáveis).
# - 0x665640 (JapJCup::create_round_table, 960 bytes; 2 chamadas até 2002-06-01 com as 8 ligas: o construtor e a
#   virada da temporada japonesa): as 5 rodadas da Copa da Liga japonesa. Uma mutação.
# - 0x661ff0 (JapEmpCup::create_round_table, 1280 bytes; 2 chamadas): as 7 rodadas da Copa do Imperador. Uma mutação.
# - 0x90b170 (UsaOpenCup::create_round_table, 947 bytes; 2 chamadas): as 5 rodadas da Copa dos EUA. Uma mutação.
# Ajudantes comuns (cqn11_rodada, cqn11_tabela, cqn11_gravados, cqn11_comparar; os da cql_10 com o prefixo do item) com a
# tabela de cada copa; as três têm o mesmo corpo da cql_10 e só mudam na tabela e no arquivo/linha da caixa (nenhum
# campo calculado). As tabelas foram extraídas da desmontagem (as duas datas e os campos de cada rodada).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte). Dentro: 0x944e46 (cdecl), 0x521e60 (7 pushes; add
# esp, 0x1c -> cdecl; o dia da semana −1 no sorteio da 1ª rodada da Copa da Liga japonesa), 0x521eb0 (9 pushes; add esp,
# 0x24 -> cdecl) e a caixa (aviso_em).
# Comparadores: lado a lado (as saídas e os bytes gravados das duas tabelas, +0..+0x10, +0x13..+0x22 e +0x5c..+0x67 de
# cada rodada; o [0xb67a34]); a tabela da cópia é liberada. Fora da fase −1 só a original.
# Determinismo: nada lê pilha, heap ou endereço como valor.
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqn11_copa_da_liga_japonesa_tabela', 0x665640, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqn11_copa_do_imperador_tabela', 0x661ff0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqn11_copa_dos_eua_tabela', 0x90b170, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das copas do Japão e dos EUA: as rodadas da Copa da Liga japonesa, da Copa do Imperador e da Copa dos EUA',
        'the Japanese and US cups\' calendar: the rounds of the J League Cup, the Emperor\'s Cup and the US Open Cup',
        [('cqn11_copa_da_liga_japonesa_tabela', 'JapJCup::create_round_table', '0x665640', 'jap_j_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Liga japonesa tem 5 rodadas, as quatro primeiras em ida e volta (7 dias entre os jogos): 1ª rodada (24 times) sorteada em 10/2 (sem dia da semana fixo) e jogada em 12/4; 2ª (16 times, entram 4) 20/4 e 5/7; quartas 13/7 e 30/8; semifinal 7/9 e 11/10; final em jogo único, sorteio 19/10, jogo 4/11 no estádio neutro [inferência: mando 4]; sem prêmio por rodada (−1); categoria 4 até as quartas e 6 depois',
          'rule: the J League Cup has 5 rounds, the first four over two legs (7 days apart): 1st round (24 teams) drawn on 10/2 (no fixed weekday) and played on 12/4; 2nd (16 teams, 4 enter) 20/4 and 5/7; quarter-finals 13/7 and 30/8; semi-final 7/9 and 11/10; single-match final, drawn 19/10, played 4/11 at a neutral ground [inference: venue 4]; no prize per round (−1); category 4 up to the quarter-finals and 6 after'),
         ('cqn11_copa_do_imperador_tabela', 'JapEmpCup::create_round_table', '0x661ff0', 'jap_emp_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa do Imperador tem 7 rodadas em jogo único com sorteio por sementes (+0xd = 0x83): 1ª rodada (64 times) sorteio 1/11, jogo 29/11; 2ª 30/11 e 6/12; 3ª (entram 16) 7/12 e 13/12; 4ª 14/12 e 20/12; quartas 21/12 e 23/12; semifinal 24/12 e 27/12; final sorteio 28/12, jogo 2/1 do ano seguinte no estádio neutro [inferência: mando 4]; sem prêmio por rodada (−1)',
          'rule: the Emperor\'s Cup has 7 single-leg rounds with a seeded draw (+0xd = 0x83): 1st round (64 teams) drawn 1/11, played 29/11; 2nd 30/11 and 6/12; 3rd (16 enter) 7/12 and 13/12; 4th 14/12 and 20/12; quarter-finals 21/12 and 23/12; semi-final 24/12 and 27/12; final drawn 28/12, played on 2/1 of the next year at a neutral ground [inference: venue 4]; no prize per round (−1)'),
         ('cqn11_copa_dos_eua_tabela', 'UsaOpenCup::create_round_table', '0x90b170', 'usa_open_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa dos EUA tem 5 rodadas em jogo único (+0x21 = 0): 2ª rodada (32 times, código 0x14) sorteio 1/6, jogo 14/6, com o bit 0x400 no sorteio; 3ª 15/6 e 25/6; quartas 26/6 e 9/8; semifinal 10/8 e 23/8; final sorteio 24/8, jogo 22/10 no estádio neutro [inferência: mando 4]; sem prêmio por rodada (−1); categoria 4 nas duas primeiras e 6 depois',
          'rule: the US Open Cup has 5 single-leg rounds (+0x21 = 0): 2nd round (32 teams, code 0x14) drawn 1/6, played 14/6, with bit 0x400 in the draw; 3rd 15/6 and 25/6; quarter-finals 26/6 and 9/8; semi-final 10/8 and 23/8; final drawn 24/8, played 22/10 at a neutral ground [inference: venue 4]; no prize per round (−1); category 4 in the first two and 6 after')])
CAMPOS = [('copa +0x40 ano; valor; lê (as três)', 'cup +0x40 year; value; reads (all three)'),
          ('rodada de copa (0x68 bytes) +0..+6 datas, turno, +7 nome, +9 índice, +0xb/+0xd/+0xf sorteio e desempates, +0x13 mando, +0x17 categoria, +0x18..+0x1e times, jogos, novos, inscritos, +0x20..+0x22 replay, jogos por confronto, dias, +0x5c..+0x64 prêmios; valor; grava (as três)',
           'cup round (0x68 bytes) +0..+6 dates, time slot, +7 name, +9 index, +0xb/+0xd/+0xf draw and tiebreaks, +0x13 venue, +0x17 category, +0x18..+0x1e teams, matches, new, entrants, +0x20..+0x22 replay, legs, days, +0x5c..+0x64 prizes; value; writes (all three)'),
          ('[0xb67a34] grava 0 na caixa de erro', '[0xb67a34] writes 0 on the error box')]
INTERESSA = ['CMRTE']
