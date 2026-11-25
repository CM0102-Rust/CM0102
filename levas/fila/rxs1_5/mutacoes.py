# Mutação (rxs1_5), no texto (comparador lado a lado do texto): o vendedor e o terceiro clube trocados nos modelos que
# citam os três clubes. Acusada pelo comparador na primeira notícia desse padrão; contagem 0 até 2002-08-01: escapa da
# cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rxs1_5_tres_clubes',
     'fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, gn, gn, np, cs, cb, cn) // rxs1_5: three clubs',
     'fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, gn, gn, np, cn, cb, cs) // mutation',
     'rxs1_5 collapsed transfer naming three clubs'),
]
