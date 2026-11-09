# Mutacoes da leva128 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva128.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('r_goleiro', "random(ftol(2000.0 - j.handling() as f64 * 40.0))", "random(ftol(2000.0 - j.handling() as f64 * 41.0))", "respond goalkeeper in the box"),
    ('r_tipo', "            3 => random(1000),\n            4 => random(0x708),", "            3 => random(1001),\n            4 => random(0x708),"),
    ('r_altura', "attack = ftol(((m.ball_roll() as i32 + 1) * attack as i16 as i32) as f64 / st);", "attack = ftol(((m.ball_roll() as i32 + 2) * attack as i16 as i32) as f64 / st);"),
    ('r_proprio', "        if random(attack as i16 as i32 + 100) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"respond ball on own side\")", "        if random(attack as i16 as i32 + 101) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"respond ball on own side\")", "respond ball on own side"),
    ('r_cabeca', "ficha5c * 200) != 0 { choose(kind)", "ficha5c * 201) != 0 { choose(kind)"),
    ('r_nota', "    if rating < 0x96 {\n        return 0;", "    if rating < 0x95 {\n        return 0;"),
    ('r_registra', "exe::cm3_match_engine_set_pending_foul(m, code as u16, j.addr() as u32, 0, 0, t, (j.cell_x() as u8), (j.cell_y() as u8), cartao, grav);", "exe::cm3_match_engine_set_pending_foul(m, code as u16, j.addr() as u32, 0, 0, t, (j.cell_y() as u8), (j.cell_x() as u8), cartao, grav);", "respond foul recorded"),
    ('r_penalti', "                m.set_fouled_player(0);\n                j.set_match_rating((j.match_rating()).wrapping_sub(500));\n                if random", "                m.set_fouled_player(0);\n                j.set_match_rating((j.match_rating()).wrapping_sub(499));\n                if random", "respond penalty"),
    ('r_limite', "let mut lim = ftol((1500.0 - p * p * 100.0)", "let mut lim = ftol((1501.0 - p * p * 100.0)"),
    ('r_domina', "            j.set_match_rating((j.match_rating()).wrapping_add(0x1c));", "            j.set_match_rating((j.match_rating()).wrapping_add(0x1d));", "respond controlled"),
    ('r_goleiro_perde', "        if random(10) != 0 {\n            (1, 0)", "        if random(9) != 0 {\n            (1, 0)", "respond goalkeeper lost"),
    ('r_perde', "narrate(call, if kind <= 2 { 0x213e } else { 0x1f71 }", "narrate(call, if kind <= 1 { 0x213e } else { 0x1f71 }", "respond lost"),
]
