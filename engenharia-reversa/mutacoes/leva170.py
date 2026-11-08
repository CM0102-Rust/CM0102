# Mutacoes do match_wrap_up do motor (4). Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('o_campos', "    for o in [0x36, 0x38, 0x34] {", "    for o in [0x36, 0x34, 0x38] {"),
    ('o_forca', "    strength(pb).wrapping_sub(strength(pa))", "    strength(pa).wrapping_sub(strength(pb))"),
    ('o_marcado', "        if pa == first {\n            return -1;", "        if pa == first {\n            return 1;"),
    ('v_lance', "    if matches!(lados as u8, 2..=4) {", "    if matches!(lados as u8, 3..=4) {"),
    ('v_tipo1', "(0x1f49, 0x2182)", "(0x1f49, 0x2183)"),
]
