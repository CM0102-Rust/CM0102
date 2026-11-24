# Mutacoes (sbF_9), carreira ENG com a base NORMAL ate 2002-06-01. Nenhuma abre caixa.
# - a 3a rodada do FA Trophy uma semana depois (2/12): a data dos jogos muda (o comparador acusa a cada pedido da
#   tabela; a carreira, na 3a rodada de dezembro);
# - os nove melhores clubes de baixo e nao dez: a lista dos 32 muda e a copa falha em fechar (o comparador acusa: o
#   original devolve 1; a carreira muda os confrontos).
MUTACOES = [
    ('sbF_9_trofeu_semana_depois',
     'cql10r((8, 10, 0, 6), (0x19, 10, 0, 5, 1, 0), 0x14, (0, 0, 2), 4, (0x20, 0x10, 0x20, 0), (2, 1, 0), (0, 0xffff_ffff, 0)), // sbF9: the FA Trophy 3rd round',
     'cql10r((8, 10, 0, 6), (2, 11, 0, 5, 1, 0), 0x14, (0, 0, 2), 4, (0x20, 0x10, 0x20, 0), (2, 1, 0), (0, 0xffff_ffff, 0)), // mutation',
     'sbF_9 fa trophy rounds'),
    ('sbF_9_nove_de_baixo',
     '    for &c in &pool[..0xa] {',
     '    for &c in &pool[..9] { // mutation',
     'sbF_9 fa trophy entrants'),
]
