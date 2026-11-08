# Mutacoes. O limit dos convocados (2 -> 0: qualquer convocado na janela barra o jogo; os choques de data 0x5ad780 mudam,
# a tabela e as datas mudam). O visitante com convocados demais liberado (o jogo e marcado onde a original recusa). A
# ordem das convocacoes invertida (consistente: o qsort termina): a 0x776fc0 para no primeiro registro depois da janela
# e passa a nao ver os convocados, e as listas de lesoes e de transferencias mudam de ordem. Nenhuma na 0x7779b0 (a
# lista das classificadas so vira noticia). O comparador e quente: sem ramo.
MUTACOES = [
    ('cpp_limite_convocados', 'soma > 2 // mais de dois convocados',
     'soma > 0 // mutacao: qualquer convocado barra',
     'cpp_12 called up in the window'),
    ('cpp_visitante_convocado', 'return 0; // o visitante com convocados demais',
     '// mutacao: o visitante nao barra',
     'cpp_12 away side called up'),
    ('cpp_ordem_das_convocacoes', 'r // a convocação mais cedo primeiro',
     '-r // mutacao: a mais tarde primeiro'),
]
