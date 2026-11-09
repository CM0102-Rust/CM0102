# Mutacoes da leva61 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva61.py --jogo 2001-08-15
MUTACOES = [
    ('q_reput', "    (*((sheet + off) as *const i16) as i32) / 100 - 50", "    (*((sheet + off) as *const i16) as i32) / 100 - 49"),
    ('q_cerco', "unsafe fn surround_count(m: usize, s: usize, x: i8, y: i8) -> i8 {\n    let mut n = 0i8;", "unsafe fn surround_count(m: usize, s: usize, x: i8, y: i8) -> i8 {\n    let mut n = 1i8;"),
    ('q_v16', "                a = a.wrapping_sub(100 * v16 as i32);", "                a = a.wrapping_sub(90 * v16 as i32);"),
    ('q_attr', "                let mut e = (*((q + match_player::SHOTS_ON_TARGET) as *const u8) as i32 + *((q + match_player::SHOTS) as *const u8) as i32 + 0xa) * 10;", "                let mut e = (*((q + match_player::SHOTS_ON_TARGET) as *const u8) as i32 + *((q + match_player::SHOTS) as *const u8) as i32 + 0xb) * 10;"),
    ('q_0x35', "                e = e.wrapping_sub(ftol(read_f64(rdata::F64_180) - *((q + match_player::MATCH_RATING) as *const i16) as f64 * read_f64(rdata::F64_0_03)));", "                e = e.wrapping_sub(ftol(read_f64(rdata::F64_180) - *((q + 0x37) as *const i16) as f64 * read_f64(rdata::F64_0_03)));"),
    ('q_7d', "                        rating = rating.wrapping_add(0x7d);", "                        rating = rating.wrapping_add(0x7c);"),
    ('q_empate', "        if best == 0 || rating as i16 > rating_best {", "        if best == 0 || rating as i16 >= rating_best {"),
    ('q_linhas', "    while r <= y.wrapping_add(1) {", "    while r <= y {"),
    ('q_bola', "                b = b.wrapping_add(e);\n                store(c,", "                b = b.wrapping_add(e) + 1;\n                store(c,"),
    ('q_ad', "                if (random(30) as f64) < read_f32(q + match_player::JUMPING) as f64 && (random(30) as f64) < read_f32(j + match_player::JUMPING) as f64 {", "                if (random(30) as f64) < read_f32(q + match_player::JUMPING) as f64 {"),
    ('q_f18', "                if (f18 as f64) < read_f64(rdata::F64_5) && read_i8(c + match_player::MOMENT_FLAIR) != 0 {", "                if (f1c as f64) < read_f64(rdata::F64_5) && read_i8(c + match_player::MOMENT_FLAIR) != 0 {"),
]
