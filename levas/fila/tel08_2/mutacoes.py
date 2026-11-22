# Mutations (tel08_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the Profile tab of a player (view 0).
# - the attribute value printed one higher: other texts in the display list, caught by the comparer;
# - the columns balanced over 2 instead of 3: other column lengths and empty rows.
MUTACOES = [
    ('tel08_2_valor_mais_1',
     '                format(JOB_TEXT, 0x98806cusize, code as i8 as i32); // "%d" tel08_2 the attribute value',
     '                format(JOB_TEXT, 0x98806cusize, code as i8 as i32 + 1); // tel08_2 mutation',
     'tel08_2 profile tab player'),
    ('tel08_2_colunas_2',
     '    let (counts, tallest) = profile_columns(PROFILE_PLAYER_ROWS, 3, rows);',
     '    let (counts, tallest) = profile_columns(PROFILE_PLAYER_ROWS, 2, rows);',
     'tel08_2 profile tab player'),
]
