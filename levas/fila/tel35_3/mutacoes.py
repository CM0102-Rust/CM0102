# Mutations (tel35_3). The automatic career opens no screen: the chain runs none. The builders' comparers catch them
# in a session of Lucas: the manager menu -> Go On Holiday / Retire.
# - "Go on holiday ?" one pixel lower;
# - the retire question in font 3.
MUTACOES = [
    ('tel35_3_holiday_y', 'tel35_text_item([0x6e, 0xa5, 0x30c, 0xc8,', 'tel35_text_item([0x6e, 0xa6, 0x30c, 0xc8,', 'tel35_3 holiday'),
    ('tel35_3_retire_font', 'tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);',
     'tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0]);', 'tel35_3 retire'),
]
