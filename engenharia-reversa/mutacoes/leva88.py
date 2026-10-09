# Mutacoes da leva88 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva88.py --jogo 2001-08-20
MUTACOES = [
    ('b_6', "    (if lado(this) == 1 { y >= 6 } else { y < 6 }) as u8", "    (if lado(this) == 1 { y >= 5 } else { y < 6 }) as u8"),
    ('m_11', "    if sortear(11) != 0 {\n        return;\n    }\n    let ff", "    if sortear(10) != 0 {\n        return;\n    }\n    let ff"),
    ('m_metade', "        *f(0x59) = (*f(0x59) as f64 * f64_em(0x968968)) as f32;", "        *f(0x59) = (*f(0x59) as f64 * f64_em(0x969040)) as f32;"),
    ('m_50', "    if b1 > 50 {", "    if b1 > 60 {"),
    ('m_teto', "    } else if *f(0x4d) as f64 > f64_em(0x968910) {\n        *f(0x4d) = 10000.0;", "    } else if *f(0x4d) as f64 > f64_em(0x968910) {\n        *f(0x4d) = 9000.0;"),
    ('i_amistoso', "            if byte(t, 0x5f) == 1 && u(t + 0x39) != 0 && na_competicao(objeto(), u(t + 0x39)) != 0 {", "            if byte(t, 0x5f) == 1 && u(t + 0x39) != 0 && na_competicao(objeto(), u(t + 0x39)) == 0 {"),
    ('i_principal', "        if na_competicao(objeto(), principal(u(t + 0x1a))) != 0 {", "        if na_competicao(objeto(), u(t + 0x1a)) != 0 {"),
    ('i_16', "    for s in 0..16 {\n        let t = tecnico(s);\n        if byte(t, 0x5f) != 1 {", "    for s in 0..15 {\n        let t = tecnico(s);\n        if byte(t, 0x5f) != 1 {"),
]
