# Mutations (tel03_5): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat:
# - the tab change writes the wrong tab: caught side by side by the club screen comparer on any tab click (param 4);
# - the squad view's second group (0x40..0x400) not cleared: caught side by side when picking another view of that
#   group on the Squad tab (param 7 keeps the old bit).
MUTACOES = [
    ('tel03_5_aba',
     '            set_screen_param(4, club_tabs_value(w)); // tel03_5 the tab change',
     '            set_screen_param(4, club_tabs_value(w) ^ 1); // tel03_5 mutation',
     'tel03_5 tab change'),
    ('tel03_5_visao',
     '            club_screen_exclusive_pick(view, bit, &[0x103f, 0x7c0, 0x800]) // tel03_5 the squad view groups',
     '            club_screen_exclusive_pick(view, bit, &[0x103f, 0x40, 0x80, 0x100, 0x200, 0x400, 0x800]) // tel03_5 mutation',
     'tel03_5 squad view'),
]
