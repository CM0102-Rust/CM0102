# Mutacoes (sx21_10). Rodam nas partidas gregas, croatas e polonesas; caem pelo comparador lado a lado (e pela
# carreira, quando o limite muda uma escalacao).
MUTACOES = [
    ('sx21_10_grecia_6',
     'fill_squad_restrictions_with(out, &[(5, if top { 5 } else { 4 })]); // sx21_10: five in the top flight, else four',
     'fill_squad_restrictions_with(out, &[(5, if top { 6 } else { 4 })]); // mutation: six',
     'sx21_10 greece squad limits'),
    ('sx21_10_croacia_3',
     'fill_squad_restrictions_with(out, &[(0, 4)]); // sx21_10: four foreign players in Croatia',
     'fill_squad_restrictions_with(out, &[(0, 3)]); // mutation: three',
     'sx21_10 croatia squad limits'),
    ('sx21_10_polonia_4',
     'fill_squad_restrictions_with(out, &[(2, 5)]); // sx21_10: five foreign players in Poland',
     'fill_squad_restrictions_with(out, &[(2, 4)]); // mutation: four',
     'sx21_10 poland squad limits'),
]
