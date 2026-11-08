# Mutacoes da leva159 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva159.py --carreira 2002-06-01
MUTACOES = [
    ('m_nivel', "        nivel -= 1;\n        if nivel < 10 || m != 0 {", "        nivel -= 2;\n        if nivel < 10 || m != 0 {"),
    ('m_lado', "    if b(0x18) >= 15 {", "    if b(0x18) >= 14 {"),
    ('c_linha', "([9, 13, 19, 26], [12, 19, 27, 30])", "([9, 13, 19, 26], [12, 19, 27, 31])"),
    ('c_passo', "let step_towards = if perto { 1 } else if setor & 1 == 0 { 5 } else { 0 };", "let step_towards = if perto { 1 } else if setor & 1 == 0 { 4 } else { 0 };"),
    ('c_coluna', "        [2, 8, 13, 18, 24]", "        [2, 8, 13, 18, 23]"),
    ('v_casa8', "casa(this, i, 0) & 8 != 0", "casa(this, i, 0) & 4 != 0"),
    ('v_a20', "if a & 0x880 != 0 && a & 0x20 != 0 {", "if a & 0x880 != 0 && a & 0x10 != 0 {"),
]
