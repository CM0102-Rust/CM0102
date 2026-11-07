# Mutacoes (cqv_13), no sorteio dos grupos do Campeonato de Clubes da Oceania (janeiro de 2002; o australiano joga): o
# embaralhamento com um clube a menos; sem a ordem pela reputacao antes de embaralhar; so metade embaralhada.
MUTACOES = [
    ('cqv13_embaralha_um_a_menos',
     'embaralhar(cqv13_d(this + 0xee), n); // cqv13: os 8 embaralhados',
     'embaralhar(cqv13_d(this + 0xee), n - 1); // mutacao: um a menos',
     'cqv_13 sorteio dos grupos da oceania'),
    ('cqv13_sem_ordenar',
     '    cqv13_ordenar(lista, cqv13_d(this + 0xf2) as u32); // cqv13: os 8 pela reputação',
     '    let _ = lista; // mutacao: sem ordenar',
     'cqv_13 sorteio dos grupos da oceania'),
    ('cqv13_embaralha_a_metade',
     'embaralhar(cqv13_d(this + 0xee), n); // cqv13: os 8 embaralhados',
     'embaralhar(cqv13_d(this + 0xee), n / 2); // mutacao: a metade',
     'cqv_13 sorteio dos grupos da oceania'),
]
