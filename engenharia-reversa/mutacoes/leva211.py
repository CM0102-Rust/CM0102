# Mutações da 211a leva (itens sim_transferencias_3, sim_diversos_1, sim_diversos_2). Uso: python mutacoes.py mutacoes\leva211.py --carreira 2002-06-01
MUTACOES = [
    ('f_estado', '    if r != 0 {\n        write_u8(r + contract_offer::STATE, 4);', '    if r != 0 {\n        write_u8(r + contract_offer::STATE, 3);'),
    ('l_motivo', '    r[4] = reason;', '    r[4] = reason ^ 1;'),
    ('n_contrato', '    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());\n    if c.is_null() {\n        return 0;\n    }\n    let p =', '    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr() | 1);\n    if c.is_null() {\n        return 0;\n    }\n    let p ='),
    ('s_faixa', '                *size = (*size).wrapping_sub(1);', '                *size = (*size).wrapping_sub(0);'),
    ('h_dia', '    if day == 6 || day == 5 { 1 } else { 2 }', '    if day == 6 { 1 } else { 2 }'),
    ('o_fator', '        5 => rep.wrapping_mul(550),', '        5 => rep.wrapping_mul(551),'),
    ('o_outra', 'e != 0 && *((e + offer::TARGET_STAFF_ID) as *const i32) == id)', 'e != 0 && *((e + offer::TARGET_STAFF_ID) as *const i32) != id)'),
    ('r_base', '/ 10 + 0x1194) as i16);', '/ 10 + 0x1195) as i16);'),
    ('n_troca', 'read_u32(game + 4), read_u32(game + 0),', 'read_u32(game + 0), read_u32(game + 4),'),
]
