# Mutacoes (cqm_5), nos construtores da Franca (segunda e terceira) e da Espanha (Primera e Segunda), na criacao da
# carreira de 8 ligas.
# - a regra de disciplina francesa trocada (0xc -> 0);
# - a espanhola trocada (0x1a -> 0);
# - a Primera sem a divisao de baixo espanhola (o bit da nacao lido ao contrario): os jogos dela somem.
MUTACOES = [
    ('cqm5_outra_disciplina_francesa',
     'disciplina: 0xc, // cqm5: a regra de disciplina francesa',
     'disciplina: 0, // mutacao: outra regra',
     'cqm_5 divisao nova'),
    ('cqm5_outra_disciplina_espanhola',
     'disciplina: 0x1a, // cqm5: a regra de disciplina espanhola',
     'disciplina: 0, // mutacao: outra regra',
     'cqm_5 divisao nova'),
    ('cqm5_sem_a_divisao_de_baixo',
     'if *((nacao + 0x11c) as *const u8) & 4 == 0 { // cqm5: a divisão de baixo espanhola',
     'if *((nacao + 0x11c) as *const u8) & 4 != 0 { // mutacao: o bit ao contrario',
     'cqm_5 divisao nova'),
]
