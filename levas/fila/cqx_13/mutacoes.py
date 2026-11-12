# Mutacoes (cqx_13) pela prova do save (mutacoes.py --prova-save N, com a carga do save de ferias: CARGA). Sem o 4o
# campo (contador): a base da prova do save roda com as rotinas desligadas.
# - os estadios dos locais todos nulos (os jogos depois da carga mudam de local e de publico);
# - o ponteiro da nacao de cada registro da CONMEBOL uma nacao adiante (so o comparador lado a lado pega: carregar o
#   save no modo comparando, mutacoes.py --carga-comparando);
# - o mesmo nos registros da UEFA.
MUTACOES = [
    ('cqx13_estadios_nulos',
     'write_usize(p, if id == -1 { 0 } else { read_usize(G_STADIUM_TABLE).wrapping_add((id as usize).wrapping_mul(0x4e)) }); // cqx13: the venue\'s stadium',
     'write_usize(p, 0); // mutation: no stadium'),
    ('cqx13_conmebol_nacao_adiante',
     'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add(read_usize(e).wrapping_mul(nation::SIZE))); // cqx13: the nation of the CONMEBOL record',
     'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add((read_usize(e) + 1).wrapping_mul(nation::SIZE))); // mutation: the next nation'),
    ('cqx13_uefa_nacao_adiante',
     'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add(read_usize(e).wrapping_mul(nation::SIZE))); // cqx13: the nation of the UEFA record',
     'write_usize(e + 4, read_usize(G_NATION_TABLE).wrapping_add((read_usize(e) + 1).wrapping_mul(nation::SIZE))); // mutation: the next nation'),
]
