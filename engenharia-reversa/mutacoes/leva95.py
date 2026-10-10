# Mutacoes da leva95 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva95.py --jogo 2001-08-20
MUTACOES = [
    ('s_formacao', "        *w(0x198) = 0x68;\n        chamar(0x6f46b0", "        *w(0x198) = 0x69;\n        chamar(0x6f46b0"),
    ('s_marca', "            sortear(0x14).wrapping_add(byte(j, 0x107) as i32) != 0", "            sortear(0x14).wrapping_add(byte(j, 0x107) as i32) == 0"),
    ('s_chance', "        let n = if ta() & 0x40 != 0 { 20 } else { 50 };", "        let n = if ta() & 0x40 != 0 { 20 } else { 51 };"),
    ('s_ataque', "    let no_ataque = (dir == 1 && y < 3) || (dir == 0 && y > 8);", "    let no_ataque = (dir == 1 && y < 4) || (dir == 0 && y > 8);"),
    ('s_canto', "                    (ax, ay) = (0, 11);", "                    (ax, ay) = (1, 11);"),
    ('s_fundo', "        ax = ((ax as i32 + 4) / 2) as i8;", "        ax = ((ax as i32 + 5) / 2) as i8;"),
    ('s_marcado', "                        if tx <= 2 {\n                            tx += 1;", "                        if tx <= 2 {\n                            tx += 2;"),
    ('s_avanca', "        let r = sortear(d as i32 * 5).wrapping_add(byte(j, 0x107) as i32);", "        let r = sortear(d as i32 * 4).wrapping_add(byte(j, 0x107) as i32);"),
    ('s_fica', "            *w(0x198) = 0x76;", "            *w(0x198) = 0x77;"),
    ('s_ftol', "    let n = ftol(*((j + 0xe1) as *const f32) as f64 + f32_em(0x968a78) as f64);", "    let n = ftol(*((j + 0xe1) as *const f32) as f64 + f32_em(0x968a78) as f64 + 1.0);"),
]
