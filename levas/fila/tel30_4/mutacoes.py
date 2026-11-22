# Mutations (tel30_4). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): the board confidence screen of his club > "Board Request" (the board
# request screen, belgium_rules.cpp), then a click on a request.
MUTACOES = [
    ('tel30_4_chosen_colour', '(read_u16(G_GUI_COLOUR_YELLOW) as u32, BOARD_REQUEST_ITEM_REQUEST as u32)',
     '(read_u16(G_GUI_COLOUR_PALE_GREY) as u32, BOARD_REQUEST_ITEM_REQUEST as u32)'),
    ('tel30_4_request_redraw', 'set_screen_param(1, tel26_item_value(item) as usize);\n        return -11;',
     'set_screen_param(1, tel26_item_value(item) as usize);\n        return -9;'),
]
