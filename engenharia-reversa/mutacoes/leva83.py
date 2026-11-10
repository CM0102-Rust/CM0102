# Mutacoes da leva83 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva83.py
MUTACOES = [
    ('p_110', "        .wrapping_add((read_i32(person) as usize).wrapping_mul(110))\n        .wrapping_add((k as isize * 20) as usize)\n}", "        .wrapping_add((read_i32(person) as usize).wrapping_mul(112))\n        .wrapping_add((k as isize * 20) as usize)\n}"),
    ('p_ligar', "    write_u8(person_block(this, person, k) + 0x44, 1);", "    write_u8(person_block(this, person, k) + 0x44, 2);"),
    ('p_sinal', "    read_i8(person_block(this, person, k) + 0x44) as i32", "    read_u8(person_block(this, person, k) + 0x44) as i32"),
    ('p_3a', "    person_block(this, person, k) + 0x3a", "    person_block(this, person, k) + 0x3c"),
    ('c_ff', "    if national_team.is_null() {\n        return 0xff;", "    if national_team.is_null() {\n        return 0;"),
    ('c_2', "    if exe::club_is_national_team(national_team) != 0 {\n        return 2;", "    if exe::club_is_national_team(national_team) != 0 {\n        return 1;"),
    ('c_bit', "    (person.playing_squad() >> 3) & 1", "    (person.playing_squad() >> 2) & 1"),
    ('g_9b', "    read_u32(G_DATABASE_NAMES_LANGUAGE)\n}", "    read_u32(G_PLAYER_INIT_LEVEL_BASE_TABLE)\n}"),
]
