# Mutacoes (cqx_10), nos pontos do Paulista (3.146 chamadas ate 2002-06-01: cada resultado do estadual): um ponto a
# mais em todo resultado (a tabela e a classificacao do Paulista mudam). Os registros sao passageiros (sem mutacao).
MUTACOES = [
    ('cqx10_paulista_um_ponto_a_mais',
     'league_points(this, club, fixture) // cqx10: the League\'s points',
     'league_points(this, club, fixture).wrapping_add(1) // mutation: one point more',
     'cqx_10 paulista points'),
]
