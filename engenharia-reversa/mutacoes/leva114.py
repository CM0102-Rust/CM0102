# Mutacoes da leva114 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva114.py --jogo 2001-08-20
MUTACOES = [
    ('s_minuto', "        let min = (*((m + engine::CLOCK) as *const i16) as i32 / 11) as i8;", "        let min = (*((m + engine::CLOCK) as *const i16) as i32 / 10) as i8;"),
    ('s_teto', "            0x3de => Some(0x5a),", "            0x3de => Some(0x59),"),
    ('s_19', "    *b(match_player::POSITION) = -3;", "    *b(match_player::POSITION) = -2;"),
    ('s_goleiro', "        call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, comissao, [1, 0, 0, 0, 0]);", "        call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, comissao, [0, 0, 0, 0, 0]);"),
    ('s_zera', "        *((m + if byte(j, match_player::SIDE) == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;", "        *((m + if byte(j, match_player::SIDE) != 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;"),
    ('s_guardar', "        *((m as isize + l * 2 + 0x8ecc) as *mut i16) = *((m + engine::CLOCK) as *const i16);", "        *((m as isize + l * 2 + 0x8ecc) as *mut i16) = *((m + engine::PERIOD_END) as *const i16);"),
    ('s_23', "        *b(0x23) = (*b(0x23)).wrapping_add(1);", "        *b(0x23) = (*b(0x23)).wrapping_add(2);"),
]
