# Mutacoes da leva60 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva60.py
MUTACOES = [
    ('s_581', "base.wrapping_add((i as usize).wrapping_mul(581))", "base.wrapping_add((i as usize).wrapping_mul(580))"),
    ('s_clubes', "        .wrapping_add(*(G_CLUB_COUNT as *const i32));", "        .wrapping_add(*(G_STAFF_COUNT as *const i32));"),
    ('s_2', "    national_team_club(nation, 2, 175)", "    national_team_club(nation, 1, 175)"),
    ('s_1', "    national_team_club(nation, 1, 176)", "    national_team_club(nation, 0, 176)"),
    ('s_nacoes', ".wrapping_sub(nations_before.wrapping_mul(*(G_NATION_COUNT as *const i32)))", ".wrapping_sub(nations_before.wrapping_mul(*(G_CITY_COUNT as *const i32)))"),
]
