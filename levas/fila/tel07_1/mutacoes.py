# Mutations (tel07_1): the screens batch runs no mutations in the chain (the automatic career opens no screen); these
# are for a session of Lucas with JOGAR_COMPARANDO.bat: open any person's profile.
# - param 0x10 written 0 instead of 1: the comparer's plan (written apart) catches it on the first profile opened.
MUTACOES = [
    ('tel07_1_param_10',
     '    set_screen_param(0x10, 1); // tel07_1 param 0x10',
     '    set_screen_param(0x10, 0); // tel07_1 mutation',
     'tel07_1 open the staff profile'),
]
