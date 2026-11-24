# Mutacoes (cx08_2), com a Argentina ate 2002-08-01 (mudam o dia de uma rodada em 7 dias; os jogos e os resultados
# mudam, e o comparador do calendario acusa na criacao do jogo):
# - a ultima rodada do Apertura vai de 13/dez a 20/dez;
# - a ultima rodada do Clausura vai de 9/mai a 16/mai.
MUTACOES = [
    ('cx08_2_apertura_ultima_rodada',
     '    (13, 11, 0, 6, 1, &[(-1, -1, -1)]), // 18: 13 Dec',
     '    (20, 11, 0, 6, 1, &[(-1, -1, -1)]), // 18: 13 Dec (mutation)',
     'cx08_2 argentine premier schedule'),
    ('cx08_2_clausura_ultima_rodada',
     '    (9, 4, 1, 6, 1, &[(-1, -1, -1)]), // 18: 9 May (+1 year)',
     '    (16, 4, 1, 6, 1, &[(-1, -1, -1)]), // 18: 9 May (+1 year) (mutation)',
     'cx08_2 argentine premier schedule'),
]
