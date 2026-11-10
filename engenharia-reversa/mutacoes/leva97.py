# Mutacoes da leva97 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva97.py --jogo 2001-08-20
MUTACOES = [
    ('d_desliga', "            if !(r2 as f64 + v4d as f64 * read_f64(rdata::F64_0_0078125) > r1 as f64) {", "            if !(r2 as f64 + v4d as f64 * read_f64(rdata::F64_0_0078125) > r1 as f64 + 1.0) {"),
    ('d_marcados', "                if prod <= random(0x384) + 0x32 {", "                if prod <= random(0x384) + 0x31 {"),
    ('d_cansa', "            *w(match_player::MATCH_RATING) = (*w(match_player::MATCH_RATING)).wrapping_add(0xfff1);", "            *w(match_player::MATCH_RATING) = (*w(match_player::MATCH_RATING)).wrapping_add(0xfff2);"),
    ('d_folego', "        let r = random(5) + 5;\n        let f = (j.addr() + match_player::STAMINA) as *mut f32;", "        let r = random(5) + 6;\n        let f = (j.addr() + match_player::STAMINA) as *mut f32;"),
    ('d_situacao', "== 5 && ta() & 1 == 0 {", "== 4 && ta() & 1 == 0 {"),
    ('d_repete', "        if r <= random(5) {\n            return;", "        if r <= random(6) {\n            return;"),
    ('d_faixa', "(exe::is_in_own_penalty_area((dir == 0) as u8, m.ball_x(), m.ball_y()) != 0 && m.ball_height() != -1)", "(exe::is_in_own_penalty_area((dir == 0) as u8, m.ball_x(), m.ball_y()) != 0 || m.ball_height() != -1)"),
    ('d_2b', "        if b(0x2b) != 0 && !and_owner {", "        if b(0x2b) > 1 && !and_owner {"),
]
