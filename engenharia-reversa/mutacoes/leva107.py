# Mutacoes da leva107 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva107.py --jogo 2001-08-20
MUTACOES = [
    ('u_choca', "        if qualquer || (y != 0 && x != 0 && choca(y, cor_outro) == 0) {", "        if qualquer || (y != 0 && x != 0 && choca(x, cor_outro) == 0) {"),
    ('u_par2', "                (x, y) = if k == 1 { (u(0x8b), u(0x8f)) } else { (u(0x93), u(0x97)) };", "                (x, y) = if k == 1 { (u(0x93), u(0x97)) } else { (u(0x8b), u(0x8f)) };"),
    ('u_fim', "    if x == 0 || y == 0 { 1 } else { k }", "    if x == 0 || y == 0 { 1 } else { k + 1 }"),
    ('u_nenhum', "            _ => return 1,", "            _ => return 3,"),
    ('u_cor', "        if c != 0 { c } else { *(0xae23ec as *const usize) + *(0x9cf17c as *const i32) as usize * 58 }", "        if c != 0 { c } else { *(0xae23ec as *const usize) }"),
]
