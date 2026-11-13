# Mutacoes (mwf_7): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf7_frasecond_ano_mais_2',
     'rebuild_season(this, 0x5bef10, 0x9ccc0c, [0x1e4, 0x1ec, 0x1f2, 0x1f9], 1) { // mwf7: the turnover of the French Second Division',
     'rebuild_season(this, 0x5bef10, 0x9ccc0c, [0x1e4, 0x1ec, 0x1f2, 0x1f9], 2) { // mutation: the year plus 2',
     'mwf_7 French Second Division rollover'),
    ('mwf7_frathird_ano_mais_2',
     'rebuild_season(this, 0x5c05d0, 0x9ccc80, [0x1ce, 0x1d5, 0x1db, 0x1e2], 1) { // mwf7: the turnover of the French National',
     'rebuild_season(this, 0x5c05d0, 0x9ccc80, [0x1ce, 0x1d5, 0x1db, 0x1e2], 2) { // mutation: the year plus 2',
     'mwf_7 French National rollover'),
    ('mwf7_itaserc1a_ano_mais_2',
     'rebuild_season(this, 0x64b670, 0x9ea474, [0x220, 0x227, 0x22d, 0x234], 1) { // mwf7: the turnover of Serie C1 A',
     'rebuild_season(this, 0x64b670, 0x9ea474, [0x220, 0x227, 0x22d, 0x234], 2) { // mutation: the year plus 2',
     'mwf_7 Serie C1 A rollover'),
]
