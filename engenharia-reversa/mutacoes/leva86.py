# Mutacoes da leva86 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva86.py --jogo 2001-08-20
# (a 0x6eecb0 roda para cada jogador em cada partida; o autoteste sintetico nao cobre o contexto dela)
MUTACOES = [
    ('d_bl', "    let bl: i8 = if b(ficha(), 0xf) > 15 { 20 }", "    let bl: i8 = if b(ficha(), 0xf) > 14 { 20 }"),
    ('d_15', "        soma37(match_pl_ftol(x * q(0x96d740)) as i16 as i32);", "        soma37(match_pl_ftol(x * q(0x969530)) as i16 as i32);"),
    ('d_b75', "        let tres_quartos = |v: i32| soma37(match_pl_ftol(v as f64 * q(0x969530)) as i16 as i32);", "        let tres_quartos = |v: i32| soma37(match_pl_ftol(v as f64 * q(0x96d740)) as i16 as i32);"),
    ('d_meio', "    let meio = |v: i32| soma37(v / 2 - 10);", "    let meio = |v: i32| soma37(v / 2 - 9);"),
    ('d_ed', "    *p37 = match_pl_ftol((q(0x96d730) - rd(0xed) * q(0x96d738)) * *p37 as f64) as i16;", "    *p37 = match_pl_ftol((q(0x96d730) - rd(0xe9) * q(0x96d738)) * *p37 as f64) as i16;"),
    ('d_10e', "    byte_em(0x10e, min2(rd(0xc1) + f(0x96d228), f(0x9689a8)));", "    byte_em(0x10e, min2(rd(0xc1) + f(0x9689a8), f(0x9689a8)));"),
    ('d_113', "    byte_em(0x113, min2(game_pow(byte(this, 0x17) as f64 + rd(0x9d) - f(0x968a78), q(0x968950)) * q(0x967888), q(0x968950)));", "    byte_em(0x113, min2(game_pow(byte(this, 0x17) as f64 + rd(0x9d) - f(0x968a78), q(0x968950)) * q(0x967880), q(0x968950)));"),
    ('d_11a', "    let v = (blq + t(0xa5) + t(0xcd) + t(0xad) + t(0x91) + t(0x7d)) / 6;", "    let v = (blq + t(0xa5) + t(0xcd) + t(0xad) + t(0x91) + t(0x7d)) / 5;"),
    ('d_bq', "    let bq = match_pl_ftol(((e5q + e5q) + q(0x967890)) * q(0x968968)) as i16 as i32;", "    let bq = match_pl_ftol((e5q + q(0x967890)) * q(0x968968)) as i16 as i32;"),
    ('d_15c', "    palavra(0x15c, (t81 + t7d + c) / 4);", "    palavra(0x15c, (t81 + t7d + c) / 3);"),
    ('d_caso2', "            2 => ((w37f * 3) as f64 + f(0x96d710)) + w * q(0x96d708),", "            2 => ((w37f * 3) as f64 + f(0x96d710)) + w * q(0x96a6e8),"),
    ('d_casos', "        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 3,", "        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 4,"),
]
