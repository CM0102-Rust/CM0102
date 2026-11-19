# As escapadas da 308a leva (itens mwg_1 a mwg_8). A confirmacao no modo comparando da cadeia vai so ate 2001-08-20;
# o jogo real compara a leva inteira ate 2002-08-01 (levas/308/ATE) com 0 divergencias.

ESCAPADAS = {
    'mwg3_goleiro_qualquer': (
        'ESCAPADA MEDIDA: a mutacao aceita como goleiro modelo qualquer jogador com goleiro >= 15, em vez de exatamente '
        '20. No `staff.dat` (a tabela de jogadores, os 30.000 primeiros registros, de onde o modelo e sorteado) ha '
        '**2.747 jogadores com goleiro 20 e so 9 entre 15 e 19**. Cada sorteio de goleiro modelo tem entao ~0,3% de '
        'chance de cair num dos 9 e mudar o resultado; nas duas corridas (carreira e comparacao ate 2001-08-20) '
        'nenhum caiu. O comparador `compare_create_virtual_player` compara a ficha copiada inteira e acusaria se '
        'caisse. Para fechar com uma mutacao que acuse sempre, trocar `== 20` por `!= 20`.',
        'MEASURED ESCAPE: the mutation accepts any player with goalkeeper >= 15 as the template goalkeeper, instead of '
        'exactly 20. In `staff.dat` (the player table, the first 30,000 records, where the template is drawn from) '
        'there are **2,747 players with goalkeeper 20 and only 9 between 15 and 19**. Each template goalkeeper draw '
        'thus has a ~0.3% chance of hitting one of the 9 and changing the result; in both runs (career and comparison '
        'until 2001-08-20) none did. The comparer `compare_create_virtual_player` compares the whole copied record and '
        'would catch it if one were hit. To close it with a mutation that always catches, replace `== 20` with `!= 20`.'),
    'mwg4_conta_qualquer': (
        'ESCAPADA EXPLICADA POR ARGUMENTO: a mutacao conta as linhas de tecnico humano com classificacao diferente de 0 '
        'em vez de igual a 1. As classificacoes das 16 linhas sao 0 (livre), 1 (em uso) e 5 (sendo criado, so durante a '
        'criacao do tecnico). Na carreira automatica de ferias o unico humano ja esta criado quando a contagem roda, '
        'entao so ha linhas 0 e 1, e `!= 0` da o mesmo que `== 1`. Nao foi medido o valor de cada linha no momento da '
        'contagem; uma mutacao que troque `== 1` por `== 0` seria acusada.',
        'ESCAPE EXPLAINED BY ARGUMENT: the mutation counts the human manager rows whose classification is not 0 instead '
        'of equal to 1. The 16 rows are classified 0 (free), 1 (in use) and 5 (being created, only while the manager is '
        'created). In the automatic holiday career the only human is already created when the count runs, so there are '
        'only rows 0 and 1, and `!= 0` gives the same as `== 1`. The value of each row at the moment of the count was '
        'not measured; a mutation replacing `== 1` with `== 0` would be caught.'),
}  # nome: (explicacao pt, explicacao en)
