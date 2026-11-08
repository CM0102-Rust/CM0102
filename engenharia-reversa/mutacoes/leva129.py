# Mutacoes da leva129 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva129.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('j_ataque', "        1 => random(400),\n        2 => random(500),", "        1 => random(401),\n        2 => random(500),"),
    ('j_altura', "attack = ftol(attack as i16 as f64 / st);", "attack = ftol(attack as i16 as f64 / (st + 0.1));"),
    ('j_proprio', "if random(attack as i16 as i32 + 100) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"play ball on own side\")", "if random(attack as i16 as i32 + 101) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"play ball on own side\")", "play ball on own side"),
    ('j_cabeca', "((*w(j, 0x164)).wrapping_shl(1), 0x25)", "(*w(j, 0x164), 0x25)", "play header"),
    ('j_forte', "valor = ftol(valor as f64 * 0.85) as i16;", "valor = ftol(valor as f64 * 0.86) as i16;", "play strong"),
    ('j_nota', "let rating = (v - strength as i32 / 3 - attack) as i16;", "let rating = (v - strength as i32 / 2 - attack) as i16;"),
    ('j_baixa', "crate::comparison::branch(\"play very low rating\");\n        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(random(2) as u8);", "crate::comparison::branch(\"play very low rating\");\n        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(random(3) as u8);", "play very low rating"),
    ('j_falta_forte', "let foul_strong = strength as i32 > random(0x96) + 0xc8;", "let foul_strong = strength as i32 > random(0x97) + 0xc8;"),
    ('j_registra', "exe::cm3_match_engine_set_pending_foul(m, lance as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);", "exe::cm3_match_engine_set_pending_foul(m, lance as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_Y), *b(j, match_player::CELL_X), cartao, grav);", "play foul recorded"),
    ('j_penalti', "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(500);\n            } else {", "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(499);\n            } else {", "play penalty"),
    ('j_venceu', "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x1e);", "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x1f);", "play won"),
    ('j_estilo', "narrar7(call, 0x1f65, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, style);", "narrar7(call, 0x1f65, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, style.wrapping_add(1));", "play won"),
    ('j_meia_lua', "espirra(call, random(3), 0);", "espirra(call, random(3), j);", "play arc"),
    ('j_perdeu', "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x14);", "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x15);", "play lost"),
]
