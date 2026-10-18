# Mutacoes da leva165 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva165.py --carreira 2002-06-01
MUTACOES = [
    ('v_desce', "        core::ptr::write_unaligned((linha + 0x56) as *mut u32, segunda);", "        core::ptr::write_unaligned((linha + 0x56) as *mut u32, u32::MAX);"),
    ('v_grava', "    let casa = (linha + 0x56 + 4 * qual as usize) as *mut u32;", "    let casa = (linha + 0x5a - 4 * qual as usize) as *mut u32;"),
    ('i_olimpica', "                *minima = 0x17;\n                *maxima = 0x17;", "                *minima = 0x17;\n                *maxima = 0x16;"),
    ('i_tipo', "    if tipo != 0 && tipo != 2 {", "    if tipo != 0 && tipo != 1 {"),
    ('h_campo', "    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a65) as *mut u32, valor);", "    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a69) as *mut u32, valor);"),
]
