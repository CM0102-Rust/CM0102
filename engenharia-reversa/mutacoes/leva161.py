# Mutacoes da leva161 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva161.py --carreira 2002-06-01
MUTACOES = [
    ('s_nota', "    let n = |o: usize| *((sheet + o) as *const i8) >= 15;", "    let n = |o: usize| *((sheet + o) as *const i8) >= 14;"),
    ('s_vaga8', "        8 => (0x13, 0x18),", "        8 => (0x13, 0x19),"),
    ('s_id', "(*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10)", "(*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x11)"),
    ('p_meia', "        0x108 | 0x208 | 0x408 => 6,", "        0x108 | 0x208 | 0x408 => 7,"),
    ('p_ponta', "        0x810 => 0xa,", "        0x810 => 0xb,"),
    ('c_idade', "    } else if b44 > 0x32 {\n        rating = rating.wrapping_add(5);", "    } else if b44 > 0x32 {\n        rating = rating.wrapping_add(4);"),
    ('c_rep', "        if rep > 0x1c52 {", "        if rep > 0x1c53 {"),
    ('c_k', "        rating = rating.wrapping_sub(3);\n        if b5b < 6 {", "        rating = rating.wrapping_sub(2);\n        if b5b < 6 {"),
    ('c_25', "        || (funcoes & 0x20020 != 0 && rating < 25)", "        || (funcoes & 0x20020 != 0 && rating < 24)"),
]
