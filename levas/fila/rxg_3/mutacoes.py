# Mutations through the 8-league career up to 2003-08-15 (the draw on 5 Feb 2003, the knockout stage on 5 Aug 2003):
# - rxg_3_faixa_larga: the random entrant's reputation bands step by 3000 instead of 1500: other clubs enter.
# - rxg_3_segundo_errado: the fourth semi-finalist is always the first group's runner-up, not the best one.
MUTACOES = [
    ('rxg_3_faixa_larga',
     '        threshold = threshold.wrapping_sub(1500); // rxg_3: the next reputation band',
     '        threshold = threshold.wrapping_sub(3000); // rxg_3: the next reputation band (mutation)',
     'rxg_3 continental entrant'),
    ('rxg_3_segundo_errado',
     '    write_usize(list + 0xc, best_second);',
     '    write_usize(list + 0xc, read_usize(tables[0] + league_table_row::SIZE)); // mutation',
     'rxg_3 club world championship knockout stage'),
]
