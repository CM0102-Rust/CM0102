MUTACOES = [
    ('l_limpeza', "*((f + match_player::POSITION) as *mut u8) = 0xfb;", "*((f + match_player::POSITION) as *mut u8) = 0x05;"),
    ('l_pontape', "narrate(if *((m + engine::CLOCK) as *mut i16) > 1 { event::KICK_OFF } else { event::START_MATCH }", "narrate(if *((m + engine::CLOCK) as *mut i16) > 1 { 0x1f74 } else { 0x1f74 }"),
    ('l_tentativas', "let c = random(0x14) as i8;", "let c = random(0x13) as i8;"),
]
