# Mutacoes (sbF_1), carreira com a Alemanha e a Holanda ate 2002-08-01. Nenhuma abre caixa.
# - a tabela alema da outra paridade: as datas das tres rodadas mudam (o comparador acusa na criacao e na virada);
# - a 1a rodada do mata-mata holandes uma semana depois: a data dos jogos muda (o comparador acusa a cada pedido da
#   tabela; a carreira, se o mata-mata for montado antes de 2002-08-01).
MUTACOES = [
    ('sbF_1_paridade_trocada',
     "let odd = (this.year() as i32) % 2 != 0; // the original's signed remainder of the movsx year",
     "let odd = (this.year() as i32) % 2 == 0; // mutation",
     'sbF_1 german league cup rounds'),
    ('sbF_1_holanda_semana_depois',
     'cql10r((0x1a, 7, 0, 3), (0x19, 8, 0, 5, 2, 0), 0xa, (0, 0x283, 0), 4, (0x28, 0x14, 0x28, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF1: the Dutch Cup 1st round',
     'cql10r((0x1a, 7, 0, 3), (2, 9, 0, 5, 2, 0), 0xa, (0, 0x283, 0), 4, (0x28, 0x14, 0x28, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mutation',
     'sbF_1 dutch cup knockout rounds'),
]
