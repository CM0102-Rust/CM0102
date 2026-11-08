# Mutacoes da leva109 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva109.py --jogo 2001-08-20
MUTACOES = [
    ('d_chance', "        if *c > 0x32 && sortear(3) != 0 {", "        if *c > 0x33 && sortear(3) != 0 {"),
    ('d_folego', "        let mut x = fl(p, 0x4d) * f4(0x968f18) + f4(0x968db0);", "        let mut x = fl(p, 0x4d) * f4(0x968f18) + f4(0x96d2dc);"),
    ('d_ramo', "        let v = if ramo > 10 {", "        let v = if ramo > 11 {"),
    ('d_grau', "            if t > d8(0x968930) {", "            if t > d8(0x9688a0) {"),
    ('d_risco', "        if sortear(50000) >= match_pl_ftol(f[i] as f64 * f4(0x968a78)) {\n            continue;", "        if sortear(50000) >= match_pl_ftol(f[i] as f64 * f4(0x968a78) * 2.0) {\n            continue;"),
    ('d_dono', "                    let k = [0x96d440, 0x96d430, 0x967880, 0x96d438, 0, 0x968fd0, 0x968a98, 0x96d448, 0, 0, 0x968918][r as usize];", "                    let k = [0x96d440, 0x96d430, 0x968918, 0x96d438, 0, 0x968fd0, 0x968a98, 0x96d448, 0, 0, 0x968918][r as usize];"),
    ('d_tipo', "                1..=3 => injure(chamar, 3, ch, p, 0),", "                1..=3 => injure(chamar, 4, ch, p, 0),"),
    ('d_outro', "            let t = sortear(9) + 1;", "            let t = sortear(9) + 2;"),
    ('d_5', "            let x = if r79 > d8(0x968950) { r79 } else { d8(0x968950) };", "            let x = if r79 > d8(0x968950) { r79 } else { d8(0x968930) };"),
]
