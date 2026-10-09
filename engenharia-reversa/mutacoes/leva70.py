# Mutacoes da leva70 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva70.py --jogo 2001-08-15
MUTACOES = [
    ('f_paciencia', "    *paciencia = (*paciencia).wrapping_add(0xaf);", "    *paciencia = (*paciencia).wrapping_add(0xae);"),
    ('f_vantagem', "    let div = if *((outro + 0x1a6) as *const usize) != j { 1.0 } else { 3.0 };", "    let div = if *((outro + 0x1a6) as *const usize) != j { 1.0 } else { 2.0 };"),
    ('f_sev', "        fator = if sev > 10 { sev as f64 * f64_em(0x967880) }", "        fator = if sev > 11 { sev as f64 * f64_em(0x967880) }"),
    ('f_modo', "        fator += modo / (byte(arb, 0x27) as f64 * f64_em(0x968968) + f64_em(0x968930));", "        fator += modo / (byte(arb, 0x27) as f64 * f64_em(0x968968) + f64_em(0x968940));"),
    ('f_rigor', "            5..=9 => -1,", "            5..=9 => 0,"),
    ('f_teto', "    } else if f64_em(0x9688b0) < fator || fator.is_nan() {\n        fator = 10.0;", "    } else if f64_em(0x9688b0) < fator || fator.is_nan() {\n        fator = 9.0;"),
    ('f_2a', "            if sortear(ftol(fator * f64_em(0x969000))) != 0 {\n                res = 1;\n                somar(sortear(dureza) + 4);", "            if sortear(ftol(fator * f64_em(0x969000))) != 0 {\n                res = 1;\n                somar(sortear(dureza) + 5);"),
    ('f_minuto', "    let minuto = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;", "    let minuto = (*((m + 0x8ed0) as *const i16) as i32 / 12) as i8;"),
    ('f_aplica', "        multiplicar(outro, (*grav as f64 * f64_em(0x968918) + f64_em(0x967890)) as f32);", "        multiplicar(outro, (*grav as f64 * f64_em(0x968918) + f64_em(0x968940)) as f32);"),
    ('f_ficha', "            if c == 1 {\n                *grav = (*grav).wrapping_add(um as i8);", "            if c == 2 {\n                *grav = (*grav).wrapping_add(um as i8);"),
]
