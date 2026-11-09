# Mutacoes refeitas da leva148 (as quatro que escaparam eram fracas). Uso: python mutacoes.py mutacoes\leva148b.py --carreira 2002-06-01
MUTACOES = [
    ('i_ceu', "*((m + engine::ABANDON_MINUTE) as *mut u8) = if random(2) != 0 { (random(0x19) as u8).wrapping_add(0xf) } else { (random(0x1e) as u8).wrapping_add(0x32) };", "*((m + engine::ABANDON_MINUTE) as *mut u8) = if random(2) != 0 { 1 } else { 1 };"),
    ('i_pontos', "                    if p > 5 && (p as i32) > random(0x1e) {", "                    if p > -100 && (p as i32) > random(0x1e) {", "prematch unhappy with position"),
    ('i_contrato', "                let r = random(200) + 0x32;", "                let r = random(200);", "prematch end of contract"),
    ('i_insatisfeito', "                push_prematch_event(m, f, club, 0, 0, k as u8, 0x13);", "                push_prematch_event(m, f, club, 0, 0, k as u8, 0x1e);", "prematch unhappy"),
]
