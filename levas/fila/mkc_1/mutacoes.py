# Mutacoes (mkc_1): a janela inglesa (junho a marco) fechada de 1 de janeiro ao fim de marco (quem se transfere e
# quem vai emprestado no inverno muda); a janela sempre aberta (emprestimos e compras em abril e maio); o emprestimo
# padrao um mes mais longo (a volta e quem pede emprestado depois de 19 de junho mudam).
MUTACOES = [
    ('mkc1_janela_sem_inverno',
     '        // mkc1: the wrapped window, from the start of the year to the closing\n',
     '        return 0; // mutacao: sem o trecho de janeiro a marco\n',
     'mkc_1 wrapped window end'),
    ('mkc1_janela_sempre_aberta',
     '        return 0; // mkc1: windows, none open',
     '        return 1; // mutacao: sempre aberta',
     'mkc_1 no open window'),
    ('mkc1_emprestimo_um_mes_a_mais',
     '    *end_month = end; // mkc1: the season end month',
     '    *end_month = if end < 0xb { end + 1 } else { end }; // mutacao: um mes a mais',
     'mkc_1 base loan period'),
]
