# Mutacao (tel17_3): o botao Reject da janela de adiamento ganha o id de Accept (uma noticia de pedido de adiamento
# de transferencia > Respond); o comparador da janela ve a lista.
MUTACOES = [
    ('tel17_3_recusar_adiamento',
     "ts_bar_button(bar, 1, 0xa9e3c4, 2); // \"Reject\"",
     "ts_bar_button(bar, 1, 0xa9e3c4, 3); // mutation: Reject with the Accept id",
     'tel17_3 delay request dialog'),
]
