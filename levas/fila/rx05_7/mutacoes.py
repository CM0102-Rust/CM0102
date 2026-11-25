# Mutações (rx05_7), nos textos (comparador lado a lado do texto): os clubes trocados no corpo da verba recusada; o
# marcador de gramática trocado no contrato sem permissão. Acusadas pelo comparador na primeira notícia de cada tipo;
# contagem 0 até 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_7_clubes_trocados',
     'fmt(NEWS_TEXT_BUFFER, 0xa98e74, 0x64, gs, gs, gb, gb, gb, gb, np, ns, nb, nb); // rx05_7: the funds refused',
     'fmt(NEWS_TEXT_BUFFER, 0xa98e74, 0x64, gs, gs, gb, gb, gb, gb, np, nb, ns, ns); // mutation',
     'rx05_7 funds refused body'),
    ('rx05_7_marcador',
     'fmt(NEWS_TEXT_BUFFER, 0xa9db94, 0x64, g, g, np, club.addr() + club::SHORT_NAME); // rx05_7: contract not extended',
     'fmt(NEWS_TEXT_BUFFER, 0xa9db94, 0x65, g, g, np, club.addr() + club::SHORT_NAME); // mutation',
     'rx05_7 contract not extended body'),
]
