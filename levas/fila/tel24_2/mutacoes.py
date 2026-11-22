# Mutations (tel24_2). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the bonus popup steps built with the next type's id (the bonuses tab comparer, Bonuses tab with a bonus set);
# - an offered release clause shown as not offered (id 0x12; the offer screen comparer, Clauses tab);
# - the squad player answer storing the next person (the offer screen answer, side by side over the params).
MUTACOES = [
    ('tel24_2_bonus_popup_id', 'ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, 0x2a + k as u32, cash, popup as u16 as u32]);',
     'ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, 0x2b + k as u32, cash, popup as u16 as u32]);'),
    ('tel24_2_clause_offered_id', 'tel26_area_button([labels, 2, *row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0x13, k]);',
     'tel26_area_button([labels, 2, *row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0x12, k]);'),
    ('tel24_2_squad_player', 'set_screen_param(0, tel26_item_value(item16) as usize);',
     'set_screen_param(0, tel26_item_value(item16) as usize + 1);'),
]
