# Mutacoes da leva113 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva113.py --jogo 2001-08-20
MUTACOES = [
    ('b_maior', "        if melhor == 0 || v as i16 > nota {", "        if melhor == 0 || v as i16 >= nota {"),
    ('b_desconto', "            v = match_pl_ftol((read_f64_at(0x9689c0) - m as f64 * read_f64_at(0x967898)) * v as i16 as f64);", "            v = match_pl_ftol((read_f64_at(0x9689c0) - m as f64 * read_f64_at(0x967880)) * v as i16 as f64);"),
    ('b_mod', "(value(x, byte(x, 0x17) as u8 as u32) as i32) < *(p as *const i32) % 20 {", "(value(x, byte(x, 0x17) as u8 as u32) as i32) < *(p as *const i32) % 21 {"),
]
