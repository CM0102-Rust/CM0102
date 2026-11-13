# Mutacoes (mwd_8): a rodada preliminar da Copa da Croacia uma semana depois (22/8/2001 em vez de 15/8); o sorteio da
# semente dos clubes da segunda divisao vira 0..9 (outros confrontos no sorteio).
MUTACOES = [
    ('mwd8_preliminar_depois',
     'cql12r((0x10, 6, 0, 4), (0xf, 7, 0, 6, 1, 0), 0xe6, (0, 1, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwd8: the preliminary round',
     'cql12r((0x10, 6, 0, 4), (0x16, 7, 0, 6, 1, 0), 0xe6, (0, 1, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mutation: 22 August',
     'mwd_8 croatian cup rounds'),
    ('mwd8_semente_menor',
     'write_u8(this.team_table() + k + 4, exe::utils_random(0x32) as u8); // mwd8: the second division seed',
     'write_u8(this.team_table() + k + 4, exe::utils_random(0xa) as u8); // mutation: seed 0..9',
     'mwd_8 croatian cup entrants'),
]
