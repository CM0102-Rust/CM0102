# Mutacoes da leva74 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva74.py
MUTACOES = [
    ('c_79', "    let e = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79));", "    let e = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(80));"),
    ('c_i1', "    if i1 >= 0 {\n        r1 = base", "    if i1 > 0 {\n        r1 = base"),
    ('c_eh_a', "    if pc != 0 && eh_a == 0 {", "    if pc != 0 {"),
    ('c_r1', "    if r1 != 0 && id_de(r1) == id_alvo {\n        return r1;", "    if r1 != 0 && id_de(r1) == id_clube {\n        return r1;"),
    ('n_4f', "    ((*((r + 0x4f) as *const u8) & 0xf) as u32).wrapping_sub(10)", "    ((*((r + 0x4f) as *const u8) & 0x1f) as u32).wrapping_sub(10)"),
    ('n_limite', "    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(2 * *(G_NATION_COUNT as *const i32));", "    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32));"),
    ('n_baixo', "        return clube as u32 & !0xff;", "        return 0;"),
]
