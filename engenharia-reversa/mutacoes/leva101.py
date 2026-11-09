# Mutacoes da leva101 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva101.py --jogo 2001-08-20
MUTACOES = [
    ('g_1000', "    let pega = strength <= 1000 && {", "    let pega = strength <= 999 && {"),
    ('g_20', "        random(n) < random(0x14)", "        random(n) < random(0x15)"),
    ('g_espalma', "    let espalma = j.moment_decisions() as i32 > random(8);", "    let espalma = j.moment_decisions() as i32 >= random(8);"),
    ('g_2b', "    *b2 = (*b2).wrapping_add(random(if espalma { 2 } else { 3 }) as i8);", "    *b2 = (*b2).wrapping_add(random(if espalma { 3 } else { 3 }) as i8);"),
    ('g_dentro', "    let (dentro, chutador) = if random(10) != 0 { (1u32, 0usize) }", "    let (dentro, chutador) = if random(11) != 0 { (1u32, 0usize) }"),
    ('g_espalhar', "    let spread = if espalma { 3 } else { 1 };", "    let spread = if espalma { 2 } else { 1 };"),
    ('g_cansa', "    *w35 = (*w35).wrapping_add(0xff06);", "    *w35 = (*w35).wrapping_add(0xff07);"),
    ('g_17', "        if exe::match_player_adjust_morale(j, r.wrapping_neg()) != 0 {", "        if exe::match_player_adjust_morale(j, r) != 0 {"),
]
