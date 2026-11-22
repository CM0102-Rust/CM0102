# Mutations (tel37_1). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat): every screen writes and reads command packets through these.
# - put_ushort writes the word one byte further on;
# - get_char does not advance the position.
MUTACOES = [
    ('tel37_1_put_ushort_offset', 'if network_buffer_room(buf, 2, 0x8b) {\n        write_u16(network_buffer_cursor(buf), value as u16);',
     'if network_buffer_room(buf, 2, 0x8b) {\n        write_u16(network_buffer_cursor(buf) + 1, value as u16);'),
    ('tel37_1_get_char_stays', 'write_u8(out, read_u8(network_buffer_cursor(buf)));\n        network_buffer_advance(buf, 1);',
     'write_u8(out, read_u8(network_buffer_cursor(buf)));\n        network_buffer_advance(buf, 0);'),
]
