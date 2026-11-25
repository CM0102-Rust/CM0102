# A text writer: the text does not go back into the simulation, so the comparer (compare_text_writer, jogo real) catches
# the mutation, not the career.
# - rxf_4_voto_sem_reputacao: the vote line drops the "{}" argument 0x64 before the club name (the text changes).
MUTACOES = [
    ('rxf_4_voto_sem_reputacao',
     '        fmt(JOB_TEXT, SHORTLIST_VOTED_WITH_CLUB, g, g, 0x64, c.addr() + club::SHORT_NAME, name.as_ptr()); // rxf_4: the vote line',
     '        fmt(JOB_TEXT, SHORTLIST_VOTED_WITH_CLUB, g, g, c.addr() + club::SHORT_NAME, name.as_ptr(), 0); // rxf_4 mutation',
     'rxf_4 shortlist news with the vote'),
    ('rxf_4_manchete_longa',
     '        fmt(JOB_TEXT, SHORTLIST_DECIDED, g, g, comp + competition::SHORT_NAME);',
     '        fmt(JOB_TEXT, SHORTLIST_DECIDED, g, g, comp + competition::NAME); // rxf_4 mutation',
     'rxf_4 shortlist news without a vote'),
]
