# Mutacoes (sx21_1). O construtor roda na criacao do jogo (todas as regras); as duas regras nas transferencias da
# carreira ENG (a Inglaterra usa as de base).
# - o limite de estrangeiros 49 em vez de 50 (comparador do construtor);
# - o motivo do limite de estrangeiros 0x15 em vez de 0x14 (comparador);
# - o motivo da oferta acima do limite 0x16 em vez de 0x14 (comparador).
MUTACOES = [
    ('sx21_1_limite_49',
     'write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x32); // sx21_1: 50 foreign players',
     'write_u8(this + transfer_rules::FOREIGN_PLAYER_LIMIT, 0x31); // mutation: 49',
     'sx21_1 base rules'),
    ('sx21_1_motivo_15',
     '*reason = 0x14; // sx21_1: the foreign limit',
     '*reason = 0x15; // mutation: the wrong reason',
     'sx21_1 foreign limit'),
    ('sx21_1_oferta_motivo',
     '*reason = 0x14; // sx21_1: offer over the foreign limit',
     '*reason = 0x16; // mutation: the wrong reason',
     'sx21_1 offer over the foreign limit'),
]
