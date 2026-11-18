# Mutacoes (mwf_15), na virada francesa de meados de 2002: os clubes do CFA ordenados sem embaralhar (largura 1 em vez
# de 4: os que sobem para o National mudam, o mesmo numero); o ano da Primeira sobe 2 (as datas de 2002/03 mudam de
# ano). Acusadas pelo estado no fim da carreira, 2002-08-01.
MUTACOES = [
    ('mwf15_cfa_sem_embaralhar',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 4); // mwf15: the CFA clubs by reputation, shuffled in fours',
     'exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 1); // mutation: no shuffle',
     'mwf_15 cfa up to national'),
]
