# Lote 4c da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4c.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('p_vaga', '        if *(p as *const i32) == id && *((p + 4) as *const i8) == vaga {', '        if *(p as *const i32) == id {'),
    ('a_torneio', '        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 {', '        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 && i > 0 {'),
    ('l_venda', '    remove_from_payment_list(gerente, id, 8, 0x880, 0x888, 9);', '    remove_from_payment_list(gerente, id, 8, 0x880, 0x888, 5);'),
    ('l_jogo', '    remove_from_payment_list(gerente, id, 2, 0x88c, 0x894, 0);', '    remove_from_payment_list(gerente, id, 1, 0x88c, 0x894, 0);'),
    ('l_conta', '    *((gerente + quantos) as *mut i32) -= 1;', '    *((gerente + quantos) as *mut i32) -= (j > i + 1) as i32;'),
    ('r_marca', '    for o in [0, 4, 8] {\n        *((ultimo + o) as *mut i32) = -1;', '    for o in [0, 4] {\n        *((ultimo + o) as *mut i32) = -1;'),
    ('c_empate', '        somar(0xc, *((liga + 0xc3) as *const i8) as i16);', '        somar(0xc, *((liga + 0xc3) as *const i8) as i16 + 1);'),
    ('e_data', '    (clubes && w(8) == data_a && w(0xa) == data_b) as i32', '    (clubes && w(8) == data_a) as i32'),
    ('b_tipo', '        return (*((jogo + 0x3a) as *const u8) == 1) as u8 + 2;', '        return (*((jogo + 0x3a) as *const u8) == 1) as u8 + 1;'),
]
