# Mutations (tel27_1). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the year award's earlier seasons go one row down (Competition screen > Awards > a year award, e.g. the Premier's
#   Player of the Year: the season column under the top three);
# - the team award's older-team arrow moves two teams (Awards > a team award, e.g. the Team of the Year > "<< Year").
MUTACOES = [
    ('tel27_1_year_rows', "tel26_area_text([area, 0, row, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);",
     "tel26_area_text([area, 0, row + 1, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);"),
    ('tel27_1_team_older', "set_screen_param(1, year.wrapping_add(1));",
     "set_screen_param(1, year.wrapping_add(2));"),
]
