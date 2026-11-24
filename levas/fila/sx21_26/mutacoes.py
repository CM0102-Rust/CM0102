# Mutacoes (sx21_26). Com a Espanha carregada: mudam quem os clubes espanhois contratam (carreira e comparador).
MUTACOES = [
    ('sx21_26_contratacao_livre',
     'if player != 0 && non_eu_foreigner(player, Club(club).nation().addr()) && restricted { // sx21_26: the top two divisions refuse',
     'if player != 0 && non_eu_foreigner(player, Club(club).nation().addr()) && !restricted { // mutation: reversed',
     'sx21_26 spain non-EU signing refused'),
    ('sx21_26_oferta_livre',
     'return refuse(0x15); // sx21_26: the lower divisions may not bid for non-EU players',
     'return refuse(0x14); // mutation: the wrong reason',
     'sx21_26 spain non-EU bid refused'),
]
