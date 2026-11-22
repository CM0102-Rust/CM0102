# parte 3 de 3 das escapadas da 320a (bancada 5)
MUTACOES = [
    ('tel39_8_uma_linha', 'while pundit_list_read_pundit_line(this, file, line.as_mut_ptr() as usize) != 0 {}', 'if pundit_list_read_pundit_line(this, file, line.as_mut_ptr() as usize) != 0 {}', 'tel39_8 pundit list'),
    ('tel41_3_flag_cleared', '        write_u32(flag, 0); // tel41_3: the flag cleared', '        // mutation: the flag stays'),
    ('tel42_2_posicao', 'rec[6] = read_u8(row + league_table_row::POSITION); // tel42_2: the final position', 'rec[6] = read_u8(row + league_table_row::POSITION).wrapping_add(1); // tel42_2: the final position (mutation)', 'tel42_2 league table into the history'),
]
