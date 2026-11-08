# Mutacoes da leva129 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva129.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('j_ataque', "        1 => sortear(400),\n        2 => sortear(500),", "        1 => sortear(401),\n        2 => sortear(500),"),
    ('j_altura', "attack = match_pl_ftol(attack as i16 as f64 / st);", "attack = match_pl_ftol(attack as i16 as f64 / (st + 0.1));"),
    ('j_proprio', "if sortear(attack as i16 as i32 + 100) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"play bola", "if sortear(attack as i16 as i32 + 101) != 0 {\n            return 0;\n        }\n        crate::comparison::branch(\"play bola", "play bola do proprio lado"),
    ('j_cabeca', "((*w(j, 0x164)).wrapping_shl(1), 0x25)", "(*w(j, 0x164), 0x25)", "play cabecada"),
    ('j_forte', "valor = match_pl_ftol(valor as f64 * 0.85) as i16;", "valor = match_pl_ftol(valor as f64 * 0.86) as i16;", "play forte"),
    ('j_nota', "let rating = (v - strength as i32 / 3 - attack) as i16;", "let rating = (v - strength as i32 / 2 - attack) as i16;"),
    ('j_baixa', "crate::comparison::branch(\"play nota muito baixa\");\n        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(sortear(2) as u8);", "crate::comparison::branch(\"play nota muito baixa\");\n        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(sortear(3) as u8);", "play nota muito baixa"),
    ('j_falta_forte', "let foul_strong = strength as i32 > sortear(0x96) + 0xc8;", "let foul_strong = strength as i32 > sortear(0x97) + 0xc8;"),
    ('j_registra', "register(m, lance as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);", "register(m, lance as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_Y), *b(j, match_player::CELL_X), cartao, grav);", "play falta registrada"),
    ('j_penalti', "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(500);\n            } else {", "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(499);\n            } else {", "play penalty"),
    ('j_venceu', "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x1e);", "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x1f);", "play venceu"),
    ('j_estilo', "narrar7(call, 0x1f65, ub(j, match_player::COMMENTARY_ID), ub(j, match_player::SIDE), u32::MAX, estilo);", "narrar7(call, 0x1f65, ub(j, match_player::COMMENTARY_ID), ub(j, match_player::SIDE), u32::MAX, estilo.wrapping_add(1));", "play venceu"),
    ('j_meia_lua', "espirra(call, sortear(3), 0);", "espirra(call, sortear(3), j);", "play meia-lua"),
    ('j_perdeu', "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x14);", "*w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x15);", "play perdeu"),
]
