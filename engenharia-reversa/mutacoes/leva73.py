# Mutacoes da leva73 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva73.py
MUTACOES = [
    ('p_nacao', "    if n != g(KEY_NATION_SPAIN) && n != g(KEY_NATION_GERMANY) && n != g(KEY_NATION_PORTUGAL) {", "    if n != g(KEY_NATION_SPAIN) && n != g(KEY_NATION_GERMANY) {"),
    ('p_ativo', "    if nacao == 0 || (ativo != 0 && byte(clube, 0x244) == 0) {", "    if nacao == 0 {"),
    ('p_aceite_a', "        (v != -1).then_some(v)", "        (v > -1).then_some(v)"),
    ('p_aceite_b', "        (v > -1).then_some(v)\n    } else {", "        (v != -1).then_some(v)\n    } else {"),
    ('p_eh_a', "            *eh_a = 1;", "            *eh_a = 2;"),
    ('p_581', "        Some(v) => (*(G_CLUB_TABLE as *const usize)).wrapping_add((v as usize).wrapping_mul(581)),", "        Some(v) => (*(G_CLUB_TABLE as *const usize)).wrapping_add((v as usize).wrapping_mul(580)),"),
    ('p_zera', "            *((clube + 0x244) as *mut u8) = 0;\n            0", "            0"),
    ('p_par', "(0x9d012c, 0x9d0130), (0x9d0120, 0x9d0134)", "(0x9d012c, 0x9d0134), (0x9d0120, 0x9d0130)"),
]
