# Mutacoes (sx21_12). Rodam nas partidas australianas, americanas e japonesas; caem pelo comparador lado a lado (e
# pela carreira, quando o limite muda uma escalacao).
MUTACOES = [
    ('sx21_12_australia_5',
     'fill_squad_restrictions_with(out, &[(2, 6)]); // sx21_12: six foreign players in Australia',
     'fill_squad_restrictions_with(out, &[(2, 5)]); // mutation: five',
     'sx21_12 australia squad limits'),
    ('sx21_12_eua_3',
     'write_u8(out + 2, 4); // sx21_12: four foreign players in the USA',
     'write_u8(out + 2, 3); // mutation: three',
     'sx21_12 usa squad limits'),
    ('sx21_12_japao_2',
     'fill_squad_restrictions_with(out, &[(2, 3)]); // sx21_12: three foreign players in Japan',
     'fill_squad_restrictions_with(out, &[(2, 2)]); // mutation: two',
     'sx21_12 japan squad limits'),
]
