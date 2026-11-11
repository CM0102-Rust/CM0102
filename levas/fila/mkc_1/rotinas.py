# O período padrão do empréstimo, a janela aberta e o teste da janela (transfer_rules.cpp), leitor C do mapa do
# mercado (levas/reorganizacao/mercado/secao_c.md, 11/11/2026): o núcleo das janelas, ponto de extensão para o Tweaks
# (abrir ou fechar a janela de uma nação, alongar o empréstimo).
# - 0x8e8bd0 (TRANSFER_RULES::get_loan_period, 37 bytes de corpo): o virtual +0x28 das nações sem período próprio (a
#   0x8d0590 escolhe pela nação do clube) e a 0x579c70 inglesa depois de 19 de junho para quem tem menos de 24 anos.
# - 0x8e8e50 (TRANSFER_RULES::find_open_window): pelos can_loan_player/can_sign_player/can_make_offer/can_end_loan de
#   13 nações (na carreira inglesa: ENGLAND_RULES::can_loan_player fora da janela) e pela 0x8cf470 (cada dia do ano).
# - 0x8e8c40 (transfer_window_contains_date, quadro SEH só das datas, sem destrutor): por todas as anteriores e pelos
#   can_sign_player/can_make_offer base (0x8e8560, 0x8e8800), da Itália, Alemanha e Bélgica.
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas na desmontagem: 0x8e8bd0 thiscall ret 0x1c (sete push; mov ecx, regra), dentro 0x54cfb0 mov
# ecx, 0xae2c90 -> thiscall ret 0 e 0x4d45f0 push clube; mov ecx, [0xae1050] -> thiscall ret 4. 0x8e8e50 thiscall
# ret 8 (push saída; push data; ecx = regra), dentro 0x8e8c40 push janela; push data; mov ecx, esi -> thiscall ret 8.
# 0x8e8c40: 0x40a7d0 push ano (word em eax com resto); push saída; mov ecx, data da janela -> thiscall ret 8; 0x54c5b0 e
# 0x54c8e0 push 1; push saída; mov ecx, data -> thiscall ret 8.
# Comparadores: lado a lado (nada sorteia nem grava fora das saídas): o período nas quatro saídas, a janela no retorno
# e na saída, o teste da janela no retorno (compare_value!).
# Determinismo: nada lê pilha ou heap como valor; as datas locais são preenchidas pelas originais.
SAVE = False
ROTINAS = [('transfer_rules_get_loan_period', 0x8e8bd0, [0x56, 0x57, 0xb9, 0x90, 0x2c, 0xae, 0x00]),
           ('transfer_rules_find_open_window', 0x8e8e50, [0x53, 0x56, 0x8b, 0xf1, 0x32, 0xdb]),
           ('transfer_window_contains_date', 0x8e8c40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o período padrão do empréstimo, a janela aberta e o teste da janela',
        'the default loan period, the open window and the window test',
        [('transfer_rules_get_loan_period', 'TRANSFER_RULES::get_loan_period', '0x8e8bd0', 'transfer_rules.cpp', 'thiscall, ret 0x1c',
          'regra: o empréstimo padrão vai do mês seguinte ao mês do fim da temporada da nação do clube, neste ano se esse mês ainda está à frente do início, senão no ano seguinte (o empréstimo que começa no próprio mês do fim da temporada dura um ano)',
          'rule: the default loan runs from next month to the season end month of the club\'s nation, this year if that month is still ahead of the start, otherwise next year (a loan starting in the season end month itself lasts a year)'),
         ('transfer_rules_find_open_window', 'TRANSFER_RULES::find_open_window', '0x8e8e50', 'transfer_rules.cpp', 'thiscall, ret 8',
          'regra: a primeira janela da nação que contém a data, devolvida na saída; nenhuma: fechado se a nação tem janelas; a nação sem janela nenhuma (Dinamarca) está sempre aberta',
          'rule: the first window of the nation that contains the date, given back in the output; none: closed if the nation has windows; a nation without any window (Denmark) is always open'),
         ('transfer_window_contains_date', 'transfer_window_contains_date', '0x8e8c40', 'transfer_rules.cpp', 'thiscall, ret 8',
          'regra: a abertura e o fechamento da janela no ano da data, as duas pontas incluídas; a janela que vira o ano (Inglaterra, junho a março) está aberta da abertura até dezembro e de janeiro até o fechamento',
          'rule: the window\'s opening and closing in the date\'s year, both ends included; a window that wraps the new year (England, June to March) is open from the opening to December and from January to the closing')])
CAMPOS = [('regras de transferência +4: a tabela das janelas (12 bytes cada: +0 a data de abertura, +6 a de fechamento, 6 bytes: +2 dia da semana, +3 dia, +4 mês, +5 fecha); ponteiro; lê',
           'transfer rules +4: the window table (12 bytes each: +0 the opening date, +6 the closing date, 6 bytes: +2 weekday, +3 day, +4 month, +5 closes); pointer; reads'),
          ('regras de transferência +8: quantas janelas; valor (byte); lê', 'transfer rules +8: how many windows; value (byte); reads')]
INTERESSA = ['Tweaks']
