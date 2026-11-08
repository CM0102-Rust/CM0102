# Mutacoes da leva121 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva121.py --jogo 2001-08-20
MUTACOES = [
    ('c_pontas', "    if byte(a, 0x19) == -2 {\n        return Some(-1);", "    if byte(a, 0x19) == -3 {\n        return Some(-1);"),
    ('c_goleiro', "    if tabela(a) == 1 && byte(u(a, 0x6d), 0xf) < 20 {", "    if tabela(a) == 1 && byte(u(a, 0x6d), 0xf) < 19 {"),
    ('c_peso', "            4 | 2 => match_pl_ftol(v as f64 * 1.35),", "            4 | 2 => match_pl_ftol(v as f64 * 1.3),"),
    ('c_oito', "    let vb = pesar(match_condition(b), 1.25);", "    let vb = pesar(match_condition(b), 1.2);"),
    ('c_condicao', "    match_condition(a).wrapping_sub(vb)", "    match_condition(a).wrapping_sub(vb).wrapping_add(1)"),
    ('c_total', "    let total = || (*((u(c, 0x37) + 0x4d) as *const u8) & 0xf) as i32 + 11;", "    let total = || (*((u(c, 0x37) + 0x4d) as *const u8) & 0xf) as i32 + 10;"),
    ('c_estado', "        if p != 0 && estado(u(G_INJURY, 0), p) != 2 {", "        if p != 0 && estado(u(G_INJURY, 0), p) != 3 {"),
    ('c_extra', "            if livre && impedido(u(G_NATIONAL_TEAMS, 0), p, 0xae2c90, u(c, 0x2f), 0, 0) == 0 && chamar(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM, c, [pos, p as u32, pos, 0, 0]) as u8 != 0 {", "            if livre && impedido(u(G_NATIONAL_TEAMS, 0), p, 0xae2c90, u(c, 0x2f), 0, 0) == 0 && chamar(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM, c, [pos, p as u32, 0, 0, 0]) as u8 != 0 {"),
]
