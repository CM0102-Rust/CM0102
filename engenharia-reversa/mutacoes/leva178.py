# Mutacoes da leva171 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva171.py --carreira 2002-06-01
MUTACOES = [
    ('m_segunda', "    let (n, q) = if second != 0 { (d(0x20), d(0x28)) } else { (d(0x10), d(0x18)) };", "    let (n, q) = if second == 0 { (d(0x20), d(0x28)) } else { (d(0x10), d(0x18)) };"),
    ('m_padrao', "        return read_f64_at(0x968998);", "        return 6.0;"),
    ('v_campo', "    core::ptr::read_unaligned((sheet + 0xc) as *const u32)", "    core::ptr::read_unaligned((sheet + 0x10) as *const u32)"),
    ('n_rep', "    if *rep < if base != 0 { 0x32 } else { 0x64 } {", "    if *rep < if base != 0 { 0x32 } else { 0x50 } {"),
    ('n_meio', "    } else if n(0x12) || n(0x13) || n(0x14) {", "    } else if n(0x12) || n(0x13) {"),
]
