# Mutacoes (cx08_7), com a Argentina ate 2002-08-01 (o comparador do calendario acusa; os jogos mudam de dia):
# - a ultima rodada da primeira fase vai de 18/dez a 25/dez;
# - a final da copa de acesso joga a partir de 11/mai em vez de 4/mai.
MUTACOES = [
    ('cx08_7_primeira_fase_ultima',
     '    (18, 11, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 24: 18 Dec',
     '    (25, 11, 0, 5, 1, &[(6, 1, 1), (-1, -1, -1)]), // 24: 18 Dec (mutation)',
     'cx08_7 argentine second schedule'),
    ('cx08_7_final_da_copa',
     'cql10r((1, 4, 1, 2), (4, 4, 1, 5, 1, 0), 0x96,',
     'cql10r((1, 4, 1, 2), (11, 4, 1, 5, 1, 0) /* mutation */, 0x96,',
     'cx08_7 argentine second schedule'),
]
