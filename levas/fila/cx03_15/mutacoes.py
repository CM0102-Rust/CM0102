# Mutacoes (cx03_15), carreira de 8 ligas. Nenhuma abre caixa; o comparador do progresso acusa a cada posicao gravada.
# - a Merconorte com o 2o lugar valendo 6 no atual;
# - a Libertadores com o 4o lugar valendo 24 no pior;
# - a Mercosul com o mata-mata na fase 3 (a fase 3 dos grupos deixa de ser convertida).
MUTACOES = [
    ('cx03_15_merconorte_segundo_vale_seis',
     '3, ([1, 5, 9, 0xd], [1, 5, 9, 0xd], [3, 5, 9, 0xd]), "cx03_15 merconorte group progress")',
     '3, ([1, 6, 9, 0xd], [1, 5, 9, 0xd], [3, 5, 9, 0xd]), "cx03_15 merconorte group progress") // mutation'),
    ('cx03_15_libertadores_quarto_vale_24',
     '[9, 9, 0x11, 0x19]), "cx03_15 libertadores group progress")',
     '[9, 9, 0x11, 0x18]), "cx03_15 libertadores group progress") // mutation'),
    ('cx03_15_mercosul_mata_mata_na_fase_3',
     'best, worst, 4, ([5, 0xb, 0xb, 0x10]',
     'best, worst, 3 /* mutation */, ([5, 0xb, 0xb, 0x10]'),
]
