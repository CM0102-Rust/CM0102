# Mutations (mwe_12): one round of each league moved 7 days later onto a free date (the game snaps rounds to the
# weekday: 1 or 2 days would land in the same place): the Allsvenskan July round (19 to 26 July, before 2 August;
# in the 2001 or 2002 season, both inside the career), the Veikkausliiga last round (28 October to 4 November) and
# the Portuguese round of 7 November (to 14 November, before 21 November).
MUTACOES = [
    ('mwe12_allsvenskan_rodada_de_julho_7_dias_depois',
     '    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19/jul (mwe12: the Allsvenskan July round)',
     '    (26, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: mutation: 26/jul',
     'mwe_12 allsvenskan schedule'),
    ('mwe12_veikkausliiga_ultima_rodada_7_dias_depois',
     '    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 32: 28/out (mwe12: the Veikkausliiga last round)',
     '    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 32: mutation: 4/nov',
     'mwe_12 veikkausliiga schedule'),
    ('mwe12_portuguesa_rodada_de_novembro_7_dias_depois',
     '    (7, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 7/nov (mwe12: the Portuguese November round)',
     '    (14, 10, 0, 6, 1, &[(4, 2, 1), (5, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: mutation: 14/nov',
     'mwe_12 portuguese premier schedule'),
]
