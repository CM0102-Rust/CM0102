# Mutacoes da leva118 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva118.py --jogo 2001-08-20
MUTACOES = [
    ('p_forca', "        own = a.wrapping_mul(50) / 11;", "        own = a.wrapping_mul(51) / 11;"),
    ('p_posse', "        Some(q) => q * 1.29 - 0.29,", "        Some(q) => q * 1.3 - 0.29,"),
    ('p_tempo', "    let g = f * (25.0 - t) + t * r;", "    let g = f * (24.0 - t) + t * r;"),
    ('p_expulsos', "            x11 = x11.wrapping_add(x13.wrapping_mul(-2));", "            x11 = x11.wrapping_add(x13.wrapping_mul(-1));"),
    ('p_placar', "    h *= 1.0 - (x11 as i32 - x12 as i32) as f64 * 0.1;", "    h *= 1.0 - (x11 as i32 - x12 as i32) as f64 * 0.11;"),
    ('p_fim', "        h *= if 1.0 < 30.0 / a { 1.0 } else { 30.0 / a };", "        h *= if 1.0 < 31.0 / a { 1.0 } else { 31.0 / a };"),
    ('p_tecnico', "        let mut f1 = (1.0 + (e - 10) as f64 * 0.06666666666666667) * h;", "        let mut f1 = (1.0 + (e - 11) as f64 * 0.06666666666666667) * h;"),
    ('p_limiar', "    } else if h < 0.7 {\n        3", "    } else if h < 0.71 {\n        3"),
    ('p_manter', "    if rodando && (h - *factor as f64).abs() < 0.5 {", "    if rodando && (h - *factor as f64).abs() < 0.4 {"),
]
