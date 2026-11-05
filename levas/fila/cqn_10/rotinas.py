# O dia das copas: a virada da Copa da Liga inglesa, as inscrições da Copa da Inglaterra e a convocação do Jogo das
# Estrelas da MLS (o mesmo slot, a virtual +0x14 update_for_date; eng_cc_cup.cpp, eng_fa_cup.cpp,
# usa_mls_all_stars.cpp). Frente das competições (Agente D, 04/11/2026), carreira de 8 ligas.
# - 0x56caf0 (EngCcCup::update_for_date, 457 bytes; 2064 chamadas até 2002-06-01 com as 8 ligas, uma por dia): no
#   primeiro dia do ano da copa, os inscritos, as rodadas, a árvore e a ordem prevista. Uma mutação. A SpaCup::
#   update_for_date (0x84d740) é igual byte a byte, a não ser pelos endereços (fica para outro item).
# - 0x570550 (EngFaCup::update_for_date, 735 bytes, quadro SEH só pela notícia do caminho de 1999; 2064 chamadas): em 3
#   de outubro, os inscritos e as rodadas e o sorteio da preliminar. Uma mutação.
# - 0x90a900 (UsaMlsAllStars::update_for_date, 126 bytes, quadro SEH pela Date local sem destrutor; 2064 chamadas): a
#   convocação das seleções da MLS no sábado de 26 de julho em diante. Uma mutação.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as três são a virtual +0x14 (push fase; push hoje; mov ecx, copa -> thiscall,
# ret 8). Dentro: 0x9452ca (push p; add esp, 4 -> cdecl), 0x56ccc0 e 0x56fab0 (mov ecx, copa; sem push -> thiscall,
# ret 0; a 0x56ccc0 está na cqm_10), as virtuais +0x8c, +0x94 e +0x5c (mov ecx; call [edx + o] -> thiscall, ret 0),
# 0x51c800 (push 0; mov ecx -> thiscall, ret 4), 0x549f70 (Date::Date, thiscall ret 0x10, num Date local; o ano num
# registrador com lixo nos bytes altos, lido como palavra: a cópia passa limpo), 0x90a980 (mov ecx -> thiscall, ret 0),
# 0x51f890 (push fase; push hoje; mov ecx -> thiscall, ret 8) e a caixa (aviso_em). Nas caixas, a original da Copa da
# Liga zera o al e a da Inglaterra põe 1 no al (o resto do eax vem da caixa): a cópia devolve 0 e 1.
# Comparadores: só a original (a virada libera e refaz as tabelas, a atualização da copa joga e sorteia); o plano das
# duas inglesas: +0x45 vai a 1 no dia da virada. O caminho de 1999 da Copa da Inglaterra (a notícia ao técnico humano) a
# cópia passa à original pelo trampolim.
# Determinismo: nada lê pilha, heap ou endereço como valor (as Dates locais começam zeradas e o construtor as preenche).
# Provas (04-05/11/2026, cópia fora do projeto: motor.rs da principal com a 273a publicada e a 274a integrada + os 9
# itens cql_10..12, cqm_10..12, cqn_10..12, 27 rotinas):
# - jogo comparando com ENG ESP ITA FRA BRA JPN AUS USA até 2002-06-01 (parou em 2002-151): 0 linhas DIVERGENCIA,
#   0 EXCECAO, nenhuma caixa (motor: 10^10 comparações, 0 divergências); a curta até 2001-10-01 também 0;
# - duas carreiras Rust com as 8 ligas até 2002-06-01 iguais entre si e ao detector (simular.py --repetir 2
#   --detector: "detector de lixo: igual"), impressão 17dfd306919e4965; todos os contadores das mutações rodaram;
# - mutações (carreira Rust de 8 ligas até 2002-06-01 contra a base 426fa70704cbf326): as 27 dos 9 itens ACUSADAS.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqn10_copa_da_liga_dia', 0x56caf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqn10_copa_da_inglaterra_dia', 0x570550, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('cqn10_jogo_das_estrelas_dia', 0x90a900, [0x6a, 0xff, 0x68, 0xa8, 0x54, 0x96, 0x00])]
DOCS = ('o dia das copas: a virada da Copa da Liga inglesa, as inscrições da Copa da Inglaterra e a convocação do Jogo das Estrelas',
        'the cups\' day: the English League Cup turnover, the FA Cup entries and the All-Star squad call-up',
        [('cqn10_copa_da_liga_dia', 'EngCcCup::update_for_date', '0x56caf0', 'eng_cc_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga inglesa é refeita uma vez por temporada (+0x45), no primeiro dia em que o ano do jogo chega ao ano da copa: as tabelas velhas são liberadas e vêm os 92 inscritos, as 7 rodadas, a árvore dos confrontos e a ordem prevista; depois, todo dia, a atualização da copa (sorteios e jogos nas datas da tabela)',
          'rule: the English League Cup is rebuilt once per season (+0x45), on the first day the game year reaches the cup year: the old tables are freed and the 92 entrants, the 7 rounds, the tie tree and the predicted order are built; then, every day, the cup update (draws and matches on the table dates)'),
         ('cqn10_copa_da_inglaterra_dia', 'EngFaCup::update_for_date', '0x570550', 'eng_fa_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Inglaterra junta os inscritos em 3 de outubro de cada ano (Date(3, 9): o mês 9 é outubro; o mapa da comunidade diz setembro), refaz as rodadas e a árvore e sorteia a preliminar, uma vez por temporada (+0x45); em 1º de outubro de 1999 perguntava ao técnico humano se o clube entra na 5ª rodada (+0xb2 marca a pergunta); depois, todo dia, a atualização da copa',
          'rule: the FA Cup gathers its entrants on 3 October each year (Date(3, 9): month 9 is October; the community map says September), rebuilds the rounds and the tree and draws the preliminary round, once per season (+0x45); on 1 October 1999 it asked the human manager whether the club enters at the 5th round (+0xb2 marks the question); then, every day, the cup update'),
         ('cqn10_jogo_das_estrelas_dia', 'UsaMlsAllStars::update_for_date', '0x90a900', 'usa_mls_all_stars.cpp', 'thiscall, ret 8',
          'regra: as duas seleções do Jogo das Estrelas da MLS são convocadas no dia de Date(26, 6, ano, 6): 26 de julho levado ao dia da semana 6 [inferência: o sábado seguinte]; o jogo é em 2 de agosto (ver cqm_12); depois, todo dia, a atualização da copa',
          'rule: the two MLS All-Star selections are called up on the day of Date(26, 6, year, 6): 26 July moved to weekday 6 [inference: the following Saturday]; the match is on 2 August (see cqm_12); then, every day, the cup update')])
CAMPOS = [('copa +0x45 já refeita nesta temporada; valor; lê e grava 1 (cqn10_copa_da_liga_dia, cqn10_copa_da_inglaterra_dia)',
           'cup +0x45 already rebuilt this season; value; reads and writes 1 (cqn10_copa_da_liga_dia, cqn10_copa_da_inglaterra_dia)'),
          ('copa +0xa7 inscritos, +0xa3 rodadas, +0xad confrontos por rodada (+0x3c rodadas); ponteiro; libera (as duas inglesas)',
           'cup +0xa7 entrants, +0xa3 rounds, +0xad ties per round (+0x3c rounds); pointer; frees (both English ones)'),
          ('Copa da Inglaterra +0xb2 a pergunta da 5ª rodada já feita; valor; lê (cqn10_copa_da_inglaterra_dia)',
           'FA Cup +0xb2 the fifth-round question already asked; value; reads (cqn10_copa_da_inglaterra_dia)'),
          ('[0xde1f58] +0x439 o ano do jogo (palavra); Date de hoje +0 dia do ano, +2 ano; valor; lê',
           '[0xde1f58] +0x439 the game year (word); today\'s Date +0 day of year, +2 year; value; reads')]
INTERESSA = ['CMRTE']
