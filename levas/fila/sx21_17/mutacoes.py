# Mutacoes (sx21_17). Rodam nas partidas escocesas; caem pelo comparador (e pela carreira, quando mudam uma
# escalacao).
MUTACOES = [
    ('sx21_17_escocia_1',
     'write_u8(out + 8, 2); // sx21_17: two in the Premier Division',
     'write_u8(out + 8, 1); // mutation: one',
     'sx21_17 scotland squad limits'),
    ('sx21_17_escocia_reservas_4',
     'write_u8(out + 0x17, 3); // sx21_17: three older reserves',
     'write_u8(out + 0x17, 4); // mutation: four',
     'sx21_17 scotland squad limits'),
    ('sx21_17_escocia_rodada',
     'read_u8(entry + 0x1c) as i8 as i32 + 2 { // sx21_17: two rounds later',
     'read_u8(entry + 0x1c) as i8 as i32 + 9 { // mutation: nine rounds later',
     'sx21_17 scotland cup-tied'),
]
