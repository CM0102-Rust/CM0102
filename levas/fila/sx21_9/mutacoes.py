# Mutacoes (sx21_9). Rodam na criacao do jogo; caem pelo comparador dos campos.
MUTACOES = [
    ('sx21_9_russia_limite',
     'russia_rules_init_transfer_windows); // sx21_9: the Russia vtable',
     'russia_rules_init_transfer_windows);\n    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x31); // mutation: 49',
     'sx21_9 russia rules'),
    ('sx21_9_espanha_7',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 6); // sx21_9: six non-EU players in Spain',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 7); // mutation: seven',
     'sx21_9 spain rules'),
]
