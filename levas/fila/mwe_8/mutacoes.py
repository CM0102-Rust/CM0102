# Mutations (mwe_8): the Portuguese Premier Division TV money doubled at the creation of the game (the clubs' cash in
# the 2001/02 season); the Liga de Honra prize share changed (paid at the end of the season, in May 2002); the Liga de
# Honra turnover two years ahead (in the summer of 2002: the year and the new schedule change).
MUTACOES = [
    ('mwe8_tv_da_primeira_portuguesa',
     'eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwe8: the Portuguese Premier TV money',
     'eng_prm_pay_tv_rights(this, 0x1e8480, 0); // mutation: twice the TV money',
     'mwe_8 portuguese premier table'),
    ('mwe8_premio_da_liga_de_honra',
     'let f = exe::league_calc_prize_fund(this, 0xc350); // mwe8: the Liga de Honra prize share',
     'let f = exe::league_calc_prize_fund(this, 0x1c350); // mutation: another share',
     'mwe_8 liga de honra table'),
    ('mwe8_liga_de_honra_dois_anos',
     'if !rebuild_season(this, 0x7d1520, 0xa7875c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1) { // mwe8: the Liga de Honra next year',
     'if !rebuild_season(this, 0x7d1520, 0xa7875c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 2) { // mutation: two years ahead',
     'mwe_8 liga de honra turnover'),
]
