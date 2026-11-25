# Mutações (rx05_8), nos textos (comparador lado a lado do texto): o modelo da cláusula sempre o do rebaixamento; o
# jogador da troca e o alvo trocados. Acusadas pelo comparador na primeira notícia de cada tipo; contagem 0 até
# 2002-08-01: escapam da cadeia por não rodar (explicado no PRONTO.txt).
MUTACOES = [
    ('rx05_8_sempre_rebaixado',
     'let t = if kind == 3 { 0xa9da60 } else { 0xa9d960 }; // rx05_8: relegated / not promoted',
     'let t = if kind != 0xff { 0xa9da60 } else { 0xa9d960 }; // mutation',
     'rx05_8 release clause competition link'),
    ('rx05_8_nomes_trocados',
     'fmt(NEWS_TEXT_BUFFER, 0xa9d550, gb, gb, gs, gs, 0x64, 0x64, nb, pp, ns, tp, nat); // rx05_8: removed from the swap',
     'fmt(NEWS_TEXT_BUFFER, 0xa9d550, gb, gb, gs, gs, 0x64, 0x64, nb, tp, ns, pp, nat); // mutation',
     'rx05_8 exchange player permit refused with the fee'),
]
