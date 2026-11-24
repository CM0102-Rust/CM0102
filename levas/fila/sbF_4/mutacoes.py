# Mutacoes (sbF_4), carreira com a Coreia ate 2002-08-01. Nenhuma abre caixa.
# - a volta da final coreana 7 dias depois da ida e nao 4: a data do jogo muda (o comparador acusa a cada pedido da
#   tabela; a carreira, na final de maio).
# Os registros de carreira so passam ao corpo provado (GreCup/ItaCup) a fase, a rodada e o numero de rodadas: sem
# mutacao propria (o comparador roda so a original, e os historicos nao voltam a simulacao antes da data).
MUTACOES = [
    ('sbF_4_final_volta_7_dias',
     "    cql10r((6, 4, 0, 6), (9, 4, 0, 2, 2, 0), 0x96, (0, 0, 0x83), 6, (2, 1, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),",
     "    cql10r((6, 4, 0, 6), (9, 4, 0, 2, 2, 0), 0x96, (0, 0, 0x83), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)), // mutation",
     'sbF_4 korean league cup knockout rounds'),
]
