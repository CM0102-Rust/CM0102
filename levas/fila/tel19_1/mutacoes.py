# Mutations (tel19_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): open the player search (Search menu > Player Search, or a scout's list),
# with results, then change the view or the page so the screen is built again (the first build runs the search and
# only the original runs).
# - the found line with "(0 filtered out)" when nothing was filtered: another text in the display list;
# - the "Inf" column header sorting by bit 3 instead of 2: another item value in the display list.
MUTACOES = [
    ('tel19_1_found_line', '    if total > found as i32 {', '    if total >= found as i32 { // tel19_1 mutation',
     'tel19_1 search screen build'),
    ('tel19_1_header_inf', '    finder_header(area, 1, 0xc, 2);', '    finder_header(area, 1, 0xc, 3); // tel19_1 mutation',
     'tel19_1 search screen build'),
]
