# Mutacoes da aposentadoria, das competicoes do amistoso e da vaga da noticia. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('a_lesao', "            let ok = if by_injury != 0 { e.reason() == 8 }", "            let ok = if by_injury != 0 { e.reason() != 8 }"),
    ('a_torneio', "        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && object != 0 {", "        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && object != 0 && i > 0 {"),
    ('n_vaga', "    read_u32(news + 5 + slot as usize * 4)", "    read_u32(news + 9 + slot as usize * 4)"),
]
