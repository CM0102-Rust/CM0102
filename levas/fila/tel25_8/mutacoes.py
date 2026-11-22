# Mutations (tel25_8). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat; the club screen's Print, to a file, with the squad, statistics and
# transfers ticked):
# - the transfers' "in" and "out" titles swapped (caught by the transfers' text comparer);
# - the appearances column of the squad statistics one narrower (caught by its text comparer);
# - the positions column of the club printout's squad one narrower (caught by the club printout's file comparer).
MUTACOES = [
    ('tel25_8_transfer_titles', 'if pass == 1 { 0xa78c4c } else { 0xa78c08 }', 'if pass == 1 { 0xa78c08 } else { 0xa78c4c }'),
    ('tel25_8_stats_apps_width', '(1, [1, 0, 1], 7), (2, [0, 0, 1], 5)', '(1, [1, 0, 1], 6), (2, [0, 0, 1], 5)'),
    ('tel25_8_positions_width', '        column(t, 0xb);', '        column(t, 0xa);'),
]
