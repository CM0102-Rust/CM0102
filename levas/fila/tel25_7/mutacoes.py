# Mutations (tel25_7). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat; the competition screen's Print popup, to a file):
# - the appearances column one narrower (caught by the text comparer of the statistics table);
# - the champions marked as qualified (caught by the text comparer of the league tables).
MUTACOES = [
    ('tel25_7_apps_width', 'printout_format_stat(stats, row, 1, t, 0xc8, 1);\n            printout_column(t, 7);', 'printout_format_stat(stats, row, 1, t, 0xc8, 1);\n            printout_column(t, 6);'),
    ('tel25_7_champions_mark', '        5 => 0xa78f10,', '        5 => 0x99c1b8,'),
]
