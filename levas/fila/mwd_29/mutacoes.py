# Mutacoes (mwd_29): na virada polonesa o sorteio entre os melhores da divisao mais baixa embaralha os 24 primeiros em
# vez dos 12: outros clubes sobem (as divisoes dos clubes no banco e os jogos de 2002/03 mudam).
MUTACOES = [
    ('mwd29_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd29: the best twelve shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: the best twenty-four shuffled',
     'mwd_29 polish lowest division up'),
]
