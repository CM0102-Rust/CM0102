# Mutacoes da leva47 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva47.py
MUTACOES = [
    ('f_base', "let (bx, by) = (3 * x + 1, 3 * y + 1);", "let (bx, by) = (3 * x + 1, 3 * y);"),
    ('f_primeiro', "for lado in [first, (first == 0) as i8] {", "for lado in [0i8, 1] {"),
    ('f_centro', "&& r == 1 && c == 1 {", "&& r == 1 && c == 2 {"),
    ('f_50', "(tentativas as i16 as i32) / 50", "(tentativas as i16 as i32) / 49"),
    ('f_maior', "if ocupada[i] as i32 > (tentativas", "if ocupada[i] as i32 >= (tentativas"),
    ('f_dono', "                        ocupada[4] = 1;\n                        (bx, by)", "                        ocupada[4] = 0;\n                        (bx, by)"),
    ('f_marca', "                    *e.add(7) = 1;\n                    k += 1;\n                }\n            }", "                    *e.add(7) = 2;\n                    k += 1;\n                }\n            }"),
    ('f_zera', "        *out.add(7 + 6 * i) = 0;", "        *out.add(7 + 6 * i) = 1;"),
    ('f_papel', "(e.add(5) as *mut u16).write_unaligned(match_player_position(p));", "(e.add(5) as *mut u16).write_unaligned(match_player_position(p) ^ 1);"),
]
