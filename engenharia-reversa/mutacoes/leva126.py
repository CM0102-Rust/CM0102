# Mutacoes da leva126 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva126.py --jogo 2001-10-01
MUTACOES = [
    ('d_folego', "let folego = random(d.wrapping_mul(d) / 50000);\n    let (mut a", "let folego = random(d.wrapping_mul(d) / 40000);\n    let (mut a"),
    ('d_ataque', "let r = random(m.ball_height() as i32 * 50);", "let r = random(m.ball_height() as i32 * 40);"),
    ('d_limpa', "if !(nota < at && random(5) != 0) {", "if !(nota < at && random(4) != 0) {"),
    ('d_registra', "if random(0x1e) == 0 && arriscado == 0 {\n                    crate::comparison::branch(\"tackle only records", "if random(0x1d) == 0 && arriscado == 0 {\n                    crate::comparison::branch(\"tackle only records"),
    ('d_cansa', "                nb(call, event::TEAM_ARGUES_WITH_REFEREE, zero);\n            }\n        }\n        apply_to_team(m.addr(), e.addr(), weight);\n        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, e.addr(), [0; 5]);\n    }\n    e.set_match_rating((e.match_rating()).wrapping_sub(0x19));", "                nb(call, event::TEAM_ARGUES_WITH_REFEREE, zero);\n            }\n        }\n        apply_to_team(m.addr(), e.addr(), weight);\n        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, e.addr(), [0; 5]);\n    }\n    e.set_match_rating((e.match_rating()).wrapping_sub(0x18));"),
    ('d_limpa_folego', "{ 0x3b } else { 0x46 }", "{ 0x3b } else { 0x45 }"),
    ('d_nota', "rating.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200)", "rating.wrapping_sub(at).wrapping_sub(199).clamp(-200, 200)"),
    ('d_chao', "tackle ball breaks loose\");\n            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m.addr(), [((j.cell_x() as u8) as u32), ((j.cell_y() as u8) as u32), 1 | zero(j) << 8, 0, 0]);\n            if m.restart_type() == restart::OPEN_PLAY {\n                let mut k = 3;", "tackle ball breaks loose\");\n            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m.addr(), [((j.cell_x() as u8) as u32), ((j.cell_y() as u8) as u32), 1 | zero(j) << 8, 0, 0]);\n            if m.restart_type() == restart::OPEN_PLAY {\n                let mut k = 2;"),
    ('d_ganho', "let gain: i16 = if controlled { 0x78 } else { 0xc8 };\n        e.set_match_rating((e.match_rating())", "let gain: i16 = if controlled { 0x78 } else { 0xc7 };\n        e.set_match_rating((e.match_rating())"),
    ('d_passa', "j.set_ball_control((j.ball_control()).wrapping_add(at.wrapping_sub(nota)).clamp(-200, 200));", "j.set_ball_control((j.ball_control()).wrapping_add(at.wrapping_sub(nota)).clamp(-199, 200));"),
    ('d_cresce', "tackle grows\");\n        let p = j.confidence() as f64 * 1.0499999523162842;", "tackle grows\");\n        let p = j.confidence() as f64 * 1.06;"),
    ('d_zera', "    *b(m, 0x8ea9) = 0;\n}\n\n/// Despacho real", "    *b(m, 0x8ea9) = 1;\n}\n\n/// Despacho real"),
]
