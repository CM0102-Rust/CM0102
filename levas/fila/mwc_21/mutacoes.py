# Mutacoes (mwc_21): na virada alema o sorteio entre os melhores da divisao da Regionalliga embaralha os 24 primeiros em
# vez dos 12: outros clubes sobem a 2. Bundesliga (as divisoes dos clubes no banco e os jogos de 2002/03 mudam).
MUTACOES = [
    ('mwc21_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc21: the best twelve shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: the best twenty-four shuffled',
     'mwc_21 german regional division up'),
]
