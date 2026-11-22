# Mutations (tel23_2). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the league progress table's club cells in style 1 instead of 2 (the league progress screen, e.g. a fixture's link
#   in the Fixtures tab of a league: the build comparer catches the display list);
# - the FIFA rankings' "one page on" button moving two pages (FIFA World Rankings, the ">" page button: the click
#   comparer catches param 2).
MUTACOES = [
    ('tel23_2_progress_club_style', 'tel26_area_button([area, 1, r as u32, 2, 0, 1, 2, colour, JOB_TEXT as u32, 1, row as u32]);',
     'tel26_area_button([area, 1, r as u32, 2, 0, 1, 1, colour, JOB_TEXT as u32, 1, row as u32]);'),
    ('tel23_2_fifa_page_on', '3 => (page + 1).min(pages),', '3 => (page + 2).min(pages),'),
]
