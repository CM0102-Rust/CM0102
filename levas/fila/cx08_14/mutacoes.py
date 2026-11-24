# Mutacoes (cx08_14), com a Belgica ate 2002-08-01 (o comparador do calendario acusa; os jogos mudam de dia):
# - a ultima rodada do grupo da repescagem vai de 31/mai a 7/jun;
# - a primeira rodada da copa da repescagem e sorteada em 1/mai em vez de 8/mai.
# (As rodadas da temporada ficam sem mutacao propria: a 7 dias de cada uma ha outra rodada.)
MUTACOES = [
    ('cx08_14_grupo_ultima',
     '    (31, 4, 1, 3, 2, &[(-1, -1, -1)]), // 5: 31 May (+1 year)',
     '    (7, 5, 1, 3, 2, &[(-1, -1, -1)]), // 5: 31 May (+1 year) (mutation)',
     'cx08_14 belgian second schedule'),
    ('cx08_14_copa_sorteio',
     'cql10r((8, 4, 1, 0), (14, 4, 1, 6, 2, 0x28), 0x78,',
     'cql10r((1, 4, 1, 0) /* mutation */, (14, 4, 1, 6, 2, 0x28), 0x78,',
     'cx08_14 belgian second schedule'),
]
