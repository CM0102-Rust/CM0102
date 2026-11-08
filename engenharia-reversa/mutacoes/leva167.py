# Mutacoes do match_wrap_up do motor (1). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('b_baixo', "        if (10..=11).contains(&row) {", "        if (9..=11).contains(&row) {"),
    ('b_coluna', "    if (2..=6).contains(&coluna) {", "    if (2..=7).contains(&coluna) {"),
    ('a_onze', "    ((minute as i16).wrapping_mul(11) < core::ptr::read_unaligned((m + 0x8ed0) as *const i16)) as u8", "    ((minute as i16).wrapping_mul(10) < core::ptr::read_unaligned((m + 0x8ed0) as *const i16)) as u8"),
    ('v_lado', "    f(m + if side != 0 { 0xaac5 } else { 0x91e2 })", "    f(m + if side == 0 { 0xaac5 } else { 0x91e2 })"),
]
