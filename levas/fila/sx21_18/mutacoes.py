# Mutacoes (sx21_18). Rodam nas partidas irlandesas e norte-irlandesas; caem pelo comparador (e pela carreira, quando
# mudam uma escalacao).
MUTACOES = [
    ('sx21_18_irlanda_limite',
     'fill_squad_restrictions_with(out, &[]); // sx21_18: no limits in Ireland',
     'fill_squad_restrictions_with(out, &[(2, 5)]); // mutation: five foreign players',
     'sx21_18 ireland squad limits'),
    ('sx21_18_irlanda_inscrito_joga',
     'return 1; // sx21_18: cup-tied in Ireland',
     'return 0; // mutation: may play',
     'sx21_18 ireland cup-tied'),
    ('sx21_18_irlanda_do_norte_inscrito_joga',
     'return 1; // sx21_18: cup-tied in Northern Ireland',
     'return 0; // mutation: may play',
     'sx21_18 northern ireland cup-tied'),
]
