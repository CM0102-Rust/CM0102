# Mutations (tel04_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat:
# - a venue choice of Past Meetings keeps the Anywhere bit: the input comparer catches the filter (param 2) when
#   Filter > At <club> is picked with Anywhere ticked (the screen opens with it);
# - the "Date Available" rows of the friendly dates get id 7 instead of 8: the build comparer catches the display list
#   (Arrange Friendly on a page without fixtures of the club).
MUTACOES = [
    ('tel04_1_bits_do_lugar',
     '                0x20 | 0x40 | 0x80 | 0x100 => Some(filter & !0x1e0 | value), // tel04_1 the venue bits',
     '                0x20 | 0x40 | 0x80 | 0x100 => Some(filter & !0x1c0 | value), // tel04_1 mutation',
     'tel04_1 past meetings build'),
    ('tel04_1_data_disponivel',
     '            tel26_area_button([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32, 8, row]); // tel04_1 the available date',
     '            tel26_area_button([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32, 7, row]); // tel04_1 mutation',
     'tel04_1 friendly dates build'),
]
