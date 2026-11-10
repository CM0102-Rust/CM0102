# Mutacoes da leva114 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva114.py --jogo 2001-08-20
MUTACOES = [
    ('s_minuto', "        let min = (m.clock() as i32 / 11) as i8;", "        let min = (m.clock() as i32 / 10) as i8;"),
    ('s_teto', "            0x3de => Some(0x5a),", "            0x3de => Some(0x59),"),
    ('s_19', "    *b(match_player::POSITION) = -3;", "    *b(match_player::POSITION) = -2;"),
    ('s_goleiro', "        call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, staff, [1, 0, 0, 0, 0]);", "        call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, staff, [0, 0, 0, 0, 0]);"),
    ('s_zera', "        *((m.addr() + if j.side() == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;", "        *((m.addr() + if j.side() != 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;"),
    ('s_guardar', "        *((m.addr() as isize + l * 2 + (engine::LAST_SUBSTITUTION_TICK as isize)) as *mut i16) = m.clock();", "        *((m.addr() as isize + l * 2 + (engine::LAST_SUBSTITUTION_TICK as isize)) as *mut i16) = m.period_end();"),
    ('s_23', "        *b(match_player::RED_CARDS) = (*b(match_player::RED_CARDS)).wrapping_add(1);", "        *b(match_player::RED_CARDS) = (*b(match_player::RED_CARDS)).wrapping_add(2);"),
]
