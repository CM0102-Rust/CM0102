# Mutacoes da leva87 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva87.py --jogo 2001-08-20
MUTACOES = [
    ('f_500', "            sum(-500);", "            sum(-400);"),
    ('f_mascara', "        mask = mask.wrapping_add(bt);", "        mask |= bt;"),
    ('f_1080', "        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = main;\n", ""),
    ('f_20', "            if (byte(sheet, 0x2f) as i32) > sortear(20)", "            if (byte(sheet, 0x2f) as i32) > sortear(19)"),
    ('f_750', "(*((club + club::REPUTATION) as *const i16)).wrapping_sub(750)", "(*((club + club::REPUTATION) as *const i16)).wrapping_sub(700)"),
    ('f_3', "                    sum(-3 * rating(j));", "                    sum(-2 * rating(j));"),
    ('f_soma', "        if j != designado() {\n            sum(rating(j));", "        if j != designado() {\n            sum(rating(j) + 1);"),
    ('f_marca', "            desempenho(j, 1);", "            desempenho(j, 0);"),
]
