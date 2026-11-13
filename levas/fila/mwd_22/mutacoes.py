# Mutacoes (mwd_22): na virada grega o sorteio entre os melhores da Terceira Divisao embaralha os 18 primeiros em vez dos
# 9: outros clubes sobem (as divisoes dos clubes no banco e os jogos de 2002/03 mudam).
MUTACOES = [
    ('mwd22_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd22: the best nine shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 6); // mutation: the best eighteen shuffled',
     'mwd_22 greek third division up'),
]
