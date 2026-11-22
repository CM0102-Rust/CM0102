# Mutations (tel25_9). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat; the club screen's Print, to a file, with the fixtures ticked, after a few
# matches played):
# - the attendance column one narrower (caught by the fixtures' text comparer on any played match);
# - the scorers line broken at 40 characters instead of 50 (caught on a match with several scorers).
MUTACOES = [
    ('tel25_9_attendance_width', 'printout_column(t, 6);\n            let side', 'printout_column(t, 5);\n            let side'),
    ('tel25_9_scorers_width', 'const PRINTOUT_SCORERS_WIDTH: usize = 50;', 'const PRINTOUT_SCORERS_WIDTH: usize = 40;'),
]
