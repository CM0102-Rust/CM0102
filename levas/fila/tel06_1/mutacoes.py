# Mutations (tel06_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat (each one must show as a divergence):
# - the away side's score printed home:away: caught side by side by the fixture row comparer on the Fixtures tab of
#   any club with a played away match;
# - the unfit threshold at 80% instead of 65%: caught by the status code comparer on the Squad tab when a player's
#   condition is between 65% and 80% (after a match day).
MUTACOES = [
    ('tel06_1_placar_visitante',
     '            lang(JOB_TEXT, SCORE, away, home);',
     '            lang(JOB_TEXT, SCORE, home, away);',
     'tel06_1 fixture row away score'),
    ('tel06_1_condicao',
     '    if condition < 0x41 {',
     '    if condition < 0x51 {',
     'tel06_1 status unfit'),
]
