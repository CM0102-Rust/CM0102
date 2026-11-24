# Mutacoes (sx19_15), para o Lucas provar o comparador na sessao dele (a carreira nao abre telas): a ficha de um
# jogador com cartoes na liga norueguesa ou na portuguesa.
MUTACOES = [
    ('sx19_15_nome_noruegues',
     'RulingBodyName::Text(0xa79c30), 0xa79c44, 0xa7) // sx19_15: the Norwegian name',
     'RulingBodyName::Text(0xa79d6c), 0xa79c44, 0xa7) // mutation: the Scottish cup name'),
    ('sx19_15_adjetivo_portugues',
     'RulingBodyName::Nationality(KEY_NATION_PORTUGAL), 0xa79cf8, 0xbc) // sx19_15: the Portuguese name',
     'RulingBodyName::Nationality(KEY_NATION_SPAIN), 0xa79cf8, 0xbc) // mutation: the Spanish adjective'),
]
