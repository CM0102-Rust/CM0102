# Mutacoes (sx21_4). Pegas pelos comparadores (valor) e, a do fim do emprestimo, pela carreira ENG (os emprestimos
# ingleses deixam de poder voltar antes do prazo).
MUTACOES = [
    ('sx21_4_slot5_zero',
     '1 // sx21_4: slot 5 allows',
     '0 // mutation: slot 5 refuses',
     'sx21_4 slot 5'),
    ('sx21_4_emprestimo_zero',
     '1 // sx21_4: the loan is allowed',
     '0 // mutation: no loans',
     'sx21_4 base loan check'),
    ('sx21_4_fim_zero',
     '1 // sx21_4: the loan may end',
     '0 // mutation: loans may not end',
     'sx21_4 base loan end'),
]
