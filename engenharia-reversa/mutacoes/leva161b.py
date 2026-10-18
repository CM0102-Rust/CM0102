# Mutacoes refeitas da leva161 (as tres originais so mudavam um limite exato que a carreira nao alcanca).
# Uso: python mutacoes.py mutacoes\leva161b.py --carreira 2002-06-01 --confirmar 2001-08-20
MUTACOES = [
    ('s_id_b', "(*(0xae2364 as *const i32)).wrapping_sub(0x10)", "(*(0xae2364 as *const i32)).wrapping_sub(0x4000)"),
    ('c_idade_b', "    } else if b44 > 0x32 {\n        nota = nota.wrapping_add(5);", "    } else if b44 > 0x32 {\n        nota = nota.wrapping_add(15);"),
    ('c_rep_b', "            nota = nota.wrapping_sub(5);\n        } else if rep > 0x1676 {", "            nota = nota.wrapping_add(20);\n        } else if rep > 0x1676 {"),
]
