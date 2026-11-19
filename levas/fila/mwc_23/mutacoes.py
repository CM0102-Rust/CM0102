# Mutacoes (mwc_23): as Supercopas da Belgica e da Holanda entram com o vice da liga no lugar do campeao (outro clube
# joga a Supercopa; o comparador lado a lado tambem pega); na Ligapokal ninguem entra com a classe 1 (o campeao da copa e o
# melhor colocado depois dele deixam de ser cabecas: os jogos da primeira fase mudam).
MUTACOES = [
    ('mwc23_vice_no_lugar_do_campeao',
     'write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwc23: the league champion',
     'write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 1)); // mutation: the runner-up',
     'mwc_23 super cup clubs'),
    ('mwc23_ligapokal_sem_cabecas',
     'if k >= 4 { // mwc23: the holder and the best placed after him seeded',
     'if k >= 6 { // mutation: nobody seeded',
     'mwc_23 german league cup entrants'),
]
