# Mutations (tel28_2). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat), managing a national team:
# - the Yes button's id equal to No's (any squad move, selection or call-up dialog: the build's display-list comparer);
# - a squad at its maximum size taken as invalid (confirming a full squad selection: the build's comparer).
MUTACOES = [
    ('tel28_2_yes_id',
     'tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]); // tel28_2 Yes',
     'tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]); // tel28_2 mutation'),
    ('tel28_2_full_squad_invalid',
     'if count <= max && count >= min {',
     'if count < max && count >= min {'),
]
