# Mutacoes (mwf_8): o ano de cada virada sobe 2 em vez de 1 (as rodadas, os sorteios e as datas da temporada
# 2002/03 mudam de ano; acusadas pelo estado no fim da carreira, 2002-08-01).
MUTACOES = [
    ('mwf8_itaserc1b_ano_mais_2',
     'rebuild_season(this, 0x64f820, 0x9ea558, [0x224, 0x22b, 0x231, 0x238], 1) { // mwf8: the turnover of Serie C1 B',
     'rebuild_season(this, 0x64f820, 0x9ea558, [0x224, 0x22b, 0x231, 0x238], 2) { // mutation: the year plus 2',
     'mwf_8 Serie C1 B rollover'),
    ('mwf8_spasecond_ano_mais_2',
     'rebuild_season(this, 0x855b40, 0xa828bc, [0x312, 0x319, 0x31f, 0x326], 1) { // mwf8: the turnover of the Spanish Second Division',
     'rebuild_season(this, 0x855b40, 0xa828bc, [0x312, 0x319, 0x31f, 0x326], 2) { // mutation: the year plus 2',
     'mwf_8 Spanish Second Division rollover'),
    ('mwf8_spalower_ano_mais_2',
     'let year = 1; // mwf8: the year of the Spanish lower league',
     'let year = 2; // mutation: the year plus 2',
     'mwf_8 spanish lower league rollover'),
]
