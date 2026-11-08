# Mutacoes da leva95 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva95.py --jogo 2001-08-20
MUTACOES = [
    ('s_formacao', "        *w(match_player::ACTION) = 0x68;\n        call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL", "        *w(match_player::ACTION) = 0x69;\n        call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL"),
    ('s_marca', "            random(0x14).wrapping_add(byte(j, match_player::MOMENT_MOVEMENT) as i32) != 0", "            random(0x14).wrapping_add(byte(j, match_player::MOMENT_MOVEMENT) as i32) == 0"),
    ('s_chance', "        let n = if ta() & 0x40 != 0 { 20 } else { 50 };", "        let n = if ta() & 0x40 != 0 { 20 } else { 51 };"),
    ('s_ataque', "    let in_attack = (dir == 1 && y < 3) || (dir == 0 && y > 8);", "    let in_attack = (dir == 1 && y < 4) || (dir == 0 && y > 8);"),
    ('s_canto', "                    (ax, ay) = (0, 11);", "                    (ax, ay) = (1, 11);"),
    ('s_fundo', "        ax = ((ax as i32 + 4) / 2) as i8;", "        ax = ((ax as i32 + 5) / 2) as i8;"),
    ('s_marcado', "                        if tx <= 2 {\n                            tx += 1;", "                        if tx <= 2 {\n                            tx += 2;"),
    ('s_avanca', "        let r = random(d as i32 * 5).wrapping_add(byte(j, match_player::MOMENT_MOVEMENT) as i32);", "        let r = random(d as i32 * 4).wrapping_add(byte(j, match_player::MOMENT_MOVEMENT) as i32);"),
    ('s_fica', "            *w(match_player::ACTION) = 0x76;", "            *w(match_player::ACTION) = 0x77;"),
    ('s_ftol', "    let n = match_pl_ftol(*((j + match_player::TEAMWORK) as *const f32) as f64 + read_f32_at(0x968a78) as f64);", "    let n = match_pl_ftol(*((j + match_player::TEAMWORK) as *const f32) as f64 + read_f32_at(0x968a78) as f64 + 1.0);"),
]
