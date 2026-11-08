# Mutacoes da leva134 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva134.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('i_zera29', "    *((d + 0x29) as *mut u8) = 0;\n    *((d + 0x2a) as *mut u8) = 0;", "    *((d + 0x29) as *mut u8) = 1;\n    *((d + 0x2a) as *mut u8) = 0;"),
    ('i_2b', "    pr(d + 0x2b, 0);", "    pr(d + 0x2b, 1);"),
    ('i_comp_zera', "            pr(comp + 0xc, 0);\n            pr(comp + 0x10, 0);", "            pr(comp + 0xc, 1);\n            pr(comp + 0x10, 0);"),
    ('i_16', "for k in 0..16 {\n                    let pe", "for k in 0..15 {\n                    let pe", "interesse humano na competicao"),
    ('i_k', "(ler(0xae2364) + k - 16)", "(ler(0xae2364) + k - 15)", "interesse humano na competicao"),
    ('i_outro', "|| (other != 0 && in_competition(objeto, other) != 0)", "|| false", "interesse humano na competicao"),
    ('i_marca_comp', "crate::comparison::branch(\"interesse humano na competicao\");\n                        pr(comp + 0xc, 1);", "crate::comparison::branch(\"interesse humano na competicao\");\n                        pr(comp + 0xc, 0);", "interesse humano na competicao"),
    ('i_6b61e0', "crate::comparison::branch(\"interesse pela 6b61e0\");\n                    pr(comp + 0xc, 1);", "crate::comparison::branch(\"interesse pela 6b61e0\");\n                    pr(comp + 0xc, 0);", "interesse pela 6b61e0"),
    ('i_byte44', "human(g(m + 0x20), 0, *((m + 0x44) as *const u8) as u32, 0, 0)", "human(g(m + 0x20), 0, *((m + 0x45) as *const u8) as u32, 0, 0)"),
    ('i_5fe420', "|| f5fe420(g(0xb63c98), m + 4) != 0;", "|| f5fe420(g(0xb63c98), m + 8) != 0;"),
    ('i_marcada', "pr(m + 0x53, 1);", "pr(m + 0x53, 2);", "interesse partida marcada"),
]
