# Mutacoes (sx21_16). Rodam nas partidas norueguesas e portuguesas; caem pelo comparador (e pela carreira, quando
# mudam uma escalacao).
MUTACOES = [
    ('sx21_16_noruega_3',
     'fill_squad_restrictions_with(out, &[(5, 2)]); // sx21_16: two non-EU players in Norway',
     'fill_squad_restrictions_with(out, &[(5, 3)]); // mutation: three',
     'sx21_16 norway squad limits'),
    ('sx21_16_portugal_4',
     'fill_squad_restrictions_with(out, &[(3, if lower { 3 } else { 4 })]); // sx21_16: three in the Second Division B',
     'fill_squad_restrictions_with(out, &[(3, 4)]); // mutation: always four',
     'sx21_16 portugal squad limits'),
    ('sx21_16_noruega_depois_da_janela',
     'return 5; // sx21_16: joined after the window closed',
     'return 0; // mutation: may play',
     'sx21_16 norway joined after the window'),
    ('sx21_16_noruega_7_dias',
     'let d = exe::date_add_days(joined, later.as_mut_ptr() as usize, 3); // sx21_16: three days',
     'let d = exe::date_add_days(joined, later.as_mut_ptr() as usize, 10); // mutation: ten days',
     'sx21_16 norway joined three days ago'),
]
