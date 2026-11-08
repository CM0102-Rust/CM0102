MUTACOES = [
    ('l_limpeza', "*((f + 0x19) as *mut u8) = 0xfb;", "*((f + 0x19) as *mut u8) = 0x05;"),
    ('l_pontape', "narrate(if *w(0x8ed0) > 1 { 0x1f74 } else { 0x212f }", "narrate(if *w(0x8ed0) > 1 { 0x1f74 } else { 0x1f74 }"),
    ('l_tentativas', "let c = sortear(0x14) as i8;", "let c = sortear(0x13) as i8;"),
]
