# Mutacoes (cqj_5), nas regras das divisoes francesas (rodam uma vez cada, na criacao da carreira de 8 ligas):
# - a primeira rebaixa 3 em vez de 2 em 2001: em abril e maio de 2002 a tabela (0x684cd0) anuncia mais um
#   rebaixamento (os registros, as clausulas e o moral);
# - a vitoria vale 2 pontos na segunda: a tabela muda de ordem;
# - o National joga 1 turno: os jogos da temporada (+0xa7) viram a metade e a tabela da os lugares por decididos no
#   inverno.
MUTACOES = [
    ('cqj5_primeira_rebaixa_3',
     'let caem = if cqj5_em_2001(this) { 2 } else { 3 }; // cqj5: a primeira rebaixa 2 em 2001',
     'let caem = if cqj5_em_2001(this) { 3 } else { 3 }; // mutacao: rebaixa 3',
     'cqj_5 regras da primeira'),
    ('cqj5_segunda_vitoria_vale_2',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: a segunda dá 3 pontos pela vitória',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 2)]); // mutacao: 2 pontos',
     'cqj_5 regras da segunda'),
    ('cqj5_national_um_turno',
     'v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: o National joga 2 turnos',
     'v.extend_from_slice(&[(0x3c, 2, 1), (0xc2, 1, 3)]); // mutacao: 1 turno',
     'cqj_5 regras do national'),
]
