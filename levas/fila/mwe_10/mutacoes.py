# Mutations (mwe_10), at the end of the 2001 Swedish and Norwegian seasons (October/November): the Superettan
# promotion play-off with a promoted club (status 0) instead of the play-off one (another club plays the tie); the
# advance without bringing the Premier Division statuses up to date (one routine for both leagues: the play-off club
# of the Premier Division may be another, and the statuses and staff records written by the update change).
MUTACOES = [
    ('mwe10_playoff_sueco_com_o_promovido',
     'if let Some(c) = first_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) { // mwe10: the Superettan play-off club',
     'if let Some(c) = first_row_club_with_status(this, table_status::PROMOTED) { // mutation: a promoted club',
     'mwe_10 superettan play-off'),
    ('mwe10_sem_atualizar_a_primeira',
     'update_premier_division_status(premier); // mwe10: the Premier Division statuses brought up to date',
     '// mutation: without the Premier Division update',
     'mwe_10 first division play-off reached'),
]
