# Mutacoes (cqy_2). Os jogos entre as divisoes da MLS sao criados uma vez por temporada (contadores "cqy_2 mls row
# round" e "cqy_2 mls ordinary round"), na criacao da carreira de 8 ligas; a temporada da MLS (marco a outubro de 2001)
# esta toda dentro da janela:
# - a rodada 12 deixa de ser rodada de linhas e a 11 passa a ser (quem joga contra quem muda nas duas);
# - o confronto da segunda com a terceira divisao parte da ultima linha da segunda + 2 (outros adversarios);
# - mandante e visitante nunca trocam (o mando de metade dos jogos muda).
MUTACOES = [
    ('cqy2_mls_rodada_de_linhas_11_no_lugar_da_12',
     'k == 2 || k == 5 || k == 8 || k == 12 || k == 15 || k == 18 // cqy2: the row rounds',
     'k == 2 || k == 5 || k == 8 || k == 11 || k == 15 || k == 18 // mutation: round 11 instead of 12',
     'cqy_2 mls row round'),
    ('cqy2_mls_segunda_contra_terceira_deslocada',
     'b = (second_last_row as i8).wrapping_add(1); // cqy2: the second against the third from its last row',
     'b = (second_last_row as i8).wrapping_add(2); // mutation: two rows on',
     'cqy_2 mls ordinary round'),
    ('cqy2_mls_sem_trocar_o_mando',
     'core::mem::swap(&mut home, &mut away); // cqy2: home and away by the parity of the round',
     'let _ = (&mut home, &mut away); // mutation: never swapped',
     'cqy_2 mls ordinary round'),
]
