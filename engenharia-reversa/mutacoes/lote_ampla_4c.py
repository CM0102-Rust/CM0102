# Lote 4c da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4c.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('p_vaga', '        if *(p as *const i32) == id && *((p + 4) as *const i8) == slot {', '        if *(p as *const i32) == id {'),
    ('a_torneio', '        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 {', '        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 && i > 0 {'),
    ('l_venda', '    remove_from_payment_list(manager, id, 8, 0x880, 0x888, 9);', '    remove_from_payment_list(manager, id, 8, 0x880, 0x888, 5);'),
    ('l_jogo', '    remove_from_payment_list(manager, id, 2, 0x88c, 0x894, 0);', '    remove_from_payment_list(manager, id, 1, 0x88c, 0x894, 0);'),
    ('l_conta', '    *((manager + how_many) as *mut i32) -= 1;', '    *((manager + how_many) as *mut i32) -= (j > i + 1) as i32;'),
    ('r_marca', '    for o in [0, 4, 8] {\n        *((last + o) as *mut i32) = -1;', '    for o in [0, 4] {\n        *((last + o) as *mut i32) = -1;'),
    ('c_empate', '        add(0xc, *((league + league::DRAW_POINTS) as *const i8) as i16);', '        add(0xc, *((league + league::DRAW_POINTS) as *const i8) as i16 + 1);'),
    ('e_data', '    (clubs && w(8) == data_a && w(0xa) == data_b) as i32', '    (clubs && w(8) == data_a) as i32'),
    ('b_tipo', '        return (*((game + 0x3a) as *const u8) == 1) as u8 + 2;', '        return (*((game + 0x3a) as *const u8) == 1) as u8 + 1;'),
]
