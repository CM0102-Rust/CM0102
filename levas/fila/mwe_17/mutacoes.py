# Mutations (mwe_17): the Super Cup with the Premier Division runner-up instead of the champion (another club plays
# the 12 August 2001 match); the Super Cup match a week later (19 August 2001); the Super Cup turnover two years ahead
# (in the summer of 2002).
MUTACOES = [
    ('mwe17_supertaca_com_o_vice',
     'let a = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0); // mwe17: the Portuguese champion',
     'let a = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 1); // mutation: the runner-up',
     'mwe_17 portuguese super cup clubs'),
    ('mwe17_supertaca_7_dias_depois',
     'cql10r((5, 6, 0, 5), (0xc, 7, 0, 5, 1, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (1, 1, 0x7e), (0, 0, 0)); // mwe17: the Portuguese Super Cup date',
     'cql10r((5, 6, 0, 5), (0x13, 7, 0, 5, 1, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (1, 1, 0x7e), (0, 0, 0)); // mutation: a week later',
     'mwe_17 portuguese super cup round'),
    ('mwe17_supertaca_dois_anos',
     'this.set_year(this.year().wrapping_add(1)); // mwe17: the Super Cup next year',
     'this.set_year(this.year().wrapping_add(2)); // mutation: two years ahead',
     'mwe_17 portuguese super cup turnover'),
]
