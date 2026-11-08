# Mutacoes da leva91 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva91.py --jogo 2001-08-20
MUTACOES = [
    ('c_alvo', "    let alvo: i8 = if gol == 1 { 11 } else { 0 };", "    let alvo: i8 = if gol == 1 { 10 } else { 0 };"),
    ('c_moeda', "        for lado in [moeda, (moeda == 0) as i8] {", "        for lado in [(moeda == 0) as i8, moeda] {"),
    ('c_dist', "        let v = grid_distance_or_1000(cx as i32 - x as i32, cy as i32 - y as i32).to_bits();", "        let v = grid_distance_or_1000(cx as i32 - x as i32, cy as i32 - y as i32 + 1).to_bits();"),
    ('c_erro', "            if erro >= dx {\n                erro = erro.wrapping_sub(dx);\n                cy = cy.wrapping_add(step_towards);", "            if erro > dx {\n                erro = erro.wrapping_sub(dx);\n                cy = cy.wrapping_add(step_towards);"),
    ('c_ponta', "            if (sobe && cx > 4) || (!sobe && cx < 4) {", "            if (sobe && cx >= 4) || (!sobe && cx < 4) {"),
    ('c_ingreme', "        if (sobe && cy > alvo) || (!sobe && cy < alvo) {", "        if (sobe && cy > alvo) || (!sobe && cy <= alvo) {"),
    ('c_f5ae', "    *((this + 0xf5ae) as *mut usize) = 0;\n    let alvo", "    let alvo"),
]
