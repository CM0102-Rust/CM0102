# Mutacoes da leva128 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva128.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('r_goleiro', "sortear(match_pl_ftol(2000.0 - read_f32_at(j + 0xa5) as f64 * 40.0))", "sortear(match_pl_ftol(2000.0 - read_f32_at(j + 0xa5) as f64 * 41.0))", "respond goleiro na area"),
    ('r_tipo', "            3 => sortear(1000),\n            4 => sortear(0x708),", "            3 => sortear(1001),\n            4 => sortear(0x708),"),
    ('r_altura', "ataque = match_pl_ftol(((byte(m, 0x8eaa) as i32 + 1) * ataque as i16 as i32) as f64 / st);", "ataque = match_pl_ftol(((byte(m, 0x8eaa) as i32 + 2) * ataque as i16 as i32) as f64 / st);"),
    ('r_proprio', "        if sortear(ataque as i16 as i32 + 100) != 0 {\n            return 0;\n        }\n        crate::comparacao::ramo(\"respond bola", "        if sortear(ataque as i16 as i32 + 101) != 0 {\n            return 0;\n        }\n        crate::comparacao::ramo(\"respond bola", "respond bola do proprio lado"),
    ('r_cabeca', "ficha5c * 200) != 0 { escolher(kind)", "ficha5c * 201) != 0 { escolher(kind)"),
    ('r_nota', "    if rating < 0x96 {\n        return 0;", "    if rating < 0x95 {\n        return 0;"),
    ('r_registra', "register(m, code as u16, j as u32, 0, 0, t, *b(j, 0x102), *b(j, 0x103), cartao, grav);", "register(m, code as u16, j as u32, 0, 0, t, *b(j, 0x103), *b(j, 0x102), cartao, grav);", "respond falta registrada"),
    ('r_penalti', "                *((m + 0x478a) as *mut usize) = 0;\n                *w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(500);\n                if sortear", "                *((m + 0x478a) as *mut usize) = 0;\n                *w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(499);\n                if sortear", "respond penalty"),
    ('r_limite', "let mut lim = match_pl_ftol((1500.0 - p * p * 100.0)", "let mut lim = match_pl_ftol((1501.0 - p * p * 100.0)"),
    ('r_domina', "            *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1c);", "            *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1d);", "respond dominou"),
    ('r_goleiro_perde', "        if sortear(10) != 0 {\n            (1, 0)", "        if sortear(9) != 0 {\n            (1, 0)", "respond goleiro perdeu"),
    ('r_perde', "narrate(call, if kind <= 2 { 0x213e } else { 0x1f71 }", "narrate(call, if kind <= 1 { 0x213e } else { 0x1f71 }", "respond perdeu"),
]
