# Mutacoes da leva160 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva160.py --carreira 2002-06-01
MUTACOES = [
    ('e_media', "            if jogos != 0 { w(o) / jogos as f64 } else { 0.0 }", "            if jogos != 0 { w(o) / (jogos as f64 + 1.0) } else { 0.0 }"),
    ('e_cem', "                w(n) / w(d) * cem", "                w(n) / w(d) * cem * 1.0000001"),
    ('e_nota', "            let nota = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + jogos as i32) as f64;", "            let nota = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + jogos as i32 + 1) as f64;"),
    ('e_vinte', "            if jogos < 20 {\n                ((20 - jogos as i32)", "            if jogos < 19 {\n                ((20 - jogos as i32)"),
    ('n_zero', "    let nota = |n: u8| if n == 0 { 0x1b } else { n as u16 * 4 };", "    let nota = |n: u8| if n == 0 { 0x1a } else { n as u16 * 4 };"),
    ('n_baixo', "        .wrapping_add(nota(e & 0xf));", "        .wrapping_add(nota(e >> 4));"),
    ('z_volante', "        if a & 0x880 != 0 && a & 8 != 0 {\n            return 0;\n        }\n        if b & 0x880 != 0 && casa(this, i, 1) & 8 != 0 {", "        if a & 0x880 != 0 && a & 0x10 != 0 {\n            return 0;\n        }\n        if b & 0x880 != 0 && casa(this, i, 1) & 8 != 0 {"),
]
