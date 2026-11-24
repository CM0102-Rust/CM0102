# Mutacoes (sx21_31), para provar o comparador numa sessao do Lucas (a cadeia nao roda mutacoes nas telas).
MUTACOES = [
    ('sx21_31_argentina_nunca',
     'if state == 3 && window == 1 && story != 0 { // sx21_31: the second window opens',
     'if state == 3 && window == 2 && story != 0 { // mutation: never',
     'sx21_31 argentina second window story'),
    ('sx21_31_alemanha_forma',
     "(1, 0) => Some(Some((0x989fc8, 0xe, false))), // sx21_31: the German lower league deadline today",
     "(1, 0) => Some(Some((0x989fc8, 2, false))), // mutation: the other adjective form",
     'sx21_31 germany deadline story'),
    ('sx21_31_belgica_hoje',
     "(1, 2) => Some(Some((0x989f20, 2, false))), // sx21_31: the Belgian deadline today",
     "(1, 2) => Some(Some((0x989e70, 2, false))), // mutation: next week",
     'sx21_31 belgium deadline story'),
]
