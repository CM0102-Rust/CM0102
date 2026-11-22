# The .his files (screens): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparers:
# - a single line break ends the paragraph (side by side by content): any club with a history -> History;
# - the category index shifted by one: Find -> History (the written history list; side by side on the return).
MUTACOES = [
    ('tel42_4_paragrafo',
     '                            if !line_break {\n                                line_break = true;',
     '                            if false && !line_break { // tel42_4 mutation\n                                line_break = true;',
     'tel42_4 .his paragraph'),
    ('tel42_4_categoria',
     '        .map_or(-1, |k| k as i32)',
     '        .map_or(-1, |k| k as i32 + 1) // tel42_4 mutation',
     'tel42_4 .his category'),
]
