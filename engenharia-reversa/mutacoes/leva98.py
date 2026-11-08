# Mutacoes da leva98 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva98.py --jogo 2001-08-20
MUTACOES = [
    ('p_2b', "    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(1);\n    let dir = byte(j, 0x27);", "    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(2);\n    let dir = byte(j, 0x27);"),
    ('p_36', "    if tipo != 0x36 {", "    if tipo != 0x37 {"),
    ('p_dobro', "            let n = match_pl_ftol(d2 as f64 * d1 as f64 + d2 as f64 * d1 as f64);", "            let n = match_pl_ftol(d2 as f64 * d1 as f64);"),
    ('p_13', "                *b(q, 0x13) = (*b(q, 0x13)).wrapping_add(1);", "                *b(q, 0x13) = (*b(q, 0x13)).wrapping_add(2);"),
    ('p_posse', "            *outro = (*outro).wrapping_sub(10);", "            *outro = (*outro).wrapping_sub(9);"),
    ('p_80', "        let r = sortear(0x50);", "        let r = sortear(0x51);"),
    ('p_8eb2', "                *b(m(), 0x8eb2) = 7;", "                *b(m(), 0x8eb2) = 6;"),
    ('p_completo', "    if byte(mm, 0x8eb2) != 0 || completo == 0 {", "    if byte(mm, 0x8eb2) != 0 || completo == 1 {"),
]
