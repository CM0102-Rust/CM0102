# Mutations (tel24_1). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - a squad role button of the set role screen built with id 4 (the display list comparer of the set role screen);
# - the job button answer kept in param 1 instead of 4 (the set role answer, side by side over the screen params);
# - the staff offer dialog saying "Withdraw ..." when offering (the display list comparer of the dialog).
MUTACOES = [
    ('tel24_1_role_button_id', 'tel26_area_button([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32, 3, role as i32 as u32]);',
     'tel26_area_button([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32, 4, role as i32 as u32]);'),
    ('tel24_1_job_param', 'set_screen_param(4, tel26_item_value(item16) as usize);',
     'set_screen_param(1, tel26_item_value(item16) as usize);'),
    ('tel24_1_offer_text', 'lang(TS_TEXT, 0x9b1018, n); // "Offer contract to <name> ?"',
     'lang(TS_TEXT, 0x9b1054, n); // "Offer contract to <name> ?"'),
]
