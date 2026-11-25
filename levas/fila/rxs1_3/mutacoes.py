# Mutações (rxs1_3), nos textos (comparador lado a lado do texto): o clube do empréstimo cancelado com o nome curto;
# a segunda pessoa trocada pela primeira; os motivos fora da recusa. Acusadas pelo comparador na primeira notícia de
# cada tipo; contagem 0 até 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rxs1_3_cancelado',
     'fmt(NEWS_TEXT_BUFFER, t, 0x64, g4, g4, g4, g4, np, long4, long4) // rxs1_3: loan cancelled',
     'fmt(NEWS_TEXT_BUFFER, t, 0x64, g4, g4, g4, g4, np, short4, long4) // mutation',
     "rxs1_3 loan cancelled for the loan club's reason"),
    ('rxs1_3_segunda_pessoa',
     'fmt(NEWS_TEXT_BUFFER, 0xa9d25c, 0x64, gs, gs, gb, gb, np, seller, buyer, second.as_ptr() as usize); // rxs1_3: second person',
     'fmt(NEWS_TEXT_BUFFER, 0xa9d25c, 0x64, gs, gs, gb, gb, np, seller, buyer, np); // mutation',
     'rxs1_3 work permit delay for a second person'),
    ('rxs1_3_motivos',
     'fmt(NEWS_TEXT_BUFFER, 0xa94e8c, gc, gc, gb, gb, club, np, bidder, rp); // rxs1_3: rejects with reasons',
     'fmt(NEWS_TEXT_BUFFER, 0xa94e04, gc, gc, gb, gb, club, np, bidder, rp); // mutation',
     'rxs1_3 player rejects move with reasons'),
]
