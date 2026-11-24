# Mutacoes (cx05_3), carreira de 8 ligas ate 2004-11-01 (grupos de 13 a 19 de outubro de 2004, mata-mata de 21 a 29).
# - o grupo B e o grupo C pegam as sorteadas do grupo anterior (o B repete o A, o C repete o B);
# - os terceiros ranqueados sao os segundos colocados (as linhas 1): as quartas-de-final mudam;
# - a melhor terceira vira a segunda terceira no vetor: os confrontos das quartas mudam.
MUTACOES = [
    ('cx05_3_grupo_anterior',
     "    let first = (g + 1) as usize * 4; // cx05_3: the group's first drawn team",
     "    let first = g as usize * 4; // mutation"),
    ('cx05_3_segundos_no_lugar_dos_terceiros',
     '(group_table + 2 * league_table_row::SIZE) as *const u8, row as *mut u8, league_table_row::SIZE); // cx05_3: the third of each group',
     '(group_table + league_table_row::SIZE) as *const u8, row as *mut u8, league_table_row::SIZE); // mutation'),
    ('cx05_3_terceira_trocada',
     '    write_usize(teams + 4, read_usize(thirds_table));',
     '    write_usize(teams + 4, read_usize(thirds_table + league_table_row::SIZE)); // mutation'),
]
