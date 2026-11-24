# Mutacoes (sx05b_16), pela carreira com as 8 ligas ate 2002-06-01 (os premios da MLS de 25 de novembro).
# - o estreante do ano sem o limite de idade;
# - o goleiro do ano escolhido entre os defensores;
# - o jogador do mes de abril a outubro (o mes de marco nao entra).
MUTACOES = [
    ('sx05b_16_estreante_sem_idade',
     '(5, 0x15, 0, 0x11),',
     '(5, -1, 0, 0x11), /* mutation */',
     'sx05b_16 usa awards day'),
    ('sx05b_16_goleiro_defensor',
     '[(3, -1, 1, 0x11), (4, -1, 6, 0x11),',
     '[(3, -1, 6, 0x11), (4, -1, 6, 0x11), /* mutation */',
     'sx05b_16 usa awards day'),
    ('sx05b_16_mes_de_abril',
     '3000, 0, mls, 3, year, 0xa, year, f, 1]),',
     '3000, 0, mls, 4, year, 0xa, year, f, 1]), // mutation',
     'sx05b_16 usa awards built'),
]
