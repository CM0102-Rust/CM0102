# Mutations (tel35_2). The automatic career opens no screen: the chain runs none. The builders' comparers catch them
# in a session of Lucas: the manager menu -> History (the written history list; open an entry for the .his screen and
# redraw it, e.g. by scrolling or coming back, since the first build only runs the original).
# - the written history entries with id 3 instead of 2;
# - the .his screen's Ok button with id 2.
MUTACOES = [
    ('tel35_2_entry_id', 'tel26_area_button([area, 0, k as u32, 1, 0, 1, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, entry as u32]);',
     'tel26_area_button([area, 0, k as u32, 1, 0, 1, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 3, entry as u32]);', 'tel35_2 written history list'),
    ('tel35_2_ok_id', 'HISTORY_FILE_TEXT as u32, 1, 0]);', 'HISTORY_FILE_TEXT as u32, 2, 0]);', 'tel35_2 history file screen'),
]
