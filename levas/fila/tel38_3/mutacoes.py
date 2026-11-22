# A base da interface (3): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - o botao Voltar/Avancar usavel na fonte do texto cinza (qualquer tela com Back/Next, ex. o perfil de um jogador);
# - o texto de um item de area nao pula o recuo (get_fields, chamada ao desenhar qualquer lista com area).
MUTACOES = [
    ('tel38_3_botao_fonte_cinza',
     "button(SCREEN_MANAGER, area, cell, 0, 0x30, grey, 0xc, 3, pale, text, id as u32, 0); // tel38_3: the usable button",
     "button(SCREEN_MANAGER, area, cell, 0, 0x30, grey, 0x2c, 3, pale, text, id as u32, 0); // mutation: the greyed font"),
    ('tel38_3_texto_sem_recuo',
     "if a != -1 && read_i16(guio_area(item, a) + 0x20e) != -1 { // tel38_3: an area item's text skips the indent",
     "if false && a != -1 && read_i16(guio_area(item, a) + 0x20e) != -1 { // mutation: no indent skipped"),
]
