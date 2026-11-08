# Mutacoes da leva99 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva99.py --jogo 2001-08-20
MUTACOES = [
    ('l_angulo', "    if ang > 0x28 && ang < 0x8c {", "    if ang > 0x28 && ang < 0x8d {"),
    ('l_cruza', "        tx = s(3).wrapping_add(3);", "        tx = s(3).wrapping_add(2);"),
    ('l_5c', "                ty = mira(by(), f15, f14, 2, 1, 2);", "                ty = mira(by(), f15, f14, 2, 2, 2);"),
    ('l_5d', "                ty = mira(by(), f15, f14, 4, 2, 3);", "                ty = mira(by(), f15, f14, 4, 3, 3);"),
    ('l_9', "            tx = mira(bx(), f12, f13, 5, 3, 3);", "            tx = mira(bx(), f12, f13, 5, 4, 3);"),
    ('l_recua', "                    if !f14 {\n                        ty = ty.wrapping_add(2);\n                    }\n                } else if !f15 {\n                    ty = ty.wrapping_sub(2);\n                }\n                refina = mode != -1;\n            }\n        } else if mode == 4 {", "                    if !f14 {\n                        ty = ty.wrapping_add(1);\n                    }\n                } else if !f15 {\n                    ty = ty.wrapping_sub(2);\n                }\n                refina = mode != -1;\n            }\n        } else if mode == 4 {"),
    ('l_ponta', "                7\n            };\n            let mut row: i8;", "                6\n            };\n            let mut row: i8;"),
    ('l_area', "                    if e > 5 || (mode == 4 && e > 3) {\n                        row -= 1;", "                    if e > 4 || (mode == 4 && e > 3) {\n                        row -= 1;"),
    ('l_fora', "    if !(0..9).contains(&byte(mm, 0x8ea7)) || !(0..12).contains(&byte(mm, 0x8ea8)) {", "    if !(0..9).contains(&byte(mm, 0x8ea7)) || !(0..11).contains(&byte(mm, 0x8ea8)) {"),
]
