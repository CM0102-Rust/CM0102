# Mutacoes da leva44 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva44.py
MUTACOES = [
    ('r_dono', "if *((this + 0xf582) as *const usize) != 0 || *((this + 0xf57a) as *const usize) != 0 || *b(0x8eaa) == 0 {", "if *((this + 0xf57a) as *const usize) != 0 || *b(0x8eaa) == 0 {"),
    ('r_parado', "        *b(0x8eaa) = 0;" + chr(10) + "        *b(0x8ea9) = 0;", "        *b(0x8eaa) = 0;"),
    ('r_v2', "    let anda = if v >= 2 {", "    let anda = if v >= 3 {"),
    ('r_v0', "    } else if v <= 0 {" + chr(10) + "        false", "    } else if v < 0 {" + chr(10) + "        false"),
    ('r_sorte', "        a >= sortear(2)", "        a > sortear(2)"),
    ('r_8ea9', "        if *b(0x8ea9) != 0 {", "        if *b(0x8ea9) > 0 {"),
    ('r_220', "if dir > 0xdc && dir < 0x140 {", "if dir >= 0xdc && dir < 0x140 {"),
    ('r_140', "} else if dir > 0x28 && dir < 0x8c {", "} else if dir > 0x28 && dir <= 0x8c {"),
    ('r_130', "} else if dir > 0x82 && dir < 0xe6 {", "} else if dir > 0x83 && dir < 0xe6 {"),
    ('r_50', "} else if dir < 0x32 || dir > 0x136 {", "} else if dir <= 0x32 || dir > 0x136 {"),
    ('r_x9', "if x < 0 || y < 0 || x >= 9 || y >= 12 {", "if x < 0 || y < 0 || x >= 10 || y >= 12 {"),
    ('r_lado', "ball_out_with(this, *b(0x8eab) as u8, call);", "ball_out_with(this, *b(0x8eae) as u8, call);"),
]
