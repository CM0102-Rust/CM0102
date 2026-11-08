# As da leva132c que escaparam por causa dos sorteios, refeitas com a tape_lock conferindo a lista de
# sorteios (pedido, resultado). As e_1e_* com o ramo certo (so roda em 2002-04: pulada ate 2001-10-01).
# Uso: python mutacoes.py mutacoes\leva132d.py --jogo 2001-10-01
MUTACOES = [
    ('e_1e_sem', 'adjust(club, 0xc, 0x4b);', 'adjust(club, 0xc, 0x4c);', 'events 0x1e no manager'),
    ('e_1e_sem2', 'adjust(club, 0xc, 0x4b);', 'adjust(club, 0xc, -0x4b);', 'events 0x1e no manager'),
    ('e_0a_sorteio', 'let b = random(5);\n                        if a < b || exe::media_has_story_for_player', 'let b = random(6);\n                        if a < b || exe::media_has_story_for_player', 'events 0x0a'),
    ('e_0a_limite', 'let r = random(2000) + 0x1194;', 'let r = random(2000) + 0x1000;', 'events 0x0a'),
    ('j_n34', 'if r as f64 * 0.1 + m + 0.5 < nota11 {', 'if r as f64 * 0.1 + m + 0.4 < nota11 {', 'players news 0x34'),
]
