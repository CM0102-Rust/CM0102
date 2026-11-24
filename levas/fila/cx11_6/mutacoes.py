# Mutacoes (cx11_6), com a Italia e a base NORMAL ate 2002-09-30 (a virada no verao de 2002 e a temporada nova
# comecando em 5 de setembro):
# - o ano de cada virada sobe 2 em vez de 1: as rodadas da temporada 2002/03 da C2/A e da C2/B vao para 2003 (nenhum
#   jogo das C2 em setembro de 2002). O comparador da virada tambem acusa (o ano tem de subir 1).
MUTACOES = [
    ('cx11_6_c2a_ano_mais_2',
     'rebuild_season(this, 0x653a90, 0x9ea654, [0x220, 0x227, 0x22d, 0x234], 1) { // cx11_6: the turnover of Serie C2 A',
     'rebuild_season(this, 0x653a90, 0x9ea654, [0x220, 0x227, 0x22d, 0x234], 2) { // mutation: the year plus 2',
     'cx11_6 Serie C2 rollover'),
    ('cx11_6_c2b_ano_mais_2',
     'rebuild_season(this, 0x657f50, 0x9ea75c, [0x226, 0x22d, 0x233, 0x23a], 1) { // cx11_6: the turnover of Serie C2 B',
     'rebuild_season(this, 0x657f50, 0x9ea75c, [0x226, 0x22d, 0x233, 0x23a], 2) { // mutation: the year plus 2',
     'cx11_6 Serie C2 rollover'),
]
