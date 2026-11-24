# Item de TELAS (cx06_1): sem mutacoes na cadeia. A mutacao abaixo e para provar o comparador do texto do progresso numa
# sessao do Lucas jogando comparando (o historico de uma selecao na repescagem diria "Group Stage").
MUTACOES = [
    ('cx06_1_progresso_repescagem',
     'if status == 1 { qualified_for_finals() } else { format(buf, 0x9a751c) } // "Playoff"',
     'if status == 1 { qualified_for_finals() } else { format(buf, 0x99b800) } // mutation',
     'cx06_1 qualifiers playoff progress'),
]
