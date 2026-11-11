# Mutacoes do match_wrap_up do motor (3). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('f_anel', "    let pos = if pos == 0 { 0xb } else { pos.wrapping_sub(1) };", "    let pos = if pos == 0 { 0xa } else { pos.wrapping_sub(1) };"),
    ('f_repetido', "        write_i16(t + 6, n - 1);", "        write_i16(t + 6, n);"),
    ('c_tempo', "    b(commentary_queue::COMMENTARY_SPEED, 0x7d);", "    b(commentary_queue::COMMENTARY_SPEED, 0x7c);"),
    ('l_fim', "    write_u32(G_MATCH_EVENTS_CONFIG_LANGUAGE, u32::MAX);", "    write_u32(G_MATCH_EVENTS_CONFIG_LANGUAGE, 0);"),
]
