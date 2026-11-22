# Mutations (tel23_3). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the stage menu items' value off by one stage (a cup or a league with stages: its Table/Draw tab's Stage menu, or
#   the Average Points tab: the item comparer catches the display list);
# - an unplayed match's "v" link in style 1 (the competition screen's Fixtures tab, or the Results tab on a day still
#   to play: the fixture row comparer catches the display list).
MUTACOES = [
    ('tel23_3_stage_item_value', 'let value = (stage + 1) * 1000 + index;', 'let value = (stage + 2) * 1000 + index;'),
    ('tel23_3_unplayed_link_style', 'tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32, 0x7d2, fixture as u32]);',
     'tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 1, score, JOB_TEXT as u32, 0x7d2, fixture as u32]);'),
]
