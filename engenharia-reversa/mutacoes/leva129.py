# Mutacoes da leva129 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva129.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('j_ataque', "        1 => random(400),\n        2 => random(500),", "        1 => random(401),\n        2 => random(500),"),
    ('j_altura', "attack = ftol(attack as i16 as f64 / st);", "attack = ftol(attack as i16 as f64 / (st + 0.1));"),
    ('j_proprio', "if random(attack as i16 as i32 + 100) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"play ball on own side\")", "if random(attack as i16 as i32 + 101) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"play ball on own side\")", "play ball on own side"),
    ('j_cabeca', "((j.move_skill(moves::HANDBALL)).wrapping_shl(1), 0x25)", "(j.move_skill(moves::HANDBALL), 0x25)", "play handball"),
    ('j_forte', "valor = ftol(valor as f64 * 0.85) as i16;", "valor = ftol(valor as f64 * 0.86) as i16;", "play strong"),
    ('j_nota', "let rating = (v - strength as i32 / 3 - attack) as i16;", "let rating = (v - strength as i32 / 2 - attack) as i16;"),
    ('j_baixa', "crate::comparison::branch(\"play very low rating\");\n        j.set_busy((((j.busy() as u8)).wrapping_add(random(2) as u8)) as i8);", "crate::comparison::branch(\"play very low rating\");\n        j.set_busy((((j.busy() as u8)).wrapping_add(random(3) as u8)) as i8);", "play very low rating"),
    ('j_falta_forte', "let foul_strong = strength as i32 > random(0x96) + 0xc8;", "let foul_strong = strength as i32 > random(0x97) + 0xc8;"),
    ('j_registra', "exe::cm3_match_engine_set_pending_foul(m, lance as u16, j.addr() as u32, 0, 0, t, (j.cell_x() as u8), (j.cell_y() as u8), cartao, grav);", "exe::cm3_match_engine_set_pending_foul(m, lance as u16, j.addr() as u32, 0, 0, t, (j.cell_y() as u8), (j.cell_x() as u8), cartao, grav);", "play foul recorded"),
    ('j_penalti', "j.set_match_rating((j.match_rating()).wrapping_sub(500));\n            } else {", "j.set_match_rating((j.match_rating()).wrapping_sub(499));\n            } else {", "play penalty"),
    ('j_venceu', "j.set_match_rating((j.match_rating()).wrapping_add(0x1e));", "j.set_match_rating((j.match_rating()).wrapping_add(0x1f));", "play won"),
    ('j_estilo', "narrar7(call, event::BLOCK_SHOT, (j.commentary_id() as u32), ((j.side() as u8) as u32), u32::MAX, style);", "narrar7(call, event::BLOCK_SHOT, (j.commentary_id() as u32), ((j.side() as u8) as u32), u32::MAX, style.wrapping_add(1));", "play won"),
    ('j_meia_lua', "espirra(call, random(3), 0);", "espirra(call, random(3), j.addr());", "play arc"),
    ('j_perdeu', "j.set_match_rating((j.match_rating()).wrapping_add(0x14));", "j.set_match_rating((j.match_rating()).wrapping_add(0x15));", "play lost"),
]
