# Mutacoes das listas de pendencias. As suspensoes sem descontar (o comparando roda a original: so o modo Rust e a
# carreira; suspensoes guardadas sao raras: PODE ESCAPAR). As ofertas sem parar no fim das pendentes (so o modo Rust e
# a carreira; PODE ESCAPAR). A decisao com 400 em vez de 500 (o comparando acusa pelo valor; a carreira, se a
# transferencia decidir por ele).
MUTACOES = [
    ('cpj_suspensoes_desconto', '*n = (*n).wrapping_sub(1); // uma a menos',
     '// mutacao: sem descontar'),
    ('cpj_pendentes_fim', 'return; // a última pendente',
     '// mutacao: segue'),
    ('cpj_decisao_500', 'as i32 / 500;',
     'as i32 / 400; // mutacao: 400'),
]
