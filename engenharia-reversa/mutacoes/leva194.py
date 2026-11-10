# Mutacoes das listas do gerador de jogadores e do premio da liga. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('r_marca', "    for o in [0, 4, 8] {\n        write_i32(last + o, -1);", "    for o in [0, 4] {\n        write_i32(last + o, -1);"),
    ('r_banco', "    remove_from_list16(regen, id, 0x3c, 0x40);", "    remove_from_list16(regen, id, 0x3c, 0x20);"),
    ('p_soma', "        parcela = parcela.wrapping_add(quota);", "        parcela = parcela.wrapping_add(quota / 2);"),
]
