# Mutacoes da leva67 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva67.py
MUTACOES = [
    ('o_ativo', "            if byte(p, 0x19) >= 0 {\n                *lista.offset(*n as isize) = p;", "            if byte(p, 0x19) > 0 {\n                *lista.offset(*n as isize) = p;"),
    ('o_ordem', "        for lado in [(primeiro as isize) as usize, (primeiro == 0) as usize] {", "        for lado in [(primeiro == 0) as usize, (primeiro as isize) as usize] {"),
    ('o_motor', "    *(G_ACTION_ORDER_MATCH_ENGINE as *mut usize) = m;\n", ""),
    ('o_qsort', "    qsort(m + 0x8f7e, *n as i32, 4, 0x6bd840);", "    qsort(m + 0x8f7e, (*n as i32) - 1, 4, 0x6bd840);"),
]
