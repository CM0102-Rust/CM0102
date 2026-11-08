# Mutacoes da contagem e da lista do elenco e de se o clube pode ser dirigido. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq10_contar', '            n = n.wrapping_add(1);\n        }\n        n\n', '            n = n.wrapping_add(2);\n        }\n        n\n'),
    ('dq10_reunir', "vetor.add(i), squad_slot(clube, i)", "vetor.add(i), squad_slot(clube, 49 - i)"),
    ('dq10_dirigido', "    (*((u(0x53) + 0x11c) as *const u8) & 2 != 0) as u8", "    (*((u(0x53) + 0x11c) as *const u8) & 4 != 0) as u8"),
]
