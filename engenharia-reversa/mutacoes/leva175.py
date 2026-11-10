# Mutacoes da leva168 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva168.py --carreira 2002-06-01
MUTACOES = [
    ('f_segunda', "        read_usize(t + tactics_fields::CUSTOM_TACTICS).wrapping_add((f - primeiras) as usize * 0x18e3)", "        read_usize(t + tactics_fields::CUSTOM_TACTICS).wrapping_add(f as usize * 0x18e3)"),
    ('f_limite', "    let limit = if human != 0 { primeiras }", "    let limit = if human == 0 { primeiras }"),
    ('p_c1', "    if c1.third_language() == code {\n        return 300;", "    if c1.third_language() == code {\n        return 301;"),
    ('p_c2', "    if c2.third_language() == code { 100 } else { 0 }", "    if c2.third_language() == code { 101 } else { 0 }"),
    ('p_esp', "            (code == 0x16 || code == 0x20) as u8", "            (code == 0x16) as u8"),
    ('n_campo', "    write_u32(t + 5 + 4 * k as usize, value);", "    write_u32(t + 9 + 4 * k as usize, value);"),
]
