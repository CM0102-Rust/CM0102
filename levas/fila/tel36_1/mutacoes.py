# Mutations (tel36_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat; the screen: a player's profile > Action > treatment / leave of absence).
# - the "Yes" button gets id 4 (every confirm screen of tel36 changes its display list);
# - the physiotherapy question takes the surgery text.
MUTACOES = [
    ('tel36_1_yes_id', 'tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);',
     'tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 4, 0]);'),
    ('tel36_1_physio_question', 'TREATMENT_PHYSIO => 0xa7a27c,', 'TREATMENT_PHYSIO => 0xa7a1d8,'),
]
