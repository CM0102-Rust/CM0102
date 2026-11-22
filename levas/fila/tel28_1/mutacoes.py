# Mutations (tel28_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the club cells' button id one lower (any club tab of Choose Friendly Teams: the tab's display-list comparer);
# - the Non-League list capped at 249 instead of 250 (a nation with more than 250 non-league clubs, e.g. England).
MUTACOES = [
    ('tel28_1_club_cell_id',
     'tel26_area_button([area, column as u32, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club as u32]);',
     'tel26_area_button([area, column as u32, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d4, club as u32]);'),
    ('tel28_1_non_league_cap',
     'let n = if n >= 0xfa { 0xfa } else { n }; // tel28_1 the Non-League cap',
     'let n = if n >= 0xf9 { 0xf9 } else { n }; // tel28_1 mutation'),
]
