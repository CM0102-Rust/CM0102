# Mutacoes (sx19_11), pela carreira ENG SCO WAL ate 2002-08-01 (a Premier escocesa tem cartoes em toda rodada):
# - a suspensao escocesa por pontos sem vermelho comecando em 7 dias (e nao em 14);
# - os pontos de um amarelo escoces somando 1 a mais.
MUTACOES = [
    ('sx19_11_sete_dias',
     'ban_start_in(0xe) }; // sx19_11: two weeks without a red card',
     'ban_start_in(7) }; // mutation: one week',
     'sx19_11 scottish points ban'),
    ('sx19_11_amarelo_mais_um',
     'add_points(scotland_offence_points(offence)); // sx19_11: the points of a Scottish booking',
     'add_points(scotland_offence_points(offence) + 1); // mutation: one point more',
     'sx19_11 scottish booking'),
]
