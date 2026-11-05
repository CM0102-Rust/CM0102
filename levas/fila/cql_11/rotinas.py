# O calendário das copas da França e da Espanha: as rodadas da Copa da França, da Copa da Liga francesa e da Copa do
# Rei (o mesmo slot, a virtual +0x3c create_round_table; fra_cup.cpp, fra_lge_cup.cpp, spa_cup.cpp). Frente das
# competições (Agente D, 04/11/2026), carreira de 8 ligas (França e Espanha jogáveis).
# - 0x5ba3f0 (FraCup::create_round_table, 1631 bytes; 1 chamada até 2002-06-01 com as 8 ligas, no construtor): as 9
#   rodadas da Copa da França. Uma mutação.
# - 0x5bcc40 (FraLgeCup::create_round_table, 1300 bytes; 1 chamada, no construtor): as 7 rodadas da Copa da Liga
#   francesa. Uma mutação.
# - 0x84d220 (SpaCup::create_round_table, 1299 bytes; 2 chamadas: o construtor e a atualização do primeiro dia): as 7
#   rodadas da Copa do Rei. Uma mutação.
# Ajudantes comuns (cql11_rodada, cql11_tabela, cql11_gravados, cql11_comparar; os mesmos da cql_10, com o prefixo do
# item) com a tabela de cada copa. As três têm o mesmo corpo (o byte da fase, as saídas, a alocação de 0x68 por rodada,
# a caixa) e só mudam na tabela e no arquivo/linha da caixa; nenhum campo calculado. As tabelas foram extraídas da
# desmontagem (as duas datas e os campos de cada rodada; todas gravam o mesmo conjunto de campos).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte). Dentro: 0x944e46 (push tamanho; add esp, 4 ->
# cdecl), 0x521e60 (7 pushes; add esp, 0x1c -> cdecl; o dia da semana −1 na 1ª rodada da Copa da França), 0x521eb0 (9
# pushes; add esp, 0x24 -> cdecl), e a caixa (aviso_em).
# Comparadores: lado a lado (as saídas e os bytes gravados das duas tabelas, +0..+0x10, +0x13..+0x22 e +0x5c..+0x67 de
# cada rodada; o [0xb67a34]); a tabela da cópia é liberada. Fora da fase −1 só a original (nenhuma grava nada).
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
ROTINAS = [('cql11_copa_da_franca_tabela', 0x5ba3f0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql11_copa_da_liga_francesa_tabela', 0x5bcc40, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cql11_copa_do_rei_tabela', 0x84d220, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das copas da França e da Espanha: as rodadas da Copa da França, da Copa da Liga francesa e da Copa do Rei',
        'the French and Spanish cups\' calendar: the rounds of the Coupe de France, the French League Cup and the Copa del Rey',
        [('cql11_copa_da_franca_tabela', 'FraCup::create_round_table', '0x5ba3f0', 'fra_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da França tem 9 rodadas em jogo único, da 6ª rodada nacional (40 times) à final: 6ª sorteio 10/10 (sem dia da semana fixo), jogo 6/11; 7ª 12/11 e 27/11 (entram 164 clubes, 184 times); 8ª 3/12 e 18/12; 64 avos 6/1 e 22/1 (entram 18); 32 avos 27/1 e 12/2; oitavas 20/2 e 4/3; quartas 5/3 e 19/3; semifinal 20/3 e 12/4; final sorteio 13/4, jogo 6/5 no estádio neutro [inferência: mando 4]; da 7ª rodada à semifinal o clube da divisão mais baixa joga em casa (+0xb = 1); o prêmio (+0x64) de 100 mil nas oitavas, 200 mil nas quartas, 400 mil na semifinal, e na final 1 milhão (+0x60) e 800 mil (+0x64); categoria 4 até as oitavas e 6 depois',
          'rule: the Coupe de France has 9 single-leg rounds, from the 6th national round (40 teams) to the final: 6th drawn 10/10 (no fixed weekday), played 6/11; 7th 12/11 and 27/11 (164 clubs enter, 184 teams); 8th 3/12 and 18/12; last 128 6/1 and 22/1 (18 enter); last 64 27/1 and 12/2; last 16 20/2 and 4/3; quarter-finals 5/3 and 19/3; semi-final 20/3 and 12/4; final drawn 13/4, played 6/5 at a neutral ground [inference: venue 4]; from the 7th round to the semi-final the lower-division club plays at home (+0xb = 1); the prize (+0x64) of 100 thousand in the last 16, 200 thousand in the quarter-finals, 400 thousand in the semi-final, and in the final 1 million (+0x60) and 800 thousand (+0x64); category 4 up to the last 16 and 6 after'),
         ('cql11_copa_da_liga_francesa_tabela', 'FraLgeCup::create_round_table', '0x5bcc40', 'fra_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Liga francesa tem 7 rodadas em jogo único: preliminar (código 0xe6, 16 times) sorteio 1/9, jogo 5/10; 1ª rodada 15/10 e 16/11 (28 times, 20 novos); 2ª 26/11 e 8/1 (32 times, 18 novos); 3ª 14/1 e 29/1; quartas 3/2 e 19/2; semifinal 4/3 e 1/4; final 2/4 e 22/4 no estádio neutro [inferência: mando 4]; 300 mil por vencer cada rodada e 1,5 milhão ao campeão; categoria 4 até as quartas e 6 depois',
          'rule: the French League Cup has 7 single-leg rounds: preliminary (code 0xe6, 16 teams) drawn 1/9, played 5/10; 1st round 15/10 and 16/11 (28 teams, 20 new); 2nd 26/11 and 8/1 (32 teams, 18 new); 3rd 14/1 and 29/1; quarter-finals 3/2 and 19/2; semi-final 4/3 and 1/4; final 2/4 and 22/4 at a neutral ground [inference: venue 4]; 300 thousand for winning each round and 1.5 million to the winner; category 4 up to the quarter-finals and 6 after'),
         ('cql11_copa_do_rei_tabela', 'SpaCup::create_round_table', '0x84d220', 'spa_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa do Rei tem 7 rodadas: preliminar (código 0xe6, 38 times, 7 dias entre os jogos) sorteio 18/8, jogo 30/8; 1ª rodada 8/9 e 13/12 (64 times, 45 novos); 2ª 15/12 e 3/1; 3ª (oitavas) 5/1 e 10/1, quartas 19/1 e 31/1 e semifinal 23/2 e 11/4 em ida e volta (7, 7 e 14 dias entre os jogos); final sorteio 26/4, jogo 26/5 no estádio neutro [inferência: mando 4]; até a semifinal o clube da divisão mais baixa joga em casa e os clubes da mesma nação se separam no sorteio (+0xb = 3); sem prêmio por rodada (−1); categoria 4 até as oitavas e 6 depois',
          'rule: the Copa del Rey has 7 rounds: preliminary (code 0xe6, 38 teams, 7 days between legs) drawn 18/8, played 30/8; 1st round 8/9 and 13/12 (64 teams, 45 new); 2nd 15/12 and 3/1; 3rd (last 16) 5/1 and 10/1, quarter-finals 19/1 and 31/1 and semi-final 23/2 and 11/4 over two legs (7, 7 and 14 days apart); final drawn 26/4, played 26/5 at a neutral ground [inference: venue 4]; up to the semi-final the lower-division club plays at home and clubs of the same nation are kept apart in the draw (+0xb = 3); no prize per round (−1); category 4 up to the last 16 and 6 after')])
CAMPOS = [('copa +0x40 ano; valor; lê (as três)', 'cup +0x40 year; value; reads (all three)'),
          ('rodada de copa (0x68 bytes) +0..+6 datas, turno, +7 nome, +9 índice, +0xb/+0xd/+0xf sorteio e desempates, +0x13 mando, +0x17 categoria, +0x18..+0x1e times, jogos, novos, inscritos, +0x20..+0x22 replay, jogos por confronto, dias, +0x5c..+0x64 prêmios; valor; grava (as três)',
           'cup round (0x68 bytes) +0..+6 dates, time slot, +7 name, +9 index, +0xb/+0xd/+0xf draw and tiebreaks, +0x13 venue, +0x17 category, +0x18..+0x1e teams, matches, new, entrants, +0x20..+0x22 replay, legs, days, +0x5c..+0x64 prizes; value; writes (all three)'),
          ('[0xb67a34] grava 0 na caixa de erro', '[0xb67a34] writes 0 on the error box')]
INTERESSA = ['CMRTE']
