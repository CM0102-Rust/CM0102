# Mutacoes da leva78 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva78.py
MUTACOES = [
    ('c_tres', "    if id == g(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) || id == g(KEY_CLUB_COMP_FRENCH_CFA) || id == g(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) {", "    if id == g(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) || id == g(KEY_CLUB_COMP_FRENCH_CFA) {"),
    ('c_107', "(id as usize).wrapping_mul(107)).wrapping_add(0x5d)", "(id as usize).wrapping_mul(108)).wrapping_add(0x5d)"),
    ('c_fase', "    if c.is_null() || (c.comp_type() as i8) != 2 || (c.club_count() as u16) == 0 {", "    if c.is_null() || (c.club_count() as u16) == 0 {"),
    ('c_rodadas', "    if c.is_null() || (c.comp_type() as i8) != 2 || (c.club_count() as u16) == 0 {", "    if c.is_null() || (c.comp_type() as i8) != 2 {"),
    ('c_60', "(first.reserve_division().addr() as i32) == target) as u32", "read_i32(first.addr() + 0x5c) == target) as u32"),
    ('o_5f', "    v(*b) - v(*a)\n}", "    v(*a) - v(*b)\n}"),
    ('o_sinal5f', "read_i8(p + 0x5f) as i32 };", "read_u8(p + 0x5f) as i32 };"),
    ('o_80', "    read_i16(*b + 0x80) as i32 - read_i16(*a + 0x80) as i32\n}", "    read_u16(*b + 0x80) as i32 - read_i16(*a + 0x80) as i32\n}"),
    ('o_desc', "        return r;\n    }\n    read_i16(y + club::REPUTATION) as i32 - read_i16(x + club::REPUTATION) as i32", "        return r;\n    }\n    read_i16(x + 0x80) as i32 - read_i16(y + 0x80) as i32"),
    ('o_cresc', "    read_i16(x + 0x80) as i32 - read_i16(y + 0x80) as i32\n}\n", "    read_i16(x + 0x80) as i32 - read_i16(y + 0x7e) as i32\n}\n"),
    ('o_nulo', "pub(crate) unsafe extern \"C\" fn order_by_80_desc(a: *const usize, b: *const usize) -> i32 {\n    let (x, y) = (*a, *b);\n    if x == 0 {\n        return (y != 0) as i32;", "pub(crate) unsafe extern \"C\" fn order_by_80_desc(a: *const usize, b: *const usize) -> i32 {\n    let (x, y) = (*a, *b);\n    if x == 0 {\n        return 1;"),
]
