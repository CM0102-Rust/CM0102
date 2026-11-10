# Mutacoes da leva113 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva113.py --jogo 2001-08-20
MUTACOES = [
    ('b_maior', "        if melhor == 0 || v as i16 > nota {", "        if melhor == 0 || v as i16 >= nota {"),
    ('b_desconto', "            v = ftol((read_f64(rdata::F64_1_1) - m as f64 * read_f64(rdata::F64_0_01)) * v as i16 as f64);", "            v = ftol((read_f64(rdata::F64_1_1) - m as f64 * read_f64(rdata::F64_0_1)) * v as i16 as f64);"),
    ('b_mod', "((exe::non_player_attribute_rating(x, (x.judgement() as u8 as u32) as i8) as i8) as i32) < *(p.addr() as *const i32) % 20 {", "((exe::non_player_attribute_rating(x, (x.judgement() as u8 as u32) as i8) as i8) as i32) < *(p.addr() as *const i32) % 21 {"),
]
