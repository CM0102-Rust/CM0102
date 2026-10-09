# Mutacoes da leva78 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva78.py
MUTACOES = [
    ('c_tres', "    if id == g(0x9cf6b0) || id == g(0x9cf5d8) || id == g(0x9cf548) {", "    if id == g(0x9cf6b0) || id == g(0x9cf5d8) {"),
    ('c_107', "(id as usize).wrapping_mul(107)).wrapping_add(0x5d)", "(id as usize).wrapping_mul(108)).wrapping_add(0x5d)"),
    ('c_fase', "    if c == 0 || byte(c, 0x43) != 2 || *((c + 0x3e) as *const u16) == 0 {", "    if c == 0 || *((c + 0x3e) as *const u16) == 0 {"),
    ('c_rodadas', "    if c == 0 || byte(c, 0x43) != 2 || *((c + 0x3e) as *const u16) == 0 {", "    if c == 0 || byte(c, 0x43) != 2 {"),
    ('c_60', "*((primeira + 0x60) as *const i32) == alvo) as u32", "*((primeira + 0x5c) as *const i32) == alvo) as u32"),
    ('o_5f', "    v(*b) - v(*a)\n}", "    v(*a) - v(*b)\n}"),
    ('o_sinal5f', "*((p + 0x5f) as *const i8) as i32 };", "*((p + 0x5f) as *const u8) as i32 };"),
    ('o_80', "    *((*b + 0x80) as *const i16) as i32 - *((*a + 0x80) as *const i16) as i32\n}", "    *((*b + 0x80) as *const u16) as i32 - *((*a + 0x80) as *const i16) as i32\n}"),
    ('o_desc', "    *((y + 0x80) as *const i16) as i32 - *((x + 0x80) as *const i16) as i32\n}", "    *((x + 0x80) as *const i16) as i32 - *((y + 0x80) as *const i16) as i32\n}"),
    ('o_cresc', "    *((x + 0x80) as *const i16) as i32 - *((y + 0x80) as *const i16) as i32\n}\n", "    *((x + 0x80) as *const i16) as i32 - *((y + 0x7e) as *const i16) as i32\n}\n"),
    ('o_nulo', "    if x == 0 {\n        return (y != 0) as i32;\n    }\n    if y == 0 {\n        return -1;\n    }\n    *((y + 0x80)", "    if x == 0 {\n        return 1;\n    }\n    if y == 0 {\n        return -1;\n    }\n    *((y + 0x80)"),
]
