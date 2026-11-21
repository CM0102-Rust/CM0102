# Mutações da 315a leva (itens mwg_11). Uso: python mutacoes.py mutacoes\leva315.py --carreira 2002-06-01
MUTACOES = [
    ('mwg11_sede_sempre_australia', 'let host = if read_u32(e + host_entry::HOST) == australia { read_u32(KEY_NATION_NEW_ZEALAND) } else { australia }; // mwg11: Australia, then New Zealand', 'let host = australia; // mutation: always Australia', 'mwg_11 oceania host alternated'),
]
