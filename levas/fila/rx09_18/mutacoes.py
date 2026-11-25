# Mutacoes (rx09_18), com a Inglaterra ate 2005-12-31 (a terceira fase e a repescagem em outubro/novembro de 2005):
# - a terceira fase com os quartos colocados (linha 3) em vez dos terceiros: outras selecoes (a carreira e o plano
#   acusam);
# - a repescagem com o perdedor da terceira fase: outra selecao (a carreira e o plano acusam);
# - as quatro primeiras com a posicao prevista 2 em vez de 1: o comparador acusa no sorteio de marco de 2005.
MUTACOES = [
    ('rx09_18_quartos_colocados',
     'let row = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_18: the third-placed sides with no exempt nation',
     'let row = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 3 } else { 1 }; // mutation: the fourth',
     'rx09_18 asian third round tie'),
    ('rx09_18_repescagem_do_perdedor',
     'let want = if read_u8(this + WC_ASIA_EXEMPT) == 1 { 2 } else { 1 }; // rx09_18: the third round winner goes on',
     'let want = if read_u8(this + WC_ASIA_EXEMPT) == 1 { 2 } else { 2 }; // mutation: the loser',
     'rx09_18 asian play-off'),
    ('rx09_18_previstas_2',
     '0 => &[(4, 1), (5, 5), (6, 6), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],',
     '0 => &[(4, 2), (5, 5), (6, 6), (8, 7), (10, 9), (20, 0xb), (30, 0x15)], // mutation',
     'rx09_18 asian predicted positions'),
]
