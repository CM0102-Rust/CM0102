# Mutations (tel33_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the credits screen (page 0: Sports Interactive, Jask, Research).
# - one credit line swapped: the first researcher (Argentina) shows the Australian researcher, another text in the list;
# - Grant Appleyard's row with the texts swapped (role on the left), another pair of texts in the list;
# - the opener leaves screen parameter 1 at 1: caught by the opener's plan check.
MUTACOES = [
    ('tel33_2_linha_trocada',
     '    (0x9b1698, 0x9b1688), // Argentina: Daniel Szmid',
     '    (0x9b1698, 0x9b166c), // Argentina: Daniel Szmid (mutation)',
     'tel33_2 research rows'),
    ('tel33_2_grant_invertido',
     '    credits_text_row(JOB_TEXT as u32, CREDITS_SECOND_TEXT as u32);',
     '    credits_text_row(CREDITS_SECOND_TEXT as u32, JOB_TEXT as u32); // mutation',
     'tel33_2 sports interactive rows'),
    ('tel33_2_parametro_1',
     '        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, 0usize, 0);',
     '        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, (k == 1) as usize, 0); // mutation',
     'tel33_2 credits opened'),
]
