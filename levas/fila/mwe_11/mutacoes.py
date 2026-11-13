# Mutations (mwe_11): the Norwegian play-off with the last promoted club (status 0) instead of the play-off one
# (another club plays the November 2001 tie); the Ykkonen advance at stage 2 without bringing the Veikkausliiga
# statuses up to date (the Premier Division play-off club and the statuses written by the update); the group
# progress codes as 2v instead of 2v - 1 (the Comp_Stats positions of every Ykkonen club in the 2001 autumn; also
# caught by the side-by-side comparer).
MUTACOES = [
    ('mwe11_playoff_noruegues_com_o_promovido',
     'let Some(c) = last_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) else { // mwe11: the Norwegian play-off club',
     'let Some(c) = last_row_club_with_status(this, table_status::PROMOTED) else { // mutation: a promoted club',
     'mwe_11 norwegian play-off'),
    ('mwe11_sem_atualizar_a_veikkausliiga',
     'update(p); // mwe11: the Veikkausliiga statuses brought up to date',
     '// mutation: without the Veikkausliiga update',
     'mwe_11 ykkonen next stage built'),
    ('mwe11_grupos_sem_intercalar',
     'c = c.wrapping_shl(1).wrapping_sub(1); // mwe11: the group places interleaved',
     'c = c.wrapping_shl(1); // mutation: 2v',
     'mwe_11 ykkonen group progress'),
]
