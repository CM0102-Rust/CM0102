# Mutações (rxs1_2), nos textos (comparador lado a lado do texto): os jogadores trocados no corpo da troca; a
# nacionalidade fora do corpo do prazo; os clubes trocados na oferta de empréstimo. Acusadas pelo comparador na
# primeira notícia de cada tipo; contagem 0 até 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rxs1_2_troca',
     'fmt(NEWS_TEXT_BUFFER, t, g0, g0, g1, g1, c0, np, c1, tp, money.as_ptr() as usize); // rxs1_2: exchange player removed',
     'fmt(NEWS_TEXT_BUFFER, t, g0, g0, g1, g1, c0, tp, c1, np, money.as_ptr() as usize); // mutation',
     'rxs1_2 exchange player removed with the fee adjusted'),
    ('rxs1_2_prazo',
     'fmt(NEWS_TEXT_BUFFER, 0xa9c218, 0x64, gb, gb, 0x64, 0x64, np, buyer, ap); // rxs1_2: on hold, no seller',
     'fmt(NEWS_TEXT_BUFFER, 0xa9c218, 0x64, gb, gb, 0x64, 0x64, np, buyer, np); // mutation',
     'rxs1_2 offer on hold without a seller'),
    ('rxs1_2_emprestimo',
     'fmt(NEWS_TEXT_BUFFER, 0xa9c550, gb, gb, gp, gp, 0x64, bidder, parent.addr() + club::SHORT_NAME, np, up); // rxs1_2: loan offer',
     'fmt(NEWS_TEXT_BUFFER, 0xa9c550, gb, gb, gp, gp, 0x64, parent.addr() + club::SHORT_NAME, bidder, np, up); // mutation',
     "rxs1_2 loan offer for another club's player"),
]
