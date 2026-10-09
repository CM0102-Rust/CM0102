# Mutacoes da leva80 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva80.py
MUTACOES = [
    ('e_ativo', "        if q != 0 && *((q + 4) as *const u32) != 0 && valida() {", "        if q != 0 && valida() {"),
    ('e_nevoa', "    let valida = || *(pessoa as *const i32) < *(0xae2364 as *const i32) && *(0x9cc270 as *const u8) != 0;", "    let valida = || *(pessoa as *const i32) < *(0xae2364 as *const i32);"),
    ('e_16', "    for k in 0..16usize {\n        let q = *((this + 4 * k) as *const usize);\n        if q != 0 && *((q + 4)", "    for k in 0..15usize {\n        let q = *((this + 4 * k) as *const usize);\n        if q != 0 && *((q + 4)"),
    ('t_menos4', "    *(this.wrapping_add(k.wrapping_mul(4)).wrapping_sub(4) as *const usize)", "    *(this.wrapping_add(k.wrapping_mul(4)) as *const usize)"),
    ('s_200', "casa(k) & setor != 0 && casa(k) & 0x200 != 0", "casa(k) & setor != 0"),
    ('s_fe', "        c == k || (c == 0xfe && *((p + 1) as *const u8) == k)", "        c == k"),
    ('s_sem', "    if jogadores == 0 {\n        return 1;\n    }\n    (0..20usize)", "    if jogadores == 0 {\n        return 0;\n    }\n    (0..20usize)"),
    ('v_ff', "== codigo).unwrap_or(0xff)", "== codigo).unwrap_or(0xb)"),
    ('d_48', "        if *b & 4 != 0 && *b & 8 != 0 {", "        if *b & 4 != 0 {"),
    ('d_menos1', "        core::ptr::write_unaligned((this + 0x5bf + 4 * k) as *mut i32, -1);", "        core::ptr::write_unaligned((this + 0x5bf + 4 * k) as *mut i32, 0);"),
    ('f_442', "        0x101176c1 | 0x101b26c1 => 0x101f06c1,", "        0x101176c1 => 0x101f06c1,"),
    ('f_433', "        0x288a8b81 => 0x281b2381,", "        0x288a8b81 => 0x281b2380,"),
    ('f_352', "0x280e07c1 | 0x091a07c1 | 0x2884ab81", "0x280e07c1 | 0x2884ab81"),
]
