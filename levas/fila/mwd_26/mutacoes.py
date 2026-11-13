# Mutacoes (mwd_26): a Supercopa entra com o vice da liga no lugar do campeao (Polonia, 18/7/2002, e Grecia): outro clube
# joga a Supercopa; o comparador lado a lado tambem pega.
MUTACOES = [
    ('mwd26_vice_no_lugar_do_campeao',
     'write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwd26: the league champion',
     'write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 1)); // mutation: the runner-up',
     'mwd_26 super cup clubs'),
]
