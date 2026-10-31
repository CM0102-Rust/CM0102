# Mutacoes (cpu_7), todas na faixa das financas (0x5986b0, chamada todo dia pelas ofertas de transferencia, pelos
# tecnicos e pela diretoria, pelas financas e pelo adiamento): o clube rico passa por confortavel, o confortavel por
# normal e o devendo (reputacao abaixo de 5000 ou acima, pelo limite da reputacao) por normal. As decisoes da IA que
# leem a faixa mudam (ofertas, orcamento, demissoes) e a carreira muda. Pode escapar a que nenhum chamador que roda ate
# 2002-06-01 distinga as duas faixas trocadas (os contadores mostram que o ramo roda).
MUTACOES = [
    ('cpu7_rica_vira_confortavel', '        return 2; // cpu7: a faixa rica',
     '        return 1; // mutacao: confortavel',
     'cpu_7 rica'),
    ('cpu7_confortavel_vira_normal', '        return 1; // cpu7: a faixa confortável',
     '        return 0; // mutacao: normal',
     'cpu_7 confortavel'),
    ('cpu7_devendo_vira_normal', '        return 0xff; // cpu7: a faixa devendo',
     '        return 0; // mutacao: normal',
     'cpu_7 devendo'),
]
