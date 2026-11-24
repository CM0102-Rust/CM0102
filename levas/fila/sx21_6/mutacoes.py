# Mutacoes (sx21_6). Os construtores rodam na criacao do jogo; as mutacoes caem pelo comparador dos campos
# (compare_transfer_rules_ctor). Sem mexer na vtable nem nas janelas (quebrariam o jogo).
MUTACOES = [
    ('sx21_6_argentina_21',
     'write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x14); // sx21_6: 20 foreign players',
     'write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x15); // mutation: 21',
     'sx21_6 argentina rules'),
    ('sx21_6_belgica_30',
     'write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0xff); // sx21_6: no non-EU limit',
     '// mutation: the base non-EU limit stays',
     'sx21_6 belgium rules'),
    ('sx21_6_franca_limite',
     'france_rules_init_transfer_windows); // sx21_6: the France vtable',
     'france_rules_init_transfer_windows);\n    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x31); // mutation: 49',
     'sx21_6 france rules'),
]
