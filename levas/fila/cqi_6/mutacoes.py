# Mutacoes (cqi_6), nas regras das divisoes italianas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a Serie A rebaixa 3 em vez de 4: em abril e maio de 2002 a tabela (0x684cd0) anuncia um rebaixamento a menos (os
#   registros, as clausulas de rebaixamento e o moral);
# - a Serie B joga 1 turno: os jogos da temporada (+0xa7, pela 0x687430) viram a metade e a tabela da como decididos
#   os lugares no inverno;
# - a vitoria vale 2 pontos na C1/A: a tabela muda de ordem.
MUTACOES = [
    ('cqi6_serie_a_rebaixa_3',
     'v.push((0xc1, 1, 4)); // cqi6: a Série A rebaixa 4',
     'v.push((0xc1, 1, 3)); // mutacao: rebaixa 3',
     'cqi_6 serie a rules'),
    ('cqi6_serie_b_um_turno',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a Série B joga 2 turnos',
     'v.extend_from_slice(&[(0x3c, 2, 1), (0xc2, 1, 3)]); // mutacao: 1 turno',
     'cqi_6 serie b rules'),
    ('cqi6_c1a_vitoria_vale_2',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a C1/A dá 3 pontos pela vitória',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 2)]); // mutacao: 2 pontos',
     'cqi_6 c1a rules'),
]
