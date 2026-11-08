# Mutacoes (cqt_12), na Liga dos Campeoes asiatica de 2001: os times do grupo 1 de tras para a frente; os do grupo 2
# idem (o mando e a ordem dos jogos de cada rodada mudam); as semifinais 1o x 1o e 2o x 2o. Os contadores ficam
# colados no efeito.
MUTACOES = [
    ('cqt12_grupo_1_de_tras_para_frente',
     'let grupo_1 = [v[0], v[1], v[2], v[3]]; // cqt12: o grupo 1',
     'let grupo_1 = [v[3], v[2], v[1], v[0]]; // mutacao: de tras para a frente',
     'cqt_12 asia groups'),
    ('cqt12_grupo_2_de_tras_para_frente',
     'let grupo_2 = [v[4], v[5], v[6], v[7]]; // cqt12: o grupo 2',
     'let grupo_2 = [v[7], v[6], v[5], v[4]]; // mutacao: de tras para a frente',
     'cqt_12 two asia groups'),
    ('cqt12_semifinais_primeiro_contra_primeiro',
     '[l(1, 0), l(2, 1), l(2, 0), l(1, 1)] // cqt12: 1º do 1 × 2º do 2, 1º do 2 × 2º do 1',
     '[l(1, 0), l(2, 0), l(1, 1), l(2, 1)] // mutacao: 1o x 1o',
     'cqt_12 asia pairings'),
]
