# Mutacoes (sx19_10), pela carreira ENG POL TUR GRE HRV RUS ate 2002-08-01 (copa turca):
# - a escala da copa turca 10 -> 8 (acusada pelo comparador do construtor);
# - a copa turca suspendendo a cada segundo amarelo (e nao a cada quarto).
MUTACOES = [
    ('sx19_10_escala',
     'rule_set: 0x26, scale: 0xa,',
     'rule_set: 0x26, scale: 8, // mutation\n   ',
     'sx19_10 turkish cup body built'),
    ('sx19_10_segundo_amarelo',
     'stepped_card_verdict(this, row, fixture, entry, &TURKEY_CUP_RULING_BODY, 4) // sx19_10: every fourth Turkish cup booking',
     'stepped_card_verdict(this, row, fixture, entry, &TURKEY_CUP_RULING_BODY, 2) // mutation: every second',
     'sx19_10 turkish cup cards'),
]
