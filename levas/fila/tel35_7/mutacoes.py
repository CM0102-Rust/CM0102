# Mutations (tel35_7): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat (the comparer runs the copy's periodic work dry and checks the original's menu bar record):
# - the "You Have News" status written as kind 2: caught on the frame the news arrive (any screen with the team menu
#   bar, once new news come in, e.g. after Continue);
# - the header's flashing phase not flipped: caught within a second on any screen with the team menu bar.
MUTACOES = [
    ('tel35_7_news_status_kind',
     'menubar_command_set_status(t, G_GUI_COLOUR_RED, t.text, 0x3e9, 1, 0, 0);',
     'menubar_command_set_status(t, G_GUI_COLOUR_RED, t.text, 0x3e9, 2, 0, 0);',
     'tel35_7 status news'),
    ('tel35_7_header_phase',
     'write_u8(rec + menubar_record::UPDATING, phase);',
     'write_u8(rec + menubar_record::UPDATING, phase ^ 1);',
     'tel35_7 header flips'),
]
