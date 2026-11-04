# Mutacoes (cqi_4), nas regras das divisoes inglesas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a Premier rebaixa 4 em vez de 3: em abril e maio de 2002 a tabela (0x684cd0) anuncia o rebaixamento de mais um
#   clube (os registros, as clausulas de rebaixamento e o moral: o rebaixamento pela tabela foi acusado na 262a);
# - a First joga 1 turno: +0xa7 (jogos da temporada) vira a metade e a tabela da como decididos os lugares muito antes
#   (anuncios de acesso e rebaixamento ja no inverno);
# - a vitoria vale 2 pontos na Second: a tabela muda de ordem e com ela as posicoes, os anuncios e as vagas.
# O contador fica no comeco de cada rotina (roda uma vez por liga).
MUTACOES = [
    ('cqi4_premier_rebaixa_4',
     'v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3',
     'v.push((0xc1, 1, 4)); // mutacao: rebaixa 4',
     'cqi_4 regras da premier'),
    ('cqi4_first_um_turno',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos',
     'v.extend_from_slice(&[(0x3c, 2, 1), (0xc2, 1, 3)]); // mutacao: 1 turno',
     'cqi_4 regras da first'),
    ('cqi4_second_vitoria_vale_2',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Second dá 3 pontos pela vitória',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 2)]); // mutacao: 2 pontos',
     'cqi_4 regras da second'),
]
