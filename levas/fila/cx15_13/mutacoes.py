# Mutacoes (cx15_13), prova do save. Nenhuma abre caixa. A gravacao asiatica e o corpo comum save_qualifier_list
# (cx15_12), ja mutado la.
# - a carga sem converter os ids da lista asiatica (a impressao da carga muda).
MUTACOES = [
    ('cx15_13_carga_sem_converter',
     '    covered!("cx15_13 asian qualifiers loaded");\n    qualifier_list_from_ids(this, q);',
     '    covered!("cx15_13 asian qualifiers loaded"); // mutation',
     'cx15_13 asian qualifiers loaded'),
]
