# Mutacoes (rx10_9), com Portugal e a base NORMAL ate 2002-08-01. Nenhuma abre caixa.
# - as posicoes previstas de 2 em 2 e nao de 3 em 3: o comparador das posicoes acusa (lado a lado).
# A virada fica sem mutacao propria (o que ela monta so e usado na temporada seguinte); o comparador confere o ano.
MUTACOES = [
    ('rx10_9_posicoes_de_dois',
     '    section_predicted_positions(this, 2, 3, 0xa78798, [0x2b7, 0x2bf]) // rx10_9: places 1, 4, 7...',
     '    section_predicted_positions(this, 2, 2, 0xa78798, [0x2b7, 0x2bf]) // mutation',
     'rx10_9 segunda b predicted'),
]
