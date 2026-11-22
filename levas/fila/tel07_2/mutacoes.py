# Mutations (tel07_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the compare screen with two players of the database.
# - "can play there" 15 -> 14: other stars and "n/a" in the position table (and the position words), caught by the
#   comparer (display list and 0xde1ce4..);
# - the goalkeeper's doubled attribute (row 13) counted three times: another goalkeeper word and stars.
MUTACOES = [
    ('tel07_2_pode_jogar_14',
     '    const CAN_PLAY: i8 = 15; // tel07_2 the "can play there" rating',
     '    const CAN_PLAY: i8 = 14; // tel07_2 mutation',
     'tel07_2 compare screen'),
    ('tel07_2_goleiro_x3',
     '            add(COMPARE_POS_GK, d * 2); // tel07_2 the goalkeeper\'s doubled attribute',
     '            add(COMPARE_POS_GK, d * 3); // tel07_2 mutation',
     'tel07_2 compare screen'),
]
