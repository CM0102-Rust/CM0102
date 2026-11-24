# Mutacoes (sx19_2), pela carreira de 8 ligas ate 2002-06-01 (a Liga espanhola tem cartoes em toda rodada; as
# suspensoes tiram jogadores das escalacoes seguintes):
# - a liga espanhola suspendendo a cada quarto amarelo (e nao a cada quinto);
# - a escala do orgao da copa espanhola 10 -> 8 (acusada pelo comparador do construtor; na carreira, muda a duracao
#   das suspensoes por pontos das partidas da Copa do Rei).
MUTACOES = [
    ('sx19_2_quarto_amarelo',
     'stepped_card_verdict(this, row, fixture, entry, &SPAIN_LEAGUE_RULING_BODY, 5) // sx19_2: every fifth Spanish league booking',
     'stepped_card_verdict(this, row, fixture, entry, &SPAIN_LEAGUE_RULING_BODY, 4) // mutation: every fourth',
     'sx19_2 spanish league cards'),
    ('sx19_2_escala_copa',
     'vtable: 0x96ee54, record_slot: 1, rule_set: 0x1b, scale: 0xa,',
     'vtable: 0x96ee54, record_slot: 1, rule_set: 0x1b, scale: 8, // mutation\n   ',
     'sx19_2 spanish cup body built'),
]
