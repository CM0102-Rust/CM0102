# Mutacao (tel17_4): o No da janela de remover jogador de troca ganha o id de Yes (tela de proposta com jogador de
# troca > clicar no jogador / Remove); o comparador da janela ve a lista.
MUTACOES = [
    ('tel17_4_nao_troca',
     "ts_bar_button(bar, 0, TS_NO, 1);",
     "ts_bar_button(bar, 0, TS_NO, 2); // mutation: No with the Yes id",
     'tel17_4 remove exchange player dialog'),
]
