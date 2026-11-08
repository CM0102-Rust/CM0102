# Mutacoes da aposentadoria, das competicoes do amistoso e da vaga da noticia. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('a_lesao', "            let ok = if by_injury != 0 { *((e + 0x18) as *const u8) == 8 }", "            let ok = if by_injury != 0 { *((e + 0x18) as *const u8) != 8 }"),
    ('a_torneio', "        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 {", "        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 && i > 0 {"),
    ('n_vaga', "    core::ptr::read_unaligned((noticia + 5 + slot as usize * 4) as *const u32)", "    core::ptr::read_unaligned((noticia + 9 + slot as usize * 4) as *const u32)"),
]
