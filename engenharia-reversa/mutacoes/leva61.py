# Mutacoes da leva61 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva61.py --jogo 2001-08-15
MUTACOES = [
    ('q_reput', "    (*((ficha + off) as *const i16) as i32) / 100 - 50", "    (*((ficha + off) as *const i16) as i32) / 100 - 49"),
    ('q_cerco', "unsafe fn surround_count(m: usize, s: usize, x: i8, y: i8) -> i8 {\n    let mut n = 0i8;", "unsafe fn surround_count(m: usize, s: usize, x: i8, y: i8) -> i8 {\n    let mut n = 1i8;"),
    ('q_v16', "                a = a.wrapping_sub(100 * v16 as i32);", "                a = a.wrapping_sub(90 * v16 as i32);"),
    ('q_attr', "                let mut e = (*((q + 0xb) as *const u8) as i32 + *((q + 0xa) as *const u8) as i32 + 0xa) * 10;", "                let mut e = (*((q + 0xb) as *const u8) as i32 + *((q + 0xa) as *const u8) as i32 + 0xb) * 10;"),
    ('q_0x35', "                e = e.wrapping_sub(match_pl_ftol(read_f64_at(0x96d2a8) - *((q + 0x35) as *const i16) as f64 * read_f64_at(0x96d2c8)));", "                e = e.wrapping_sub(match_pl_ftol(read_f64_at(0x96d2a8) - *((q + 0x37) as *const i16) as f64 * read_f64_at(0x96d2c8)));"),
    ('q_7d', "                        nota = nota.wrapping_add(0x7d);", "                        nota = nota.wrapping_add(0x7c);"),
    ('q_empate', "        if melhor == 0 || nota as i16 > nota_melhor {", "        if melhor == 0 || nota as i16 >= nota_melhor {"),
    ('q_linhas', "    while r <= y.wrapping_add(1) {", "    while r <= y {"),
    ('q_bola', "                b = b.wrapping_add(e);\n                guardar(c,", "                b = b.wrapping_add(e) + 1;\n                guardar(c,"),
    ('q_ad', "                if (sortear(30) as f64) < read_f32_at(q + 0xad) as f64 && (sortear(30) as f64) < read_f32_at(j + 0xad) as f64 {", "                if (sortear(30) as f64) < read_f32_at(q + 0xad) as f64 {"),
    ('q_f18', "                if (f18 as f64) < read_f64_at(0x9688a0) && byte(c, 0x109) != 0 {", "                if (f1c as f64) < read_f64_at(0x9688a0) && byte(c, 0x109) != 0 {"),
]
