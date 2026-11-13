# Mutacoes (mwa_5), na Supercopa da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01 (a edicao de 2001 e jogada
# em 5/8/2001):
# - o jogo uma semana depois (5/8 -> 12/8);
# - o vice da liga no lugar do campeao (outro clube joga);
# - a virada sobe o ano em 2 (a edicao de 2002 e depois da data final: so o comparador da virada acusa).
MUTACOES = [
    ('mwa5_jogo_semana_depois',
     'cql10r((5, 6, 0, 5), (5, 7, 0, 5, 1, 0), 0, (0, 0x200, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwa5: the Charity Shield match',
     'cql10r((5, 6, 0, 5), (0xc, 7, 0, 5, 1, 0), 0, (0, 0x200, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutation: a week later',
     'mwa_5 northern irish charity shield rounds'),
    ('mwa5_vice_da_liga',
     'let b = latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0);',
     'let b = latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 1); // mutation: the runner-up',
     'mwa_5 northern irish charity shield clubs'),
    ('mwa5_virada_ano_mais_dois',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x78a1e0, 0xa695f0, [0x125, 0x12c, 0x133], 1, false,',
     'usa_mls_all_stars_cup_start_new_season(Cup(this), 0x78a1e0, 0xa695f0, [0x125, 0x12c, 0x133], 2, false,',
     'mwa_5 northern irish charity shield turnover'),
]
