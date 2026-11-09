# Mutacoes da leva86 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva86.py --jogo 2001-08-20
# (a 0x6eecb0 roda para cada jogador em cada partida; o autoteste sintetico nao cobre o contexto dela)
MUTACOES = [
    ('d_bl', "    let bl: i8 = if (read_i8((sheet()) + 0xf) as i32) > 15 { 20 }", "    let bl: i8 = if (read_i8((sheet()) + 0xf) as i32) > 14 { 20 }"),
    ('d_15', "        soma37(ftol(x * q(0x96d740)) as i16 as i32);", "        soma37(ftol(x * q(0x969530)) as i16 as i32);"),
    ('d_b75', "        let tres_quartos = |v: i32| soma37(ftol(v as f64 * q(0x969530)) as i16 as i32);", "        let tres_quartos = |v: i32| soma37(ftol(v as f64 * q(0x96d740)) as i16 as i32);"),
    ('d_meio', "    let middle = |v: i32| soma37(v / 2 - 10);", "    let middle = |v: i32| soma37(v / 2 - 9);"),
    ('d_ed', "    *p37 = ftol((q(0x96d730) - (read_f32(this + match_player::VERSATILITY) as f64) * q(0x96d738)) * *p37 as f64) as i16;", "    *p37 = ftol((q(0x96d730) - (read_f32(this + match_player::THROW_INS) as f64) * q(0x96d738)) * *p37 as f64) as i16;"),
    ('d_10e', "    byte_em(0x10e, min2((read_f32(this + match_player::PACE) as f64) + f(0x96d228), f(0x9689a8)));", "    byte_em(0x10e, min2((read_f32(this + match_player::PACE) as f64) + f(0x9689a8), f(0x9689a8)));"),
    ('d_113', "    byte_em(0x113, min2(game_pow(read_i8(this + match_player::MATCH_FORM) as f64 + (read_f32(this + match_player::FLAIR) as f64) - f(0x968a78), q(0x968950)) * q(0x967888), q(0x968950)));", "    byte_em(0x113, min2(game_pow(read_i8(this + match_player::MATCH_FORM) as f64 + (read_f32(this + match_player::FLAIR) as f64) - f(0x968a78), q(0x968950)) * q(0x967880), q(0x968950)));"),
    ('d_11a', "    let v = (blq + t(match_player::HANDLING) + t(match_player::POSITIONING) + t(match_player::JUMPING) + t(match_player::DECISIONS) + t(match_player::ANTICIPATION)) / 6;", "    let v = (blq + t(match_player::HANDLING) + t(match_player::POSITIONING) + t(match_player::JUMPING) + t(match_player::DECISIONS) + t(match_player::ANTICIPATION)) / 5;"),
    ('d_bq', "    let bq = ftol(((e5q + e5q) + q(0x967890)) * q(0x968968)) as i16 as i32;", "    let bq = ftol((e5q + q(0x967890)) * q(0x968968)) as i16 as i32;"),
    ('d_15c', "    word(0x15c, (t81 + t7d + c) / 4);", "    word(0x15c, (t81 + t7d + c) / 3);"),
    ('d_caso2', "            2 => ((w37f * 3) as f64 + f(0x96d710)) + w * q(0x96d708),", "            2 => ((w37f * 3) as f64 + f(0x96d710)) + w * q(0x96a6e8),"),
    ('d_casos', "        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 3,", "        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 4,"),
]
