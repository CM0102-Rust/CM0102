# Mutacoes (sx21_14). Rodam nas partidas espanholas, italianas e francesas; caem pelo comparador (e pela carreira,
# quando mudam uma escalacao).
MUTACOES = [
    ('sx21_14_espanha_amistoso_base',
     'return 0; // sx21_14: friendlies and reserve matches',
     'return base_ineligibility_code(this, player, fixture); // mutation: the base rules',
     'sx21_14 spain friendly or reserve'),
    ('sx21_14_italia_3',
     'fill_squad_restrictions_with(out, &[(6, 0x15), (8, if top { 4 } else { 5 })]); // sx21_14: four in Serie A and B',
     'fill_squad_restrictions_with(out, &[(6, 0x15), (8, if top { 3 } else { 5 })]); // mutation: three',
     'sx21_14 italy squad limits'),
    ('sx21_14_franca_4',
     'fill_squad_restrictions_with(out, &[(5, if top { 5 } else { 3 })]); // sx21_14: five in the French top flight',
     'fill_squad_restrictions_with(out, &[(5, if top { 4 } else { 3 })]); // mutation: four',
     'sx21_14 france squad limits'),
]
