MUTACOES = [
    ('l_limpeza', "*((f + match_player::POSITION) as *mut u8) = 0xfb;", "*((f + match_player::POSITION) as *mut u8) = 0x05;"),
    ('l_pontape', "narrate(if *w(engine::CLOCK) > 1 { 0x1f74 } else { 0x212f }", "narrate(if *w(engine::CLOCK) > 1 { 0x1f74 } else { 0x1f74 }"),
    ('l_tentativas', "let c = sortear(0x14) as i8;", "let c = sortear(0x13) as i8;"),
]
