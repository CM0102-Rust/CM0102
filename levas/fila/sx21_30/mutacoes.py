# Mutacoes (sx21_30), para provar o comparador numa sessao do Lucas (a cadeia nao roda mutacoes nas telas).
MUTACOES = [
    ('sx21_30_italia_hoje',
     "1 => if free { 0x9eaa94 } else { 0x9eaae8 }, // sx21_30: the Italian deadline today",
     "1 => if free { 0x9eaae8 } else { 0x9eaa94 }, // mutation: swapped",
     'sx21_30 italy deadline story'),
    ('sx21_30_noruega_forma',
     '2 => (0xa698a8, 0xe), // sx21_30: the Norwegian deadline next week',
     '2 => (0xa698a8, 2), // mutation: the other adjective form',
     'sx21_30 norway deadline story'),
]
