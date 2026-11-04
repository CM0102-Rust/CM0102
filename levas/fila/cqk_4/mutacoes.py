# Mutacoes (cqk_4), nas regras das divisoes espanholas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a Primera rebaixa 4 em vez de 3: em abril e maio de 2002 a tabela (0x684cd0) anuncia mais um rebaixamento (os
#   registros, as clausulas e o moral);
# - a Segunda joga 1 turno: os jogos da temporada (+0xa7, pela 0x687430) viram a metade e a tabela da os lugares por
#   decididos no inverno;
# - a vitoria vale 2 pontos na Segunda: a tabela muda de ordem.
# Ficou de fora (provado pelo agente): a vitoria valendo 2 na divisao de baixo escapa (nada da carreira ate 2002-06-01
# le a tabela dela).
MUTACOES = [
    ('cqk4_primera_rebaixa_4',
     'v.push((0xc1, 1, 3)); // cqk4: a Primera rebaixa 3',
     'v.push((0xc1, 1, 4)); // mutacao: rebaixa 4',
     'cqk_4 regras da primera'),
    ('cqk4_segunda_um_turno',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a Segunda joga 2 turnos',
     'v.extend_from_slice(&[(0x3c, 2, 1), (0xc2, 1, 3)]); // mutacao: 1 turno',
     'cqk_4 regras da segunda'),
    ('cqk4_segunda_vitoria_vale_2',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a Segunda joga 2 turnos',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 2)]); // mutacao: 2 pontos',
     'cqk_4 regras da segunda'),
]
