# Mutacoes (cqv_13), no sorteio dos grupos do Campeonato de Clubes da Oceania (janeiro de 2002; o australiano joga): o
# embaralhamento com um clube a menos; sem a ordem pela injury_word_0c antes de embaralhar; so metade embaralhada.
MUTACOES = [
    ('cqv13_embaralha_um_a_menos',
     'exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), n); // cqv13: the 8 shuffled',
     'exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), n - 1); // mutacao: um a menos',
     'cqv_13 oceania group draw'),
    ('cqv13_sem_ordenar',
     '    predicted_by_reputation_sort(list, read_usize(this + 0xf2) as u32); // cqv13: the 8 by reputation',
     '    let _ = list; // mutacao: sem ordenar',
     'cqv_13 oceania group draw'),
    ('cqv13_embaralha_a_metade',
     'exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), n); // cqv13: the 8 shuffled',
     'exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), n / 2); // mutacao: a metade',
     'cqv_13 oceania group draw'),
]
