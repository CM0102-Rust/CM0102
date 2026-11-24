# Mutacoes (sx18_6), pela carreira com LIGAS 'ENG ESP ITA FRA BRA JPN AUS USA' ate 2002-06-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_6_quinto_amarelo',
     'if count == 4 || count == 7 ||',
     'if count == 5 || count == 7 || // mutation\n           ',
     'sx18_6 italian league booking ban'),
    ('sx18_6_escala_copa',
     'rule_set: 0x12, scale: 8,',
     'rule_set: 0x12, scale: 9, // mutation\n   ',
     'sx18_6 italian cup body built'),
]
