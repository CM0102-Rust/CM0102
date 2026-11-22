# The Setup screens (4): no mutation by the career (the screens batch runs none). For a session of Lucas's with
# JOGAR_COMPARANDO.bat, caught by the comparer on the box named:
# - the take control box's buttons swapped in id (No answers like Yes): its display list differs at once (Select Team
#   -> click a club).
MUTACOES = [
    ('tel11_4_botoes_trocados',
     '    setup_box_buttons(0x98e8cc, 0x20, 0x98e8d0, 0x21); // "No", "Yes"',
     '    setup_box_buttons(0x98e8cc, 0x21, 0x98e8d0, 0x20); // tel11_4 mutation',
     'tel11_4 build take control'),
]
