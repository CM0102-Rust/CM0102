# Mutacoes da leva165 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva165.py --carreira 2002-06-01
MUTACOES = [
    ('v_desce', "        write_u32(row + tactics_row::PLAYMAKER, second);", "        write_u32(row + tactics_row::PLAYMAKER, u32::MAX);"),
    ('v_grava', "    let home = (row + tactics_row::PLAYMAKER + 4 * which as usize) as *mut u32;", "    let home = (row + tactics_row::SECOND_PLAYMAKER - 4 * which as usize) as *mut u32;"),
    ('i_olimpica', "                *minimum = 0x17;\n                *maxima = 0x17;", "                *minimum = 0x17;\n                *maxima = 0x16;"),
    ('i_tipo', "    if kind != 0 && kind != 2 {", "    if kind != 0 && kind != 1 {"),
    ('h_campo', "    write_u32(read_usize(t) + i as usize * 0xa6ef + human_manager_record::HOLIDAY_STAMP, value);", "    write_u32(read_usize(t) + i as usize * 0xa6ef + 0x9a69, value);"),
]
