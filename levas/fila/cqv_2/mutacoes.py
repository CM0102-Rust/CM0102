# Mutacoes (cqv_2). As posicoes previstas rodam duas vezes cada na carreira de 8 ligas ate 2002-06-01 (na criacao e na
# virada; contadores "cqv_2 <liga>"); as previstas mudam a expectativa de cada clube (na Serie C, cqs_4, e na Italia,
# cqm_4, mexer nelas foi acusado no primeiro dia):
# - pernambucano: os semifinalistas de baixo na frente;
# - carioca: sem ordenar pela forca;
# - catarinense: o 1o e o 2o trocados;
# - paulista: sem o 3o dividido;
MUTACOES = [
    ('cqv2_pernambucano_lugares',
     'lugares: &[1, 2, 3, 3], ramo: "cqv_2 pernambucano"',
     'lugares: &[3, 3, 1, 2], ramo: "cqv_2 pernambucano" /* mutacao: os semifinalistas de baixo na frente */',
     'cqv_2 pernambucano'),
    ('cqv2_carioca_sem_ordenar',
     'ordenar(v, n as i32); // cqv2: a ordem pela força',
     'if e.ramo != "cqv_2 carioca" { ordenar(v, n as i32); } // mutacao: sem ordenar pela forca',
     'cqv_2 carioca'),
    ('cqv2_catarinense_lugares',
     'lugares: &[1, 2, 3, 3], ramo: "cqv_2 catarinense"',
     'lugares: &[2, 1, 3, 3], ramo: "cqv_2 catarinense" /* mutacao: o 1o e o 2o trocados */',
     'cqv_2 catarinense'),
    ('cqv2_paulista_lugares',
     'lugares: &[1, 2, 3, 3], ramo: "cqv_2 paulista"',
     'lugares: &[1, 2, 3, 4], ramo: "cqv_2 paulista" /* mutacao: sem o 3o dividido */',
     'cqv_2 paulista'),
]
