# Mutacoes da leva65 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva65.py
MUTACOES = [
    ('t_humano', "    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) != 0 && human != 0 {", "    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) != 0 {"),
    ('t_menos1', "        if exe::club_is_national_team(club) != 0 {\n            return -1;", "        if exe::club_is_national_team(club) != 0 {\n            return -2;"),
    ('t_15', ".wrapping_add(if exe::database_is_national_team(club) != 0 { 0xf } else { -1 })", ".wrapping_add(if exe::database_is_national_team(club) != 0 { 0xe } else { -1 })"),
    ('t_m1', ".wrapping_add(if exe::database_is_national_team(club) != 0 { 0xf } else { -1 })", ".wrapping_add(if exe::database_is_national_team(club) != 0 { 0xf } else { 0 })"),
    ('t_91', "    read_u8(table.wrapping_add((l as usize).wrapping_mul(0x91)) + tactics_row::SHOOTOUT_ORDER + pos as usize)", "    read_u8(table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x85 + pos as usize)"),
    ('t_l0', "    if l == -1 {\n        return 0;", "    if l == -1 {\n        return 1;"),
]
