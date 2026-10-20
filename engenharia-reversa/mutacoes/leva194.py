# Mutacoes das listas do gerador de jogadores e do premio da liga. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('r_marca', "    for o in [0, 4, 8] {\n        *((ultimo + o) as *mut i32) = -1;", "    for o in [0, 4] {\n        *((ultimo + o) as *mut i32) = -1;"),
    ('r_banco', "    tirar_da_lista_de_16(regen, id, 0x3c, 0x40);", "    tirar_da_lista_de_16(regen, id, 0x3c, 0x20);"),
    ('p_soma', "        parcela = parcela.wrapping_add(cota);", "        parcela = parcela.wrapping_add(cota / 2);"),
]
