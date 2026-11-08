# Mutacoes da ultima nota e da busca de jogadores. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('u_bits', "    let rating = *((record + 0x26) as *const u8) & 0xf;", "    let rating = *((record + 0x26) as *const u8) & 0x7;"),
    ('u_padrao', "    if rating != 0 { rating as f32 } else { *(0x96e2b8 as *const f32) }", "    if rating != 0 { rating as f32 } else { 6.0 }"),
    ('b_vaga', "    *((search + 4) as *mut i8) = slot;", "    *((search + 4) as *mut i8) = slot.wrapping_add(1);"),
]
