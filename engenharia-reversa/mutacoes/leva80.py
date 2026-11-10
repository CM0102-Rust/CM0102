# Mutacoes da leva80 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva80.py
MUTACOES = [
    ('e_ativo', "        if q != 0 && *((q + 4) as *const u32) != 0 && valid() {", "        if q != 0 && valid() {"),
    ('e_nevoa', "    let valid = || read_i32(person) < read_i32(G_STAFF_COUNT) && *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0;", "    let valid = || read_i32(person) < read_i32(G_STAFF_COUNT);"),
    ('e_16', "    for k in 0..16usize {\n        let q = read_usize(this + 4 * k);\n        if q != 0 && *((q + 4)", "    for k in 0..15usize {\n        let q = read_usize(this + 4 * k);\n        if q != 0 && *((q + 4)"),
    ('t_menos4', "    read_usize(this.wrapping_add(k.wrapping_mul(4)).wrapping_sub(4))", "    read_usize(this.wrapping_add(k.wrapping_mul(4)))"),
    ('s_200', "home(k) & setor != 0 && home(k) & (slot_cells::CENTRE as u16) != 0", "home(k) & setor != 0"),
    ('s_fe', "        c == k || (c == 0xfe && *((p + 1) as *const u8) == k)", "        c == k"),
    ('s_sem', "    if players == 0 {\n        return 1;\n    }\n    (0..20usize)", "    if players == 0 {\n        return 0;\n    }\n    (0..20usize)"),
    ('v_ff', "== code).unwrap_or(0xff)", "== code).unwrap_or(0xb)"),
    ('d_48', "        if *b & 4 != 0 && *b & 8 != 0 {", "        if *b & 4 != 0 {"),
    ('d_menos1', "        write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, -1);", "        write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, 0);"),
    ('f_442', "        0x101176c1 | 0x101b26c1 => 0x101f06c1,", "        0x101176c1 => 0x101f06c1,"),
    ('f_433', "        0x288a8b81 => 0x281b2381,", "        0x288a8b81 => 0x281b2380,"),
    ('f_352', "0x280e07c1 | 0x091a07c1 | 0x2884ab81", "0x280e07c1 | 0x2884ab81"),
]
