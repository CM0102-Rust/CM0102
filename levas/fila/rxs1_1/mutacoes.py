# Mutações (rxs1_1), nos textos (comparador lado a lado do texto): a porcentagem do corpo da cláusula; a contagem de
# jogos; os clubes do pedido de adiamento trocados. Acusadas pelo comparador na primeira notícia de cada tipo; contagem
# 0 até 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rxs1_1_porcentagem',
     'fmt(NEWS_TEXT_BUFFER, 0xa97290, g1, g1, g2, g2, 0x64, g3, g3, c1, money.as_ptr() as usize, percentage, np, c2, c3); // rxs1_1: sell-on clause',
     'fmt(NEWS_TEXT_BUFFER, 0xa97290, g1, g1, g2, g2, 0x64, g3, g3, c1, money.as_ptr() as usize, percentage + 1, np, c2, c3); // mutation',
     'rxs1_1 sale percentage body'),
    ('rxs1_1_contagem',
     'fmt(NEWS_TEXT_BUFFER, t, gp, gp, gc, gc, 0x64, gp, gp, payer, club, mp, np, count, payer); // rxs1_1: appearance count',
     'fmt(NEWS_TEXT_BUFFER, t, gp, gp, gc, gc, 0x64, gp, gp, payer, club, mp, np, count + 1, payer); // mutation',
     'rxs1_1 appearance money with a count'),
    ('rxs1_1_adiamento',
     'fmt(NEWS_TEXT_BUFFER, t, ga, ga, 0x64, go, go, ca, np, co); // rxs1_1: the delay request',
     'fmt(NEWS_TEXT_BUFFER, t, ga, ga, 0x64, go, go, co, np, ca); // mutation',
     'rxs1_1 delay request body'),
]
