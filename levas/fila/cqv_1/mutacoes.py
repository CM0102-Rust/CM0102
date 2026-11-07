# Mutacoes (cqv_1). As posicoes previstas rodam duas vezes cada na carreira de 8 ligas ate 2002-06-01 (na criacao e na
# virada; contadores "cqv_1 <liga>"); as previstas mudam a expectativa de cada clube (na Serie C, cqs_4, e na Italia,
# cqm_4, mexer nelas foi acusado no primeiro dia):
# - mineiro: o 1o e o 2o trocados;
# - norte: sem ordenar pela forca;
# - nordeste: sem o 3o dividido;
# - paranaense: o 1o e o 2o trocados;
MUTACOES = [
    ('cqv1_mineiro_lugares',
     'lugares: &[1, 2], ramo: "cqv_1 mineiro"',
     'lugares: &[2, 1], ramo: "cqv_1 mineiro" /* mutacao: o 1o e o 2o trocados */',
     'cqv_1 mineiro'),
    ('cqv1_norte_sem_ordenar',
     'ordenar(v, n as i32); // cqv1: a ordem pela força',
     'if e.ramo != "cqv_1 norte" { ordenar(v, n as i32); } // mutacao: sem ordenar pela forca',
     'cqv_1 norte'),
    ('cqv1_nordeste_lugares',
     'lugares: &[1, 2, 3, 3], ramo: "cqv_1 nordeste"',
     'lugares: &[1, 2, 3, 4], ramo: "cqv_1 nordeste" /* mutacao: sem o 3o dividido */',
     'cqv_1 nordeste'),
    ('cqv1_paranaense_lugares',
     'lugares: &[1, 2], ramo: "cqv_1 paranaense"',
     'lugares: &[2, 1], ramo: "cqv_1 paranaense" /* mutacao: o 1o e o 2o trocados */',
     'cqv_1 paranaense'),
]
