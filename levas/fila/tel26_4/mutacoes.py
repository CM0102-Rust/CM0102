# Mutations (tel26_4). The automatic career opens no screen: the chain runs none. The builders' comparers (display
# list side by side) catch them in a session of Lucas's: a club's profile -> Apply for job / the manager's club ->
# Board Confidence.
# - the confirm screens' "Yes" button with id 3 (apply, resign and job offer share the builder);
# - the "Board Request" button moved one pixel.
MUTACOES = [
    ('tel26_4_yes_id', '    text(JOB_TEXT, 0x98e8d0);\n    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);', '    text(JOB_TEXT, 0x98e8d0);\n    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);'),
    ('tel26_4_board_request_x', 'tel26_add_button([0x6e, 0x82, 0xeb, 0x96,', 'tel26_add_button([0x6f, 0x82, 0xeb, 0x96,'),
]
