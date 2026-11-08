# Mutacoes (cql_6), nos construtores das divisoes inglesas (a criacao da carreira de 8 ligas).
# - a regra de disciplina das quatro divisoes trocada (9 -> 0): as suspensoes mudam ja nos primeiros cartoes;
# - a First Division sem o play-off (sem fases alem da liga): os jogos do play-off de maio de 2002 somem;
# - o mesmo na Second Division.
MUTACOES = [
    ('cql6_outra_disciplina',
     '*((this + 0x50) as *mut u8) = 9; // cql6: a regra de disciplina das ligas inglesas',
     '*((this + 0x50) as *mut u8) = 0; // mutacao: outra regra de disciplina',
     'cql_6 new english division'),
    ('cql6_first_sem_playoff',
     'fases: 1, // cql6: o play-off da First Division',
     'fases: 0, // mutacao: sem o play-off',
     'cql_6 new english division'),
    ('cql6_second_sem_playoff',
     'fases: 1, // cql6: o play-off da Second Division',
     'fases: 0, // mutacao: sem o play-off',
     'cql_6 new english division'),
]
