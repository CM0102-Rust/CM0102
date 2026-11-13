# Mutacoes (mwd_28): a Segunda Divisao polonesa rebaixa 5 em 2001 em vez de 6 (as situacoes da tabela e o rebaixamento
# de 2002 mudam).
MUTACOES = [
    ('mwd28_polonesa_rebaixa_5',
     'let down = if League(this).year() == 0x7d1 { 6 } else { 4 }; // mwd28: six down in 2001',
     'let down = if League(this).year() == 0x7d1 { 5 } else { 4 }; // mutation: five down',
     'mwd_28 polish second division rules'),
]
