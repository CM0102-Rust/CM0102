# Telas: a carreira automatica nao as roda; o comparador acusa numa sessao do Lucas (JOGAR_COMPARANDO.bat).
# put_long e put_sized_block sao lado a lado; put_sized_block roda num jogo de uma maquina (Display::add_area, 0x560004:
# os pesos das linhas de toda area), put_long so em rede (send_widget_click, o clique de um cliente);
# animate_widgets so a original com o plano (o letreiro e as cores), acusada em modo Rust pela tela (nada gira).
MUTACOES = [
    ('tel46_1_long_trocado', '        write_u32(network_buffer_cursor(buf), value); // tel46_1: the long', '        write_u32(network_buffer_cursor(buf), value ^ 1); // mutation'),
    ('tel46_1_bloco_curto', '    network_buffer_advance(buf, n as u32); // tel46_1: past the block', '    network_buffer_advance(buf, (n as u32).wrapping_sub(1)); // mutation'),
    ('tel46_1_cores_iguais', '    write_u16(item + guio_fields::COLOUR, read_u16(item + guio_fields::TEXT_COLOUR)); // tel46_1: the colours swap', '    write_u16(item + guio_fields::COLOUR, colour); // mutation: no swap'),
]
