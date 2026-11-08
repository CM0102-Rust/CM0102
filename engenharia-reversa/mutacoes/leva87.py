# Mutacoes da leva87 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva87.py --jogo 2001-08-20
MUTACOES = [
    ('f_500', "            soma(-500);", "            soma(-400);"),
    ('f_mascara', "        mascara = mascara.wrapping_add(bt);", "        mascara |= bt;"),
    ('f_1080', "        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = principal;\n", ""),
    ('f_20', "            if (byte(ficha, 0x2f) as i32) > sortear(20)", "            if (byte(ficha, 0x2f) as i32) > sortear(19)"),
    ('f_750', "(*((clube + club::REPUTATION) as *const i16)).wrapping_sub(750)", "(*((clube + club::REPUTATION) as *const i16)).wrapping_sub(700)"),
    ('f_3', "                    soma(-3 * nota(j));", "                    soma(-2 * nota(j));"),
    ('f_soma', "        if j != designado() {\n            soma(nota(j));", "        if j != designado() {\n            soma(nota(j) + 1);"),
    ('f_marca', "            desempenho(j, 1);", "            desempenho(j, 0);"),
]
