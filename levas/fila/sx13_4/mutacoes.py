# Mutacoes (sx13_4, TELAS: a cadeia nao as roda; para provar o comparador numa sessao do Lucas).
# - O caractere guardado e o seguinte: qualquer tecla de texto (o nome de um arquivo ao gravar) e acusada.
# - O x do mouse vem da palavra alta: qualquer movimento do mouse e acusado.
MUTACOES = [
    ('sx13_4_caractere_seguinte',
     '        WM_CHAR => write_u8(G_LAST_CHAR_CODE, wparam as u8), // sx13_4: the character code',
     '        WM_CHAR => write_u8(G_LAST_CHAR_CODE, (wparam as u8).wrapping_add(1)), // mutation: the next character'),
    ('sx13_4_x_da_palavra_alta',
     '            write_u32(DISPLAY_MOUSE_X, lparam & 0xffff); // sx13_4: x from the low word',
     '            write_u32(DISPLAY_MOUSE_X, lparam >> 16); // mutation: x from the high word'),
]
