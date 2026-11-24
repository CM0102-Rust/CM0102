# Mutacoes (sx21_36), para provar o comparador numa sessao do Lucas (a cadeia nao roda mutacoes nas telas). O
# construtor e a retirada so rodam a original no comparador: a mutacao da taxa aparece no jogo (o valor da proposta).
MUTACOES = [
    ('sx21_36_momento_ignorado',
     'if (b.completion_timing() ^ a.completion_timing()) & 0xf != 0 { // sx21_36: the completion timing',
     'if (b.completion_timing() ^ a.completion_timing()) & 0 != 0 { // mutation: the timing is ignored',
     'sx21_36 same terms'),
]
