# Mutacoes (sx18_3), pela carreira ENG ate 2002-06-01: as partidas das selecoes (amistosos e eliminatorias) passam pelo
# orgao internacional; a suspensao tira o jogador da convocacao/escalacao seguinte da selecao.
# - suspensao a cada terceiro amarelo (e nao a cada segundo).
# A virada (0x7df3c0) fica sem mutacao propria: so roda na virada das competicoes da regra 0, que pode cair fora da
# janela; o comparador confere a tabela compactada quando ela roda.
MUTACOES = [
    ('sx18_3_terceiro_amarelo',
     'plain_card_verdict(row, entry, false, 2, [0xff, 0xff, 0xff, 0]) // sx18_3: every second international booking',
     'plain_card_verdict(row, entry, false, 3, [0xff, 0xff, 0xff, 0]) // mutation: every third',
     'sx18_3 international cards'),
]
