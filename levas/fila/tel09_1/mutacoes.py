# Mutations (tel09_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the Transfer tab of a player of another club (the accessors run only there).
# - the morale read from the Bosman byte: another value, caught by the accessor's comparer;
# - the board confidence read two bytes further: another word, caught by the accessor's comparer.
MUTACOES = [
    ('tel09_1_moral_bosman',
     '    Contract(this).morale() as u8\n',
     '    Contract(this).leaving_on_bosman() // tel09_1 mutation\n',
     'tel09_1 contract morale'),
    ('tel09_1_confianca_2',
     '.wrapping_mul(0x49)) + board_record::CONFIDENCE)\n',
     '.wrapping_mul(0x49)) + board_record::CONFIDENCE + 2) // tel09_1 mutation\n',
     'tel09_1 board confidence'),
]
