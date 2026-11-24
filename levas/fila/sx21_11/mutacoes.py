# Mutacoes (sx21_11). Caem pelos comparadores (e pela carreira, quando mudam uma escalacao ou um emprestimo).
MUTACOES = [
    ('sx21_11_russia_4',
     'write_u8(out + 2, 3); // sx21_11: three foreign players in the Russian top flight',
     'write_u8(out + 2, 4); // mutation: four',
     'sx21_11 russia squad limits'),
    ('sx21_11_turquia_5',
     'write_u8(out + 2, 6); // sx21_11: six foreign players in the Turkish top flight',
     'write_u8(out + 2, 5); // mutation: five',
     'sx21_11 turkey squad limits'),
    ('sx21_11_grecia_sem_emprestimo',
     '1 // sx21_11: Greek loans are allowed',
     '0 // mutation: no loans',
     'sx21_11 greece loan check'),
]
