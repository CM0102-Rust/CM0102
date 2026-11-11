# Lote 4c da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4c.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limit 7200
MUTACOES = [
    ('p_vaga', '        if read_i32(p) == id && *((p + held_ban::SLOT) as *const i8) == slot {', '        if read_i32(p) == id {'),
    ('a_torneio', '        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && object != 0 {', '        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && object != 0 && i > 0 {'),
    ('l_venda', '    remove_from_payment_list(TransferManagerFields(manager), id, 8, 0x880, 0x888, 9);', '    remove_from_payment_list(TransferManagerFields(manager), id, 8, 0x880, 0x888, 5);'),
    ('l_jogo', '    remove_from_payment_list(TransferManagerFields(manager), id, 2, 0x88c, 0x894, 0);', '    remove_from_payment_list(TransferManagerFields(manager), id, 1, 0x88c, 0x894, 0);'),
    ('l_conta', '    *((manager.addr() + how_many) as *mut i32) -= 1;', '    *((manager.addr() + how_many) as *mut i32) -= (j > i + 1) as i32;'),
    ('r_marca', '    for o in [0, 4, 8] {\n        write_i32(last + o, -1);', '    for o in [0, 4] {\n        write_i32(last + o, -1);'),
    ('c_empate', '        add(0xc, (league.draw_points() as i8) as i16);', '        add(0xc, (league.draw_points() as i8) as i16 + 1);'),
    ('e_data', '    (clubs && w(8) == data_a && w(match_stats_entry::YEAR) == data_b) as i32', '    (clubs && w(8) == data_a) as i32'),
    ('b_tipo', '        return (game.category() == match_category::FRIENDLY) as u8 + 2;', '        return (game.category() == 1) as u8 + 1;'),
]
