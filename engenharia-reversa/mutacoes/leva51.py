# Mutacoes da leva51 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva51.py
MUTACOES = [
    ('d_goleiro', "    if byte(p, match_player::POSITION) < 0 || match_player_position(p) & 1 != 0 {\n        return 0xff;", "    if byte(p, match_player::POSITION) < 0 {\n        return 0xff;"),
    ('d_set6', "        if (rpl & 6 != 0 && rql & 0x40 == 0)", "        if (rpl & 2 != 0 && rql & 0x40 == 0)"),
    ('d_set8', "            || (rpl & 8 != 0 && rql & 0x20 == 0)", "            || (rpl & 8 != 0 && rql & 0x10 == 0)"),
    ('d_set40', "            || (rpl & 0x40 != 0 && rql & 6 == 0)", "            || (rpl & 0x40 != 0 && rql & 4 == 0)"),
    ('d_par', "(0x100, 0x400), (0x200, 0x200)", "(0x100, 0x400), (0x200, 0x100)"),
    ('d_linha', "        0x20 => 4,\n        0x40 => 5,\n        _ => side_b,", "        0x20 => 4,\n        0x40 => 6,\n        _ => side_b,"),
    ('d_coluna_q', "            0x800 => 1,\n            0x400 => 3,", "            0x800 => 2,\n            0x400 => 3,"),
    ('d_menor', "        if chosen == -1 || dist < best {", "        if chosen == -1 || dist <= best {"),
    ('d_tabela', "read_f32_at(G_CELL_DISTANCE_TABLE + 4 * (dr as usize * 9 + dc as usize))", "read_f32_at(G_CELL_DISTANCE_TABLE + 4 * (dr as usize * 8 + dc as usize))"),
    ('d_goleiro_q', "        if byte(q, match_player::POSITION) < 0 || match_player_position(q) & 1 != 0 {\n            continue;\n        }\n        split(", "        if byte(q, match_player::POSITION) < 0 {\n            continue;\n        }\n        split("),
]
