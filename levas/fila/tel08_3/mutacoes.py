# Mutations (tel08_3): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the Transfer tab of a player.
# - an unlisted player's availability shown as "Listed for loan": another text in the display list, caught by the
#   comparer;
# - the empty label cell also on the first row of a section (>= instead of >): one more item in the Future section of
#   any player with a contract.
MUTACOES = [
    ('tel08_3_disponibilidade',
     '        lang(JOB_TEXT, 0xa8ae3cusize, TRANSFER_TAB_COMMENT); // "  Unknown<%s - COMMENT - unknown availabilty>" tel08_3 unknown availability',
     '        lang(JOB_TEXT, 0xa8ae6cusize); // tel08_3 mutation',
     'tel08_3 transfer tab'),
    ('tel08_3_celula_vazia',
     '        if self.row > self.first {',
     '        if self.row >= self.first { // tel08_3 mutation',
     'tel08_3 transfer tab mood'),
]
