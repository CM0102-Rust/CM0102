# Mutacoes da leva98 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva98.py --jogo 2001-08-20
MUTACOES = [
    ('p_2b', "    *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(1);\n    let dir = byte(j, match_player::SIDE);", "    *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(2);\n    let dir = byte(j, match_player::SIDE);"),
    ('p_36', "    if kind != 0x36 {", "    if kind != 0x37 {"),
    ('p_dobro', "            let n = match_pl_ftol(d2 as f64 * d1 as f64 + d2 as f64 * d1 as f64);", "            let n = match_pl_ftol(d2 as f64 * d1 as f64);"),
    ('p_13', "                *b(q, match_player::OFFSIDES) = (*b(q, match_player::OFFSIDES)).wrapping_add(1);", "                *b(q, match_player::OFFSIDES) = (*b(q, match_player::OFFSIDES)).wrapping_add(2);"),
    ('p_posse', "            *other = (*other).wrapping_sub(10);", "            *other = (*other).wrapping_sub(9);"),
    ('p_80', "        let r = random(0x50);", "        let r = random(0x51);"),
    ('p_8eb2', "                *b(m(), 0x8eb2) = 7;", "                *b(m(), 0x8eb2) = 6;"),
    ('p_completo', "    if byte(mm, engine::RESTART_TYPE) != 0 || full == 0 {", "    if byte(mm, engine::RESTART_TYPE) != 0 || full == 1 {"),
]
