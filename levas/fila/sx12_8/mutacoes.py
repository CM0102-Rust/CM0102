# Mutacoes (sx12_8), item de telas: a cadeia nao as roda; provam o comparador numa sessao do Lucas jogando comparando
# (a tela do elenco, numeros das camisas).
MUTACOES = [
    ('sx12_troca_de_numeros',
     '    write_i32(pb, held); // sx12_8: the two holders swapped',
     '    let _ = (pb, held); // mutation: the second number keeps its holder',
     'sx12_8 swap squad numbers'),
    ('sx12_numero_do_jogador',
     '            n as i8 // sx12_8: the player\'s number',
     '            n as i8 + 1 // mutation: the next number',
     'sx12_8 player squad number'),
]
