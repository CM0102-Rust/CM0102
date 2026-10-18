# Mutacoes do fechamento do motor (3). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('f_anel', "    let pos = if pos == 0 { 0xb } else { pos.wrapping_sub(1) };", "    let pos = if pos == 0 { 0xa } else { pos.wrapping_sub(1) };"),
    ('f_repetido', "        core::ptr::write_unaligned((t + 6) as *mut i16, n - 1);", "        core::ptr::write_unaligned((t + 6) as *mut i16, n);"),
    ('c_tempo', "    b(0x2db0, 0x7d);", "    b(0x2db0, 0x7c);"),
    ('l_fim', "    *(0xa1599c as *mut u32) = u32::MAX;", "    *(0xa1599c as *mut u32) = 0;"),
]
