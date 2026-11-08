# Mutacoes (cpu_7), todas na band das financas (0x5986b0, chamada todo dia pelas ofertas de transferencia, pelos
# tecnicos e pela diretoria, pelas financas e pelo adiamento): o clube rico passa por confortavel, o confortavel por
# normal e o devendo (injury_word_0c abaixo de 5000 ou acima, pelo limit da injury_word_0c) por normal. As decisoes da IA que
# leem a band mudam (ofertas, orcamento, demissoes) e a carreira muda. Pode escapar a que nenhum chamador que roda ate
# 2002-06-01 distinga as duas faixas trocadas (os contadores mostram que o ramo roda).
MUTACOES = [
    ('cpu7_rica_vira_confortavel', '        return 2; // cpu7: a band rica',
     '        return 1; // mutacao: confortavel',
     'cpu_7 rich'),
    ('cpu7_confortavel_vira_normal', '        return 1; // cpu7: a band confortável',
     '        return 0; // mutacao: normal',
     'cpu_7 comfortable'),
    ('cpu7_devendo_vira_normal', '        return 0xff; // cpu7: a band devendo',
     '        return 0; // mutacao: normal',
     'cpu_7 in debt'),
]
