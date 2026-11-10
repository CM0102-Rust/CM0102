# Mutacoes da leva50 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva50.py
MUTACOES = [
    ('g_limite', '(IDX_IS_SECOND_NATIONAL_TEAM)(0);\n    }\n    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT));', '(IDX_IS_SECOND_NATIONAL_TEAM)(0);\n    }\n    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT)) + 1;'),
    ('a_troca', "let (este, outro) = if side_b == 0 { (home, away) } else { (away, home) };", "let (este, outro) = (home, away);"),
    ('a_bonus', "        (false, true) => (0, 1),", "        (false, true) => (1, 0),"),
    ('a_fase3', "as i32) == 3 {\n        *fase3 = 1;", "as i32) == 4 {\n        *fase3 = 1;"),
    ('a_ida', "    if first_leg_home != -1 {", "    if first_leg_home != 0 {"),
    ('a_4c', "*outro = (*outro).wrapping_add(partida.away_aggregate());", "*outro = (*outro).wrapping_add(partida.home_aggregate());"),
    ('a_bit4', "            if f & 4 == 0 && team_ok {", "            if team_ok {"),
    ('a_tempo', "this.period_end() > 0x3de;", "this.period_end() >= 0x3de;"),
    ('a_menor', "                    if goals_away + (bonus_este as i32) < first_leg_home as i32 {", "                    if goals_away + (bonus_este as i32) <= first_leg_home as i32 {"),
    ('a_maior', "                } else if goals_away + (bonus_outro as i32) > first_leg_home as i32 {", "                } else if goals_away + (bonus_outro as i32) >= first_leg_home as i32 {"),
    ('a_ret', "        (*home as u8).wrapping_sub(*away as u8)", "        (*away as u8).wrapping_sub(*home as u8)"),
]
