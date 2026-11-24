# Mutacoes (cx03_10), carreira com as ligas da Escocia e de Gales ate 2002-08-01. Nenhuma abre caixa.
# - o 3o lugar da Copa da Premier galesa vale 13 no pior: o comparador do progresso acusa;
# - a Copa da Liga galesa leva o terceiro melhor segundo no lugar do segundo: outro clube joga as quartas (a carreira);
# - a Copa da Premier galesa sem sementes (todas 1): o sorteio das quartas muda (a carreira).
MUTACOES = [
    ('cx03_10_premier_galesa_terceiro_vale_treze',
     '            1 | 2 => 5,\n            3 => 9,\n            _ => 0xd,',
     '            1 | 2 => 5,\n            3 => 0xd, // mutation\n            _ => 0xd,'),
    ('cx03_10_liga_galesa_terceiro_segundo',
     '    write_usize(teams + 0x1c, read_usize(ranking.table() + league_table_row::SIZE));',
     '    write_usize(teams + 0x1c, read_usize(ranking.table() + 2 * league_table_row::SIZE)); // mutation'),
    ('cx03_10_premier_galesa_sem_sementes',
     '    let seeds: [u8; 8] = [2, 1, 2, 1, 2, 1, 2, 1];',
     '    let seeds: [u8; 8] = [1, 1, 1, 1, 1, 1, 1, 1]; // mutation'),
]
