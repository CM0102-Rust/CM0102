# Mutacoes da leva75 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva75.py
MUTACOES = [
    ('k_005', "(level as f64 + level as f64)) * 0.05 + 10.0;", "(level as f64)) * 0.05 + 10.0;"),
    ('k_01', "    let y = (x * x * 0.1 + x) * read_f64(rdata::F64_0_3333333333) + 0.5;", "    let y = (x * x * 0.1 + x) / 3.0 + 0.5;"),
    ('k_20', "} else if !(y < 20.0) { 20.0 }", "} else if !(y < 19.0) { 19.0 }"),
    ('k_1', "    ftol(if !(y > 1.0) { 1.0 }", "    ftol(if !(y > 1.0) { 0.0 }"),
    ('p_limite', "    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {\n        return;\n    }\n    let reg", "    if person == 0 || read_i32(person) > read_i32(G_STAFF_COUNT) {\n        return;\n    }\n    let reg"),
    ('p_3', "    if *level == 3 {\n        return;\n    }", ""),
    ('p_15', "    *level = if c < 15 && r == 0 {", "    *level = if c <= 15 && r == 0 {"),
    ('p_6', "    } else if *level == 0 && r < 6 {", "    } else if r < 6 {"),
    ('p_17', "    let c = exe::non_player_attribute_rating(obs, obs.judgement()) as i8;", "    let c = exe::non_player_attribute_rating(obs, obs.interference()) as i8;"),
    ('n_lados', "        let club = Club(read_usize(fixture + if lado == 0 { 0x20 } else { 0x1c }));\n        if exe::human_manager_is_human_club", "        let club = Club(read_usize(fixture + if lado == 0 { 0x1c } else { 0x20 }));\n        if exe::human_manager_is_human_club"),
    ('n_625', "            let e = data + lado * 0x625 + match_record::ROW_STAFF_ID + vaga * 0x49;", "            let e = data + lado * 0x624 + match_record::ROW_STAFF_ID + vaga * 0x49;"),
    ('n_reserva', "(vaga > 10 && *((e + match_record_row::MINUTE_ON) as *const u8) == 0xff)", "(vaga >= 10 && *((e + match_record_row::MINUTE_ON) as *const u8) == 0xff)"),
    ('n_mais4', "            if i == -1 || read_i32(e + 4) == -1 ||", "            if i == -1 ||"),
    ('n_limite', "            if p == 0 || read_i32(p) < n() {", "            if p == 0 || read_i32(p) <= n() {"),
    ('n_quadro', "        let obj = read_usize(read_usize(G_FOG_OF_WAR).wrapping_add(k.wrapping_mul(4)).wrapping_sub(4));", "        let obj = read_usize(read_usize(G_FOG_OF_WAR).wrapping_add(k.wrapping_mul(4)));"),
]
