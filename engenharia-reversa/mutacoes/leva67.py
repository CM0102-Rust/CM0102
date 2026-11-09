# Mutacoes da leva67 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva67.py
MUTACOES = [
    ('o_ativo', "            if read_i8(p + match_player::POSITION) >= 0 {\n                *list.offset(*n as isize) = p;", "            if read_i8(p + match_player::POSITION) > 0 {\n                *list.offset(*n as isize) = p;"),
    ('o_ordem', "        for lado in [(first as isize) as usize, (first == 0) as usize] {", "        for lado in [(first == 0) as usize, (first as isize) as usize] {"),
    ('o_motor', "    *(G_ACTION_ORDER_MATCH_ENGINE as *mut usize) = m;\n", ""),
    ('o_qsort', "    exe::crt_qsort(m + engine::ACTIVE_LIST, (*n as i32) as u32, (4) as u32, 0x6bd840);", "    exe::crt_qsort(m + engine::ACTIVE_LIST, ((*n as i32) - 1) as u32, (4) as u32, 0x6bd840);"),
]
