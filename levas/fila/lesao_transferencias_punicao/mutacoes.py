# Mutacoes da leva169 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva169.py --carreira 2002-06-01
MUTACOES = [
    ('r_marca', "    core::ptr::write_unaligned((r + 0xa) as *mut i16, marca);", "    core::ptr::write_unaligned((r + 0xa) as *mut i16, marca.wrapping_add(1));"),
    ('r_float', "        let x = (v as f64 * read_f64_at(0x96bac0) - read_f64_at(0x968e18)) * ritmo as f64;", "        let x = (v as f64 * read_f64_at(0x96bac0) - 2.0) * ritmo as f64;"),
    ('r_limite', "    if (v as i32) < 0x4b0 - soma as i32 {", "    if (v as i32) < 0x4b0 - soma as i32 - 1 {"),
    ('r_mais1', "        let s = sorteio(ritmo as i32).wrapping_add(1) as i16;", "        let s = sorteio(ritmo as i32).wrapping_add(2) as i16;"),
    ('t_item', "        if *p == *(e as *const i32) {", "        if *p == *((e + 4) as *const i32) {"),
    ('d_tipo', "    if *(e as *const i8) == tipo {", "    if *(e as *const i8) != tipo {"),
]
