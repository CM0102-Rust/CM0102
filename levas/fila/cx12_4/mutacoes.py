# Mutacoes (cx12_4), com a Italia e a base NORMAL ate 2002-09-30 (a virada no verao de 2002 e a temporada nova
# comecando em 5 de setembro):
# - a repescagem de acesso da C2/C recebe o calendario do play-out (1 rodada em vez de 2: sem a final, outro clube
#   sobe em maio de 2002; acusada tambem pelo comparador, lado a lado);
# - o ano da virada sobe 2 em vez de 1: as rodadas da temporada 2002/03 da C2/C vao para 2003 (nenhum jogo da C2/C em
#   setembro de 2002). O comparador da virada tambem acusa (o ano tem de subir 1).
MUTACOES = [
    ('cx12_4_c2c_repescagem_de_uma_rodada',
     'create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea83c, rows: [0xab, 0x155, 0x18f] }) // cx12_4: the C2/C schedule',
     'create_c1_schedule(League(this), if stage == 0 { 1 } else { stage }, rounds, extra, league, &C1BracketVariant { file: 0x9ea83c, rows: [0xab, 0x155, 0x18f] }) // mutation: the play-out schedule',
     'cx12_4 serie c2 c schedule'),
    ('cx12_4_c2c_ano_mais_2',
     'rebuild_season(this, 0x65c0e0, 0x9ea83c, [0x220, 0x227, 0x22d, 0x234], 1) { // cx12_4: the turnover of Serie C2 C',
     'rebuild_season(this, 0x65c0e0, 0x9ea83c, [0x220, 0x227, 0x22d, 0x234], 2) { // mutation: the year plus 2',
     'cx12_4 serie c2 c rollover'),
]
