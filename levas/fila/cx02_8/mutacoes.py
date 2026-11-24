# Mutacoes (cx02_8), carreira com Belgica, Holanda, Alemanha e Dinamarca ate 2002-08-01. Nenhuma abre caixa.
# - a Copa da Dinamarca com 5 reservas e nao 4: o banco das partidas muda e o comparador acusa;
# - a Copa da Liga alema com 7 reservas e nao 6: o comparador acusa;
# - o mando trocado na Copa da Alemanha (o clube da Bundesliga em casa): os confrontos mudam de mando.
MUTACOES = [
    ('cx02_8_dinamarca_5_reservas',
     '    cup.set_substitutes_named(4);',
     '    cup.set_substitutes_named(5); // mutation'),
    ('cx02_8_liga_alema_7_reservas',
     '    cup.set_substitutes_named(6);',
     '    cup.set_substitutes_named(7); // mutation'),
    ('cx02_8_mando_trocado',
     '        (true, false) => b,\n        (false, true) => a,',
     '        (true, false) => a, // mutation\n        (false, true) => b,'),
]
