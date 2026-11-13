# Mutacoes (mwb_19): as honrarias tiradas do quinto grupo em vez do mata-mata (outro campeao no historico); o peso da
# classificacao nos grupos 80 em vez de 40 (a reputacao dos tecnicos); os clubes do grupo na ordem inversa (outros
# jogos; vale tambem para a Copa Premier galesa, mwb_21).
MUTACOES = [
    ('mwb19_honrarias_do_quinto_grupo', 'let knockout = read_usize(League(this).stages() + 4 * 5); // mwb19: the League Cup knockout slot', 'let knockout = read_usize(League(this).stages() + 4 * 4); // mutation: the fifth group', 'mwb_19 welsh league cup honours'),
    ('mwb19_peso_da_classificacao_nos_grupos', 'european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28); // mwb19: the group qualification weight', 'european_cup_add_qualified_record(club, comp, 0, 0x78, 0x50); // mutation: weight 80', 'mwb_19 welsh league cup staff records'),
    ('mwb19_grupo_ao_contrario', 'let e = read_usize(t + WELSH_CUP_POOL) + (g as usize + 1 + k * stride) * 4; // mwb19: the group clubs in pool order', 'let e = read_usize(t + WELSH_CUP_POOL) + (g as usize + 1 + (n as usize - 1 - k) * stride) * 4; // mutation: reversed', 'mwb_19 welsh league cup group'),
]
