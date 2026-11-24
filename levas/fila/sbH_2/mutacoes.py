# Mutacoes (sbH_2) pela carreira com a Finlandia ate 2002-08-01 (as repescagens em outubro de 2001; os quatro da
# Kakkonen sobem na virada do fim de 2001, pelo FinPrm::refresh_first_division_clubs):
# - a repescagem de acesso sem o cruzamento (primeiro contra primeiro): os jogos e quem sobe mudam (o comparador acusa);
# - tres promovidos do norte (e nao dois): um dos clubes que sobem para a Ykkonen muda na virada;
# - a repescagem de rebaixamento com o setimo no lugar do oitavo do grupo norte: o jogo e quem cai mudam.
MUTACOES = [
    ('sbH_2_sem_cruzamento',
     '    for (k, club) in [table_club(l, 0), table_club(south, 1), table_club(l, 1), table_club(south, 0)].into_iter().enumerate() {',
     '    for (k, club) in [table_club(l, 0), table_club(south, 0), table_club(l, 1), table_club(south, 1)].into_iter().enumerate() { // mutation',
     'sbH_2 finnish first division play-offs'),
    ('sbH_2_tres_do_norte',
     '    for (side, upto) in [(Some(true), 2), (Some(false), 2), (None, 4)] {',
     '    for (side, upto) in [(Some(true), 3), (Some(false), 3), (None, 4)] { // mutation',
     'sbH_2 finnish first division play-offs'),
    ('sbH_2_setimo_do_norte',
     '    let mut clubs = [table_club(l, 7), 0, table_club(south, 7), 0];',
     '    let mut clubs = [table_club(l, 6), 0, table_club(south, 7), 0]; // mutation',
     'sbH_2 finnish first division play-offs'),
]
