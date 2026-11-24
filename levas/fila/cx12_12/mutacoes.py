# Mutacoes (cx12_12), com a Noruega ate 2002-08-01 (a virada norueguesa no fim de 2001, a temporada de 2002 a partir
# de abril):
# - os quatro promovidos da Segunda sao o 5o ao 8o da lista embaralhada (no lugar do 1o ao 4o): outros clubes na
#   Primeira de 2002 e outros jogos desde abril;
# - os doze primeiros do norte vao ao Grupo 2 e os doze seguintes ao Grupo 1: outros jogos nos grupos 1 e 2 da Segunda
#   de 2002 (os tamanhos iguais);
# - a vaga da repescagem 12 vira 10 como a melhor: a carreira quase nao le a previsao; acusada pelo comparador, lado a
#   lado nos 3 bytes.
MUTACOES = [
    ('cx12_12_outros_promovidos',
     'exe::league_move_club_to_division(this, read_usize(list + j * 4), comp_record_of_key(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), 1);',
     'exe::league_move_club_to_division(this, read_usize(list + (j + 4) * 4), comp_record_of_key(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), 1); // mutation: the next four',
     'cx12_12 norwegian second division promotion'),
    ('cx12_12_grupos_1_e_2_trocados',
     'Club(list[k as usize]).set_division(Competition(comp_record_of_key(norwegian_second_division_group(k)))); // cx12_12: by latitude',
     'Club(list[k as usize]).set_division(Competition(comp_record_of_key(norwegian_second_division_group(if k < 24 { (k + 12) % 24 } else { k })))); // mutation: groups 1 and 2 swapped',
     'cx12_12 norwegian second division regrouping'),
    ('cx12_12_premier_melhor_10',
     'premier_play_off_progress(this, club, current, best, worst, 0xc, 0xb, 0xe) // cx12_12: the play-off place',
     'premier_play_off_progress(this, club, current, best, worst, 0xc, 0xa, 0xe) // mutation: best 10',
     'cx12_12 norwegian premier progress'),
]
