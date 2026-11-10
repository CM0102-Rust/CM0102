# Mutacoes da leva86 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva86.py --jogo 2001-08-20
# (a 0x6eecb0 roda para cada jogador em cada partida; o autoteste sintetico nao cobre o contexto dela)
MUTACOES = [
    ('d_bl', "    let bl: i8 = if (read_i8(sheet().addr() + 0xf) as i32) > 15 { 20 }", "    let bl: i8 = if (read_i8(sheet().addr() + 0xf) as i32) > 14 { 20 }"),
    ('d_15', "        soma37(ftol(x * read_f64(rdata::F64_NEG_1_5)) as i16 as i32);", "        soma37(ftol(x * read_f64(rdata::F64_NEG_0_75)) as i16 as i32);"),
    ('d_b75', "        let tres_quartos = |v: i32| soma37(ftol(v as f64 * read_f64(rdata::F64_NEG_0_75)) as i16 as i32);", "        let tres_quartos = |v: i32| soma37(ftol(v as f64 * read_f64(rdata::F64_NEG_1_5)) as i16 as i32);"),
    ('d_meio', "    let middle = |v: i32| soma37(v / 2 - 10);", "    let middle = |v: i32| soma37(v / 2 - 9);"),
    ('d_ed', "    *p37 = ftol((read_f64(rdata::F64_1_079) - (this.versatility() as f64) * read_f64(rdata::F64_0_0395)) * *p37 as f64) as i16;", "    *p37 = ftol((read_f64(rdata::F64_1_079) - (this.throw_ins() as f64) * read_f64(rdata::F64_0_0395)) * *p37 as f64) as i16;"),
    ('d_10e', "    byte_em(0x10e, min2((this.pace() as f64) + (read_f32(rdata::F32_4) as f64), (read_f32(rdata::F32_2) as f64)));", "    byte_em(0x10e, min2((this.pace() as f64) + (read_f32(rdata::F32_2) as f64), (read_f32(rdata::F32_2) as f64)));"),
    ('d_113', "    byte_em(0x113, min2(game_pow(this.match_form() as f64 + (this.flair() as f64) - (read_f32(rdata::F32_10) as f64), read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_05), read_f64(rdata::F64_2)));", "    byte_em(0x113, min2(game_pow(this.match_form() as f64 + (this.flair() as f64) - (read_f32(rdata::F32_10) as f64), read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_1), read_f64(rdata::F64_2)));"),
    ('d_11a', "    let v = (blq + t(match_player::HANDLING) + t(match_player::POSITIONING) + t(match_player::JUMPING) + t(match_player::DECISIONS) + t(match_player::ANTICIPATION)) / 6;", "    let v = (blq + t(match_player::HANDLING) + t(match_player::POSITIONING) + t(match_player::JUMPING) + t(match_player::DECISIONS) + t(match_player::ANTICIPATION)) / 5;"),
    ('d_bq', "    let bq = ftol(((e5q + e5q) + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5)) as i16 as i32;", "    let bq = ftol((e5q + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5)) as i16 as i32;"),
    ('d_15c', "    word(0x15c, (t81 + t7d + c) / 4);", "    word(0x15c, (t81 + t7d + c) / 3);"),
    ('d_caso2', "            2 => ((w37f * 3) as f64 + (read_f32(rdata::F32_418) as f64)) + w * read_f64(rdata::F64_0_13),", "            2 => ((w37f * 3) as f64 + (read_f32(rdata::F32_418) as f64)) + w * read_f64(rdata::F64_0_775),"),
    ('d_casos', "        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 3,", "        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 4,"),
]
