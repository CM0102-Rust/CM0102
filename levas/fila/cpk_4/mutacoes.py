# Mutacoes (com o ramo de cada uma). O alcance da nacao com o tipo 2 (o comparando acusa pelas saidas na primeira
# partida de liga; a carreira pela escalacao automatica, que deixa de achar as regras da nacao: os estrangeiros acima
# do limit passam a jogar). A regra de outra nacao (o comparando acusa pelo valor; a carreira pela escalacao). O
# amistoso com a primeira vaga do clube (o comparando acusa no primeiro amistoso achado depois da primeira vaga; a
# carreira pelos amistosos de pre-temporada).
MUTACOES = [
    ('cpk_escopo_nacao', '1 // a nação da competição',
     '2 // mutacao: o tipo 2',
     'cpk_4 nation scope'),
    ('cpk_regra_nacao', 'cpk4_d(r + 0xa) == nacao && *((r + 0x12) as *const u8) == tipo // a regra da nação',
     'cpk4_d(r + 0xa) != nacao && *((r + 0x12) as *const u8) == tipo // mutacao: outra nacao',
     'cpk_4 match rule'),
    ('cpk_amistoso_vaga', 'return e + 25 * i + 9; // a vaga i',
     'return e + 9; // mutacao: a primeira vaga',
     'cpk_4 friendly slot'),
]
