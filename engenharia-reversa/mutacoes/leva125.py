# Mutacoes da leva125 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva125.py --jogo 2001-10-01
MUTACOES = [
    ('f_vazio', "        return 0;\n    }\n    if *b(0x8eb2) != 0 {", "        return 1;\n    }\n    if *b(0x8eb2) != 0 {"),
    ('f_marcada', "(aqui < marcada && lado == 1)", "(aqui <= marcada && lado == 1)"),
    ('f_oito', "        *b(0x8eb2) = 7;\n        *b(0x8eae) = zero(j) as u8;", "        *b(0x8eb2) = 6;\n        *b(0x8eae) = zero(j) as u8;"),
    ('f_codigo', "0x2d => 0x20fd,", "0x2d => 0x20fc,"),
    ('f_cartao', "6 if !sofreu => 0x1ff5,", "6 if !sofreu => 0x1ff6,"),
    ('f_area', "byte(u(0x4761), 0x27) == 1 { (10..=11).contains(&y) }", "byte(u(0x4761), 0x27) == 1 { (9..=11).contains(&y) }"),
    ('f_penalti', "*d = (*d).wrapping_sub(500);\n            som = 0x1f7a;", "*d = (*d).wrapping_sub(499);\n            som = 0x1f7a;"),
    ('f_parada', "matches!(k, 0 | 5..=13 | 0x3b | 0x3c)", "matches!(k, 0 | 5..=12 | 0x3b | 0x3c)"),
    ('f_nove', "if sortear(9) == 0 {\n        narrar(chamar, 0x20df", "if sortear(8) == 0 {\n        narrar(chamar, 0x20df"),
    ('f_lado', "let lado_de = |p: usize| if sofreu { zero(p) } else { ub(p, 0x27) };", "let lado_de = |p: usize| if sofreu { ub(p, 0x27) } else { zero(p) };"),
    ('p_achou', "                *((this + 0x1a6) as *mut u8) = 0;\n                *((this + 0x1a5) as *mut u8) = 0;\n                return 1;", "                *((this + 0x1a6) as *mut u8) = 0;\n                *((this + 0x1a5) as *mut u8) = 1;\n                return 1;"),
    ('f_cansar', "chamar(0x716e10, u(0x4761), [0; 5]);", "chamar(0x716e10, u(0x4761), [1, 0, 0, 0, 0]);"),
]
