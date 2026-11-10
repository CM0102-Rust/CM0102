# Mutacoes da ultima nota e da busca de jogadores. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('u_bits', "    let rating = read_u8(record + player_stats_entry::LAST_RATING) & 0xf;", "    let rating = read_u8(record + player_stats_entry::LAST_RATING) & 0x7;"),
    ('u_padrao', "    if rating != 0 { rating as f32 } else { read_f32(0x96e2b8) }", "    if rating != 0 { rating as f32 } else { 6.0 }"),
    ('b_vaga', "    write_i8(search + 4, slot);", "    write_i8(search + 4, slot.wrapping_add(1));"),
]
