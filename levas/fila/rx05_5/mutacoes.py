# Mutações (rx05_5), no texto da transferência acertada (comparador lado a lado do texto): o modelo "num negócio de
# <valor>" troca pelo "por <valor>"; acusada pelo comparador na primeira notícia com o valor acima do dinheiro (o
# texto não volta à simulação: a carreira não acusa). Só roda no mundo (59 chamadas até 2002-08-01).
MUTACOES = [
    ('rx05_5_modelo_trocado',
     'fmt(NEWS_TEXT_BUFFER, 0xa9c0c0, 0x64, g4, g4, g5, g5, np, n4, n5, mp); // rx05_5: in a deal worth',
     'fmt(NEWS_TEXT_BUFFER, 0xa9bf10, 0x64, g4, g4, g5, g5, np, n4, n5, mp); // mutation',
     'rx05_5 deal worth'),
]
