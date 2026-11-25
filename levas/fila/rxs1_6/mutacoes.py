# Mutações (rxs1_6), nos textos (comparador lado a lado do texto): o técnico no lugar do jogador na lista com
# motivos; os motivos fora do pedido de saída. Acusadas pelo comparador na primeira notícia de cada tipo; contagem 0
# até 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rxs1_6_lista_motivos',
     'fmt(NEWS_TEXT_BUFFER, t, gc, gc, gp, gp, cl, pp, n1, b1, n5, sp); // rxs1_6: listed with reasons',
     'fmt(NEWS_TEXT_BUFFER, t, gc, gc, gp, gp, cl, pp, b1, n1, n5, sp); // mutation',
     "rxs1_6 transfer listed with the player's reasons"),
    ('rxs1_6_pedido',
     'fmt(NEWS_TEXT_BUFFER, 0x9acc1c, gc, gc, np, cl, sp); // rxs1_6: requested to leave',
     'fmt(NEWS_TEXT_BUFFER, 0x9acc1c, gc, gc, np, cl, np); // mutation',
     "rxs1_6 transfer request with the player's reasons"),
]
