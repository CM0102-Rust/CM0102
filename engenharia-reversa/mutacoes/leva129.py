# Mutacoes da leva129 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva129.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('j_ataque', "        1 => sortear(400),\n        2 => sortear(500),", "        1 => sortear(401),\n        2 => sortear(500),"),
    ('j_altura', "ataque = match_pl_ftol(ataque as i16 as f64 / st);", "ataque = match_pl_ftol(ataque as i16 as f64 / (st + 0.1));"),
    ('j_proprio', "if sortear(ataque as i16 as i32 + 100) != 0 {\n            return 0;\n        }\n        crate::comparacao::ramo(\"play bola", "if sortear(ataque as i16 as i32 + 101) != 0 {\n            return 0;\n        }\n        crate::comparacao::ramo(\"play bola", "play bola do proprio lado"),
    ('j_cabeca', "((*w(j, 0x164)).wrapping_shl(1), 0x25)", "(*w(j, 0x164), 0x25)", "play cabecada"),
    ('j_forte', "valor = match_pl_ftol(valor as f64 * 0.85) as i16;", "valor = match_pl_ftol(valor as f64 * 0.86) as i16;", "play forte"),
    ('j_nota', "let nota = (v - forca as i32 / 3 - ataque) as i16;", "let nota = (v - forca as i32 / 2 - ataque) as i16;"),
    ('j_baixa', "crate::comparacao::ramo(\"play nota muito baixa\");\n        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2) as u8);", "crate::comparacao::ramo(\"play nota muito baixa\");\n        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(3) as u8);", "play nota muito baixa"),
    ('j_falta_forte', "let falta_forte = forca as i32 > sortear(0x96) + 0xc8;", "let falta_forte = forca as i32 > sortear(0x97) + 0xc8;"),
    ('j_registra', "registrar(m, lance as u16, j as u32, 0, 0, t, *b(j, 0x102), *b(j, 0x103), cartao, grav);", "registrar(m, lance as u16, j as u32, 0, 0, t, *b(j, 0x103), *b(j, 0x102), cartao, grav);", "play falta registrada"),
    ('j_penalti', "*w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(500);\n            } else {", "*w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(499);\n            } else {", "play penalty"),
    ('j_venceu', "*w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1e);", "*w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1f);", "play venceu"),
    ('j_estilo', "narrar7(chamar, 0x1f65, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);", "narrar7(chamar, 0x1f65, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo.wrapping_add(1));", "play venceu"),
    ('j_meia_lua', "espirra(chamar, sortear(3), 0);", "espirra(chamar, sortear(3), j);", "play meia-lua"),
    ('j_perdeu', "*w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x14);", "*w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x15);", "play perdeu"),
]
