# parte 1 de 3 das escapadas da 320a (bancada 3); tel39_6_mesma_frase ja medida (0 divergencias ate 2002-06-01)
MUTACOES = [
    ('tel41_2_trailing_zeros', '            // tel41_2: the trailing clubs with 0 leave the table\n            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 {', '            // mutation: the clubs with 0 stay\n            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 || true {'),
    ('tel41_4_instances', '    write_i32(COMP_STATS_INSTANCES, read_i32(COMP_STATS_INSTANCES).wrapping_sub(1)); // tel41_4: one instance fewer', '    // mutation: the count stays'),
]
