# Mutações da 227a leva (itens idx_1, idx_2). Uso: python mutacoes.py mutacoes\leva227.py --prova-save 227
MUTACOES = [
    ('ix1_clube', '*(global as *mut u32) = linha; // chave de clube', '*(global as *mut u32) = linha + 1; // chave de clube'),
    ('ix1_troca', '*(g as *mut u32) = novo; // chave remapeada', '*(g as *mut u32) = antigo; // chave remapeada'),
    ('ix1_nacao', '*(global as *mut u32) = linha; // chave de nação', '*(global as *mut u32) = linha + 1; // chave de nação'),
    ('ix2_comp_clube', '*(global as *mut u32) = linha; // chave de competição de clube', '*(global as *mut u32) = linha + 1; // chave de competição de clube'),
    ('ix2_comp_pessoa', '*(global as *mut u32) = linha; // chave de competição de pessoa', '*(global as *mut u32) = linha + 1; // chave de competição de pessoa'),
    ('ix2_devolucao', 'r as u32 // o eax do último _stricmp (chave de competição de clube)', '(r != 0) as u32 // o eax do último _stricmp (chave de competição de clube)'),
]
