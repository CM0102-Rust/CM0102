# Mutacoes da leva90 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva90.py --jogo 2001-08-20
MUTACOES = [
    ('j_2b', "        let p = *lista.add(k as usize);\n        if byte(p, 0x2b) <= 1 && byte(p, 0x19) >= 0 {", "        let p = *lista.add(k as usize);\n        if byte(p, 0x2b) < 1 && byte(p, 0x19) >= 0 {"),
    ('j_19', "        let p = *lista.add(k as usize);\n        if byte(p, 0x2b) <= 1 && byte(p, 0x19) >= 0 {", "        let p = *lista.add(k as usize);\n        if byte(p, 0x2b) <= 1 && byte(p, 0x19) > 0 {"),
    ('j_args', "match jogada(p, [a4, a3, a6, a7]) as i8 {", "match jogada(p, [a3, a4, a6, a7]) as i8 {"),
    ('j_12', "*((p + 0x12) as *mut u8) = (*((p + 0x12) as *const u8)).wrapping_add(1);", "*((p + 0x12) as *mut u8) = (*((p + 0x12) as *const u8)).wrapping_add(2);"),
    ('j_ret2', "                        *((this + 0xf5ae) as *mut usize) = p;\n                        return 2;", "                        *((this + 0xf5ae) as *mut usize) = p;\n                        return 1;"),
    ('j_modo', "            if modo == 2 {\n                n = n.wrapping_add(1);", "            if modo == 3 {\n                n = n.wrapping_add(1);"),
    ('t_fim', "        while y < fim || (ate_o_fim && y == fim) {", "        while y < fim {"),
    ('t_11', "true, 11, &|x, y, v|", "true, 8, &|x, y, v|"),
    ('t_lado', "    let lado = (byte(this, 0x8eae) == 0) as i8;\n    percorrer_linha", "    let lado = (byte(this, 0x8eae) != 0) as i8;\n    percorrer_linha"),
]
