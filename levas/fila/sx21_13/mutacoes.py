# Mutacoes (sx21_13). Rodam nas partidas brasileiras e espanholas; caem pelo comparador (e pela carreira, quando
# mudam uma escalacao).
MUTACOES = [
    ('sx21_13_brasil_3',
     'write_u8(out + 2, 2); // sx21_13: two foreign players in Brazil',
     'write_u8(out + 2, 3); // mutation: three',
     'sx21_13 brazil squad limits'),
    ('sx21_13_brasil_inscrito_joga',
     'return 4; // sx21_13: played for another club',
     'return 0; // mutation: may play',
     'sx21_13 brazil cup-tied'),
    ('sx21_13_espanha_4',
     '3 // sx21_13: three in 2001 and 2002',
     '4 // mutation: four',
     'sx21_13 spain squad limits'),
]
