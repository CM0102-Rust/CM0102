# Mutacoes da leva84 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva84.py
MUTACOES = [
    ('p_negativo', "    let i = read_i32(reg + 4);\n    if i < 0 {", "    let i = read_i32(reg + 4);\n    if i < -1 {"),
    ('p_107', "    read_usize(G_CLUB_COMP_TABLE).wrapping_add((i as usize).wrapping_mul(107))\n}", "    read_usize(G_CLUB_COMP_TABLE).wrapping_add((i as usize).wrapping_mul(108))\n}"),
    ('b_classe2', "    } else if flag == 0 {\n        2\n    } else {\n        1\n    };", "    } else if flag == 0 {\n        1\n    } else {\n        1\n    };"),
    ('b_flag', "        if flag == 0 { 0 } else { 1 }", "        if flag == 0 { 0 } else { 2 }"),
    ('b_42', "    national_block_byte(this, Club(club), flag, 0x42)", "    national_block_byte(this, Club(club), flag, 0x41)"),
    ('e_a8', "    (id == read_u32(G_KEY_CLUB_MLS_ALL_STARS_EAST) || id == read_u32(G_KEY_CLUB_MLS_ALL_STARS_WEST)) as u32", "    (id == read_u32(G_KEY_CLUB_MLS_ALL_STARS_EAST)) as u32"),
    ('n_principal', "    let s = exe::database_get_national_team(person.nation());", "    let s = person.nation();"),
    ('d_marca', "    write_u8(this + 8, read_u8(source + 8));", "    write_u8(this + 8, read_u8(source + 7));"),
]
