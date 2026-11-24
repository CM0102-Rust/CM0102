# Mutacoes (sx21_15). Rodam nas partidas coreanas e argentinas; caem pelo comparador (e pela carreira, quando mudam
# uma escalacao).
MUTACOES = [
    ('sx21_15_coreia_2',
     'fill_squad_restrictions_with(out, &[(0, 3)]); // sx21_15: three foreign players in South Korea',
     'fill_squad_restrictions_with(out, &[(0, 2)]); // mutation: two',
     'sx21_15 korea squad limits'),
    ('sx21_15_coreia_todos_jogam',
     'base_ineligibility_code(this, player, fixture) // sx21_15: the base answer',
     '{ base_ineligibility_code(this, player, fixture); 0 } // mutation: anyone may play',
     'sx21_15 korea eligibility'),
    ('sx21_15_argentina_3',
     'fill_squad_restrictions_with(out, &[(2, 4)]); // sx21_15: four foreign players in Argentina',
     'fill_squad_restrictions_with(out, &[(2, 3)]); // mutation: three',
     'sx21_15 argentina squad limits'),
]
