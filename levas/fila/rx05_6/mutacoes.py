# Mutações (rx05_6), no texto da oferta recusada (comparador lado a lado do texto): o atraso recusado sai com os
# clubes trocados; acusada pelo comparador na primeira notícia desse tipo (o texto não volta à simulação: a carreira
# não acusa). Só roda no mundo (34 chamadas até 2002-08-01).
MUTACOES = [
    ('rx05_6_clubes_trocados',
     'Delay => fmt(b, template, g4(), g4(), g5, g5, 0x64, n4(), n5, np), // rx05_6: the delay refused',
     'Delay => fmt(b, template, g4(), g4(), g5, g5, 0x64, n5, n4(), np), // mutation',
     'rx05_6 offer rejected body'),
]
