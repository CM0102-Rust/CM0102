# Mutacoes da leva159 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva159.py --carreira 2002-06-01
MUTACOES = [
    ('m_nivel', "        level -= 1;\n        if level < 10 || m != 0 {", "        level -= 2;\n        if level < 10 || m != 0 {"),
    ('m_lado', "    if p.left_side() >= 15 {", "    if p.left_side() >= 14 {"),
    ('c_linha', "([9, 13, 19, 26], [12, 19, 27, 30])", "([9, 13, 19, 26], [12, 19, 27, 31])"),
    ('c_passo', "let step_towards = if near { 1 } else if setor & 1 == 0 { 5 } else { 0 };", "let step_towards = if near { 1 } else if setor & 1 == 0 { 4 } else { 0 };"),
    ('c_coluna', "        [2, 8, 13, 18, 24]", "        [2, 8, 13, 18, 23]"),
    ('v_casa8', "exe::formation_get_slot_position(this, i, 0) & 8 != 0", "exe::formation_get_slot_position(this, i, 0) & 4 != 0"),
    ('v_a20', "if a & 0x880 != 0 && a & 0x20 != 0 {", "if a & 0x880 != 0 && a & 0x10 != 0 {"),
]
