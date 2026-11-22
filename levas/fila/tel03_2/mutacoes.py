# Mutations (tel03_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat:
# - the landmark text column always narrow when the rows do not fit: another x1/x2 in the Landmarks tab's text, caught
#   by the build's display-list comparer (open History > Landmarks of a club with more landmarks than rows);
# - the next page goes two pages on: caught by the input comparer (the params) on the Landmarks page button of a club
#   with over 100 landmarks.
MUTACOES = [
    ('tel03_2_coluna_marcos',
     '        let x2 = if count > rows && count <= 100 { 0x2f6 } else { 0x30c }; // tel03_2 the landmark text column',
     '        let x2 = if count > rows && count <= 100 { 0x2f0 } else { 0x30c }; // tel03_2 mutation',
     'tel03_2 club history build'),
    ('tel03_2_pagina_seguinte',
     '                set_screen_param(0x20, (page + 1).min(pages) as usize); // tel03_2 the next page',
     '                set_screen_param(0x20, (page + 2).min(pages) as usize); // tel03_2 mutation',
     'tel03_2 club history build'),
]
