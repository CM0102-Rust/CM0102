# Mutacoes (sx14_2). Texto de noticia: acusadas pelo comparador (compare_text_writer, texto e vinculos), como as de
# texto aceitas em 12/11/2026; o texto nao volta para a simulacao. A noticia 0x138c tem o texto pedido 115 vezes na
# carreira ENG ate 2002-06-01.
# - o placar com o menor primeiro (todo corpo de reacao com gols diferentes);
# - a manchete de furia cita o adversario em vez do proprio clube.
MUTACOES = [
    ('sx14_2_placar_invertido',
     'goals_a.max(goals_b), goals_a.min(goals_b)); // "<goals>-<opp goals>"',
     'goals_a.min(goals_b), goals_a.max(goals_b)); // mutation',
     'sx14_2 body'),
    ('sx14_2_furia_do_adversario',
     '(0xa0bbcc, club) // "Board fury at <club> performance"',
     '(0xa0bbcc, other) // mutation',
     'sx14_2 headline'),
]
