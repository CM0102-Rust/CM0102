# Mutations (tel35_1). The automatic career opens no screen: the chain runs none. The builder's comparer (display list
# side by side) catches them in a session of Lucas: a player's profile -> Discipline Player (Fine.cpp).
# - the reason buttons with id 4 instead of 3;
# - the period text in column 4 instead of 3.
MUTACOES = [
    ('tel35_1_reason_id', 'tel26_area_button([reasons, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 3, k as u32]);',
     'tel26_area_button([reasons, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 4, k as u32]);', 'tel35_1 discipline reasons'),
    ('tel35_1_period_column', 'tel26_area_text([area, 3, row as u32, 1, tel35_colour(G_GUI_COLOUR_BLACK), 0xc, 3, pale, JOB_TEXT as u32]);',
     'tel26_area_text([area, 4, row as u32, 1, tel35_colour(G_GUI_COLOUR_BLACK), 0xc, 3, pale, JOB_TEXT as u32]);', 'tel35_1 discipline period'),
]
