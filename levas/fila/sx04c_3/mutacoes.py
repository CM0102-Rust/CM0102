# Mutacoes (sx04c_3), pela carreira com 'ENG KOR ARG ITA FRA ESP AUS' ate 2002-06-01 (os premios coreanos de 26 de
# novembro de 2001 e o jogador do mes de marco a novembro).
# - o jogador do ano entre os jovens (ate 21);
# - o jogador do mes so a partir de abril;
# - os premios em 26 de dezembro.
MUTACOES = [
    ('sx04c_3_jogador_jovem',
     '[(3, -1, 0x11), (5, 0x15, 0x11)',
     '[(3, 0x15, 0x11), (5, 0x15, 0x11) /* mutation */',
     'sx04c_3 korean awards day'),
    ('sx04c_3_mes_abril',
     '1500, 750, u32::MAX, 2, year, 0xa, year, f, 1]),',
     '1500, 750, u32::MAX, 3, year, 0xa, year, f, 1]), // mutation',
     'sx04c_3 korean awards built'),
    ('sx04c_3_dezembro',
     '    if awards_day_is(today, 0x1a, 0xa) {',
     '    if awards_day_is(today, 0x1a, 0xb) { // mutation'),
]
