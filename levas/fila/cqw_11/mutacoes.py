# Mutacoes (cqw_11), na eliminatoria da Oceania que o jogo de 2001 monta ja no comeco: a Nova Caledonia na contest
# (outra selecao nos grupos); as dez sem a ordem pela injury_word_0c (os grupos pares e impares mudam); a final com o
# campeao do segundo grupo mandando primeiro.
MUTACOES = [
    ('cqw11_com_a_nova_caledonia',
     'let excluded = id == read_usize(KEY_NATION_NEW_CALEDONIA) as i32 || id == read_usize(KEY_NATION_PAPUA_NEW_GUINEA) as i32; // cqw11: without New Caledonia and Papua New Guinea',
     'let excluded = id == read_usize(KEY_NATION_PAPUA_NEW_GUINEA) as i32; // mutation: with New Caledonia',
     'cqw_11 qualifying nation'),
    ('cqw11_sem_ordenar',
     '    exe::crt_qsort(read_usize(this.addr() + 0xef), k as u32, 4, 0x4b07d0); // cqw11: the ten by reputation',
     '    // mutation: not sorted',
     'cqw_11 qualifying nation'),
    ('cqw11_final_invertida',
     '    write_u32(v, read_u32(exe::league_get_table_row(this, 0))); // cqw11: the first group winner is listed first\n    write_u32(v + 4, read_u32(exe::league_get_table_row(read_usize(this.stages()), 0)));',
     '    write_u32(v + 4, read_u32(exe::league_get_table_row(this, 0))); // mutation: inverted\n    write_u32(v,read_u32(exe::league_get_table_row(read_usize(this.stages()), 0)));',
     'cqw_11 oceania final'),
]
