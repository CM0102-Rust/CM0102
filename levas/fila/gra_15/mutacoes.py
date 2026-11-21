# Mutacoes (gra_15). A rotina roda em toda partida (5160 chamadas na carreira ENG); os recordes em si nao voltam
# para a simulacao (as noticias vao para a fila e o texto so para o tecnico humano), mas os pontos do jogador decidem
# os sorteios da preferencia (utils_random 4000/4000/4000/7000 x ate 50 do elenco) e mudam o gerador desde os
# primeiros jogos da liga (18/08/2001):
# - o minimo de pontos para os sorteios 40 -> 30;
# - a nota acima de 7 vale (nota - 5)^4 em vez de (nota - 6)^4;
# - o melhor em campo vale 20 pontos em vez de 30;
# - a media de publico da temporada dividida por jogos + 1 (so o comparador acusa: o primeiro jogo em casa da liga).
MUTACOES = [
    ('gra15_pontos_minimos',
     'if (p as i16) < 40 {',
     'if (p as i16) < 30 {',
     'gra_15 liking draws'),
    ('gra15_nota',
     'points = ftol(game_pow((rating as i32 - 6) as f64, read_f64(rdata::F64_4)));',
     'points = ftol(game_pow((rating as i32 - 5) as f64, read_f64(rdata::F64_4)));',
     'gra_15 player points'),
    ('gra15_melhor_em_campo',
     'points = points.wrapping_add(30);',
     'points = points.wrapping_add(20);',
     'gra_15 man of the match'),
    ('gra15_media_de_publico',
     'total.wrapping_div(home as i32) } else { 0 });',
     'total.wrapping_div(home as i32 + 1) } else { 0 });',
     'gra_15 home match'),
]
