# Mutacoes (cqt_13): as quartas da Copa Africana de 2002 com 1o A x 2o D e 1o C x 2o B; com 2o C x 2o A e 1o B x 1o D;
# as finais da Recopa asiatica sem cruzar o par do meio (os dois da mesma zona se enfrentam). Os contadores ficam
# colados no efeito.
MUTACOES = [
    ('cqt13_quartas_a_contra_d',
     '(-1, 0), (0, 1), // cqt13: 1º A × 2º B\n    (1, 0), (2, 1), // cqt13: 1º C × 2º D',
     '(-1, 0), (2, 1), // mutacao: 1o A x 2o D\n    (1, 0), (0, 1), // mutacao: 1o C x 2o B',
     'cqt_13 quartas da copa africana'),
    ('cqt13_quartas_segundos_contra_segundos',
     '(1, 1), (2, 0), // cqt13: 2º C × 1º D\n    (0, 0), (-1, 1), // cqt13: 1º B × 2º A',
     '(1, 1), (-1, 1), // mutacao: 2o C x 2o A\n    (0, 0), (2, 0), // mutacao: 1o B x 1o D',
     'cqt_13 quartas da copa africana'),
    ('cqt13_finais_sem_cruzar',
     'v.swap(1, 2); // cqt13: cruza o par do meio',
     '// mutacao: sem cruzar',
     'cqt_13 finalistas da recopa asiatica'),
]
