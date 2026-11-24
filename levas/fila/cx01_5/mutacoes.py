# Mutacao (cx01_5): os idiomas 0 e 1 (o ingles) passam a usar a tabela do idioma 2: o comparador acusa logo nos
# primeiros rotulos (genero diferente de 0 ou o texto "Cannot find index" que a original nao escreve). Nao abre caixa.
MUTACOES = [
    ('cx01_5_ingles_com_genero',
     '        2 => 0,\n        3 => 2,',
     '        0 | 1 | 2 => 0, // mutation\n        3 => 2,'),
]
