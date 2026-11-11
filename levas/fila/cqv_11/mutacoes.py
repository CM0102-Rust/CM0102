# Mutacoes (cqv_11), nas quartas da Mercosul de 2001 (clubes brasileiros jogam): o quarto segundo no lugar do terceiro
# (outro clube nas quartas); os terceiros dos grupos disputando as vagas de melhor segundo; o primeiro do grupo A
# trocado com o primeiro segundo no vetor das quartas (outro sorteio de confrontos).
MUTACOES = [
    ('cqv11_quarto_segundo',
     'write_usize(v + (k + s as usize) * 4, read_usize(exe::league_get_table_row(runners_up, s))); // cqv11: runner-up s into the quarter-finals',
     'write_usize(v + (k + s as usize) * 4, read_usize(exe::league_get_table_row(runners_up, s + 1))); // mutacao: o seguinte',
     'cqv_11 best runners-up'),
    ('cqv11_terceiros_na_disputa',
     '+ league_table_row::SIZE; // cqv11: the row of the group runner-up',
     '+ 2 * league_table_row::SIZE; // mutacao: o terceiro',
     'cqv_11 best runners-up'),
    ('cqv11_ordem_das_quartas',
     'let eight = v; // cqv11: the 8 quarter-finalists in order',
     'let eight = { let a = read_usize(v); write_usize(v, read_usize(v + k * 4)); write_usize(v + k * 4, a); v }; // mutacao: trocados',
     'cqv_11 best runners-up'),
]
