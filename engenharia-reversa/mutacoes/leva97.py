# Mutacoes da leva97 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva97.py --jogo 2001-08-20
MUTACOES = [
    ('d_desliga', "            if !(r2 as f64 + v4d as f64 * read_f64_at(0x96d818) > r1 as f64) {", "            if !(r2 as f64 + v4d as f64 * read_f64_at(0x96d818) > r1 as f64 + 1.0) {"),
    ('d_marcados', "                if prod <= sortear(0x384) + 0x32 {", "                if prod <= sortear(0x384) + 0x31 {"),
    ('d_cansa', "            *w(0x35) = (*w(0x35)).wrapping_add(0xfff1);", "            *w(0x35) = (*w(0x35)).wrapping_add(0xfff2);"),
    ('d_folego', "        let r = sortear(5) + 5;\n        let f = (j + match_player::STAMINA) as *mut f32;", "        let r = sortear(5) + 6;\n        let f = (j + match_player::STAMINA) as *mut f32;"),
    ('d_situacao', "== 5 && ta() & 1 == 0 {", "== 4 && ta() & 1 == 0 {"),
    ('d_repete', "        if r <= sortear(5) {\n            return;", "        if r <= sortear(6) {\n            return;"),
    ('d_faixa', "(band((dir == 0) as u8, byte(m, 0x8ea7), byte(m, 0x8ea8)) != 0 && byte(m, 0x8ea9) != -1)", "(band((dir == 0) as u8, byte(m, 0x8ea7), byte(m, 0x8ea8)) != 0 || byte(m, 0x8ea9) != -1)"),
    ('d_2b', "        if b(0x2b) != 0 && !and_owner {", "        if b(0x2b) > 1 && !and_owner {"),
]
