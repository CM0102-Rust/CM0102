# Mutacoes da leva149 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva149.py --carreira 2002-06-01
MUTACOES = [
    ('e_mando', "        *b(0x19) = 0;\n        *b(0x1a) = 3;", "        *b(0x19) = 0;\n        *b(0x1a) = 2;"),
    ('e_mandante', "        *b(0x19) = 1;\n        *b(0x1a) = 0;", "        *b(0x19) = 1;\n        *b(0x1a) = 1;"),
    ('e_banco', "    let banco = (*((jogo + 0x4d) as *const u8) >> 4) & 0xf;", "    let banco = ((*((jogo + 0x4d) as *const u8) >> 4) & 0xf).saturating_sub(1);"),
    ('e_modo', "        n = automatica(t, outro, 2);", "        n = automatica(t, outro, 1);"),
    ('e_linhas', "    *d(0x2b) = m + 0x91e2 + (lado as u8 as i8 as isize * 0x18e3) as usize;", "    *d(0x2b) = m + 0x91e2 + ((1 - lado as u8 as i8) as isize * 0x18e3) as usize;"),
    ('e_ficha', "        *((t + 2) as *mut u16) = *((ficha + 8) as *const u16);", "        *((t + 2) as *mut u16) = *((ficha + 0xa) as *const u16);"),
    ('e_tecnico', "            if id >= base - 0x10 && id < base && byte(x, 0x5f) != 0 {", "            if id >= base - 0x10 && id < base && byte(x, 0x5f) == 0 {"),
    ('e_casa', "        *vagas = 0;\n        let total", "        *vagas = 1;\n        let total"),
]
