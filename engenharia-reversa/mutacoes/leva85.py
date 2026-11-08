# Mutacoes da leva85 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva85.py --jogo 2001-08-20
# (a 0x6ec010 roda para cada jogador em cada partida; o autoteste sintetico nao cobre o contexto dela)
MUTACOES = [
    ('a_nan', "        if !(v >= q(0x9689a0)) {", "        if v < q(0x9689a0) {"),
    ('a_idade', "        if age < 18 {", "        if age < 17 {"),
    ('a_f5', "    wr(0xf5, (byte(this, match_player::FRUSTRATION) as f64 + v44) - l14 as f64 / h + l24 as f64 * q(0x968968));", "    wr(0xf5, (byte(this, match_player::FRUSTRATION) as f64 + v44) - l14 as f64 / h + l24 as f64 * q(0x967880));"),
    ('a_7d', "    wr(0x7d, rd(match_player::ANTICIPATION) * q(0x968968) + q(0x9688a0));", "    wr(0x7d, rd(match_player::ANTICIPATION) * q(0x968968) + q(0x9688b0));"),
    ('a_150', "    let maior150 = *p3b > 150;", "    let maior150 = *p3b > 160;"),
    ('a_pos', "            } else if x & 0x40 != 0 {\n                5", "            } else if x & 0x40 != 0 {\n                4"),
    ('a_l18', "            l1c = (l18 as f64 * q(0x968fa0)) as f32;", "            l1c = (l1c as f64 * q(0x968fa0)) as f32;"),
    ('a_cl', "            cl = cl.min(18);", "            cl = cl.min(17);"),
    ('a_sorte50', "if c45() & 0x40000 != 0 && b(p, staff::PRESSURE) + l12f < random(50) {", "if c45() & 0x40000 != 0 && b(p, staff::PRESSURE) + l12f < random(40) {"),
    ('a_gosta', "    if gosta(person(), club_side()) != 0 {\n        wr(0xf5, rd(match_player::WORK_RATE) + f(0x968990));", "    if gosta(person(), club_side()) != 0 {\n        wr(0xf5, rd(match_player::WORK_RATE) + f(0x96b008));"),
    ('a_hist', "                let r = e as f64 * f(0x96d664) / c0c as f64;", "                let r = e as f64 * f(0x969078) / c0c as f64;"),
    ('a_3b', "    *p3b = match_pl_ftol(*p3b as f64 * q(0x968968) + q(0x96d6e0)) as i16;", "    *p3b = match_pl_ftol(*p3b as f64 * q(0x968968) + q(0x968e08)) as i16;"),
]
