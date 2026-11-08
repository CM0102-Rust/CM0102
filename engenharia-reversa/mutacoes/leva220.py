# Mutações da 220a leva (itens save_s3_a, save_s3_b). Uso: python mutacoes.py mutacoes\leva220.py --prova-save 220
MUTACOES = [
    ('sv3_criar_id', '        save_game_data(file.as_ptr(), 0x17);', '        save_game_data(file.as_ptr(), 0x16);'),
    ('sv3_criar_frente', '                        *m = mark;\n                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;\n                        *(G_SELECTED_NATION_COUNT as *mut i32) += 1;', '                        *m = mark;\n                        *(G_NATIONS_TO_LOAD_COUNT as *mut i32) += 1;\n                        *(G_SELECTED_NATION_COUNT as *mut i32) += 2;'),
    ('sv3_gravar_id_unico', '|| !field(0xdd7120, 4, 1, 0x29bd, 0xa81aa4)', '|| !field(0xdd7124, 4, 1, 0x29bd, 0xa81aa4)'),
    ('sv3_gravar_resumo', '    if !field(summary.as_ptr() as usize, 0xf74, 1, 0x29b5, 0xa81ae0)', '    if !field(summary.as_ptr() as usize, 0xf70, 1, 0x29b5, 0xa81ae0)'),
    ('sv3_gravar_fifa', '    regras_fifa(l);', '    let _ = regras_fifa;'),
]
