# Mutacoes da leva59 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva59.py
MUTACOES = [
    ('d_neg', "    if i > -1 {\n        (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)", "    if i > 0 {\n        (*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)"),
    ('d_110', "(*(G_STAFF_TABLE as *const usize)).wrapping_add(110 * i as usize)", "(*(G_STAFF_TABLE as *const usize)).wrapping_add(100 * i as usize)"),
    ('d_4', "    if exe::formation_get_marking_type(formation, read_i8(this + match_player::POSITION)) & 4 == 0 {\n        return 0;", "    if exe::formation_get_marking_type(formation, read_i8(this + match_player::POSITION)) & 1 == 0 {\n        return 0;"),
    ('d_ativo', ".find(|&q| read_i8(q + match_player::POSITION) >= 0 && *((q + match_player::STAFF) as *const usize) == sheet)", ".find(|&q| *((q + match_player::STAFF) as *const usize) == sheet)"),
    ('d_10', "    if exe::formation_get_marking_type(formation, read_i8(this + match_player::POSITION)) & 0x10 == 0 {\n        return target;", "    if exe::formation_get_marking_type(formation, read_i8(this + match_player::POSITION)) & 0x20 == 0 {\n        return target;"),
    ('d_aa5d', "*((engine + 0xaa5d + 0x18e3 * o) as *const u32) & 0x1000_0000 != 0", "*((engine + 0xaa5d + 0x18e3 * o) as *const u32) & 0x2000_0000 != 0"),
    ('d_af', "|| read_i8(engine + o + 0xaf) > 1;", "|| read_i8(engine + o + 0xaf) > 2;"),
    ('d_40', "    if b == a || b & 0x40 != 0 || read_i8(this + match_player::MOMENT_DECISIONS) == 0 {", "    if b == a || read_i8(this + match_player::MOMENT_DECISIONS) == 0 {"),
    ('d_ab', "    if b == a || b & 0x40 != 0 ||", "    if b & 0x40 != 0 ||"),
]
