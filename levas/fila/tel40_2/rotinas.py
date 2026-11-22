# A vida do objeto de rede e os pacotes dele: destruir um TcpIp e um Network, fechar a conexao de um jogador, mandar
# um pacote, montar e destruir um buffer de rede (os metodos de Network e NetworkBuffer que o mapa poe em
# national_teams_screens.cpp; leitor tel40, rodada 2 das telas, 22/11/2026). Usa network_game, NETWORK_LIST_BYE,
# NETWORK_PORT, network_strlen, tcpip_fn (tel36_14), FN_TCPIP_SEND_DATAGRAM (tel11_2), network_buffer (tel37_1),
# tel26_error_box.
# Convencoes conferidas na desmontagem: todas thiscall. TcpIp::~TcpIp 0x77a870 ret 0 (so e alcancada pelos
# tratadores de desenrolar 0x95e063/0x95e086 do Network::Network); Network::~Network 0x77a890 ret 0 (quadro SEH; quem
# destroi chama ele e depois operator delete); release_slot_socket 0x77ac00 (short jogador 1..16) ret 4; send_packet
# 0x77ac30 (dados, tamanho, jogador short) ret 0xc -> int; NetworkBuffer 0x77b630 (capacidade) ret 4 -> this;
# ~NetworkBuffer 0x77b6e0 ret 0. Os metodos do TcpIp (tcpip.cpp): close_all_sockets 0x8ae2a0 ret 0; close_connection
# 0x8ae340 (short) ret 4; send_to_connection 0x8ae8d0 (short, dados, n) ret 0xc -> int (-2 = fechada); send_datagram
# 0x8aec90 (buf, len, porta, para, a, b) ret 0x18. Os "soquetes" da tabela do Network sao indices de conexao do TcpIp,
# empurrados como dword com a metade alta do registrador (o chamado le a word; a copia passa a word).
# RELATIVAS: o prologo do TcpIp::~TcpIp termina num call rel32 (0x77a873 call 0x8ae2a0): entra em RELATIVAS
# (rust/src/detours.rs) como (0x77a870, 3); sem isso tirar essa entrada.
# O que roda sem rede (conferido nos chamadores): NetworkBuffer::NetworkBuffer (os dois buffers globais antes do
# WinMain, 0x55eefa e 0x55ef2a, e o do gerente de telas 0x7fc3ab, na abertura do jogo) e ~NetworkBuffer (no atexit,
# 0x55ef15/0x55ef45, e 0x7fc5bf). send_packet so com sessao (os chamadores conferem o jogador remoto antes); os
# destrutores e release_slot_socket so com sessao.
# Defeitos do original: o datagrama de despedida manda o texto, os 50 bytes do endereco e mais 50 bytes do que estava
# na pilha (o tamanho e o do texto + 100); a copia manda zeros nesses 50. release_slot_socket fecha a conexao sem
# marcar a vaga livre.
# Comparadores: so a original (soquetes, memoria); o construtor e o destrutor do buffer conferem o plano depois.
# Iguais no cm0102-1080.exe (conferir com assinaturas.py --conferir).
TELAS = True
ROTINAS = [('tcpip_destructor', 0x77a870, [0x56, 0x8b, 0xf1, 0xe8, 0x28, 0x3a, 0x13, 0x00]),
           ('network_destructor', 0x77a890, [0x6a, 0xff, 0x68, 0x8b, 0xe0, 0x95, 0x00]),
           ('network_release_slot_socket', 0x77ac00, [0x66, 0x8b, 0x44, 0x24, 0x04]),
           ('network_send_packet', 0x77ac30, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('network_buffer_construct', 0x77b630, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_destruct', 0x77b6e0, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('a vida do objeto de rede e os pacotes dele: destruir um TcpIp e um Network, fechar a conexão de um jogador, mandar um pacote, montar e destruir um buffer de rede',
        "the network object's life and its packets: destroying a TcpIp and a Network, closing a player's connection, sending a packet and building and destroying a network buffer",
        [('tcpip_destructor', 'TcpIp::~TcpIp', '0x77a870', 'national_teams_screens.cpp', 'thiscall',
          'fecha todos os soquetes e libera a tabela de conexões (só pelo desenrolar de uma exceção na montagem do Network)',
          'closes every socket and frees the connection table (only through the unwinding of an exception while the Network is built)'),
         ('network_destructor', 'Network::~Network', '0x77a890', 'national_teams_screens.cpp', 'thiscall',
          'hospedando, despede-se dos servidores da lista ("CM3 LIST SERVER BYE" com o endereço, por difusão; defeito: mais 50 bytes do que estava na pilha); depois fecha os soquetes e libera a tabela',
          'when hosting, says goodbye to the list servers ("CM3 LIST SERVER BYE" with the address, broadcast; defect: 50 more bytes of whatever the stack held); then closes the sockets and frees the table'),
         ('network_release_slot_socket', 'Network::release_slot_socket', '0x77ac00', 'national_teams_screens.cpp', 'thiscall, ret 4',
          'fecha a conexão do jogador 1 a 16 (outro número nada); a vaga não fica marcada livre',
          'closes the connection of player 1 to 16 (any other number nothing); the slot is not marked free'),
         ('network_send_packet', 'Network::send_packet', '0x77ac30', 'national_teams_screens.cpp', 'thiscall, ret 0xc',
          'manda um pacote, antecedido do tamanho em 4 bytes, à conexão de um jogador ou, hospedando, a todas para o jogador 0; conexão achada fechada fica livre (e, hospedando, o gancho de saída é avisado); tamanho 0 ou menos dá a caixa de erro, 50000 ou mais devolve 0; devolve se o último envio deu certo',
          "sends a packet, preceded by its 4-byte length, to one player's connection or, when hosting, to all of them for player 0; a connection found closed is freed (and, when hosting, the leave hook told); a length of 0 or less gives the error box, 50000 or more returns 0; returns whether the last send worked"),
         ('network_buffer_construct', 'NetworkBuffer::NetworkBuffer', '0x77b630', 'national_teams_screens.cpp', 'thiscall, ret 4',
          'um buffer de rede começa vazio com a capacidade dada e a memória própria; sem memória o jogo para (caixa de erro, limpeza, saída)',
          'a network buffer starts empty with the given capacity and its own storage; with no memory the game stops (error box, cleanup, exit)'),
         ('network_buffer_destruct', 'NetworkBuffer::~NetworkBuffer', '0x77b6e0', 'national_teams_screens.cpp', 'thiscall',
          'libera a memória do buffer, quando há, e zera o ponteiro',
          "frees the buffer's storage, when it has one, and clears the pointer")])
