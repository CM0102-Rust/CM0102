# Mutacoes das levas 130-132 que escaparam da carreira, agora no modo comparacao (jogo real).
# Prova que a comparacao lado a lado do pos-jogo pega o que as impressoes da carreira nao pegam.
# Uso: python mutacoes.py mutacoes\leva132c.py --jogo 2001-10-01
MUTACOES = [
    ('e_1e_sem', "adjust(club, 0xc, 0x4b);", "adjust(club, 0xc, 0x4c);", "events 0x1e"),
    ('e_1e_sem2', "adjust(club, 0xc, 0x4b);", "adjust(club, 0xc, -0x4b);", "events 0x1e"),
    ('e_0a_sorteio', "let b = random(5);\n                        if a < b || exe::media_has_story_for_player", "let b = random(6);\n                        if a < b || exe::media_has_story_for_player", "events 0x0a"),
    ('e_0a_limite', "let r = random(2000) + 0x1194;", "let r = random(2000) + 0x1000;", "events 0x0a"),
    ('j_n34', "if r as f64 * 0.1 + m + 0.5 < nota11 {", "if r as f64 * 0.1 + m + 0.4 < nota11 {", "players news 0x34"),
    ('c_57', "if read_i8(p + staff::AMBITION) <= (r as u8).wrapping_add(10) as i8", "if read_i8(p + staff::AMBITION) <= (r as u8).wrapping_add(0) as i8", "club youth"),
    ('c_ficha', "|| w(g(p + staff::PLAYER) + 0xb) >= 0x109a {", "|| w(g(p + staff::PLAYER) + 0xb) >= 0x2000 {", "club youth"),
    ('c_n', "                && n > 2\n", "                && n > 0\n", "club squad"),
    ('c_elenco57', "&& read_i8(s + staff::AMBITION) > 0xf", "&& read_i8(s + staff::AMBITION) > 0x5", "club squad"),
]
