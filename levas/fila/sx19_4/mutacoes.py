# Mutacoes (sx19_4), pela carreira de 8 ligas ate 2002-06-01:
# - a escala do orgao da Oceania 10 -> 8 (acusada pelo comparador do construtor);
# - a Oceania suspendendo a cada segundo amarelo (competicoes da Oceania; se a carreira nao acusar, o comparador das
#   regras acusa na primeira partida com o segundo amarelo de um jogador).
MUTACOES = [
    ('sx19_4_escala',
     'rule_set: 0x23, scale: 0xa,',
     'rule_set: 0x23, scale: 8, // mutation\n   ',
     'sx19_4 oceanian body built'),
    ('sx19_4_segundo_amarelo',
     'plain_card_verdict(row, entry, false, 3, own_rules(&OCEANIA_RULING_BODY)) // sx19_4: every third Oceanian booking',
     'plain_card_verdict(row, entry, false, 2, own_rules(&OCEANIA_RULING_BODY)) // mutation: every second',
     'sx19_4 oceanian cards'),
]
