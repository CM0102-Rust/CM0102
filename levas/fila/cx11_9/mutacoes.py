# Mutacoes (cx11_9), com a Italia e a base NORMAL ate 2002-09-30:
# - a melhor e a pior colocacao trocadas na C2/A e a atual no lugar da melhor na C2/B. Como na C1 (cqk_6), a carreira
#   quase nao le a previsao ate o fim da temporada, entao podem escapar dela; sao acusadas pelo comparador, lado a lado
#   nos 3 bytes da entrada, a cada atualizacao de status.
MUTACOES = [
    ('cx11_9_c2a_melhor_e_pior_trocadas',
     'set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8) // cx11_9: the C2/A progress',
     'set_c1_club_progress(this, club, stage as u8, current as u8, worst as u8, best as u8) // mutation: best and worst swapped',
     'cx11_9 serie c2 progress'),
    ('cx11_9_c2b_atual_como_melhor',
     'set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8) // cx11_9: the C2/B progress',
     'set_c1_club_progress(this, club, stage as u8, current as u8, current as u8, worst as u8) // mutation: the current as the best',
     'cx11_9 serie c2 progress'),
]
