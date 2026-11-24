# As noticias de financas da diretoria, do setor interditado e da crise financeira (fin_prm.cpp pelo mapa; os asserts
# dizem finance.CPP; leitor cx10, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS): as tres saem pelos eventos de
# financas da semana (process_club_finance_events 0x59ed70 e os eventos raros try_finance_boost_event,
# try_board_takeover, try_cash_injection, ja em Rust), chamadas pelo exe::finance_post_*.
# - 0x59f300 finance_post_finance_boost_news thiscall(clube, valor, verba, tipo) ret 0x10 (o ecx nao e lido): noticia
#   0x1779 com o id do clube, o valor, a verba e o tipo (byte com sinal); clube de situacao profissional 3: nada e
#   enviado; tipo 2 ou 7 com nacao: a todos os tecnicos da nacao (News::send_to_nation_managers 0x7863f0); senao ao
#   tecnico do clube (News::send_to_club_manager 0x7861e0).
# - 0x59f900 finance_post_stand_closed_news thiscall(clube, lugares, tipo) ret 0xc: noticia 0x177a com o id do clube, o
#   id do estadio (-1 sem estadio), os lugares (palavra com sinal) e o tipo (byte com sinal), ao tecnico do clube.
# - 0x59fc60 finance_post_financial_crisis_news thiscall(clube) ret 4: noticia 0x177b com o id do clube e o sorteio de 2
#   (utils_random 0x90cfc0 cdecl, palavra com sinal), ao tecnico do clube.
# Convencoes conferidas: NewsItem::NewsItem 0x77bbe0 (lea ecx, local; push 0; push tipo -> thiscall, ret 8),
# NewsItem::set_slot 0x785790 (thiscall, ret 8), os dois envios (ecx = [0xb7431c]; 2 pushes -> thiscall, ret 8). O
# quadro SEH so cobre o desenrolar de excecao do NewsItem local (0xe0 bytes). Prologo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe (inteiras).
# Comparadores: so a original (enviam noticias; nao podem rodar duas vezes).
SAVE = False
ROTINAS = [('finance_post_finance_boost_news', 0x59f300, [0x6a, 0xff, 0x68, 0x7b, 0x90, 0x95, 0x00]),
           ('finance_post_stand_closed_news', 0x59f900, [0x6a, 0xff, 0x68, 0xcb, 0x90, 0x95, 0x00]),
           ('finance_post_financial_crisis_news', 0x59fc60, [0x6a, 0xff, 0x68, 0x0b, 0x91, 0x95, 0x00])]
DOCS = ('as notícias das finanças: a diretoria, o setor interditado e a crise financeira',
        'the finance news: the board, the closed stand and the financial crisis',
        [('finance_post_finance_boost_news', 'finance_post_finance_boost_news', '0x59f300', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0x10',
          'envia a notícia das finanças da diretoria (patrocínio, ações, verba, nova diretoria, abertura de capital, injeção de dinheiro) com o clube, o valor, a verba e o tipo; o clube de situação profissional 3 não recebe nada; a troca de diretoria e a abertura de capital vão a todos os técnicos da nação, o resto ao técnico do clube',
          'sends the board finance news (sponsorship, shares, budget, new board, flotation, cash injection) with the club, the amount, the budget and the kind; a club of professional status 3 gets nothing; the takeover and the flotation go to every manager of the nation, the rest to the club\'s manager'),
         ('finance_post_stand_closed_news', 'finance_post_stand_closed_news', '0x59f900', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'envia ao técnico do clube a notícia de que a fiscalização interditou parte do estádio, com o estádio (−1 sem estádio), os lugares e o tipo',
          'sends the club\'s manager the news that the safety inspector closed part of the ground, with the stadium (−1 without one), the places and the kind'),
         ('finance_post_financial_crisis_news', 'finance_post_financial_crisis_news', '0x59fc60', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 4',
          'envia ao técnico do clube a notícia da crise financeira, com uma redação sorteada entre duas',
          'sends the club\'s manager the financial crisis news, with one of two wordings drawn at random')])
CAMPOS = []
INTERESSA = []
