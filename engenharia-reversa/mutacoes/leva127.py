# Mutacoes da leva127 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva127.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('t_fora', "if x < 0 || y < 0 || x >= 9 || y >= 12 {\n        covered!(\"contest out of play\");", "if x < 0 || y < 0 || x >= 8 || y >= 12 {\n        covered!(\"contest out of play\");", "contest out of play"),
    ('t_angulo', "            if d as i16 > 0xb4 {\n                d = 0x168 - d;", "            if d as i16 > 0xb3 {\n                d = 0x168 - d;"),
    ('t_vence', "if nota1 as i16 > nota0 as i16 {", "if nota1 as i16 >= nota0 as i16 {"),
    ('t_folego', "write_i16(wv + match_player::MATCH_RATING, read_i16(wv + match_player::MATCH_RATING).wrapping_add(4));", "write_i16(wv + match_player::MATCH_RATING, read_i16(wv + match_player::MATCH_RATING).wrapping_add(5));", "contest foul by the winner"),
    ('t_agora', "random(ftol(exe::match_player_distance_to_opponent_goal(d) as f64 * 10.0 + 2.0)) != 0", "random(ftol(exe::match_player_distance_to_opponent_goal(d) as f64 * 10.0 + 3.0)) != 0", "contest foul by the winner"),
    ('t_pendente', "            m.set_pending_play_x((x as u8) as i8);\n            m.set_pending_play_offender(MatchPlayer(who));", "            m.set_pending_play_x((y as u8) as i8);\n            m.set_pending_play_offender(MatchPlayer(who));", "contest pending foul"),
    ('t_penalti', "write_i16(f + match_player::MATCH_RATING, read_i16(f + match_player::MATCH_RATING).wrapping_sub(500));", "write_i16(f + match_player::MATCH_RATING, read_i16(f + match_player::MATCH_RATING).wrapping_sub(499));", "contest penalty"),
    ('t_lado', "let side_narrated = if modo == 1 { read_u8(v + match_player::SIDE) as u32  } else { zero(MatchPlayer(f)) };", "let side_narrated = if modo == 1 { zero(MatchPlayer(v)) } else { zero(MatchPlayer(f)) };", "contest punida modo 1"),
    ('t_extra', "let (alvo, lado_extra) = if modo == 1 { (v, zero(MatchPlayer(v))) } else { (f, (read_u8(f + match_player::SIDE) as u32)) };", "let (alvo, lado_extra) = if modo == 1 { (v, zero(MatchPlayer(v))) } else { (f, zero(MatchPlayer(f))) };", "contest punida modo 3"),
    ('t_defesa', "let strength = if l == 0 { nota_w + 0x11 } else { nota_w - nota_l };", "let strength = if l == 0 { nota_w + 0x11 } else { nota_w - nota_l + 1 };", "contest defence"),
    ('t_leitura', "write_i16(l + match_player::MATCH_RATING, read_i16(l + match_player::MATCH_RATING).wrapping_add(if exe::match_player_get_position(l) == 1 { -600 } else { -10 }));", "write_i16(l + match_player::MATCH_RATING, read_i16(l + match_player::MATCH_RATING).wrapping_add(if exe::match_player_get_position(l) == 1 { -600 } else { -11 }));", "contest leitura 1f5d"),
    ('t_domina', "write_i16(wv + match_player::BALL_CONTROL, (nota_w - nota_l - 225) as i16);", "write_i16(wv + match_player::BALL_CONTROL, (nota_w - nota_l - 224) as i16);"),
]
