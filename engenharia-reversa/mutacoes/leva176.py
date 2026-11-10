# Mutacoes da leva169 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva169.py --carreira 2002-06-01
MUTACOES = [
    ('r_marca', "    write_i16(r + physical_record::CONDITION, mark);", "    write_i16(r + physical_record::CONDITION, mark.wrapping_add(1));"),
    ('r_float', "        let x = (v as f64 * read_f64(rdata::F64_NEG_0_0035) - read_f64(rdata::F64_2_5)) * ritmo as f64;", "        let x = (v as f64 * read_f64(rdata::F64_NEG_0_0035) - 2.0) * ritmo as f64;"),
    ('r_limite', "    if (v as i32) < 0x4b0 - sum as i32 {", "    if (v as i32) < 0x4b0 - sum as i32 - 1 {"),
    ('r_mais1', "        let s = exe::utils_random(ritmo as i32).wrapping_add(1) as i16;", "        let s = exe::utils_random(ritmo as i32).wrapping_add(2) as i16;"),
    ('t_item', "        if *p == read_i32(e) {", "        if *p == read_i32(e + 4) {"),
    ('d_tipo', "    if read_i8(e) == kind {", "    if read_i8(e) != kind {"),
]
