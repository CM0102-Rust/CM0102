# Mutacoes (sx21_28). Com Portugal carregado: mudam quem os clubes portugueses contratam (carreira e comparador).
MUTACOES = [
    ('sx21_28_sete_na_primeira',
     '6 // sx21_28: six in the Premier Division',
     '7 // mutation: seven',
     'sx21_28 portugal non-EU signing refused'),
    ('sx21_28_motivo',
     '*reason = 0x15; // sx21_28: over the non-EU limit',
     '*reason = 0x14; // mutation: the wrong reason',
     'sx21_28 portugal non-EU signing refused'),
    ('sx21_28_comprador_motivo',
     'return refuse(0x15); // sx21_28: the buyer over the non-EU limit',
     'return refuse(0x14); // mutation: the wrong reason',
     'sx21_28 portugal buyer over the non-EU limit'),
]
