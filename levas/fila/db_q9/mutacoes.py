# Mutacoes do moral, do vetor do nao jogador e das marcas de elenco. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('dq9_moral', "    let simples = moral.wrapping_add(d);", "    let simples = moral.wrapping_add(d).wrapping_add(1);"),
    ('dq9_vetor', "0x22, 0x11, 0x10, 0x17", "0x22, 0x10, 0x11, 0x17"),
    ('dq9_elenco', "    *p = mantem | e;", "    *p = mantem | e | 0x10;"),
]
