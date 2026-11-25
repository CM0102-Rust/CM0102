# Mutacoes (rx10_11), com as 8 ligas ate 2003-08-15. Nenhuma abre caixa.
# - a primeira edicao no primeiro ano par a partir de 2004: o Mundial de Clubes de 2003 nao acontece (a carreira: o
#   sorteio de fevereiro e os jogos de julho e agosto de 2003 somem; o comparador do construtor acusa o ano).
MUTACOES = [
    ('rx10_11_ano_par',
     '    while year % 2 == 0 || year < 0x7d3 {',
     '    while year % 2 != 0 || year < 0x7d4 { // mutation',
     'rx10_11 new club world championship'),
]
