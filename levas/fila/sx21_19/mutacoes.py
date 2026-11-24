# Mutacoes (sx21_19). Rodam nas partidas e nas transferencias belgas e alemas; caem pelo comparador (e pela carreira,
# quando mudam uma escalacao ou uma transferencia).
MUTACOES = [
    ('sx21_19_belgica_3',
     'fill_squad_restrictions_with(out, &[(6, 0x15), (8, 2)]); // sx21_19: two in Belgium',
     'fill_squad_restrictions_with(out, &[(6, 0x15), (8, 3)]); // mutation: three',
     'sx21_19 belgium squad limits'),
    ('sx21_19_alemanha_4',
     'fill_squad_restrictions_with(out, &[(5, if cup { 3 } else { 5 })]); // sx21_19: three in the German Cup',
     'fill_squad_restrictions_with(out, &[(5, if cup { 4 } else { 5 })]); // mutation: four',
     'sx21_19 germany squad limits'),
    ('sx21_19_alemanha_segunda_janela',
     'return refuse(); // sx21_19: only the top two divisions buy in the second window',
     '// mutation: lower divisions buy in the second window',
     'sx21_19 germany lower division in the second window'),
]
