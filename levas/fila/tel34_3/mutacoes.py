# Ban appeal (tel34_3): a ban news item's Appeal button; the discipline_appeal_dialog_build comparer catches both.
# - a day ban asked with the game ban text;
# - a one-game ban treated as no game ban.
MUTACOES = [
    ('tel34_3_day_text',
     'lang(JOB_TEXT, 0x9c35d8, 100, 100, number.as_ptr(), name.as_ptr());',
     'lang(JOB_TEXT, 0x9c364c, 100, 100, number.as_ptr(), name.as_ptr());'),
    ('tel34_3_games',
     '    if games > 0 {\n        crt(number',
     '    if games > 1 {\n        crt(number'),
]
