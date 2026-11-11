# Mutacoes da leva156 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva156.py --carreira 2002-06-01
MUTACOES = [
    ('t_numero', "        format(dest(out), 0x98806c, if t(pos + 2) == b'1' { number.wrapping_add(1) } else { number });", "        format(dest(out), 0x98806c, number.wrapping_add(2));"),
    ('t_variante', "                    if left <= 0 {", "                    if left <= 1 {"),
    ('c_contagem', "                *((u(G_MATCH_DAY, 0) + match_day_fields::FINISHED_COUNT) as *mut i32) += 1;", "                *((u(G_MATCH_DAY, 0) + match_day_fields::FINISHED_COUNT) as *mut i32) += 2;"),
]
