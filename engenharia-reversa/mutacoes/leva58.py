# Mutacoes da leva58 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva58.py
MUTACOES = [
    ('g_a', "        0x1 | 0x2 | 0x4 | 0x8 | 0x10 => 0x1f,", "        0x1 | 0x2 | 0x4 | 0x8 | 0x10 => 0xf,"),
    ('g_e', "        0x10000 | 0x20000 | 0x4000_0000 => 0x4003_0000,", "        0x10000 | 0x20000 | 0x4000_0000 => 0x0003_0000,"),
    ('g_f', "        0x40000 | 0x80000 | 0x8000_0000 => 0x800c_0000,", "        0x40000 | 0x80000 | 0x8000_0000 => 0x000c_0000,"),
    ('g_alto', "        0x4_0000_0000 | 0x8_0000_0000 => 0xc_0000_0000,", "        0x4_0000_0000 | 0x8_0000_0000 => 0x3_0000_0000,"),
    ('g_187f', "    *home = (*home & !group) | mask;\n    *((this + 0x187f) as *mut u32) = 0;", "    *home = (*home & !group) | mask;"),
    ('k_43', "let sum = 10 * (byte(pd, 0x43) as i32 + byte(pd, 0x33) as i32 + byte(pd, 0x27) as i32);", "let sum = 10 * (byte(pd, 0x43) as i32 + byte(pd, 0x33) as i32 + byte(pd, 0x23) as i32);"),
    ('k_4d', "        let mut v = ftol(sum as f64 + read_f32_at(j + match_player::STAMINA) as f64);", "        let mut v = ftol(sum as f64 + read_f32_at(j + 0x51) as f64);"),
    ('k_gol', "        if r & 1 != 0 {\n            continue;\n        }", "        if r & 1 != 0 && r == 0 {\n            continue;\n        }"),
    ('k_4', "            v -= if r & 0x800 != 0 || r & 0x80 != 0 { 500 } else { 1000 };", "            v -= if r & 0x800 != 0 { 500 } else { 1000 };"),
    ('k_8', "            if r & 0x800 == 0 && r & 0x80 == 0 {\n                v -= 300;", "            if r & 0x800 == 0 && r & 0x80 == 0 {\n                v -= 400;"),
    ('k_20', "            v += 2000;", "            v += 1000;"),
    ('k_8500', "        if v as i16 > 8500 {", "        if v as i16 > 8400 {"),
]
