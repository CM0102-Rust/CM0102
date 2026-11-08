# Mutacoes da leva124 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva124.py --jogo 2001-10-01
MUTACOES = [
    ('n_cinco', "            *b(0x8ea2) = 5;\n            *b(engine::SHOOTOUT_KICKS_LEFT) = 5;", "            *b(0x8ea2) = 5;\n            *b(engine::SHOOTOUT_KICKS_LEFT) = 4;"),
    ('n_vez', "            let time = byte(m, (0x8e9f + lu) as usize) as i32 % 11;", "            let time = byte(m, (0x8e9f + lu) as usize) as i32 % 10;"),
    ('n_forca', "            let mut strength = (0xc8 - r.wrapping_mul(restam + 4)) << 1;", "            let mut strength = (0xc8 - r.wrapping_mul(restam + 3)) << 1;"),
    ('n_precisa', "                Some(2.25)", "                Some(2.5)"),
    ('n_direcao', "                *((j + match_player::BALL_CONTROL) as *mut i16) = c.clamp(-0xc8, 0xc8);", "                *((j + match_player::BALL_CONTROL) as *mut i16) = c.clamp(-0xc7, 0xc8);"),
    ('n_reacao', "                    if n >= 4 || defendeu != 0 || u(engine::BALL_HOLDER) != j {", "                    if n >= 3 || defendeu != 0 || u(engine::BALL_HOLDER) != j {"),
    ('n_fim', "                *b(engine::SHOOTOUT_STATE) = if mode as u8 == 4 { 3 } else { 6 };", "                *b(engine::SHOOTOUT_STATE) = if mode as u8 == 4 { 6 } else { 3 };"),
    ('n_placar', "            *((game + fixture::AWAY_PENALTIES) as *mut u8) = *b(0xf5f2);", "            *((game + fixture::AWAY_PENALTIES) as *mut u8) = *b(engine::SIDE_STATS);"),
]
