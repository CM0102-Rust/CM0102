# Mutacoes (sx19_1), pela carreira ENG ate 2002-06-01:
# - a escala do orgao sul-americano 10 -> 8 (acusada pelo comparador do construtor, que confere os campos que a
#   original grava contra a classe);
# - o sul-americano suspendendo a cada terceiro amarelo (eliminatorias sul-americanas; se a carreira nao acusar, o
#   comparador das regras acusa na primeira partida com o segundo amarelo).
MUTACOES = [
    ('sx19_1_escala',
     'rule_set: 2, scale: 0xa,',
     'rule_set: 2, scale: 8, // mutation\n   ',
     'sx19_1 south american body built'),
    ('sx19_1_terceiro_amarelo',
     'plain_card_verdict(row, entry, false, 2, own_rules(&SOUTH_AMERICA_RULING_BODY)) // sx19_1: every second South American booking',
     'plain_card_verdict(row, entry, false, 3, own_rules(&SOUTH_AMERICA_RULING_BODY)) // mutation: every third',
     'sx19_1 south american cards'),
]
