MUTACOES = [
    ('l_limpeza', "write_u8(f + match_player::POSITION, 0xfb);", "write_u8(f + match_player::POSITION, 0x05);"),
    ('l_pontape', "narrate(if read_i16(m + engine::CLOCK) > 1 { event::KICK_OFF } else { event::START_MATCH }", "narrate(if read_i16(m + engine::CLOCK) > 1 { 0x1f74 } else { 0x1f74 }"),
    ('l_tentativas', "let c = random(0x14) as i8;", "let c = random(0x13) as i8;"),
]
