# Mutations (tel32_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat: open the credits screen and let it scroll through the sections.
# - the right column of a name list starts at x 0x1c3: another item in the display list (Testing names);
# - the heading value 0x64 -> 0x65: another heading item (every section);
# - the role column in the 0xaebdec colour: another colour word in the display list (Feral, Eidos, Translation).
MUTACOES = [
    ('tel32_1_coluna_direita',
     '                tel32_cell(0x1c2, 0x2f8, i, job_colour(JOB_COLOUR_3184), name);',
     '                tel32_cell(0x1c3, 0x2f8, i, job_colour(JOB_COLOUR_3184), name);',
     'tel32_1 testing rows'),
    ('tel32_1_titulo_64',
     '        tel32_item([2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, bdec, 1, 1, bdea, text, 0, 0x64, value, heading_area]);',
     '        tel32_item([2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, bdec, 1, 1, bdea, text, 0, 0x65, value, heading_area]);',
     'tel32_1 testing rows'),
    ('tel32_1_cor_funcao',
     '        tel32_cell(0x1c2, 0x2f8, i, job_colour(TEL32_COLOUR_BDDE), JOB_TEXT as u32);',
     '        tel32_cell(0x1c2, 0x2f8, i, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32);',
     'tel32_1 eidos rows'),
]
