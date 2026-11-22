# Mutations (tel28_3). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): open the inbox on a read item (the build compares only then):
# - the open item's headline cell one pixel narrower (the build's display-list comparer);
# - the Next Unread button's id 6 instead of 5 (with unread news in the inbox).
MUTACOES = [
    ('tel28_3_open_headline_width', 'tel26_area_text([area, 1, *row as u32, 0x210, job_colour(NEWS_COLOUR_OPEN), 1, 2, colour, JOB_TEXT as u32]);', 'tel26_area_text([area, 1, *row as u32, 0x20f, job_colour(NEWS_COLOUR_OPEN), 1, 2, colour, JOB_TEXT as u32]);'),
    ('tel28_3_next_unread_id', '        tel26_add_button([0x28f, y2 as u32, 0x30c, (y2 + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 5, 0]);', '        tel26_add_button([0x28f, y2 as u32, 0x30c, (y2 + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 6, 0]);'),
]
