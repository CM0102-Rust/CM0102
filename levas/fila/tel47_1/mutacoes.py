# As duas primeiras o comparador lado a lado acusa em qualquer partida em rede (o parse no "entrar" de
# host_or_join_game, o format na lista de servidores e nos enderecos locais da tela de rede); a terceira so numa
# partida em rede em modo Rust (o loopback falha e o jogo diz que nao ha TCP/IP).
MUTACOES = [
    ('tel47_1_endereco_invertido', '        write_u8(address + 4 + k, b[k] as u8); // tel47_1: the address bytes in order',
     '        write_u8(address + 4 + k, b[3 - k] as u8); // mutation: the bytes reversed'),
    ('tel47_1_formato_sem_ultimo', '        read_u8(address + 7) as u32); // tel47_1: the four bytes',
     '        0u32); // mutation: the last byte lost'),
    ('tel47_1_loopback_falha', '        ws_connect(client, a, 16) != -1 // tel47_1: the loopback connection made',
     '        ws_connect(client, a, 16) == -1 // mutation: the test inverted'),
]
