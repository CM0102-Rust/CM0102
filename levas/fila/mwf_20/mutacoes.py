# Mutacoes (mwf_20): a primeira rodada da Copa da Coreia uma semana antes (23/10 em vez de 30/10); os clubes da K League
# com semente 1 (outro sorteio).
MUTACOES = [
    ('mwf20_primeira_rodada_antes',
     'cql12r((0x15, 9, 0, 0), (0x1e, 9, 0, 2, 2, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 3), (0, 0xffffffff, 0)), // mwf20: the first round',
     'cql12r((0x15, 9, 0, 0), (0x17, 9, 0, 2, 2, 0), 0x1e, (0, 0x83, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 3), (0, 0xffffffff, 0)), // mutation: 23 October',
     'mwf_20 korean fa cup rounds'),
    ('mwf20_semente_k_league',
     'write_u8(this.team_table() + 6 * k as usize + 4, 0); // mwf20: the K League seed',
     'write_u8(this.team_table() + 6 * k as usize + 4, 1); // mutation: seed 1',
     'mwf_20 korean fa cup entrants'),
]
