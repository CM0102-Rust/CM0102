# Mutacoes da leva168 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva168.py --carreira 2002-06-01
MUTACOES = [
    ('f_segunda', "        core::ptr::read_unaligned((t + 0x5f1) as *const usize).wrapping_add((f - primeiras) as usize * 0x18e3)", "        core::ptr::read_unaligned((t + 0x5f1) as *const usize).wrapping_add(f as usize * 0x18e3)"),
    ('f_limite', "    let limite = if humano != 0 { primeiras }", "    let limite = if humano == 0 { primeiras }"),
    ('p_c1', "    if *((c1 + 0x79) as *const i8) == cod {\n        return 300;", "    if *((c1 + 0x79) as *const i8) == cod {\n        return 301;"),
    ('p_c2', "    if *((c2 + 0x79) as *const i8) == cod { 100 } else { 0 }", "    if *((c2 + 0x79) as *const i8) == cod { 101 } else { 0 }"),
    ('p_esp', "            (cod == 0x16 || cod == 0x20) as u8", "            (cod == 0x16) as u8"),
    ('n_campo', "    core::ptr::write_unaligned((t + 5 + 4 * k as usize) as *mut u32, valor);", "    core::ptr::write_unaligned((t + 9 + 4 * k as usize) as *mut u32, valor);"),
]
