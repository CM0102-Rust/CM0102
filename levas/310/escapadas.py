# As escapadas da 310a leva (itens mwc_18 a mwc_25: Belgica, Holanda, Alemanha e Dinamarca; ENG BEL HOL GER DEN ate 2002-08-01).

ESCAPADAS = {
    'mwc22_quarta_belga': (
        'MUTACAO EQUIVALENTE NESTA BASE (medido). A rotina `bel_fa_cup_collect_entrants` ordena por forca os clubes belgas '
        'da Quarta Divisao (grupos A a D), embaralha os 44 melhores (`comp_util_sort_and_shuffle_top_clubs(lista, n, 0x2c)`) '
        'e copia os 44 primeiros; a mutacao embaralha so os 22 melhores. Com n <= 22 as duas embaralham a lista inteira com '
        'os mesmos sorteios, e o resultado e o mesmo. Medido: o contador `mwc_22 belgian cup entrants` marca 1 execucao no '
        'jogo real da leva (a rotina roda na janela), e o comparador `compare_bel_fa_cup_collect_entrants` -- lado a lado, '
        'com o gerador reposto, conferindo cada entrada -- deu 0 divergencias com a mutacao aplicada (comparando ate a '
        'data da cadeia e na carga do save de ferias). Portanto, nesta base (DATABASE=MINIMUM), a rotina encontra no '
        'maximo 22 clubes da Quarta Divisao belga. A regra dos 44 so se distingue numa base com mais de 22 desses clubes. '
        '(O comentario do codigo diz "the best 132"; o valor e 0x2c = 44.)',
        'EQUIVALENT MUTATION IN THIS DATABASE (measured). `bel_fa_cup_collect_entrants` sorts the Belgian Fourth Division '
        'clubs (groups A to D) by strength, shuffles the best 44 (`comp_util_sort_and_shuffle_top_clubs(list, n, 0x2c)`) and '
        'copies the first 44; the mutation shuffles only the best 22. With n <= 22 both shuffle the whole list with the '
        'same draws, and the result is the same. Measured: the counter `mwc_22 belgian cup entrants` shows 1 run in the '
        "batch's real game (the routine runs in the window), and the comparer `compare_bel_fa_cup_collect_entrants` -- side "
        'by side, with the random generator restored, checking every entry -- gave 0 divergences with the mutation applied '
        '(comparing to the chain date and in the holiday save load). So in this database (DATABASE=MINIMUM) the routine '
        'finds at most 22 Belgian Fourth Division clubs. The rule of 44 only shows with more than 22 of them. (The code '
        'comment says "the best 132"; the value is 0x2c = 44.)'),
}  # nome: (explicacao pt, explicacao en)
