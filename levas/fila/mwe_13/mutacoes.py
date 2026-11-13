# Mutations (mwe_13): the Liga de Honra round of 7 November 2001 moved to 14 November (free: the next is on the 21st);
# the Superettan promotion play-off a week later (the first leg on 22 instead of 15 November 2001); the Ykkonen round
# of 17 June moved to 24 June (free: the next is on 8 July; the 2002 season, built at the end of 2001).
MUTACOES = [
    ('mwe13_liga_de_honra_rodada_de_novembro_7_dias_depois',
     '    (7, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 9: 7/nov (mwe13: the Liga de Honra November round)',
     '    (14, 10, 0, 6, 1, &[(6, 2, 1), (-1, -1, -1)]), // 9: mutation: 14/nov',
     'mwe_13 liga de honra schedule'),
    ('mwe13_playoff_da_superettan_7_dias_depois',
     'cql10r((9, 0xa, 0, 0), (0xf, 0xa, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwe13: the Superettan play-off dates',
     'cql10r((0x10, 0xa, 0, 0), (0x16, 0xa, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mutation: a week later',
     'mwe_13 superettan play-off round'),
    ('mwe13_ykkonen_rodada_de_junho_7_dias_depois',
     '    (17, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 17/jun (mwe13: the Ykkonen June round)',
     '    (24, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: mutation: 24/jun',
     'mwe_13 ykkonen group schedule'),
]
