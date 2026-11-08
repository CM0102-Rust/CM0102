# Mutacoes da leva165 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva165.py --carreira 2002-06-01
MUTACOES = [
    ('v_desce', "        core::ptr::write_unaligned((row + tactics_row::PLAYMAKER) as *mut u32, second);", "        core::ptr::write_unaligned((row + tactics_row::PLAYMAKER) as *mut u32, u32::MAX);"),
    ('v_grava', "    let home = (row + tactics_row::PLAYMAKER + 4 * which as usize) as *mut u32;", "    let home = (row + tactics_row::SECOND_PLAYMAKER - 4 * which as usize) as *mut u32;"),
    ('i_olimpica', "                *minima = 0x17;\n                *maxima = 0x17;", "                *minima = 0x17;\n                *maxima = 0x16;"),
    ('i_tipo', "    if kind != 0 && kind != 2 {", "    if kind != 0 && kind != 1 {"),
    ('h_campo', "    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a65) as *mut u32, value);", "    core::ptr::write_unaligned((*(t as *const usize) + i as usize * 0xa6ef + 0x9a69) as *mut u32, value);"),
]
