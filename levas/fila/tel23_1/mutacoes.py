# Mutations (tel23_1). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat), on the competition history screen:
# - the record text drawn 8 pixels further right (any list tab from its second build on: the tab comparer catches the
#   display list);
# - the Players tab's View pick written to the Teams view param (Players tab, pick another type in View: the click
#   comparer catches the params).
MUTACOES = [
    ('tel23_1_record_text_x', 'history_draw_wrapped(t, area, 0xbe, 0x2ee', 'history_draw_wrapped(t, area, 0xc6, 0x2ee'),
    ('tel23_1_players_view_param', 'set_screen_param(0x17, tel26_item_value(w) as usize);', 'set_screen_param(0x14, tel26_item_value(w) as usize);'),
]
