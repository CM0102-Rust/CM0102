# Mutações (rx05_9), nos textos (comparador lado a lado do texto): o vendedor e o comprador trocados na parte do
# clube anterior; o "cancelou o negócio" no lugar do "nova oferta". Acusadas pelo comparador na primeira notícia de
# cada tipo; contagem 0 até 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_9_clubes_trocados',
     'fmt(NEWS_TEXT_BUFFER, 0xa9e1cc, gf, gf, 0x64, gs, gs, gb, gb, former.addr() + club::SHORT_NAME, money.as_ptr() as usize, np, seller.addr() + club::SHORT_NAME, buyer.addr() + club::SHORT_NAME); // rx05_9: fee cut',
     'fmt(NEWS_TEXT_BUFFER, 0xa9e1cc, gf, gf, 0x64, gs, gs, gb, gb, former.addr() + club::SHORT_NAME, money.as_ptr() as usize, np, buyer.addr() + club::SHORT_NAME, seller.addr() + club::SHORT_NAME); // mutation',
     'rx05_9 former club fee cut body'),
    ('rx05_9_desfecho_trocado',
     '0 => 0xa9dcb8, // "<club> have made a new offer."',
     '0 => 0xa9439c, // mutation',
     'rx05_9 loan negotiation outcome'),
]
