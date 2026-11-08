# Mutacoes da leva160 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva160.py --carreira 2002-06-01
MUTACOES = [
    ('e_media', "            if games != 0 { w(o) / games as f64 } else { 0.0 }", "            if games != 0 { w(o) / (games as f64 + 1.0) } else { 0.0 }"),
    ('e_cem', "                w(n) / w(d) * cem", "                w(n) / w(d) * cem * 1.0000001"),
    ('e_nota', "            let rating = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + games as i32) as f64;", "            let rating = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + games as i32 + 1) as f64;"),
    ('e_vinte', "            if games < 20 {\n                ((20 - games as i32)", "            if games < 19 {\n                ((20 - games as i32)"),
    ('n_zero', "    let rating = |n: u8| if n == 0 { 0x1b } else { n as u16 * 4 };", "    let rating = |n: u8| if n == 0 { 0x1a } else { n as u16 * 4 };"),
    ('n_baixo', "        .wrapping_add(rating(e & 0xf));", "        .wrapping_add(rating(e >> 4));"),
    ('z_volante', "        if a & 0x880 != 0 && a & 8 != 0 {\n            return 0;\n        }\n        if b & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 8 != 0 {", "        if a & 0x880 != 0 && a & 0x10 != 0 {\n            return 0;\n        }\n        if b & 0x880 != 0 && exe::formation_get_slot_position(this, i, 1) & 8 != 0 {"),
]
