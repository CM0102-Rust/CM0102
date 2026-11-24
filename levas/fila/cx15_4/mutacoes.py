# Mutacoes (cx15_4), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - na mesma nacao, a menor sede primeiro: o campo neutro escolhido muda (outro estadio, outro publico);
# - no mesmo continente, a menor sede primeiro (so quando um jogo procura sede no continente; pode escapar da carreira:
#   o comparador da rotina acusa na comparacao).
MUTACOES = [
    ('cx15_4_nacao_menor_primeiro',
     '    covered!("cx15_4 venues of one nation by capacity");\n    stadium_compare_venues_by_capacity(a, b)',
     '    covered!("cx15_4 venues of one nation by capacity");\n    stadium_compare_venues_by_capacity(b, a) // mutation',
     'cx15_4 venues of one nation by capacity'),
    ('cx15_4_continente_menor_primeiro',
     '    covered!("cx15_4 venues of one continent by capacity");\n    stadium_compare_venues_by_capacity(a, b)',
     '    covered!("cx15_4 venues of one continent by capacity");\n    stadium_compare_venues_by_capacity(b, a) // mutation',
     'cx15_4 venues of one continent by capacity'),
]
