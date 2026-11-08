# Mutacoes da leva60 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva60.py
MUTACOES = [
    ('s_581', "base.wrapping_add((i as usize).wrapping_mul(581))", "base.wrapping_add((i as usize).wrapping_mul(580))"),
    ('s_clubes', "        .wrapping_add(*(G_CLUB_COUNT as *const i32));", "        .wrapping_add(*(G_STAFF_COUNT as *const i32));"),
    ('s_2', "    clube_da_selecao(nacao, 2, 175)", "    clube_da_selecao(nacao, 1, 175)"),
    ('s_1', "    clube_da_selecao(nacao, 1, 176)", "    clube_da_selecao(nacao, 0, 176)"),
    ('s_nacoes', ".wrapping_sub(nacoes_antes.wrapping_mul(*(G_NATION_COUNT as *const i32)))", ".wrapping_sub(nacoes_antes.wrapping_mul(*(G_CITY_COUNT as *const i32)))"),
]
