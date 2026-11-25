# Mutações (rxs1_4), nos textos (comparador lado a lado do texto): os clubes trocados na recusa da troca; o segundo
# jogador trocado pelo primeiro na dispensa; o valor pedido no lugar do oferecido na negociação. Acusadas pelo
# comparador na primeira notícia de cada tipo; contagem 0 até 2002-08-01: escapam da cadeia por não rodar (explicado no
# PRONTO.txt).
MUTACOES = [
    ('rxs1_4_troca',
     'fmt(NEWS_TEXT_BUFFER, 0xa950d4, g1, g1, g0, g0, c1, np, c0, tp, money.as_ptr() as usize); // rxs1_4: swap rejected',
     'fmt(NEWS_TEXT_BUFFER, 0xa950d4, g1, g1, g0, g0, c0, np, c1, tp, money.as_ptr() as usize); // mutation',
     'rxs1_4 swap player rejects with the fee adjusted'),
    ('rxs1_4_dispensa',
     'fmt(NEWS_TEXT_BUFFER, 0xa9b964, g5, g5, g5, g5, g4, g4, np, c5, c5, c4, sp, money.as_ptr() as usize); // rxs1_4: released from the exchange',
     'fmt(NEWS_TEXT_BUFFER, 0xa9b964, g5, g5, g5, g5, g4, g4, np, c5, c5, c4, np, money.as_ptr() as usize); // mutation',
     'rxs1_4 released exchange player with the fee increased'),
    ('rxs1_4_negociacao',
     'fmt(NEWS_TEXT_BUFFER, 0xa94470, gs, gs, seller, mp, lp, np) // rxs1_4: money and players',
     'fmt(NEWS_TEXT_BUFFER, 0xa94470, gs, gs, seller, lp, mp, np) // mutation',
     'rxs1_4 negotiation for money and players'),
]
