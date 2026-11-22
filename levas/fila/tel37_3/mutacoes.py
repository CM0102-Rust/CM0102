# Mutations (tel37_3). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat): any screen resets the Display and adds areas.
# - add_area puts the row count where the column count goes;
# - reset puts command 3 instead of 2 in the packet buffer.
MUTACOES = [
    ('tel37_3_add_area_columns', 'display_packet_put(columns, 1, 0x8a);', 'display_packet_put(rows, 1, 0x8a);'),
    ('tel37_3_reset_command', 'write_u8(network_buffer_cursor(DISPLAY_PACKET_BUFFER), 2);', 'write_u8(network_buffer_cursor(DISPLAY_PACKET_BUFFER), 3);'),
]
