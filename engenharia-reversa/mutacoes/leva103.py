# Mutacoes da leva103 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva103.py --jogo 2001-08-20
MUTACOES = [
    ('p_passou', "    let passou = |fim: i32| *w(0x8ed0) as i32 > byte(m, 0x8eb7) as i32 + fim;", "    let passou = |fim: i32| *w(0x8ed0) as i32 >= byte(m, 0x8eb7) as i32 + fim;"),
    ('p_intervalo', "            *tb(0x45) = byte(m, 0xf5bd);", "            *tb(0x45) = byte(m, 0xf5f3);"),
    ('p_20f1', "            fala(chamar, 0x20f1);", "            fala(chamar, 0x20f2);"),
    ('p_3de', "            chamar(0x6ce460, m, [0x3de, 0, 0, 0, 0]);\n            recuperar(0);", "            chamar(0x6ce460, m, [0x3df, 0, 0, 0, 0]);\n            recuperar(0);"),
    ('p_teto0', "            recuperar(0);", "            recuperar(1);"),
    ('p_saida', "        *b(0x8eb6) = (a == 0) as i8;", "        *b(0x8eb6) = a;"),
    ('p_placar', "            *tb(0x44) = byte(m, 0xf5f3);\n            let regra", "            *tb(0x44) = byte(m, 0xf5bd);\n            let regra"),
    ('p_fim', "                chamar(0x6ce460, m, [0, 0, 0, 0, 0]);\n                *w(0x8ed4) = 0;\n                return 4;", "                chamar(0x6ce460, m, [0, 0, 0, 0, 0]);\n                *w(0x8ed4) = 1;\n                return 4;"),
    ('p_ordem', "                *w(0x8ed4) = 0;\n                classificar(c);", "                classificar(c);\n                *w(0x8ed4) = 0;"),
]
