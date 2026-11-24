# Mutacoes (sx21_8). Rodam na criacao do jogo; caem pelo comparador dos campos e das janelas.
# - a segunda janela japonesa abrindo em 29 de maio (o dia, sem caixa: 29 e valido);
# - um campo a mais nos construtores do Japao e de Portugal.
MUTACOES = [
    ('sx21_8_japao_29_de_maio',
     '[0x0f, 0x01, 0x06, 0x1e, 0x04, 0x01], // sx21_8: Japan\'s second window opens on 30 May',
     '[0x0f, 0x01, 0x06, 0x1d, 0x04, 0x01], // mutation: 29 May',
     'sx21_8 japan windows'),
    ('sx21_8_japao_limite',
     'japan_rules_init_transfer_windows); // sx21_8: the Japan vtable',
     'japan_rules_init_transfer_windows);\n    write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x31); // mutation: 49',
     'sx21_8 japan rules'),
    ('sx21_8_portugal_limite',
     'portugal_rules_init_transfer_windows); // sx21_8: the Portugal vtable',
     'portugal_rules_init_transfer_windows);\n    write_u8(this + transfer_rules::NON_EU_PLAYER_LIMIT, 0x1d); // mutation: 29',
     'sx21_8 portugal rules'),
]
