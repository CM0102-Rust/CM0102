# Mutacoes (cx03_20), carreira de 8 ligas ate 2002-06-01. Nenhuma abre caixa (as duas listas seguem com mais de 30).
# O comparador (os dois rodam, lado a lado nos elencos) acusa.
# - o Oeste com a conferencia do Leste na segunda passada;
# - os lesionados tambem convocados;
# - as estrelas so das tres primeiras linhas.
MUTACOES = [
    ('cx03_20_oeste_com_conferencia_do_leste',
     'collect_all_star_candidates(&mut west, second_stage, first_stage, 0x414, &date);',
     'collect_all_star_candidates(&mut west, second_stage, first_stage, 0x413, &date); // mutation'),
    ('cx03_20_lesionados_convocados',
     '                if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {',
     '                if false && exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 { // mutation'),
    ('cx03_20_tres_linhas',
     '    for row in 0..4usize {\n        for pass in 0..2 {',
     '    for row in 0..3usize { // mutation\n        for pass in 0..2 {'),
]
