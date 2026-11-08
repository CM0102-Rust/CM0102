# Mutacoes. O recorde com o minimo de 400.000 (o comparando acusa pelo valor; a carreira, se a transferencia decidir).
# A match_condition sem o teto de 9000 (o comparando acusa no primeiro jogador em recuperacao com match_condition alta; a carreira
# acusa pelas notas do motor). O limite sem o 3o (o comparando acusa na primeira partida com ele).
MUTACOES = [
    ('cpj_recorde_minimo', 'rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {',
     'rep.wrapping_mul(25).wrapping_add(200_000).wrapping_mul(2) { // mutacao: 400.000'),
    ('cpj_condicao_teto', '9000 // o teto de quem se recupera',
     'v // mutacao: sem o teto'),
    ('cpj_limite_terceiro', '(2, t[2]) // o 3º limite',
     '(2, t[1]) // mutacao: o 2o byte'),
]
