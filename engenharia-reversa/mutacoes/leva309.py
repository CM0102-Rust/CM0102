# Mutações da 309a leva (itens cqx_13). Uso: python mutacoes.py mutacoes\leva309.py --prova-save 309
MUTACOES = [
    ('cqx13_estadios_nulos', "write_usize(p, if id == -1 { 0 } else { read_usize(G_STADIUM_TABLE).wrapping_add((id as usize).wrapping_mul(0x4e)) }); // cqx13: the venue's stadium", 'write_usize(p, 0); // mutation: no stadium'),
    ('cqx13_conmebol_nacao_adiante', 'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add(read_usize(e).wrapping_mul(nation::SIZE))); // cqx13: the nation of the CONMEBOL record', 'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add((read_usize(e) + 1).wrapping_mul(nation::SIZE))); // mutation: the next nation'),
    ('cqx13_uefa_nacao_adiante', 'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add(read_usize(e).wrapping_mul(nation::SIZE))); // cqx13: the nation of the UEFA record', 'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add((read_usize(e) + 1).wrapping_mul(nation::SIZE))); // mutation: the next nation'),
]
