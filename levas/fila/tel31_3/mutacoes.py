# Mutations (tel31_3), to prove the comparers in a session of Lucas's (JOGAR_COMPARANDO.bat):
# - the terminate screen's "No" button carries the "Yes" id (display list comparer of the build);
# - the squad order by statistic 7 instead of 6 (compare_order, when a squad list sorts by appearances).
MUTACOES = [
    ('tel31_3_terminate_no_id',
     'JOB_TEXT as u32, TERMINATE_NO as u32, 0]);',
     'JOB_TEXT as u32, TERMINATE_YES as u32, 0]);'),
    ('tel31_3_squad_stat',
     'sb, 6, 1, 6.5) } else { 0.0 };',
     'sb, 7, 1, 6.5) } else { 0.0 };'),
]
