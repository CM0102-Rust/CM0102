# Mutacoes (sx19_3), pela carreira de 8 ligas ate 2002-06-01 (MLS e Copa do Rei; as suspensoes tiram jogadores das
# escalacoes seguintes):
# - a copa espanhola suspendendo a cada terceiro amarelo (e nao a cada segundo);
# - a suspensao americana por pontos so a partir de 40 (e nao de 32).
MUTACOES = [
    ('sx19_3_copa_terceiro',
     'stepped_card_verdict(this, row, fixture, entry, &SPAIN_CUP_RULING_BODY, 2) // sx19_3: every second Spanish cup booking',
     'stepped_card_verdict(this, row, fixture, entry, &SPAIN_CUP_RULING_BODY, 3) // mutation: every third',
     'sx19_3 spanish cup cards'),
    ('sx19_3_eua_40_pontos',
     'if points >= 0x20 { // sx19_3: the US 32-point ban',
     'if points >= 0x28 { // mutation: from 40 points',
     'sx19_3 us points ban'),
]
