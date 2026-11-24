# Mutacoes (sx19_13), pela carreira ENG SCO WAL ate 2002-08-01 (League of Wales):
# - a escala galesa 8 -> 10 (acusada pelo comparador do construtor; na carreira, muda a duracao das suspensoes por
#   pontos dos vermelhos);
# - Gales suspendendo a cada terceiro amarelo (e nao a cada quinto).
MUTACOES = [
    ('sx19_13_escala',
     'rule_set: 0x28, scale: 8,',
     'rule_set: 0x28, scale: 0xa, // mutation\n   ',
     'sx19_13 welsh body built'),
    ('sx19_13_terceiro_amarelo',
     'stepped_card_verdict(this, row, fixture, entry, &WALES_RULING_BODY, 5) // sx19_13: every fifth Welsh booking',
     'stepped_card_verdict(this, row, fixture, entry, &WALES_RULING_BODY, 3) // mutation: every third',
     'sx19_13 welsh cards'),
]
