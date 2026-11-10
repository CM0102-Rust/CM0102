# Mutacoes da leva48 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva48.py
MUTACOES = [
    ('u_1000', "    if n < 1000 {\n        (n % 10) as u8", "    if n <= 1000 {\n        (n % 10) as u8"),
    ('u_resto', "        (n % 10) as u8", "        (n.rem_euclid(10)) as u8"),
    ('a_guardada', "    if comp == read_usize(this + 4) {\n        return 1;", "    if comp == read_usize(this + 4) {\n        return 0;"),
    ('a_127', "wrapping_sub(0x7f)) as u32", "wrapping_sub(0x7e)) as u32"),
    ('c_maior', "    if id >= *(G_STAFF_COUNT as *const i32) {", "    if id > *(G_STAFF_COUNT as *const i32) {"),
    ('c_79', "(table as isize + 79 * id as isize)", "(table as isize + 80 * id as isize)"),
    ('c_neg', "    if i < 0 {\n        return 0;\n    }\n    (*(this", "    if i < -1 {\n        return 0;\n    }\n    (*(this"),
    ('c_80', "wrapping_add((80 * i) as usize)", "wrapping_add((79 * i) as usize)"),
    ('s_2', "wrapping_mul(2));", "wrapping_mul(3));"),
    ('s_maior', "    (read_i32(club) >= limit) as u32", "    (read_i32(club) > limit) as u32"),
]
