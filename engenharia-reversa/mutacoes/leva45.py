# Mutacoes da leva45 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva45.py
MUTACOES = [
    ('t_limite', "{ 8 } else { 1 };", "{ 7 } else { 1 };"),
    ('t_alvo0', "let limit: i32 = if target != 0 && target == ", "let limit: i32 = if target == "),
    ('t_110', "1 => 0x6e - match_pl_ftol(c) as i8 as i32,", "1 => 0x6f - match_pl_ftol(c) as i8 as i32,"),
    ('t_i8', "1 => 0x6e - match_pl_ftol(c) as i8 as i32,", "1 => 0x6e - match_pl_ftol(c),"),
    ('t_k2', "2 => match_pl_ftol(read_f64_at(0x96d860) / v),", "2 => match_pl_ftol(read_f64_at(0x96aff8) / v),"),
    ('t_k5', "5 => match_pl_ftol(read_f64_at(0x96d850) / v),", "5 => match_pl_ftol(read_f64_at(0x96d858) / v),"),
    ('t_menor', "(sortear(n) < limit) as u8", "(sortear(n) <= limit) as u8"),
    ('t_quadrado', "let v = match_pl_ftol(game_pow(c, read_f64_at(0x968950))) as f64;", "let v = (game_pow(c, read_f64_at(0x968950))) as f64;"),
    ('t_ftol', "    if x.is_finite() && x.abs() < 9.223372036854775808e18 {", "    if true {"),
    ('t_outros', "        _ => return 0,\n    };\n    (sortear(n)", "        _ => 0,\n    };\n    (sortear(n)"),
]
