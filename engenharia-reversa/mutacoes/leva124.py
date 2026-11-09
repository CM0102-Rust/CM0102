# Mutacoes da leva124 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva124.py --jogo 2001-10-01
MUTACOES = [
    ('n_cinco', "            m.set_shootout_kicks_left_side1((5) as i8);\n            *((m.addr() + engine::SHOOTOUT_KICKS_LEFT) as *mut u8) = 5;", "            m.set_shootout_kicks_left_side1((5) as i8);\n            *((m.addr() + engine::SHOOTOUT_KICKS_LEFT) as *mut u8) = 4;"),
    ('n_vez', "            let team = read_i8(m.addr() + ((0x8e9f + lu) as usize)) as i32 % 11;", "            let team = read_i8(m.addr() + ((0x8e9f + lu) as usize)) as i32 % 10;"),
    ('n_forca', "            let mut strength = (0xc8 - r.wrapping_mul(restam + 4)) << 1;", "            let mut strength = (0xc8 - r.wrapping_mul(restam + 3)) << 1;"),
    ('n_precisa', "                Some(2.25)", "                Some(2.5)"),
    ('n_direcao', "                j.set_ball_control(c.clamp(-0xc8, 0xc8));", "                j.set_ball_control(c.clamp(-0xc7, 0xc8));"),
    ('n_reacao', "                    if n >= 4 || defendeu != 0 || m.ball_holder() != j.addr() {", "                    if n >= 3 || defendeu != 0 || m.ball_holder() != j.addr() {"),
    ('n_fim', "                m.set_shootout_state((if mode as u8 == 4 { 3 } else { 6 }) as i8);", "                m.set_shootout_state((if mode as u8 == 4 { 6 } else { 3 }) as i8);"),
    ('n_placar', "            game.set_away_penalties((*((m.addr() + 0xf5f2) as *mut u8)) as i8);", "            game.set_away_penalties((*((m.addr() + engine::SIDE_STATS) as *mut u8)) as i8);"),
]
