# Mutacoes do match_wrap_up do motor (6). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('p_parte', "                *(p as *mut u16) |= 0x8000;", "                *(p as *mut u16) |= 0x4000;"),
    ('p_anel', "    *c = if c.read().wrapping_add(1) > 0xb { 0 } else { c.read().wrapping_add(1) };", "    *c = if c.read().wrapping_add(1) > 0xa { 0 } else { c.read().wrapping_add(1) };"),
    ('n_longa', "                name(d(0x198), d(0x19c), d(0x1a0), buf, size, longa, d(0x190));", "                name(d(0x198), d(0x19c), d(0x1a0), buf, size, 5, d(0x190));"),
    ('h_peso', "    let weight = (c32 / 2 + 10) as i8 as i32;", "    let weight = (c32 / 2 + 9) as i8 as i32;"),
    ('h_idade', "            rep = rep.wrapping_sub((age as i16).wrapping_mul(age as i16));", "            rep = rep.wrapping_sub(age as i16);"),
    ('h_certeza', "        certeza = (match_pl_ftol(x + read_f64_at(0x968968)) as i8).min(0x14);", "        certeza = (match_pl_ftol(x + read_f64_at(0x968968)) as i8).min(0x13);"),
]
