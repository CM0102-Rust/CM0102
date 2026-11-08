# Mutações da 227a leva (itens idx_1, idx_2). Uso: python mutacoes.py mutacoes\leva227.py --prova-save 227
MUTACOES = [
    ('ix1_clube', '*(global as *mut u32) = row; // club key', '*(global as *mut u32) = row + 1; // club key'),
    ('ix1_troca', '*(g as *mut u32) = new; // chave remapeada', '*(g as *mut u32) = old; // chave remapeada'),
    ('ix1_nacao', '*(global as *mut u32) = row; // nation key', '*(global as *mut u32) = row + 1; // nation key'),
    ('ix2_comp_clube', '*(global as *mut u32) = row; // club competition key', '*(global as *mut u32) = row + 1; // club competition key'),
    ('ix2_comp_pessoa', '*(global as *mut u32) = row; // person competition key', '*(global as *mut u32) = row + 1; // person competition key'),
    ('ix2_devolucao', 'r as u32 // the eax of the last _stricmp (club competition key)', '(r != 0) as u32 // the eax of the last _stricmp (club competition key)'),
]
