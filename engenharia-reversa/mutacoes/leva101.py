# Mutacoes da leva101 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva101.py --jogo 2001-08-20
MUTACOES = [
    ('g_1000', "    let pega = forca <= 1000 && {", "    let pega = forca <= 999 && {"),
    ('g_20', "        sortear(n) < sortear(0x14)", "        sortear(n) < sortear(0x15)"),
    ('g_espalma', "    let espalma = byte(j, 0x106) as i32 > sortear(8);", "    let espalma = byte(j, 0x106) as i32 >= sortear(8);"),
    ('g_2b', "    *b2 = (*b2).wrapping_add(sortear(if espalma { 2 } else { 3 }) as i8);", "    *b2 = (*b2).wrapping_add(sortear(if espalma { 3 } else { 3 }) as i8);"),
    ('g_dentro', "    let (dentro, chutador) = if sortear(10) != 0 { (1u32, 0usize) }", "    let (dentro, chutador) = if sortear(11) != 0 { (1u32, 0usize) }"),
    ('g_espalhar', "    let espalhar = if espalma { 3 } else { 1 };", "    let espalhar = if espalma { 2 } else { 1 };"),
    ('g_cansa', "    *w35 = (*w35).wrapping_add(0xff06);", "    *w35 = (*w35).wrapping_add(0xff07);"),
    ('g_17', "        if adjust_17(j, r.wrapping_neg()) != 0 {", "        if adjust_17(j, r) != 0 {"),
]
