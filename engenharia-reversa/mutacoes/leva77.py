# Mutacoes da leva77 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva77.py
MUTACOES = [
    ('l_6', "read_usize(in_the + 6));", "read_usize(in_the + 4));"),
    ('l_valor', "    read_u32(in_the)\n}", "    read_u32(in_the + 2)\n}"),
    ('w_limite', "pub(crate) unsafe extern \"thiscall\" fn word_0c(this: usize, id: i32) -> i32 {\n    if id >= read_i32(G_STAFF_COUNT) {", "pub(crate) unsafe extern \"thiscall\" fn word_0c(this: usize, id: i32) -> i32 {\n    if id > read_i32(G_STAFF_COUNT) {"),
    ('w_31', "    read_i16(read_usize(this).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc)) as i32", "    read_i16(read_usize(this).wrapping_add((id as usize).wrapping_mul(32)).wrapping_add(0xc)) as i32"),
    ('w_sinal', "    read_i16(read_usize(this).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc)) as i32", "    read_u16(read_usize(this).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc)) as i32"),
    ('a_1000', "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })", "    v.wrapping_sub(if w <= 1000 { w as i32 % 10 } else { 0 })"),
    ('a_sinal', "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })", "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 }) & 0xffff | v & !0xffff"),
    ('a_rem', "    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })", "    v.wrapping_sub(if w < 1000 { w as i32 - w as i32 / 10 * 10 + (w < 0) as i32 * 10 } else { 0 })"),
    ('g_seis', "[KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any", "[KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA].iter().any"),
    ('m_outro', "    let other = (side_of_player(MatchPlayer(target)) == 0) as i8;\n    (0..20)", "    let other = side_of_player(MatchPlayer(target));\n    (0..20)"),
    ('m_ativo', "        j.position() >= 0 && exe::match_player_find_man_marking_target(j) == target\n    }).count()", "        j.position() > 0 && exe::match_player_find_man_marking_target(j) == target\n    }).count()"),
    ('q_outro', "    let other = (side_of_player(MatchPlayer(target)) == 0) as i8;\n    for k in 0..20", "    let other = side_of_player(MatchPlayer(target));\n    for k in 0..20"),
    ('q_ultimo', "            return j.addr();\n        }\n    }\n    0\n}", "            return j.addr() + 1;\n        }\n    }\n    0\n}"),
]
