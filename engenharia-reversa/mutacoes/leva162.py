# Mutacoes da leva162 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva162.py --carreira 2002-06-01
MUTACOES = [
    ('g_vaga', "    core::ptr::write_unaligned((table_row(t, l) + 2 + 4 * slot as usize) as *mut u32, *person);", "    core::ptr::write_unaligned((table_row(t, l) + 6 + 4 * slot as usize) as *mut u32, *person);"),
    ('g_pendente', "            if *active != 0 && *(e as *const usize) == club && *((e + undo_slot::TEAM) as *const u32) == human {", "            if *active != 0 && *(e as *const usize) == club {"),
    ('p_ficha', "    if p.player() == 0 {\n        core::ptr::write_unaligned(home, -1);", "    if p.staff_preferences() == 0 {\n        core::ptr::write_unaligned(home, -1);"),
    ('p_nacao', "        if nation.is_null() || nation.three_letter_name() != ", "        if nation.three_letter_name() != "),
    ('f_grava', "    core::ptr::write_unaligned(table_row(t, l) as *mut i16, formation);", "    core::ptr::write_unaligned(table_row(t, l) as *mut i16, formation.wrapping_add(1));"),
]
