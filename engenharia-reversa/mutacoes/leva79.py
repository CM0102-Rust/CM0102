# Mutacoes da leva79 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva79.py
MUTACOES = [
    ('j_forma', "as i32) as f64 * 0.002 + 1.5;", "as i32) as f64 * 0.003 + 1.5;"),
    ('j_teto', "    } else if forma > 4.0 {\n        forma = 4.0;", "    } else if forma > 3.0 {\n        forma = 3.0;"),
    ('j_44', "        c = *((r + 0x44) as *const i8) as f64 * 0.013333333333333334 + 0.5;", "        c = *((r + 0x44) as *const u8) as f64 * 0.013333333333333334 + 0.5;"),
    ('j_tipo6', "            6 => c - 15.0,", "            6 => c - 16.0,"),
    ('j_tipo1', "            1 => c + 2.5,", "            1 => c + 2.0,"),
    ('j_75', "        byte(pessoa, 0x22) as u8 as f64 * 75.0", "        byte(pessoa, 0x22) as u8 as f64 * 70.0"),
    ('j_1000', "    let y = if x > 1000.0 { (x - 1000.0) * 0.000125 + 1.0 } else { 1.0 };", "    let y = if x > 1000.0 { (x - 1000.0) * 0.00012 + 1.0 } else { 1.0 };"),
    ('j_n2', "    match_pl_ftol(e * y - (n * n + 5.0) * 0.1 + forma + c + 0.5)", "    match_pl_ftol(e * y - (n + 5.0) * 0.1 + forma + c + 0.5)"),
    ('j_lado', "        *((t + if byte(this, 0x27) == 0 { 0x1c } else { 0x20 }) as *const usize)", "        *((t + 0x1c) as *const usize)"),
    ('j_22sinal', "        byte(pessoa, 0x22) as u8 as f64 * 75.0", "        byte(pessoa, 0x22) as f64 * 75.0"),
]
