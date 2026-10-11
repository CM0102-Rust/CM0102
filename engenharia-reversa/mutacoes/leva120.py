# Mutacoes da leva120 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva120.py --jogo 2001-08-20
MUTACOES = [
    ('i_fator', "        let mut f38 = (resto as f64 * 0.05 + 1.0) * f40 * 1.2;", "        let mut f38 = (resto as f64 * 0.05 + 1.0) * f40 * 1.25;"),
    ('i_ritmo', "    let r = if ritmo <= 8 {\n        2", "    let r = if ritmo <= 9 {\n        2"),
    ('i_passe', "        if (a(0x12) as f64) < q {", "        if (a(0x12) as f64) <= q + 1.0 {"),
    ('i_cobranca', "        if k1 - l28 < a(8) as f64 {", "        if k1 - l28 + 1.0 < a(8) as f64 {"),
    ('i_cruzamento', "                if f38 * 23.0 < b() as f64 {", "                if f38 * 22.0 < b() as f64 {"),
    ('i_desarme', "            } else if f38 * 25.0 - (tc(0xe) as i32 / 4) as f64 > (a(0x15) + a(0xb)) as f64 {", "            } else if f38 * 26.0 - (tc(0xe) as i32 / 4) as f64 > (a(0x15) + a(0xb)) as f64 {"),
    ('i_impedimento', "        } else if d() < -5 {", "        } else if d() < -4 {"),
    ('i_acc', "                    acc = acc.wrapping_add(8 - v40);", "                    acc = acc.wrapping_add(9 - v40);"),
    ('i_chute', "                if (tc(0x1f) as i32 / 5) as f64 + f38 * 5.0 > a3 as f64 {", "                if (tc(0x1f) as i32 / 5) as f64 + f38 * 6.0 > a3 as f64 {"),
]
