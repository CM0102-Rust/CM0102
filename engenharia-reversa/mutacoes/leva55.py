# Mutacoes da leva55 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva55.py
MUTACOES = [
    ('j_nulo', "    let st = *((p + 0x69) as *const usize);\n    if st == 0 {\n        return;", "    let st = *((p + 0x69) as *const usize);\n    if st == 0 && p == 1 {\n        return;"),
    ('j_22', "    let n_home = ceiling(casa, 22).wrapping_sub(level as i16);", "    let n_home = ceiling(casa, 21).wrapping_sub(level as i16);"),
    ('j_5d', "(comp != 0 && *((comp + competition::CLUB_COMP_NATION) as *const usize) == club_p)", "(comp != 0 && *((comp + competition::CLUB_COMP_CONTINENT) as *const usize) == club_p)"),
    ('j_53', "|| *((meu + club::NATION) as *const usize) == club_p {", "|| *((other + club::NATION) as *const usize) == club_p {"),
    ('j_4', "        if (x as i16) < 4 { 4 } else { x as i16 }", "        if (x as i16) < 5 { 5 } else { x as i16 }"),
    ('j_fmundo', "    } else if byte(partida, 0x3a) == 8 {\n        f_mundo = 1;", "    } else if byte(partida, 0x3a) == 8 {\n        f_mundo = 2;"),
    ('j_1_3', "    let dif = (value as f64 - global_stats_ratio(alt != 0)) * read_f32_at(0x96d490) as f64;", "    let dif = (value as f64 - global_stats_ratio(alt != 0)) * read_f64_at(0x968968);"),
    ('j_bonus', "        novo_mundo = novo_mundo.max(t);", ""),
    ('j_375', "band(atual, read_f32_at(0x96d210) as f64, read_f32_at(0x96d208) as f64,", "band(atual, read_f32_at(0x96d210) as f64, read_f32_at(0x96d20c) as f64,"),
    ('j_100', "band(mundo, read_f32_at(0x96d1a8) as f64, (nv * 100) as f64,", "band(mundo, read_f32_at(0x96d1a8) as f64, (nv * 101) as f64,"),
    ('j_3000', "cap(c / f_mundo, 3000, mundo)", "cap(c / f_mundo, 5000, mundo)"),
    ('j_fatual', "cap(c / f_atual, 5000, atual)", "cap(c, 5000, atual)"),
    ('j_k', "    let k = |r: i32| (read_f64_at(0x968950) - r as f64 * read_f64_at(0x96a5c0)) as f32;", "    let k = |r: i32| (read_f64_at(0x968950) - r as f64 * read_f64_at(0x96a5c0) * 2.0) as f32;"),
]
