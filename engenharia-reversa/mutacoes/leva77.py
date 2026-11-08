# Mutacoes da leva77 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva77.py
MUTACOES = [
    ('l_6', "core::ptr::read_unaligned((no + 6) as *const usize));", "core::ptr::read_unaligned((no + 4) as *const usize));"),
    ('l_valor', "    core::ptr::read_unaligned(no as *const u32)\n}", "    core::ptr::read_unaligned((no + 2) as *const u32)\n}"),
    ('w_limite', "    if id >= *(G_STAFF_COUNT as *const i32) {\n        return 0;\n    }\n    core::ptr::read_unaligned(", "    if id > *(G_STAFF_COUNT as *const i32) {\n        return 0;\n    }\n    core::ptr::read_unaligned("),
    ('w_31', "(id as usize).wrapping_mul(31)).wrapping_add(0xc) as *const i16) as i32", "(id as usize).wrapping_mul(32)).wrapping_add(0xc) as *const i16) as i32"),
    ('w_sinal', "(0xc) as *const i16) as i32", "(0xc) as *const u16) as i32"),
    ('a_1000', "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })", "    v.wrapping_sub(if w <= 1000 { w as i32 % 10 } else { 0 })"),
    ('a_sinal', "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })", "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 }) & 0xffff | v & !0xffff"),
    ('a_rem', "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })", "    v.wrapping_sub(if w < 1000 { w as i32 - w as i32 / 10 * 10 + (w < 0) as i32 * 10 } else { 0 })"),
    ('g_seis', "[KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any", "[KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA].iter().any"),
    ('m_outro', "    let other = (lado(target) == 0) as i8;\n    (0..20)", "    let other = lado(target);\n    (0..20)"),
    ('m_ativo', "        *((j + match_player::POSITION) as *const i8) >= 0 && marked(j) == target\n    }).count()", "        *((j + match_player::POSITION) as *const i8) > 0 && marked(j) == target\n    }).count()"),
    ('q_outro', "    let other = (lado(target) == 0) as i8;\n    for k in 0..20", "    let other = lado(target);\n    for k in 0..20"),
    ('q_ultimo', "            return j;\n        }\n    }\n    0\n}", "            return j + 1;\n        }\n    }\n    0\n}"),
]
