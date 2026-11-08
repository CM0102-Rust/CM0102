# Mutacoes da leva142 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva142.py --carreira 2002-06-01 --confirmar 2001-10-01
# d_: 0x70c040 (desfecho do shot); c_: 0x70a910 (shot a gol). O codigo das duas e parecido: as ancoras
# levam o nome da narracao ou do ramo de cada uma.
MUTACOES = [
    ('d_qualidade', "    let qualidade = match_pl_ftol(r2.wrapping_sub(r1).wrapping_mul(fator as i32 + 1) as f64 / f as f64);\n    let g = hold_position_one(m, lado);", "    let qualidade = match_pl_ftol(r2.wrapping_sub(r1).wrapping_mul(fator as i32 + 2) as f64 / f as f64);\n    let g = hold_position_one(m, lado);"),
    ('d_parte', "    if parte > 0x47 {\n        shot_outcome_goal(", "    if parte > 0x46 {\n        shot_outcome_goal(", "desfecho gol"),
    ('d_escanteio', "        crate::comparacao::ramo(\"desfecho corner\");\n        destaque(chamar, 0x2111, 0x210f, 0x1fe6);\n        somar(g, 0x8c);", "        crate::comparacao::ramo(\"desfecho corner\");\n        destaque(chamar, 0x2111, 0x210f, 0x1fe6);\n        somar(g, 0x8d);", "desfecho corner"),
    ('d_linha', "        *bm(0x8eae) = outro;\n        *((m + 0xf582) as *mut u32) = 0;\n        narrar(chamar, 0x211b, byte(p, 0x2e) as u8 as u32, lado as u8, 0);\n        *bm(0x8eb2) = 3;", "        *bm(0x8eae) = outro;\n        *((m + 0xf582) as *mut u32) = 0;\n        narrar(chamar, 0x211b, byte(p, 0x2e) as u8 as u32, lado as u8, 0);\n        *bm(0x8eb2) = 4;", "desfecho linha de fundo"),
    ('d_gol_placar', "    let gols = bm(0xf5bd + outro as usize * 0x36);", "    let gols = bm(0xf5bd + lado as usize * 0x36);", "desfecho gol"),
    ('d_fator', "    apply_to_team(m, p, f32::from_bits(0x3fd9999a));", "    apply_to_team(m, p, f32::from_bits(0x3fd9999b));", "desfecho gol"),
    ('c_qualidade', "    let qualidade = match_pl_ftol(r2.wrapping_sub(r1).wrapping_mul(fator as i32 + 1) as f64 / f as f64);\n    let g = hold_position_one(m, outro as i8);", "    let qualidade = match_pl_ftol(r2.wrapping_sub(r1).wrapping_mul(fator as i32 + 2) as f64 / f as f64);\n    let g = hold_position_one(m, outro as i8);"),
    ('c_alvo', "        mais(p, 0xa);\n        if tipo == 1 {\n            mais(p, 0xb);", "        mais(p, 0xa);\n        if tipo == 2 {\n            mais(p, 0xb);"),
    ('c_fora', "            narrar(chamar, 0x1fc6 + (tipo as u32 - 8), p2e, p27, 0);", "            narrar(chamar, 0x1fc6 + (tipo as u32 - 7), p2e, p27, 0);", "shot a gol para fora"),
    ('c_gol_atuacao', "        mais(p, 0xc);\n        somar(p, 0x89);", "        mais(p, 0xc);\n        somar(p, 0x8a);", "shot a gol gol"),
    ('c_contagens', "        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {", "        for o in [0x3d, 0x3f, 0x41, 0x43] {", "shot a gol gol"),
    ('c_hat_trick', "    if byte(p, 0xc) == 3 {\n        crate::comparacao::ramo(\"shot a gol hat-trick\");", "    if byte(p, 0xc) == 4 {\n        crate::comparacao::ramo(\"shot a gol hat-trick\");", "shot a gol hat-trick"),
    ('c_marca', "    } else if c(0x45) == 1 || redondo(c(0x45), 5) {", "    } else if c(0x45) == 1 || redondo(c(0x45), 6) {", "shot a gol marca"),
    ('c_marca_a7', "        narrar(chamar, codigo, p2e, p27, n);", "        narrar(chamar, codigo, p2e, p27, n.wrapping_add(1));", "shot a gol marca"),
    ('c_fator', "    apply_to_team(m, p, f32::from_bits(0x3dcccccd));", "    apply_to_team(m, p, f32::from_bits(0x3e4ccccd));", "shot a gol gol"),
    ('c_tardio', "            let bl = if b < a { byte(m, (4 + lado as isize) as usize) } else { -1 };", "            let bl = if b <= a { byte(m, (4 + lado as isize) as usize) } else { -1 };", "shot a gol empate tardio"),
    ('c_escanteio', "            crate::comparacao::ramo(\"shot a gol corner\");\n            destaque(chamar, 0x2111, 0x210f, 0x1fe6);\n            narrar(chamar, 0x211c, p2e, p27, 0);\n            somar(g, 0x8c);", "            crate::comparacao::ramo(\"shot a gol corner\");\n            destaque(chamar, 0x2111, 0x210f, 0x1fe6);\n            narrar(chamar, 0x211c, p2e, p27, 0);\n            somar(g, 0x8b);", "shot a gol corner"),
    ('c_defesa', "            crate::comparacao::ramo(\"shot a gol defesa\");\n            let r = sortear(3);\n            somar(g, 0xc8);", "            crate::comparacao::ramo(\"shot a gol defesa\");\n            let r = sortear(3);\n            somar(g, 0xc9);", "shot a gol defesa"),
    ('c_defesa_media', "            crate::comparacao::ramo(\"shot a gol defesa media\");\n            somar(g, 0x15e);", "            crate::comparacao::ramo(\"shot a gol defesa media\");\n            somar(g, 0x15f);", "shot a gol defesa media"),
    ('c_tecnico', "        let t = u(u(pt, if lado == 0 { 0x1c } else { 0x20 }), 0x53);", "        let t = u(u(pt, if lado == 0 { 0x20 } else { 0x1c }), 0x53);", "shot a gol gol"),
]
