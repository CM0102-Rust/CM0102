# Search For (tel34_6): menu bar > Search, Nation tab (the draw_find_nations_results comparer, on a rebuild after the
# first search) and the Players tab with a search giving more than 100 results, then Next (the
# find_screen_players_input comparer).
# - the federation column shows the continent's name;
# - the page moves by 15.
MUTACOES = [
    ('tel34_6_federation',
     'tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, continent + CONTINENT_FEDERATION_NAME);',
     'tel34_find_cell_text(area, 2, row, JOB_COLOUR_BDD4, continent + CONTINENT_NAME);'),
    ('tel34_6_page',
     '        4 => on(start.wrapping_add(0x10)),',
     '        4 => on(start.wrapping_add(0xf)),'),
]
