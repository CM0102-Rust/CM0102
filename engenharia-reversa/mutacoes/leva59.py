# Mutacoes da leva59 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva59.py
MUTACOES = [
    ('d_neg', "    if i > -1 {\n        (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)", "    if i > 0 {\n        (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)"),
    ('d_110', "(*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)", "(*(G_STAFF_TABLE as *const usize)).wrapping_add(100 * i as usize)"),
    ('d_4', "    if marca(formacao, byte(this, 0x19)) & 4 == 0 {\n        return 0;", "    if marca(formacao, byte(this, 0x19)) & 1 == 0 {\n        return 0;"),
    ('d_ativo', ".find(|&q| byte(q, 0x19) >= 0 && *((q + 0x69) as *const usize) == ficha)", ".find(|&q| *((q + 0x69) as *const usize) == ficha)"),
    ('d_10', "    if marca(formacao, byte(this, 0x19)) & 0x10 == 0 {\n        return alvo;", "    if marca(formacao, byte(this, 0x19)) & 0x20 == 0 {\n        return alvo;"),
    ('d_aa5d', "*((motor + 0xaa5d + 0x18e3 * o) as *const u32) & 0x1000_0000 != 0", "*((motor + 0xaa5d + 0x18e3 * o) as *const u32) & 0x2000_0000 != 0"),
    ('d_af', "|| byte(motor + o, 0xaf) > 1;", "|| byte(motor + o, 0xaf) > 2;"),
    ('d_40', "    if b == a || b & 0x40 != 0 || byte(this, 0x106) == 0 {", "    if b == a || byte(this, 0x106) == 0 {"),
    ('d_ab', "    if b == a || b & 0x40 != 0 ||", "    if b & 0x40 != 0 ||"),
]
