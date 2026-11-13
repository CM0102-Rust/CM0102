# Mutacoes (mwd_6): na virada croata o sorteio entre os melhores da terceira divisao embaralha os 8 primeiros em vez dos 4:
# outros clubes sobem (as divisoes dos clubes no banco e os jogos de 2002/03 a partir de 24/7 mudam).
MUTACOES = [
    ('mwd6_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd6: the best four, shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 8); // mutation: shuffle the best eight',
     'mwd_6 croatian third tier up'),
]
