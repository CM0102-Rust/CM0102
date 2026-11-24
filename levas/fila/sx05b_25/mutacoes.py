# Mutacoes (sx05b_25, telas: a cadeia nao as roda; para provar o comparador numa sessao do Lucas).
# - o historico das equipes sem a equipe atual;
# - a formacao do modo 0 com o centroavante no lugar do ponta.
MUTACOES = [
    ('sx05b_25_sem_equipe_atual',
     '    exe::llist_add_tail(list, this + ta::RECORD, ta::RECORD_SIZE);\n    list',
     '    list // mutation: the current team not listed',
     'sx05b_25 current team listed'),
    ('sx05b_25_modo_zero_trocado',
     '    [1, 0x804, 0x84, 0x104, 0x404, 0x110, 0x810, 0x410, 0x140, 0x440, 0x90],',
     '    [1, 0x804, 0x84, 0x104, 0x404, 0x110, 0x810, 0x410, 0x140, 0x90, 0x440], // mutation'),
]
