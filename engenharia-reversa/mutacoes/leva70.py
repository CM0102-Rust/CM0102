# Mutacoes da leva70 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva70.py --jogo 2001-08-15
MUTACOES = [
    ('f_paciencia', "    *paciencia = (*paciencia).wrapping_add(0xaf);", "    *paciencia = (*paciencia).wrapping_add(0xae);"),
    ('f_vantagem', "    let div = if other.last_fouled_by() != j.addr() { 1.0 } else { 3.0 };", "    let div = if other.last_fouled_by() != j.addr() { 1.0 } else { 2.0 };"),
    ('f_sev', "        factor = if sev > 10 { sev as f64 * read_f64(rdata::F64_0_1) }", "        factor = if sev > 11 { sev as f64 * read_f64(rdata::F64_0_1) }"),
    ('f_modo', "        factor += mode / (read_i8(arb + official::PRESSURE) as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_3));", "        factor += mode / (read_i8(arb + official::PRESSURE) as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_1_5));"),
    ('f_rigor', "            5..=9 => -1,", "            5..=9 => 0,"),
    ('f_teto', "    } else if read_f64(rdata::F64_10) < factor || factor.is_nan() {\n        factor = 10.0;", "    } else if read_f64(rdata::F64_10) < factor || factor.is_nan() {\n        factor = 9.0;"),
    ('f_2a', "            if random(ftol(factor * read_f64(rdata::F64_6))) != 0 {\n                res = 1;\n                add(random(dureza) + 4);", "            if random(ftol(factor * read_f64(rdata::F64_6))) != 0 {\n                res = 1;\n                add(random(dureza) + 5);"),
    ('f_minuto', "\n    let minute = (m.clock() as i32 / 11) as i8;", "\n    let minute = (m.clock() as i32 / 12) as i8;"),
    ('f_aplica', "        exe::match_player_scale_anger(other, (*grav as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_1)) as f32);", "        exe::match_player_scale_anger(other, (*grav as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_1_5)) as f32);"),
    ('f_ficha', "            if c == 1 {\n                *grav = (*grav).wrapping_add(um as i8);", "            if c == 2 {\n                *grav = (*grav).wrapping_add(um as i8);"),
]
