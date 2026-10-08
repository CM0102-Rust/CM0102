# Mutacoes da leva45 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva45.py
MUTACOES = [
    ('t_limite', "{ 8 } else { 1 };", "{ 7 } else { 1 };"),
    ('t_alvo0', "let limite: i32 = if alvo != 0 && alvo == ", "let limite: i32 = if alvo == "),
    ('t_110', "1 => 0x6e - ftol(c) as i8 as i32,", "1 => 0x6f - ftol(c) as i8 as i32,"),
    ('t_i8', "1 => 0x6e - ftol(c) as i8 as i32,", "1 => 0x6e - ftol(c),"),
    ('t_k2', "2 => ftol(f64_em(0x96d860) / v),", "2 => ftol(f64_em(0x96aff8) / v),"),
    ('t_k5', "5 => ftol(f64_em(0x96d850) / v),", "5 => ftol(f64_em(0x96d858) / v),"),
    ('t_menor', "(sortear(n) < limite) as u8", "(sortear(n) <= limite) as u8"),
    ('t_quadrado', "let v = ftol(pow_do_jogo(c, f64_em(0x968950))) as f64;", "let v = (pow_do_jogo(c, f64_em(0x968950))) as f64;"),
    ('t_ftol', "    if x.is_finite() && x.abs() < 9.223372036854775808e18 {", "    if true {"),
    ('t_outros', "        _ => return 0,\n    };\n    (sortear(n)", "        _ => 0,\n    };\n    (sortear(n)"),
]
