# Mutacoes dos amistosos e do registro de tecnico. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('a_comp', "    (competition == read_usize(friendly + 4)) as u32", "    (competition != read_usize(friendly + 4)) as u32"),
    ('a_ano', "    *b.add(0xb) = year;", "    *b.add(0xb) = year.wrapping_add(1);"),
    ('r_ids', "    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {", "    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e] {"),
]
