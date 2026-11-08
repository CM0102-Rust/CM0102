# Mutacoes da leva119 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva119.py --jogo 2001-08-20
MUTACOES = [
    ('f_duas', "            let sum = a.wrapping_add(b);", "            let sum = a.wrapping_add(a);"),
    ('f_metade', "            table[g * 5 + s] = if n < 11 && g == 6 { sum / 2 } else { sum };", "            table[g * 5 + s] = if n < 11 && g == 5 { sum / 2 } else { sum };"),
    ('f_tempo', "        let mut x = ((3000.0 - t as f64) * 5.0 + v * t as f64) * 0.005;", "        let mut x = ((3000.0 - t as f64) * 5.0 + v * t as f64) * 0.006;"),
    ('f_favorita', "            x *= if !favorita {\n                0.1", "            x *= if !favorita {\n                0.2"),
    ('f_goleiro', "            if p1 & 1 != 0 {\n                band = 2;", "            if p1 & 1 != 0 {\n                band = 1;"),
    ('f_meia', "            if p2 & 2 != 0 {\n                more(&mut f18, 0.5);", "            if p2 & 2 != 0 {\n                more(&mut f18, 0.25);"),
    ('f_equilibrio', "        let e = 44.0 - ((32.0 - s_) - (z - f20 as f64 * 0.5).abs() * (z - f20 as f64)) * 0.8;", "        let e = 44.0 - ((33.0 - s_) - (z - f20 as f64 * 0.5).abs() * (z - f20 as f64)) * 0.8;"),
    ('f_maior', "        if melhor == -1 || r > melhor_r {", "        if melhor == -1 || r >= melhor_r {"),
    ('f_escala', "    if escala <= 2000 {\n        escala = 2000;", "    if escala <= 2500 {\n        escala = 2500;"),
]
