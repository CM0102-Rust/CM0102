# Mutacoes dos amistosos e do registro de tecnico. Uso: python mutacoes.py mutacoes\levaN.py --carreira 2002-06-01
MUTACOES = [
    ('a_comp', "    (competicao == *((amistoso + 4) as *const usize)) as u32", "    (competicao != *((amistoso + 4) as *const usize)) as u32"),
    ('a_ano', "    *b.add(0xb) = ano;", "    *b.add(0xb) = ano.wrapping_add(1);"),
    ('r_ids', "    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {", "    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e] {"),
]
