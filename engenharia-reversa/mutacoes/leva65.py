# Mutacoes da leva65 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva65.py
MUTACOES = [
    ('t_humano', "    if and_human(clube, 1, 1, 0, 0) != 0 && human != 0 {", "    if and_human(clube, 1, 1, 0, 0) != 0 {"),
    ('t_menos1', "        if second(clube) != 0 {\n            return -1;", "        if second(clube) != 0 {\n            return -2;"),
    ('t_15', ".wrapping_add(if is_national_team(clube) != 0 { 0xf } else { -1 })", ".wrapping_add(if is_national_team(clube) != 0 { 0xe } else { -1 })"),
    ('t_m1', ".wrapping_add(if is_national_team(clube) != 0 { 0xf } else { -1 })", ".wrapping_add(if is_national_team(clube) != 0 { 0xf } else { 0 })"),
    ('t_91', "    *((table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x86 + pos as usize) as *const u8)", "    *((table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x85 + pos as usize) as *const u8)"),
    ('t_l0', "    if l == -1 {\n        return 0;", "    if l == -1 {\n        return 1;"),
]
