# A base da interface (1): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat (cada uma deve aparecer como divergencia):
# - o titulo da tela 10 pixels mais baixo (qualquer tela com titulo, ex. o Inbox);
# - o botao sem a propria cor de destaque (qualquer tela com botoes fora de area).
MUTACOES = [
    ('tel38_1_titulo_baixo',
     "scrman_add_item(sm, [1, 0x64, 0xa, 0x316, 0x46, 0, 0, 0x30, colour, 0, 0xc, 7, text_colour, text, 0, 0, 0, u32::MAX]) // tel38_1: the title rectangle",
     "scrman_add_item(sm, [1, 0x64, 0x14, 0x316, 0x46, 0, 0, 0x30, colour, 0, 0xc, 7, text_colour, text, 0, 0, 0, u32::MAX]) // mutation: the title lower"),
    ('tel38_1_botao_sem_destaque',
     "scrman_add_item(sm, [2, x1, y1, x2, y2, 0, 0, flags, colour, colour, font, style, text_colour, text, 0, id, data, u32::MAX]) // tel38_1: a button item",
     "scrman_add_item(sm, [2, x1, y1, x2, y2, 0, 0, flags, colour, 0, font, style, text_colour, text, 0, id, data, u32::MAX]) // mutation: no highlight colour"),
]
