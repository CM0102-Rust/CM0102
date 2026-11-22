# FA meeting (tel34_5): club screen > Board Confidence > FA Request; the fa_meeting_screen_draw comparer catches it
# once a request is picked.
# - the chosen request not highlighted (the other one is).
MUTACOES = [
    ('tel34_5_highlight',
     'let colour = if chosen == request { job_colour(JOB_COLOUR_BDEC) }',
     'let colour = if chosen != request { job_colour(JOB_COLOUR_BDEC) }'),
]
