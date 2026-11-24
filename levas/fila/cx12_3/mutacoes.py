# Mutacoes (cx12_3), com a Italia e a base NORMAL ate 2002-09-30:
# - a vitoria vale 2 pontos na C2/C: a tabela muda de ordem ao longo da temporada, e com ela os da repescagem, do
#   play-out e o campeao (acusada tambem pelo comparador das regras, lado a lado).
# O construtor e a tabela ficam sem mutacao propria: os moldes (construct_italian_division, ita_ser_a_fill_league_table)
# ja sao provados na C1 (cqm_4, cqj_4) e na C2/A (cx11_2, cx11_3); as linhas trocadas ou contadas de outro jeito
# derrubam o jogo (provado no cqj_4).
MUTACOES = [
    ('cx12_3_c2c_vitoria_vale_2',
     '*v.last_mut().unwrap() = (league::LOWER_DIVISION_ID, 4, 0xffff_ffff); // cx12_3: nothing below Serie C2/C',
     '*v.last_mut().unwrap() = (league::LOWER_DIVISION_ID, 4, 0xffff_ffff); v.push((league::WIN_POINTS, 1, 2)); // mutation: 2 points',
     'cx12_3 serie c2 c rules'),
]
