# Reforcos fortes da leva130 (as fracas escaparam: valores no limit de 1, sorteios sem virada).
# Uso: python mutacoes.py mutacoes\leva130c.py --carreira 2002-06-01
MUTACOES = [
    ('e_1e_sem2', "adjust(clube, 0xc, 0x4b);", "adjust(clube, 0xc, -0x4b);", "eventos 0x1e"),
    ('e_0a_limite2', "let r = sortear(2000) + 0x1194;", "let r = sortear(2000);", "eventos 0x0a"),
]
