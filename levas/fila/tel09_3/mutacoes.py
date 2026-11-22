# Mutations (tel09_3): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: on a person's profile, click another tab of the tab bar (command 0x11, compared side by side).
# - the new tab written as value ^ 1: another parameter 1, caught by the comparer;
# - commands 0x3c / 0x3d writing the opposite value of parameter 0x14.
MUTACOES = [
    ('tel09_3_aba_trocada',
     '                set_screen_param(1, value() as usize);\n',
     '                set_screen_param(1, value() as usize ^ 1); // tel09_3 mutation\n',
     'tel09_3 profile input'),
    ('tel09_3_parametro_14',
     '            set_screen_param(0x14, (id == 0x3c) as usize);\n',
     '            set_screen_param(0x14, (id == 0x3d) as usize); // tel09_3 mutation\n',
     'tel09_3 profile input'),
]
