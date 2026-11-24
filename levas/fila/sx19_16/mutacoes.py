# Mutacoes (sx19_16), para o Lucas provar o comparador na sessao dele (a carreira nao abre telas): a ficha de um
# jogador com cartoes na liga espanhola ou na galesa.
MUTACOES = [
    ('sx19_16_nome_espanhol',
     'RulingBodyName::Text(0xa79e98), 0xa79ea8, 0xb7) // sx19_16: the Spanish league name',
     'RulingBodyName::Text(0xa79e50), 0xa79ea8, 0xb7) // mutation: the Spanish cup name'),
    ('sx19_16_adjetivo_gales',
     'RulingBodyName::Nationality(KEY_NATION_WALES), 0xa7a050, 0xb9) // sx19_16: the Welsh name',
     'RulingBodyName::Nationality(KEY_NATION_SCOTLAND), 0xa7a050, 0xb9) // mutation: the Scottish adjective'),
]
