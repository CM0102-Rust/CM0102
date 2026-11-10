# Mutacoes da leva67 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva67.py
MUTACOES = [
    ('o_ativo', "            if p.position() >= 0 {\n                *list.offset(*n as isize) = p.addr();", "            if p.position() > 0 {\n                *list.offset(*n as isize) = p.addr();"),
    ('o_ordem', "        for lado in [(first as isize) as usize, (first == 0) as usize] {", "        for lado in [(first == 0) as usize, (first as isize) as usize] {"),
    ('o_motor', "    write_usize(G_ACTION_ORDER_MATCH_ENGINE, m.addr());\n", ""),
    ('o_qsort', "    exe::crt_qsort(m.addr() + engine::ACTIVE_LIST, (*n as i32) as u32, (4) as u32, 0x6bd840);", "    exe::crt_qsort(m.addr() + engine::ACTIVE_LIST, ((*n as i32) - 1) as u32, (4) as u32, 0x6bd840);"),
]
