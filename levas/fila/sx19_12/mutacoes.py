# Mutacoes (sx19_12), pela carreira ENG SCO WAL ate 2002-08-01:
# - a copa escocesa suspendendo so de tres amarelos em diante (e nao de dois);
# - na virada, as suspensoes desta temporada esquecidas (a anterior fica zerada); a virada escocesa roda em 2002
#   dentro da janela, mas o que ela grava so pesa nas suspensoes por pontos da temporada nova: o comparador da virada
#   (lado a lado sobre as entradas) acusa.
MUTACOES = [
    ('sx19_12_copa_tres',
     'if count > 1 { // sx19_12: a Scottish cup ban per two bookings',
     'if count > 2 { // mutation: from three bookings',
     'sx19_12 scottish cup booking ban'),
    ('sx19_12_virada_esquece',
     'write_u8(e + SCOTLAND_PAST_BANS, ((past >> 3) & 7) | ((season & 7) << 3) | (past & 0xc0)); // sx19_12: the bans move back a season',
     'write_u8(e + SCOTLAND_PAST_BANS, ((past >> 3) & 7) | (past & 0xc0)); // mutation: this season forgotten',
     'sx19_12 scottish new season'),
]
