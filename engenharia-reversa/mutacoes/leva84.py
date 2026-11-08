# Mutacoes da leva84 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva84.py
MUTACOES = [
    ('p_negativo', "    if i < 0 {\n        return 0;\n    }\n    (*(0xae23d0", "    if i < -1 {\n        return 0;\n    }\n    (*(0xae23d0"),
    ('p_107', "    (*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(107))\n}", "    (*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(108))\n}"),
    ('b_classe2', "    } else if flag == 0 {\n        2\n    } else {\n        1\n    };", "    } else if flag == 0 {\n        1\n    } else {\n        1\n    };"),
    ('b_flag', "        if flag == 0 { 0 } else { 1 }", "        if flag == 0 { 0 } else { 2 }"),
    ('b_42', "    national_block_byte(this, clube, flag, 0x42)", "    national_block_byte(this, clube, flag, 0x41)"),
    ('e_a8', "    (id == *(G_KEY_CLUB_MLS_ALL_STARS_EAST as *const u32) || id == *(G_KEY_CLUB_MLS_ALL_STARS_WEST as *const u32)) as u32", "    (id == *(G_KEY_CLUB_MLS_ALL_STARS_EAST as *const u32)) as u32"),
    ('n_principal', "    let s = principal(*((pessoa + staff::NATION) as *const usize));", "    let s = *((pessoa + staff::NATION) as *const usize);"),
    ('d_marca', "    *((this + 8) as *mut u8) = *((origem + 8) as *const u8);", "    *((this + 8) as *mut u8) = *((origem + 7) as *const u8);"),
]
