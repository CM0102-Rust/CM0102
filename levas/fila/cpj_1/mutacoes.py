# Mutacoes. O construtor com a data um byte antes (recordes fora da impressao: PODE ESCAPAR da carreira; o comparando
# acusa). A permissao sem conferir a marca (o comparando acusa no primeiro sem permissao; a carreira, pelas notas do
# motor). O limit sem o comunitario (o comparando acusa no primeiro comunitario de nacao com a regra 0xe; a carreira,
# se a shortlist decidir).
MUTACOES = [
    ('cpj_mais_velho_data', 'init_default_date(this + 0x18); // a data do mais velho',
     'init_default_date(this + 0x17); // mutacao: a data um byte antes'),
    ('cpj_permissao_marca', 'return 0; // sem a marca da permissão',
     '// mutacao: sem conferir a marca'),
    ('cpj_limite_comunitario', 'return 1; // comunitário',
     '// mutacao: sem o comunitario'),
]
