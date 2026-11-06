# O calendário das copas italianas: as rodadas da Copa da Itália, da Copa da Itália da Série C e das duas Supercopas (o
# mesmo slot, a virtual +0x3c create_schedule/create_round_table; ita_cup.cpp, ita_c_cup.cpp, ita_super.cpp,
# ita_c1_super.cpp). Frente das competições (Agente D, 05/11/2026), carreira de 8 ligas (a Itália jogável).
# - 0x63ec50 (ItaCup::create_schedule, 1236 bytes; 10 chamadas até 2002-06-01 com as 8 ligas): as 3 rodadas de cada
#   grupo e as 5 do mata-mata da Copa da Itália. Uma mutação.
# - 0x63cf90 (ItaCCup::create_schedule, 1315 bytes; 18 chamadas): as 5 rodadas de cada grupo e as 5 do mata-mata da
#   Copa da Série C. Uma mutação.
# - 0x65f920 (ItaSuper::create_round_table, 308 bytes; 1 chamada, no construtor): o jogo da Supercopa. Uma mutação.
# - 0x63c890 (ItaC1Super::create_round_table, 309 bytes; 1 chamada): o jogo da Supercopa da Série C1 (passageira).
# Reaproveita do motor.rs os ajudantes da 276a (cql10_rodada, cql10_tabela, cql10_gravados, cql10_comparar, Cql10R): o
# mata-mata e as supercopas têm o mesmo corpo das copas inglesas; os grupos (rodadas de 0x41 bytes, como as das ligas,
# pela 0x68a160 e 0x68a1c0) são o cqo10_tabela, no molde da 0x92c290 (Copa do Mundo, cqf_6). As tabelas foram
# extraídas da desmontagem (as duas datas e os campos de cada rodada; todas gravam o mesmo conjunto de campos).
# Iguais no cm0102-1080.exe (as quatro inteiras).
# Convenções conferidas (lição da 258a): as quatro são a virtual +0x3c (push saida; push &extra; push &rodadas; push
# fase; mov ecx, copa -> thiscall, ret 0x10; a fase lida como byte com sinal, cmp al, 7 / 0xf com jge). Dentro: 0x944e46
# (push tamanho; add esp, 4 -> cdecl), 0x68a160 (9 pushes; add esp junto -> cdecl: tabela, rodada, dia, mês, anos 0,
# dia da semana, código, ano em registrador com lixo nos 16 bits altos (a 0x68a160 usa só a palavra: a cópia passa
# limpo), valor 0), 0x68a1c0 (7 pushes -> cdecl: tabela, rodada, jogo 0, −1, −1, −1, 0), 0x521e60 e 0x521eb0 (pela
# cql10_rodada) e a caixa (aviso_em).
# Comparadores: lado a lado (a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais; o
# [0xb67a34]); as saídas e os bytes gravados das duas tabelas (grupos: +0..+0xb e +0x3d..+0x40 de cada rodada; mata-mata:
# os da cql10_gravados) têm de ser iguais; a tabela da cópia é liberada (0x9452ca). Acima do mata-mata só a original
# (nenhuma grava nada).
# Determinismo: nada lê pilha, heap ou endereço como valor (a tabela nova só é escrita).
# Provas (05-06/11/2026, cópia fora do projeto: motor.rs da principal com a 277a publicada e a 278a integrada + os 12
# itens cqo_10..13, cqp_10..13, cqq_10..13, 39 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2001-10-01 (parou em 2001-273): 0 linhas DIVERGENCIA fora
#   da cqo11_copa_c_inscritos (8 dezenas embaralhadas, não 9: corrigida e conferida de novo até 2001-01-05, 0
#   divergências), 0 EXCECAO, nenhuma caixa;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão b690295afd222f6a; os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base f29c85195c2a534f): as três ACUSADAS (o mata-mata
#   da Copa da Itália em 2001-259, os grupos da Série C em 2001-214, a Supercopa em 2001-210). A Supercopa da C1 é
#   passageira (a rodada única roda: contador 1).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqo10_copa_da_italia_tabela', 0x63ec50, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo10_copa_c_tabela', 0x63cf90, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo10_supercopa_tabela', 0x65f920, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqo10_supercopa_c1_tabela', 0x63c890, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário das copas italianas: as rodadas da Copa da Itália, da Copa da Série C e das duas Supercopas',
        'the Italian cups\' calendar: the rounds of the Coppa Italia, the Serie C Cup and the two Super Cups',
        [('cqo10_copa_da_italia_tabela', 'ItaCup::create_schedule', '0x63ec50', 'ita_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Itália começa por grupos de três rodadas em agosto (12 e 19, sábado [inferência: dia da semana 6], e 29, terça), cada grupo um estágio de liga (é_liga = 1, id 0x3e9 + fase); depois o mata-mata (fase 7) em ida e volta: 1ª rodada sorteada em 30/8 e jogada em 19/9 (16 times que vêm dos grupos e 16 novos), 2ª sorteio 25/10, jogo 11/11 (8 novos), quartas 29/11 e 12/12, semifinal 10/1 e 23/1, final sorteio 7/2 e jogos em 6/3 e 57 dias depois; categoria 4 até a 2ª rodada e 6 depois; sem prêmio por rodada (−1)',
          'rule: the Coppa Italia starts with three-round groups in August (12 and 19, Saturday [inference: weekday 6], and 29, Tuesday), each group a league stage (is_league = 1, id 0x3e9 + stage); then the knockout (stage 7) over two legs: 1st round drawn on 30/8 and played on 19/9 (16 teams from the groups and 16 new), 2nd draw 25/10, match 11/11 (8 new), quarter-finals 29/11 and 12/12, semi-final 10/1 and 23/1, final draw 7/2 and legs on 6/3 and 57 days later; category 4 up to the 2nd round and 6 after; no per-round prize (−1)'),
         ('cqo10_copa_c_tabela', 'ItaCCup::create_schedule', '0x63cf90', 'ita_c_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Série C começa por grupos de cinco rodadas em agosto (12, 19, 22, 26 e 29; id 0x420 + fase); o mata-mata (fase 0xf) em ida e volta: 1ª rodada sorteada em 30/8 e jogada em 11/10 (32 times), 2ª 26/10 e 1/11, quartas 9/11 e 15/11, semifinal 30/11 e 11 de abril, final sorteio 26/4 e jogo em 9 de maio; 14 dias entre os jogos, 7 na 2ª rodada e na final; categoria 4 até a 2ª rodada e 6 depois',
          'rule: the Serie C Cup starts with five-round groups in August (12, 19, 22, 26 and 29; id 0x420 + stage); the knockout (stage 0xf) over two legs: 1st round drawn on 30/8 and played on 11/10 (32 teams), 2nd 26/10 and 1/11, quarter-finals 9/11 and 15/11, semi-final 30/11 and 11 April, final draw 26/4 and match on 9 May; 14 days between legs, 7 in the 2nd round and the final; category 4 up to the 2nd round and 6 after'),
         ('cqo10_supercopa_tabela', 'ItaSuper::create_round_table', '0x65f920', 'ita_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Itália é um jogo único (2 times) em 19 de agosto, sorteio em 5 de julho, categoria 6, desempate 0x203, sem prêmios',
          'rule: the Italian Super Cup is a single match (2 teams) on 19 August, drawn on 5 July, category 6, tie-break 0x203, no prizes'),
         ('cqo10_supercopa_c1_tabela', 'ItaC1Super::create_round_table', '0x63c890', 'ita_c1_super.cpp', 'thiscall, ret 0x10',
          'regra: a Supercopa da Série C1 é um jogo único em 21 de maio do ano seguinte, sorteio em 15 de maio, categoria 6, sem prêmios',
          'rule: the Serie C1 Super Cup is a single match on 21 May of the following year, drawn on 15 May, category 6, no prizes')])
CAMPOS = [('copa +0x40 ano da copa; valor (palavra com sinal); lê (as quatro)',
           'cup +0x40 cup year; value (signed word); reads (all four)'),
          ('rodada de copa (0x68 bytes) +0..+0x22 e +0x5c..+0x67: datas, nome, bits, categoria, times, confronto, prêmios; valor; grava (mata-mata e supercopas)',
           'cup round (0x68 bytes) +0..+0x22 and +0x5c..+0x67: dates, name, bits, category, teams, tie, prizes; value; writes (knockout and super cups)'),
          ('rodada de grupo (0x41 bytes) +0 dia do ano, +2 ano relativo, +4 código, +5..+0xb jogo 0 vazio, +0x3d valor; valor; grava (grupos)',
           'group round (0x41 bytes) +0 day of year, +2 relative year, +4 code, +5..+0xb empty match 0, +0x3d value; value; writes (groups)')]
INTERESSA = ['CMRTE']
