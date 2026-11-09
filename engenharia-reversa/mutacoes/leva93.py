# Mutacoes da leva93 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva93.py --jogo 2001-08-20
MUTACOES = [
    ('k_centro', "        this.set_ball_x(4);\n        this.set_ball_y(11);", "        this.set_ball_x(4);\n        this.set_ball_y(10);"),
    ('k_nibble', "            move_to_cell(j, (home & 0xf) as i8, (home >> 4) as i8);\n            exe::cm3_match_engine_update_offside_lines(this);", "            move_to_cell(j, (home >> 4) as i8, (home & 0xf) as i8);\n            exe::cm3_match_engine_update_offside_lines(this);"),
    ('k_passou', "                d >= 0\n", "                d > 0\n"),
    ('k_sai', "        let sai_jogando = side_ball as i32 == l as i32;", "        let sai_jogando = side_ball as i32 != l as i32;"),
    ('k_2b', "            exe::match_player_set_action_delay(j, 0);", "            exe::match_player_set_action_delay(j, 1);"),
    ('k_19a', "    who.set_facing((if this.ball_y() != 0 { 0xb4 } else { 0 }) as i16);", "    who.set_facing((if this.ball_y() != 0 { 0xb5 } else { 0 }) as i16);"),
]
