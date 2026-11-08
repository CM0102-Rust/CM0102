# Mutações da 211a leva (itens sim_transferencias_3, sim_diversos_1, sim_diversos_2). Uso: python mutacoes.py mutacoes\leva211.py --carreira 2002-06-01
MUTACOES = [
    ('f_estado', '    if r != 0 {\n        *((r + 0x50) as *mut u8) = 4;', '    if r != 0 {\n        *((r + 0x50) as *mut u8) = 3;'),
    ('l_motivo', '    r[4] = motivo;', '    r[4] = motivo ^ 1;'),
    ('n_contrato', '    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;\n    if c == 0 {\n        return 0;\n    }\n    let p =', '    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c | 1;\n    if c == 0 {\n        return 0;\n    }\n    let p ='),
    ('s_faixa', '                *tamanho = (*tamanho).wrapping_sub(1);', '                *tamanho = (*tamanho).wrapping_sub(0);'),
    ('h_dia', '    if dia == 6 || dia == 5 { 1 } else { 2 }', '    if dia == 6 { 1 } else { 2 }'),
    ('o_fator', '        5 => rep.wrapping_mul(550),', '        5 => rep.wrapping_mul(551),'),
    ('o_outra', 'e != 0 && *((e + 0xc) as *const i32) == id)', 'e != 0 && *((e + 0xc) as *const i32) != id)'),
    ('r_base', '/ 10 + 0x1194) as i16);', '/ 10 + 0x1195) as i16);'),
    ('n_troca', 'd(4), d(0),', 'd(0), d(4),'),
]
