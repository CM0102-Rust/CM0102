# Mutacoes da leva162 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva162.py --carreira 2002-06-01
MUTACOES = [
    ('g_vaga', "    core::ptr::write_unaligned((linha_da_tabela(t, l) + 2 + 4 * slot as usize) as *mut u32, *person);", "    core::ptr::write_unaligned((linha_da_tabela(t, l) + 6 + 4 * slot as usize) as *mut u32, *person);"),
    ('g_pendente', "            if *active != 0 && *(e as *const usize) == clube && *((e + 4) as *const u32) == human {", "            if *active != 0 && *(e as *const usize) == clube {"),
    ('p_ficha', "    if core::ptr::read_unaligned((p + 0x61) as *const usize) == 0 {\n        core::ptr::write_unaligned(home, -1);", "    if core::ptr::read_unaligned((p + 0x65) as *const usize) == 0 {\n        core::ptr::write_unaligned(home, -1);"),
    ('p_nacao', "        if nacao == 0 || core::ptr::read_unaligned((nacao + nation::THREE_LETTER_NAME)", "        if core::ptr::read_unaligned((nacao + nation::THREE_LETTER_NAME)"),
    ('f_grava', "    core::ptr::write_unaligned(linha_da_tabela(t, l) as *mut i16, formacao);", "    core::ptr::write_unaligned(linha_da_tabela(t, l) as *mut i16, formacao.wrapping_add(1));"),
]
