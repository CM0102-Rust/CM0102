# As quatro da leva130 que escaparam da carreira, refeitas com a impressao diaria do banco.
# Uso: python mutacoes.py mutacoes\leva130b.py --carreira 2002-06-01
MUTACOES = [
    ('e_0a_sorteio', 'let b = sortear(5);\n                        if a < b || protegido', 'let b = sortear(6);\n                        if a < b || protegido', 'eventos 0x0a'),
    ('e_0a_limite', 'let r = sortear(2000) + 0x1194;', 'let r = sortear(2000) + 0x1000;', 'eventos 0x0a'),
    ('e_1e_tecnico', 'adjust(club, 0xa, 0x32);', 'adjust(club, 0xa, 0x33);', 'eventos 0x1e tecnico'),
    ('e_1e_sem', 'adjust(club, 0xc, 0x4b);', 'adjust(club, 0xc, 0x4c);', 'eventos 0x1e'),
]
