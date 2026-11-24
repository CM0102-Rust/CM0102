# Mutacoes (sx18_9), pela carreira com LIGAS 'ENG ESP ITA FRA BRA JPN AUS USA' ate 2002-06-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_9_desconto',
     'write_u8(entry + ruling_body_entry::YELLOW_CARDS, count.wrapping_sub(3));',
     'write_u8(entry + ruling_body_entry::YELLOW_CARDS, count.wrapping_sub(2)); // mutation',
     'sx18_9 brazilian booking ban'),
    ('sx18_9_escala_regional',
     'rule_set: 7, scale: 5,',
     'rule_set: 7, scale: 6, // mutation\n   ',
     'sx18_9 brazilian regional body built'),
]
