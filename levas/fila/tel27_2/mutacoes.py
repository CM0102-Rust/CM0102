# Mutations (tel27_2). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat):
# - the file box lists one empty row fewer (Game > Load Game: the list under "File :" with fewer than six saves);
# - the confirmation box puts "Yes" over "No" (Game > Save Game > an existing name > Ok: the Yes/No box).
MUTACOES = [
    ('tel27_2_empty_rows', "    while row < 6 {\n        tel26_area_text([list, 0, row,",
     "    while row < 5 {\n        tel26_area_text([list, 0, row,"),
    ('tel27_2_yes_column', "comp_lang_format()(JOB_TEXT, 0x98e8d0);\n    tel26_area_button([area, 1, 0,",
     "comp_lang_format()(JOB_TEXT, 0x98e8d0);\n    tel26_area_button([area, 0, 0,"),
]
