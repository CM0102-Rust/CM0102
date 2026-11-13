# Mutacoes (mwd_23): na virada russa o sorteio entre os candidatos das zonas embaralha os 18 primeiros em vez dos 9:
# outros clubes sobem a Primeira Divisao (o banco e os jogos de 2002 mudam).
MUTACOES = [
    ('mwd23_sobem_outros',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd23: the best nine shuffled',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n, 6); // mutation: the best eighteen shuffled',
     'mwd_23 russian zones up'),
]
