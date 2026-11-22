# Mutations (tel07_3): the screens batch runs no mutations in the chain (the automatic career opens no screen); these
# are for a session of Lucas with JOGAR_COMPARANDO.bat: open any player's profile (click his name).
# - the Injuries & Bans tab gets number 3 instead of 2: another item code in the tab bar, caught by the comparer's
#   display list on any profile;
# - the statistics table's stripes swapped: other colours in the six rows, caught on the profile of any player of the
#   database.
MUTACOES = [
    ('tel07_3_aba_lesoes',
     '    write_u32(item(k) + 0xd0, 2); // tel07_3 the Injuries & Bans tab',
     '    write_u32(item(k) + 0xd0, 3); // tel07_3 mutation',
     'tel07_3 build the staff profile'),
    ('tel07_3_listras',
     '                let colour = if (k as i32 - 1) % 2 != 0 { colour_word(0xaebdd4) } else { colour_word(G_GUI_COLOUR_YELLOW) }; // tel07_3 the stripes',
     '                let colour = if (k as i32 - 1) % 2 == 0 { colour_word(0xaebdd4) } else { colour_word(G_GUI_COLOUR_YELLOW) }; // tel07_3 mutation',
     'tel07_3 build the staff profile'),
]
