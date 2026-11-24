# Mutacoes (sx21_7). Os construtores rodam na criacao do jogo; as mutacoes caem pelo comparador dos campos.
MUTACOES = [
    ('sx21_7_alemanha_limite',
     'germany_rules_init_transfer_windows); // sx21_7: the Germany vtable',
     'germany_rules_init_transfer_windows);\n    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0x1d); // mutation: 29',
     'sx21_7 germany rules'),
    ('sx21_7_grecia_7',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 6); // sx21_7: six non-EU players in Greece',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 7); // mutation: seven',
     'sx21_7 greece rules'),
    ('sx21_7_italia_39',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0x28); // sx21_7: forty non-EU players in Italy',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0x27); // mutation: 39',
     'sx21_7 italy rules'),
]
