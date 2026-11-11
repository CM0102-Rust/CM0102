# Mutacoes da leva85 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva85.py --jogo 2001-08-20
# (a 0x6ec010 roda para cada jogador em cada partida; o autoteste sintetico nao cobre o contexto dela)
MUTACOES = [
    ('a_nan', "        if !(v >= read_f64(rdata::F64_0)) {", "        if v < read_f64(rdata::F64_0) {"),
    ('a_idade', "        if age < 18 {", "        if age < 17 {"),
    ('a_f5', "    wr(match_player::WORK_RATE, (this.frustration() as f64 + v44) - l14 as f64 / h + l24 as f64 * read_f64(rdata::F64_0_5));", "    wr(match_player::WORK_RATE, (this.frustration() as f64 + v44) - l14 as f64 / h + l24 as f64 * read_f64(rdata::F64_0_1));"),
    ('a_7d', "    wr(match_player::ANTICIPATION, (this.anticipation() as f64) * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_5));", "    wr(match_player::ANTICIPATION, (this.anticipation() as f64) * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_10));"),
    ('a_150', "    let maior150 = *p3b > 150;", "    let maior150 = *p3b > 160;"),
    ('a_pos', "            } else if x & 0x40 != 0 {\n                5", "            } else if x & 0x40 != 0 {\n                4"),
    ('a_l18', "            l1c = (l18 as f64 * read_f64(rdata::F64_1_025)) as f32;", "            l1c = (l1c as f64 * read_f64(rdata::F64_1_025)) as f32;"),
    ('a_cl', "            cl = cl.min(18);", "            cl = cl.min(17);"),
    ('a_sorte50', "if c45() & 0x40000 != 0 && (p.pressure() as i32) + l12f < random(50) {", "if c45() & 0x40000 != 0 && (p.pressure() as i32) + l12f < random(40) {"),
    ('a_gosta', "    if exe::staff_favours_club(person(), club_side()) != 0 {\n        wr(match_player::WORK_RATE, (this.work_rate() as f64) + (read_f32(rdata::F32_5) as f64));", "    if exe::staff_favours_club(person(), club_side()) != 0 {\n        wr(match_player::WORK_RATE, (this.work_rate() as f64) + (read_f32(rdata::F32_2_5) as f64));"),
    ('a_hist', "                let r = e as f64 * (read_f32(rdata::F32_40) as f64) / c0c as f64;", "                let r = e as f64 * (read_f32(rdata::F32_7_5) as f64) / c0c as f64;"),
    ('a_3b', "    *p3b = ftol(*p3b as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_80)) as i16;", "    *p3b = ftol(*p3b as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_15)) as i16;"),
]
