# O gerente de telas (registros e pacotes): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes
# nesta leva. Estas provam os comparadores numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - o registro novo sem gancho fica na frente (sem a troca com o registro 1): carregar um save (-load ou Carregar Jogo)
#   com um tecnico humano (add_manager_record lado a lado acusa);
# - a primeira linha visivel da rolagem com 1 a mais: rolar qualquer lista (send_area_scroll_row lado a lado acusa);
# - o comando de estado 8 no lugar do 7: qualquer barra de progresso (criar jogo, salvar; a montagem do pacote acusa).
MUTACOES = [
    ('tel40_1_registro_na_frente', '    exe::utils_swap_bytes(r as *mut i32, channel_record_at(sm, c, 1) as *mut i32, 0x30); // tel40_1: behind the current record\n', '    // mutation: the new record stays in front\n'),
    ('tel40_1_linha_mais_um', '        write_u32(network_buffer_cursor(NET_SEND_BUFFER), row); // tel40_1: the first visible row', '        write_u32(network_buffer_cursor(NET_SEND_BUFFER), row.wrapping_add(1)); // mutation: one row more'),
    ('tel40_1_comando_de_estado', '    scrman_command_packet(sm, 7); // tel40_1: the status command', '    scrman_command_packet(sm, 8); // mutation: the widget command'),
]
