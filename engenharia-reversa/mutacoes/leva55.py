# Mutacoes da leva55 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva55.py
MUTACOES = [
    ('j_nulo', "    let st = p.non_player();\n    if st.is_null() {\n        return;", "    let st = p.non_player();\n    if st.is_null() && p.addr() == 1 {\n        return;"),
    ('j_22', "    let n_home = ceiling(casa, 22).wrapping_sub(level as i16);", "    let n_home = ceiling(casa, 21).wrapping_sub(level as i16);"),
    ('j_5d', "(!comp.is_null() && comp.nation() == club_p)", "(!comp.is_null() && comp.continent() == club_p.addr())"),
    ('j_53', "|| my.nation() == club_p {", "|| other.nation() == club_p {"),
    ('j_4', "        if (x as i16) < 4 { 4 } else { x as i16 }", "        if (x as i16) < 5 { 5 } else { x as i16 }"),
    ('j_fmundo', "    } else if (fixture.category() as i8) == 8 {\n        f_mundo = 1;", "    } else if (fixture.category() as i8) == 8 {\n        f_mundo = 2;"),
    ('j_1_3', "    let dif = (value as f64 - global_stats_ratio(alt != 0)) * read_f32(rdata::F32_1_299999952) as f64;", "    let dif = (value as f64 - global_stats_ratio(alt != 0)) * read_f64(rdata::F64_0_5);"),
    ('j_bonus', "        new_world = new_world.max(t);", ""),
    ('j_375', "band(atual, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_375) as f64,", "band(atual, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_300) as f64,"),
    ('j_100', "band(mundo, read_f32(rdata::F32_500) as f64, (nv * 100) as f64,", "band(mundo, read_f32(rdata::F32_500) as f64, (nv * 101) as f64,"),
    ('j_3000', "cap(c / f_mundo, 3000, mundo)", "cap(c / f_mundo, 5000, mundo)"),
    ('j_fatual', "cap(c / f_atual, 5000, atual)", "cap(c, 5000, atual)"),
    ('j_k', "    let k = |r: i32| (read_f64(rdata::F64_2) - r as f64 * read_f64(rdata::F64_0_0001)) as f32;", "    let k = |r: i32| (read_f64(rdata::F64_2) - r as f64 * read_f64(rdata::F64_0_0001) * 2.0) as f32;"),
]
