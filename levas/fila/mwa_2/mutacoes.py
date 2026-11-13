# Mutacoes (mwa_2), na Copa de Leinster, carreira ENG IRL NIR ate 2002-08-01:
# - a primeira rodada jogada uma semana depois (23/8 -> 30/8);
# - os regionais sao o 2o ao 5o do sorteio (outro clube entra na copa);
# - a virada sobe o ano em 2.
MUTACOES = [
    ('mwa2_primeira_rodada_semana_depois',
     'cql10r((1, 7, 0, 1), (0x17, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwa2: the first round of the Leinster cup',
     'cql10r((1, 7, 0, 1), (0x1e, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mutation: a week later',
     'mwa_2 leinster cup rounds'),
    ('mwa2_regionais_deslocados',
     'write_usize(this.team_table() + k * 6, pool[k]);',
     'write_usize(this.team_table() + k * 6, pool[k + 1]); // mutation: the 2nd to the 5th',
     'mwa_2 leinster cup clubs'),
    ('mwa2_virada_ano_mais_dois',
     'start_new_cup_season(Cup(this), 0x636a80, 0x9e9f0c, [0x167, 0x16e, 0x175], 1, "mwa_2 leinster cup turnover")',
     'start_new_cup_season(Cup(this), 0x636a80, 0x9e9f0c, [0x167, 0x16e, 0x175], 2, "mwa_2 leinster cup turnover")',
     'mwa_2 leinster cup turnover'),
]
