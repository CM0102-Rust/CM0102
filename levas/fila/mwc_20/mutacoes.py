# Mutacoes (mwc_20): na virada dinamarquesa o sorteio entre os melhores da divisao mais baixa embaralha os 24 primeiros
# em vez dos 12: outros clubes sobem a Segunda Divisao (as divisoes dos clubes no banco e os jogos de 2002/03 mudam).
MUTACOES = [
    ('mwc20_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc20: the best twelve shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: the best twenty-four shuffled',
     'mwc_20 danish lowest division up'),
]
