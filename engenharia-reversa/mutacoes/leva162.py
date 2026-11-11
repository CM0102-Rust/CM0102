# Mutacoes da leva162 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva162.py --carreira 2002-06-01
MUTACOES = [
    ('g_vaga', "    write_u32(table_row(t, l) + 2 + 4 * slot as usize, *person);", "    write_u32(table_row(t, l) + 6 + 4 * slot as usize, *person);"),
    ('g_pendente', "            if *active != 0 && *(e as *const usize) == club && *((e + undo_slot::TEAM) as *const u32) == human {", "            if *active != 0 && *(e as *const usize) == club {"),
    ('p_ficha', "    if p.player().is_null() {\n        core::ptr::write_unaligned(home, -1);", "    if p.staff_preferences() == 0 {\n        core::ptr::write_unaligned(home, -1);"),
    ('p_nacao', "        if national.is_null() || national.nation().addr() != ", "        if national.nation().addr() != "),
    ('f_grava', "    write_i16(table_row(t, l), formation);", "    write_i16(table_row(t, l), formation.wrapping_add(1));"),
]
