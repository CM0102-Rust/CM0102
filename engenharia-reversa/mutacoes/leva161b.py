# Mutacoes refeitas da leva161 (as tres originais so mudavam um limit exato que a carreira nao reaches).
# Uso: python mutacoes.py mutacoes\leva161b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('s_id_b', "(*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10)", "(*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x4000)"),
    ('c_idade_b', "    } else if b44 > 0x32 {\n        rating = rating.wrapping_add(5);", "    } else if b44 > 0x32 {\n        rating = rating.wrapping_add(15);"),
    ('c_rep_b', "            rating = rating.wrapping_sub(5);\n        } else if rep > 0x1676 {", "            rating = rating.wrapping_add(20);\n        } else if rep > 0x1676 {"),
]
