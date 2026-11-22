# Mutations (tel03_4): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat:
# - the search window of a club 8 -> 9 weeks: the next match comparer checks the fixture the original leaves in param 3
#   against the copy's pick, so it is caught on a club whose previous or next fixture is between 8 and 9 weeks away
#   (Previous/Next on the club screen's Next Match tab);
# - the cancel mode switch stuck: caught side by side by the fixtures comparer (Fixtures tab, the Cancel button).
MUTACOES = [
    ('tel03_4_janela',
     '    let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 }; // tel03_4 the search window',
     '    let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 9 }; // tel03_4 mutation',
     'tel03_4 next match neighbour'),
    ('tel03_4_modo_cancelar',
     '            set_screen_param(0xe, (cancel_mode == 0) as usize); // tel03_4 the cancel mode switch',
     '            set_screen_param(0xe, (cancel_mode != 0) as usize); // tel03_4 mutation',
     'tel03_4 next match neighbour'),
]
