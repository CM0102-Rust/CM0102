# Mutacoes da leva68 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva68.py
MUTACOES = [
    ('v_vazia', "        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == *((linha + 0x52) as *const i32) {", "        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) != *((linha + 0x52) as *const i32) {"),
    ('v_menos', "    -((minus_one != 0) as i8)", "    -((minus_one == 0) as i8)"),
    ('v_20', "    for vaga in 0..20usize {", "    for vaga in 0..19usize {"),
    ('m_52', "    *((linha + 0x52) as *mut i32) = i;\n    1", "    *((linha + 0x52) as *mut i32) = i + 1;\n    1"),
    ('m_vazia', "    if i == -1 {\n        return 0;\n    }\n    *((linha + 0x52)", "    if i == -2 {\n        return 0;\n    }\n    *((linha + 0x52)"),
    ('p_56', "    let i = *((linha + 0x56 + 4 * which as usize) as *const i32);", "    let i = *((linha + 0x56 + 4 * (1 - which) as usize) as *const i32);"),
    ('p_neg', "    if i < 0 {\n        return 0;\n    }\n    (*(0xae23bc", "    if i < -1 {\n        return 0;\n    }\n    (*(0xae23bc"),
]
