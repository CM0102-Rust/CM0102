# Mutacoes da leva126 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva126.py --jogo 2001-10-01
MUTACOES = [
    ('d_folego', "let folego = random(d.wrapping_mul(d) / 50000);\n    let (mut a", "let folego = random(d.wrapping_mul(d) / 40000);\n    let (mut a"),
    ('d_ataque', "let r = random(byte(m, engine::BALL_HEIGHT) as i32 * 50);", "let r = random(byte(m, engine::BALL_HEIGHT) as i32 * 40);"),
    ('d_limpa', "if !(nota < at && random(5) != 0) {", "if !(nota < at && random(4) != 0) {"),
    ('d_registra', "if random(0x1e) == 0 && arriscado == 0 {\n                    crate::comparison::branch(\"tackle so registra", "if random(0x1d) == 0 && arriscado == 0 {\n                    crate::comparison::branch(\"tackle so registra"),
    ('d_cansa', "*w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(0x19);", "*w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(0x18);"),
    ('d_limpa_folego', "{ 0x3b } else { 0x46 }", "{ 0x3b } else { 0x45 }"),
    ('d_nota', "rating.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200)", "rating.wrapping_sub(at).wrapping_sub(199).clamp(-200, 200)"),
    ('d_chao', "tackle bola espirra\");\n            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [ub(j, match_player::CELL_X), ub(j, match_player::CELL_Y), 1 | zero(j) << 8, 0, 0]);\n            if byte(m, engine::RESTART_TYPE) == 0 {\n                let mut k = 3;", "tackle bola espirra\");\n            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [ub(j, match_player::CELL_X), ub(j, match_player::CELL_Y), 1 | zero(j) << 8, 0, 0]);\n            if byte(m, engine::RESTART_TYPE) == 0 {\n                let mut k = 2;"),
    ('d_ganho', "let gain: i16 = if dominou { 0x78 } else { 0xc8 };\n        *w(e, match_player::MATCH_RATING)", "let gain: i16 = if dominou { 0x78 } else { 0xc7 };\n        *w(e, match_player::MATCH_RATING)"),
    ('d_passa', "*w(j, match_player::BALL_CONTROL) = (*w(j, match_player::BALL_CONTROL)).wrapping_add(at.wrapping_sub(nota)).clamp(-200, 200);", "*w(j, match_player::BALL_CONTROL) = (*w(j, match_player::BALL_CONTROL)).wrapping_add(at.wrapping_sub(nota)).clamp(-199, 200);"),
    ('d_cresce', "tackle cresce\");\n        let p = read_f32_at(j + match_player::CONFIDENCE) as f64 * 1.0499999523162842;", "tackle cresce\");\n        let p = read_f32_at(j + match_player::CONFIDENCE) as f64 * 1.06;"),
    ('d_zera', "    *b(m, 0x8ea9) = 0;\n}\n\n/// Despacho real", "    *b(m, 0x8ea9) = 1;\n}\n\n/// Despacho real"),
]
