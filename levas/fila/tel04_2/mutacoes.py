# Mutations (tel04_2): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat:
# - the tour's continent rule dropped: other nations open (buttons instead of grey text), caught by the tour nation
#   build comparer (Arrange Tour with a club of tier below 5 and All Nations);
# - the decider choice stored in the teams field: caught by the Configure input comparer (the copy) on any Decider
#   click.
MUTACOES = [
    ('tel04_2_regra_do_continente',
     '    !(tier(read_usize(G_FRIENDLY), club) < 5 && read_usize(home + nation::CONTINENT) != read_usize(n + nation::CONTINENT)) // tel04_2 the continent rule',
     '    !(tier(read_usize(G_FRIENDLY), club) < 0 && read_usize(home + nation::CONTINENT) != read_usize(n + nation::CONTINENT)) // tel04_2 mutation',
     'tel04_2 tour nations build'),
    ('tel04_2_desempate',
     '            write_u16(copy + 0xcd, value as u16); // tel04_2 the decider',
     '            write_u16(copy + 0xcf, value as u16); // tel04_2 mutation',
     'tel04_2 tournament settings build'),
]
