# Mutacoes da leva142 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva142.py --carreira 2002-06-01 --confirmar 2001-10-01
# d_: 0x70c040 (desfecho do shot); c_: 0x70a910 (shot a gol). O codigo das duas e parecido: as ancoras
# levam o nome da narracao ou do ramo de cada uma.
MUTACOES = [
    ('d_qualidade', "    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(factor as i32 + 1) as f64 / f as f64);\n    let g = hold_position_one(m, side);", "    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(factor as i32 + 2) as f64 / f as f64);\n    let g = hold_position_one(m, side);"),
    ('d_parte', "    if part > 0x47 {\n        shot_outcome_goal(", "    if part > 0x46 {\n        shot_outcome_goal(", "outcome goal"),
    ('d_escanteio', "        crate::comparison::branch(\"outcome corner\");\n        highlight(call, 0x2111, 0x210f, 0x1fe6);\n        add(g, 0x8c);", "        crate::comparison::branch(\"outcome corner\");\n        highlight(call, 0x2111, 0x210f, 0x1fe6);\n        add(g, 0x8d);", "outcome corner"),
    ('d_linha', "        *bm(engine::POSSESSION_SIDE) = other;\n        *((m + engine::BALL_HOLDER) as *mut u32) = 0;\n        narrate(call, 0x211b, byte(p, match_player::COMMENTARY_ID) as u8 as u32, side as u8, 0);\n        *bm(engine::RESTART_TYPE) = 3;", "        *bm(engine::POSSESSION_SIDE) = other;\n        *((m + engine::BALL_HOLDER) as *mut u32) = 0;\n        narrate(call, 0x211b, byte(p, match_player::COMMENTARY_ID) as u8 as u32, side as u8, 0);\n        *bm(engine::RESTART_TYPE) = 4;", "outcome byline"),
    ('d_gol_placar', "    let goals = bm(0xf5bd + other as usize * 0x36);", "    let goals = bm(0xf5bd + side as usize * 0x36);", "outcome goal"),
    ('d_fator', "    apply_to_team(m, p, f32::from_bits(0x3fd9999a));", "    apply_to_team(m, p, f32::from_bits(0x3fd9999b));", "outcome goal"),
    ('c_qualidade', "    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(factor as i32 + 1) as f64 / f as f64);\n    let g = hold_position_one(m, other as i8);", "    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(factor as i32 + 2) as f64 / f as f64);\n    let g = hold_position_one(m, other as i8);"),
    ('c_alvo', "        more(p, 0xa);\n        if kind == 1 {\n            more(p, 0xb);", "        more(p, 0xa);\n        if kind == 2 {\n            more(p, 0xb);"),
    ('c_fora', "            narrate(call, 0x1fc6 + (kind as u32 - 8), p2e, p27, 0);", "            narrate(call, 0x1fc6 + (kind as u32 - 7), p2e, p27, 0);", "shot on target wide"),
    ('c_gol_atuacao', "        more(p, 0xc);\n        add(p, 0x89);", "        more(p, 0xc);\n        add(p, 0x8a);", "shot on target goal"),
    ('c_contagens', "        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {", "        for o in [0x3d, 0x3f, 0x41, 0x43] {", "shot on target goal"),
    ('c_hat_trick', "    if byte(p, match_player::GOALS) == 3 {\n        crate::comparison::branch(\"shot on target hat-trick\");", "    if byte(p, match_player::GOALS) == 4 {\n        crate::comparison::branch(\"shot on target hat-trick\");", "shot on target hat-trick"),
    ('c_marca', "    } else if c(0x45) == 1 || redondo(c(0x45), 5) {", "    } else if c(0x45) == 1 || redondo(c(0x45), 6) {", "shot on target mark"),
    ('c_marca_a7', "        narrate(call, code, p2e, p27, n);", "        narrate(call, code, p2e, p27, n.wrapping_add(1));", "shot on target mark"),
    ('c_fator', "    apply_to_team(m, p, f32::from_bits(0x3dcccccd));", "    apply_to_team(m, p, f32::from_bits(0x3e4ccccd));", "shot on target goal"),
    ('c_tardio', "            let bl = if b < a { byte(m, (4 + side as isize) as usize) } else { -1 };", "            let bl = if b <= a { byte(m, (4 + side as isize) as usize) } else { -1 };", "shot on target late equaliser"),
    ('c_escanteio', "            crate::comparison::branch(\"shot on target corner\");\n            highlight(call, 0x2111, 0x210f, 0x1fe6);\n            narrate(call, 0x211c, p2e, p27, 0);\n            add(g, 0x8c);", "            crate::comparison::branch(\"shot on target corner\");\n            highlight(call, 0x2111, 0x210f, 0x1fe6);\n            narrate(call, 0x211c, p2e, p27, 0);\n            add(g, 0x8b);", "shot on target corner"),
    ('c_defesa', "            crate::comparison::branch(\"shot on target save\");\n            let r = random(3);\n            add(g, 0xc8);", "            crate::comparison::branch(\"shot on target save\");\n            let r = random(3);\n            add(g, 0xc9);", "shot on target save"),
    ('c_defesa_media', "            crate::comparison::branch(\"shot on target medium save\");\n            add(g, 0x15e);", "            crate::comparison::branch(\"shot on target medium save\");\n            add(g, 0x15f);", "shot on target medium save"),
    ('c_tecnico', "        let t = u(u(pt, if side == 0 { 0x1c } else { 0x20 }), 0x53);", "        let t = u(u(pt, if side == 0 { 0x20 } else { 0x1c }), 0x53);", "shot on target goal"),
]
