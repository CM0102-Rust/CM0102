# Mutacoes (cqo_6), na criacao da carreira de 8 ligas: a 1a rodada da Serie A, da Serie B e da primeira divisao
# francesa levada a outro dia da semana (os jogos dela caem em outro dia, e o resto da carreira muda desde agosto).
MUTACOES = [
    ('cqo6_serie_a_primeira_rodada_em_outro_dia',
     'const CQO6_SERIE_A: [Cqo6Rodada; 34] = [\n    (28, 7, 0, 6,',
     'const CQO6_SERIE_A: [Cqo6Rodada; 34] = [\n    (28, 7, 0, 2,',
     'cqo_6 calendario da serie a'),
    ('cqo6_serie_b_primeira_rodada_em_outro_dia',
     'const CQO6_SERIE_B: [Cqo6Rodada; 38] = [\n    (29, 7, 0, 6,',
     'const CQO6_SERIE_B: [Cqo6Rodada; 38] = [\n    (29, 7, 0, 2,',
     'cqo_6 calendario da serie b'),
    ('cqo6_francesa_primeira_rodada_em_outro_dia',
     'const CQO6_PRIMEIRA_FRANCESA: [(bool, Cqo6Rodada); 38] = [\n    (false, (31, 6, 0, 5,',
     'const CQO6_PRIMEIRA_FRANCESA: [(bool, Cqo6Rodada); 38] = [\n    (false, (31, 6, 0, 2,',
     'cqo_6 calendario da primeira francesa'),
]
