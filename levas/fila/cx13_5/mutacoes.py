# Mutacoes (cx13_5), com a Polonia ate 2002-08-01:
# - as posicoes previstas de tras para frente (15 - k: acusada pelo comparador, lado a lado; as expectativas da
#   diretoria leem a posicao prevista);
# - a ordem da fase final invertida (o grupo de rebaixamento antes do do titulo: muda as posicoes da fase final e quem
#   cai e e campeao em maio de 2002; acusada tambem pelo compare_order!);
# - a cadeia sem a troca (ninguem sobe nem desce entre a Primeira e a Segunda polonesas: outra temporada 2002/03).
MUTACOES = [
    ('cx13_5_previstas_invertidas',
     'exe::comp_stats_set_club_final_position(l.comp_stats(), club, k); // cx13_5: the predicted place',
     'exe::comp_stats_set_club_final_position(l.comp_stats(), club, 0xf - k); // mutation: backwards',
     'cx13_5 polish predicted positions'),
    ('cx13_5_rebaixamento_antes',
     'rank(a) - rank(b) // cx13_5: the championship group first',
     'rank(b) - rank(a) // mutation: the relegation group first',
     'cx13_5 polish final order'),
    ('cx13_5_cadeia_sem_troca',
     'exe::league_promote_and_relegate(this, final_stage, comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), 1, 1, -1, -1) // cx13_5: the final stage and the Second',
     '{ let _ = final_stage; 0 } // mutation: nobody moves',
     'cx13_5 polish chain'),
]
