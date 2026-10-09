# Mutacoes da leva69 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva69.py
MUTACOES = [
    ('c_limite', "    if *(clube as *const i32) >= limite {\n        let continente", "    if *(clube as *const i32) > limite {\n        let continente"),
    ('c_cont0', "        if continente != 0 && (nacao == 0 || d(nacao, 0x71) != continente) {", "        if continente != 0 && (nacao != 0 && d(nacao, 0x71) != continente) {"),
    ('c_divisao', "        if nacao == d(base, 0x5d) && d(clube, 0x57) != 0 && d(clube, 0x57) == base {", "        if nacao == d(base, 0x5d) && d(clube, 0x57) != 0 {"),
    ('c_outro', "        if d(nacao, 0x71) != d(base, 0x59) {\n            return 0;", "        if d(nacao, 0x71) == d(base, 0x59) {\n            return 0;"),
    ('c_grupo', "        if byte(fase, 0x43) == 2 {\n            grupo = fase;", "        if byte(fase, 0x43) == 1 {\n            grupo = fase;"),
    ('c_passo', "if grupo != 0 { (grupo, 0xb1, 0x3b) }", "if grupo != 0 { (grupo, 0xb1, 0x3a) }"),
    ('c_ate', "    while i <= n {", "    while i < n {"),
    ('e_lista', "[0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964]\n        .iter()", "[0x9cf79cusize, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c]\n        .iter()"),
]
