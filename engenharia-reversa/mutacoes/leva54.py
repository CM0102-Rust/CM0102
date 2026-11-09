# Mutacoes da leva54 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva54.py
MUTACOES = [
    ('n_menos2', "    let mut level: i8 = if *(partida.addr() as *const i32) == -2 {", "    let mut level: i8 = if *(partida.addr() as *const i32) == -1 {"),
    ('n_500', "((rep_de(Club(outro)) / 500 + comp.reputation()", "((rep_de(Club(outro)) / 1000 + comp.reputation()"),
    ('n_teto', "        v.min(20)\n", "        v.min(19)\n"),
    ('n_21', ".wrapping_add(16).max(21) as i16;", ".wrapping_add(16).max(20) as i16;"),
    ('n_mais1', "        ceiling().wrapping_sub(level as i16).wrapping_add(1)\n", "        ceiling().wrapping_sub(level as i16)\n"),
    ('n_codigos', "matches!(w30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b)", "matches!(w30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9a)"),
    ('n_reset', "        level = (comp.reputation() as i8);\n", "\n"),
    ('n_alt', "    let (a, b) = if alt { (0x20, 0x28) } else { (0x10, 0x18) };", "    let (a, b) = if alt { (0x10, 0x18) } else { (0x20, 0x28) };"),
    ('n_quadrado', "        f = (f.abs() as f64 * f as f64) as f32;", "        f = (f as f64 * f as f64) as f32;"),
    ('n_bonus', "        if t as i16 > new {", "        if t as i16 >= new + 1 && t as i16 > 0 {"),
    ('n_limite', "    let limit = (nv * 300 + 4000) as i16;", "    let limit = (nv * 300 + 3000) as i16;"),
    ('n_baixo', "    let v = if low > mf as f64 {", "    let v = if low >= mf as f64 + 1.0 {"),
    ('n_clamp', "    let r = (ftol(v) as i16).clamp(1, 10000);", "    let r = (ftol(v) as i16).clamp(0, 10000);"),
]
