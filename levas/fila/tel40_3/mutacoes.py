# Rede: so se prova numa partida em rede (duas maquinas). guio_copy_text_to_caller e lado a lado: a mutacao do texto
# e acusada pelo comparador num jogo em rede; a do comando do clique so em modo Rust (o comparador roda a original).
MUTACOES = [
    ('tel40_3_texto_nao_volta', '        tel26_copy_text(target as *mut u8, (item + display_item::TEXT) as *const u8); // tel40_3: the entry\'s text back', '        let _ = target; // mutation: the text does not come back'),
    ('tel40_3_comando_errado', '    send_buffer_put(6, 1, 0x85); // tel40_3: the click command', '    send_buffer_put(8, 1, 0x85); // mutation: the widget command'),
]
