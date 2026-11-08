# Mutacoes da leva127 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva127.py --jogo 2001-10-01
# O quarto campo e o ramo (marca ramo("...") no motor.rs): sem passagem na base, a mutacao e pulada.
MUTACOES = [
    ('t_fora', "if x < 0 || y < 0 || x >= 9 || y >= 12 {\n        crate::comparison::branch(\"contest fora do campo\");", "if x < 0 || y < 0 || x >= 8 || y >= 12 {\n        crate::comparison::branch(\"contest fora do campo\");", "contest fora do campo"),
    ('t_angulo', "            if d as i16 > 0xb4 {\n                d = 0x168 - d;", "            if d as i16 > 0xb3 {\n                d = 0x168 - d;"),
    ('t_vence', "if nota1 as i16 > nota0 as i16 {", "if nota1 as i16 >= nota0 as i16 {"),
    ('t_folego', "*w(wv, match_player::MATCH_RATING) = (*w(wv, match_player::MATCH_RATING)).wrapping_add(4);", "*w(wv, match_player::MATCH_RATING) = (*w(wv, match_player::MATCH_RATING)).wrapping_add(5);", "contest falta do vencedor"),
    ('t_agora', "sortear(match_pl_ftol(to_goal(d) as f64 * 10.0 + 2.0)) != 0", "sortear(match_pl_ftol(to_goal(d) as f64 * 10.0 + 3.0)) != 0", "contest falta do vencedor"),
    ('t_pendente', "            *b(m, engine::PENDING_PLAY_X) = x as u8;\n            *((m + engine::PENDING_PLAY_OFFENDER) as *mut usize) = who;", "            *b(m, engine::PENDING_PLAY_X) = y as u8;\n            *((m + engine::PENDING_PLAY_OFFENDER) as *mut usize) = who;", "contest falta pendente"),
    ('t_penalti', "*w(f, match_player::MATCH_RATING) = (*w(f, match_player::MATCH_RATING)).wrapping_sub(500);", "*w(f, match_player::MATCH_RATING) = (*w(f, match_player::MATCH_RATING)).wrapping_sub(499);", "contest penalty"),
    ('t_lado', "let lado_narrado = if modo == 1 { ub(v, match_player::SIDE) } else { zero(f) };", "let lado_narrado = if modo == 1 { zero(v) } else { zero(f) };", "contest punida modo 1"),
    ('t_extra', "let (alvo, lado_extra) = if modo == 1 { (v, zero(v)) } else { (f, ub(f, match_player::SIDE)) };", "let (alvo, lado_extra) = if modo == 1 { (v, zero(v)) } else { (f, zero(f)) };", "contest punida modo 3"),
    ('t_defesa', "let strength = if l == 0 { nota_w + 0x11 } else { nota_w - nota_l };", "let strength = if l == 0 { nota_w + 0x11 } else { nota_w - nota_l + 1 };", "contest defesa"),
    ('t_leitura', "*w(l, match_player::MATCH_RATING) = (*w(l, match_player::MATCH_RATING)).wrapping_add(if role(l) == 1 { -600 } else { -10 });", "*w(l, match_player::MATCH_RATING) = (*w(l, match_player::MATCH_RATING)).wrapping_add(if role(l) == 1 { -600 } else { -11 });", "contest leitura 1f5d"),
    ('t_domina', "*w(wv, match_player::BALL_CONTROL) = (nota_w - nota_l - 225) as i16;", "*w(wv, match_player::BALL_CONTROL) = (nota_w - nota_l - 224) as i16;"),
]
