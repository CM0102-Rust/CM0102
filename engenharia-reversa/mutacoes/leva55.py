# Mutacoes da leva55 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva55.py
MUTACOES = [
    ('j_nulo', "    let st = *((p + 0x69) as *const usize);\n    if st == 0 {\n        return;", "    let st = *((p + 0x69) as *const usize);\n    if st == 0 && p == 1 {\n        return;"),
    ('j_22', "    let n_casa = teto(casa, 22).wrapping_sub(nivel as i16);", "    let n_casa = teto(casa, 21).wrapping_sub(nivel as i16);"),
    ('j_5d', "(comp != 0 && *((comp + competition::CLUB_COMP_NATION) as *const usize) == clube_p)", "(comp != 0 && *((comp + competition::CLUB_COMP_CONTINENT) as *const usize) == clube_p)"),
    ('j_53', "|| *((meu + 0x53) as *const usize) == clube_p {", "|| *((outro + 0x53) as *const usize) == clube_p {"),
    ('j_4', "        if (x as i16) < 4 { 4 } else { x as i16 }", "        if (x as i16) < 5 { 5 } else { x as i16 }"),
    ('j_fmundo', "    } else if byte(partida, 0x3a) == 8 {\n        f_mundo = 1;", "    } else if byte(partida, 0x3a) == 8 {\n        f_mundo = 2;"),
    ('j_1_3', "    let dif = (valor as f64 - razao_global(alt != 0)) * f32_em(0x96d490) as f64;", "    let dif = (valor as f64 - razao_global(alt != 0)) * f64_em(0x968968);"),
    ('j_bonus', "        novo_mundo = novo_mundo.max(t);", ""),
    ('j_375', "faixa(atual, f32_em(0x96d210) as f64, f32_em(0x96d208) as f64,", "faixa(atual, f32_em(0x96d210) as f64, f32_em(0x96d20c) as f64,"),
    ('j_100', "faixa(mundo, f32_em(0x96d1a8) as f64, (nv * 100) as f64,", "faixa(mundo, f32_em(0x96d1a8) as f64, (nv * 101) as f64,"),
    ('j_3000', "cap(c / f_mundo, 3000, mundo)", "cap(c / f_mundo, 5000, mundo)"),
    ('j_fatual', "cap(c / f_atual, 5000, atual)", "cap(c, 5000, atual)"),
    ('j_k', "    let k = |r: i32| (f64_em(0x968950) - r as f64 * f64_em(0x96a5c0)) as f32;", "    let k = |r: i32| (f64_em(0x968950) - r as f64 * f64_em(0x96a5c0) * 2.0) as f32;"),
]
