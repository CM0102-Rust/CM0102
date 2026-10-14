# As da leva132c que escaparam por causa dos sorteios, refeitas com a fita conferindo a lista de
# sorteios (pedido, resultado). As e_1e_* com o ramo certo (so roda em 2002-04: pulada ate 2001-10-01).
# Uso: python mutacoes.py mutacoes\leva132d.py --jogo 2001-10-01
MUTACOES = [
    ('e_1e_sem', 'ajustar(clube, 0xc, 0x4b);', 'ajustar(clube, 0xc, 0x4c);', 'eventos 0x1e sem tecnico'),
    ('e_1e_sem2', 'ajustar(clube, 0xc, 0x4b);', 'ajustar(clube, 0xc, -0x4b);', 'eventos 0x1e sem tecnico'),
    ('e_0a_sorteio', 'let b = sortear(5);\n                        if a < b || protegido', 'let b = sortear(6);\n                        if a < b || protegido', 'eventos 0x0a'),
    ('e_0a_limite', 'let r = sortear(2000) + 0x1194;', 'let r = sortear(2000) + 0x1000;', 'eventos 0x0a'),
    ('j_n34', 'if r as f64 * 0.1 + m + 0.5 < nota11 {', 'if r as f64 * 0.1 + m + 0.4 < nota11 {', 'jogadores noticia 0x34'),
]
