# Mutations (tel04_3): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat:
# - the reply column of the invited clubs in another column: caught by the build comparer (Invited Clubs of a hosted
#   tournament);
# - Back of the invited clubs leaves the invite session on: caught by the input comparer (+0x29) on Back of a hosted
#   tournament's Invited Clubs.
MUTACOES = [
    ('tel04_3_coluna_da_resposta',
     '            tel26_area_text([area, 3, row, 1, 0, 1, 2, grey, JOB_TEXT as u32]); // tel04_3 the reply column',
     '            tel26_area_text([area, 2, row, 1, 0, 1, 2, grey, JOB_TEXT as u32]); // tel04_3 mutation',
     'tel04_3 invitations build'),
    ('tel04_3_sessao_de_convites',
     '            write_u32(t + 0x29, 0); // tel04_3 the invite session',
     '            write_u32(t + 0x29, 1); // tel04_3 mutation',
     'tel04_3 invitations build'),
]
