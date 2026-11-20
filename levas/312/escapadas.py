# As escapadas da 312a leva (itens mwe_18 a mwe_21: copas da Finlandia, Noruega e Portugal, Supertaca; ENG SWE FIN NOR POR ate 2002-08-01).

ESCAPADAS = {
    'mwe19_reservas_entram': (
        'MUTACAO EQUIVALENTE NESTA BASE (medido). `nor_cup_collect_entrants` deixa de fora os times reservas, os clubes com '
        '"2" no nome curto (`crt_strstr(short_name, "2")`); a mutacao os deixa entrar. Medido: o contador '
        '`mwe_19 norwegian cup entrants` marca 1 execucao no jogo real da leva (a rotina roda na janela), e o comparador '
        '`compare_nor_cup_collect_entrants` -- lado a lado, com o gerador reposto, conferindo cada uma das entradas -- deu '
        '0 divergencias com a mutacao aplicada (comparando ate a data da cadeia e na carga do save de ferias). Portanto, '
        'nesta base (DATABASE=MINIMUM), nenhum clube das divisoes norueguesas carregadas que a rotina percorre tem "2" no '
        'nome curto: a regra dos reservas so se distingue numa base que traga os times B noruegueses.',
        'EQUIVALENT MUTATION IN THIS DATABASE (measured). `nor_cup_collect_entrants` leaves out the reserve sides, the clubs '
        'with "2" in the short name (`crt_strstr(short_name, "2")`); the mutation lets them in. Measured: the counter '
        "`mwe_19 norwegian cup entrants` shows 1 run in the batch's real game (the routine runs in the window), and the "
        'comparer `compare_nor_cup_collect_entrants` -- side by side, with the random generator restored, checking every '
        'entry -- gave 0 divergences with the mutation applied (comparing to the chain date and in the holiday save load). '
        'So in this database (DATABASE=MINIMUM) no club of the loaded Norwegian divisions the routine walks has "2" in its '
        'short name: the reserve rule only shows with a database that carries the Norwegian B teams.'),
}  # nome: (explicacao pt, explicacao en)
