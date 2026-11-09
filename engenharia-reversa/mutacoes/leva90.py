# Mutacoes da leva90 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva90.py --jogo 2001-08-20
MUTACOES = [
    ('j_2b', "        let p = MatchPlayer(*list.add(k as usize));\n        if p.busy() <= 1 && p.position() >= 0 {", "        let p = MatchPlayer(*list.add(k as usize));\n        if p.busy() < 1 && p.position() >= 0 {"),
    ('j_19', "        let p = MatchPlayer(*list.add(k as usize));\n        if p.busy() <= 1 && p.position() >= 0 {", "        let p = MatchPlayer(*list.add(k as usize));\n        if p.busy() <= 1 && p.position() > 0 {"),
    ('j_args', "match play(p.addr(), [a4, a3, a6, a7]) as i8 {", "match play(p.addr(), [a3, a4, a6, a7]) as i8 {"),
    ('j_12', "p.set_balls_contested((p.balls_contested()).wrapping_add(1));", "p.set_balls_contested((p.balls_contested()).wrapping_add(2));"),
    ('j_ret2', "                        *((this + engine::SHOT_BLOCKER) as *mut usize) = p.addr();\n                        return 2;", "                        *((this + engine::SHOT_BLOCKER) as *mut usize) = p.addr();\n                        return 1;"),
    ('j_modo', "            if mode == 2 {\n                n = n.wrapping_add(1);", "            if mode == 3 {\n                n = n.wrapping_add(1);"),
    ('t_fim', "        while y < fim || (until_end && y == fim) {", "        while y < fim {"),
    ('t_11', "true, 11, &|x, y, v|", "true, 8, &|x, y, v|"),
    ('t_lado', "    let side = (read_i8(this + engine::POSSESSION_SIDE) == 0) as i8;\n    walk_line_cells", "    let side = (read_i8(this + engine::POSSESSION_SIDE) != 0) as i8;\n    walk_line_cells"),
]
