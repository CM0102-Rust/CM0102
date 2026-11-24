# Mutacoes (sx18_7), pela carreira com LIGAS 'ENG ESP ITA FRA BRA JPN AUS USA' ate 2002-06-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_7_terceiro_amarelo_copa',
     'if count % 2 == 0 { // sx18_7: every second Italian cup booking',
     'if count % 3 == 0 { // mutation',
     'sx18_7 italian cup booking ban'),
    ('sx18_7_escala_francesa',
     'rule_set: 0xc, scale: 5,',
     'rule_set: 0xc, scale: 6, // mutation\n   ',
     'sx18_7 french body built'),
    ('sx18_7_escala_brasileira',
     'rule_set: 6, scale: 5,',
     'rule_set: 6, scale: 4, // mutation\n   ',
     'sx18_7 brazilian national body built'),
]
