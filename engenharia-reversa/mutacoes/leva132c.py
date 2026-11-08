# Mutacoes das levas 130-132 que escaparam da carreira, agora no modo comparacao (jogo real).
# Prova que a comparacao lado a lado do pos-jogo pega o que as impressoes da carreira nao pegam.
# Uso: python mutacoes.py mutacoes\leva132c.py --jogo 2001-10-01
MUTACOES = [
    ('e_1e_sem', "adjust(clube, 0xc, 0x4b);", "adjust(clube, 0xc, 0x4c);", "eventos 0x1e"),
    ('e_1e_sem2', "adjust(clube, 0xc, 0x4b);", "adjust(clube, 0xc, -0x4b);", "eventos 0x1e"),
    ('e_0a_sorteio', "let b = sortear(5);\n                        if a < b || protegido", "let b = sortear(6);\n                        if a < b || protegido", "eventos 0x0a"),
    ('e_0a_limite', "let r = sortear(2000) + 0x1194;", "let r = sortear(2000) + 0x1000;", "eventos 0x0a"),
    ('j_n34', "if r as f64 * 0.1 + m + 0.5 < nota11 {", "if r as f64 * 0.1 + m + 0.4 < nota11 {", "jogadores noticia 0x34"),
    ('c_57', "if byte(p, 0x57) <= (r as u8).wrapping_add(10) as i8", "if byte(p, 0x57) <= (r as u8).wrapping_add(0) as i8", "clube jovem"),
    ('c_ficha', "|| w(g(p + staff::PLAYER) + 0xb) >= 0x109a {", "|| w(g(p + staff::PLAYER) + 0xb) >= 0x2000 {", "clube jovem"),
    ('c_n', "                && n > 2\n", "                && n > 0\n", "clube elenco"),
    ('c_elenco57', "&& byte(s, 0x57) > 0xf", "&& byte(s, 0x57) > 0x5", "clube elenco"),
]
