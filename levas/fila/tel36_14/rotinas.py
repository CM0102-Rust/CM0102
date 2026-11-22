# The network game connection (the Network methods kept in national_teams_screens.cpp by the map; reader tel36, the
# screens round, 21/11/2026). Helpers of the network game screens and of the ScreenManager: Network::Network from
# ScreenManager::start_network_session 0x7ff500, get_local_addresses from ScreenManager::get_local_addresses 0x7fc7c0,
# host_or_join_game from setup_create_or_join_network_game 0x8388e0, receive_packet from ScreenManager::monitor
# 0x7fc7e0 and run_network_client 0x7fdc00. They only run in a network game (never in the automatic career).
# Conventions checked in the disassembly, all thiscall on the Network (a TcpIp): Network::Network(ok*, on_connect,
# on_disconnect) ret 0xc -> this (an SEH frame around the TcpIp base, no destructor called); get_local_addresses(out)
# ret 4 -> short; host_or_join_game(join, name) ret 8; receive_packet(out, slot*) ret 8. TcpIp methods (tcpip.cpp,
# thiscall): TcpIp 0x8ae180 (0x10) ret 4; start_winsock 0x8ae260; open_broadcast_sockets 0x8aea80 (port) ret 4;
# open_listen_sockets 0x8ae370 (port) ret 4; send_datagram 0x8aec90 (buf, len, port, to, a, b) ret 0x18;
# parse_ipv4_address 0x8ae0d0 (sockaddr*, text) ret 8; connect_to_host 0x8ae640 (port, sockaddr*) ret 8 -> ax;
# close_broadcast_sockets 0x8aec60; tcpip_scan_adapter_addresses 0x8aee20 (entries, count*) ret 8;
# format_ipv4_address 0x8ae140 (sockaddr*, out) ret 8; accept_connection 0x8ae840 -> ax; receive_datagram 0x8aed80
# (buf, 2000, from*, a*, b*) ret 0x14; can_resolve_address 0x8aee00 (from*) ret 4; receive_from_connection 0x8ae9c0
# (socket, buf, n) ret 0xc. gethostname through 0x944c28 (stdcall import). The sockets are pushed as dwords whose high
# half is the register's (the callee reads the word; the copy passes the word).
# Defects of the original: a "BYE" datagram from a server that is not listed raises the server count by one (an empty
# or stale entry appears); kept.
# Equal in cm0102-1080.exe.
# Comparers: only the original (Winsock and the sockets cannot run twice); the proof is a network game in Rust mode.
TELAS = True
ROTINAS = [('network_network', 0x77a7d0, [0x6a, 0xff, 0x68, 0x68, 0xe0, 0x95, 0x00]),
           ('network_get_local_addresses', 0x77a980, [0x81, 0xec, 0x84, 0x02, 0x00, 0x00]),
           ('network_host_or_join_game', 0x77a9f0, [0x81, 0xec, 0xe4, 0x07, 0x00, 0x00]),
           ('network_receive_packet', 0x77af20, [0x81, 0xec, 0x28, 0x08, 0x00, 0x00])]
DOCS = ('a conexão do jogo em rede',
        'the network game connection',
        [('network_network', 'Network::Network', '0x77a7d0', 'national_teams_screens.cpp', 'thiscall, ret 0xc',
          'o objeto de rede começa sem conexões nem servidores; com o Winsock aberto, abre os soquetes de difusão na porta do jogo (6667) e diz em ok se conseguiu',
          'the network object starts with no connections and no servers; with Winsock started it opens the broadcast sockets on the game port (6667) and reports in ok whether it did'),
         ('network_get_local_addresses', 'Network::get_local_addresses', '0x77a980', 'national_teams_screens.cpp', 'thiscall, ret 4',
          'os endereços da máquina como texto (20 bytes cada); devolve quantos',
          "the machine's addresses as text (20 bytes each); returns how many"),
         ('network_host_or_join_game', 'Network::host_or_join_game', '0x77a9f0', 'national_teams_screens.cpp', 'thiscall, ret 8',
          'hospedar abre os soquetes de escuta, põe a máquina na lista com o nome dado e se anuncia por difusão; entrar conecta ao servidor listado com esse nome (ou ao nome como endereço) e fecha a difusão; devolve se está conectado',
          'hosting opens the listen sockets, lists the machine under the given name and announces it by broadcast; joining connects to the listed server of that name (or to the name as an address) and closes the broadcast; returns whether it is connected'),
         ('network_receive_packet', 'Network::receive_packet', '0x77af20', 'national_teams_screens.cpp', 'thiscall, ret 8',
          'a cada volta: hospedando, aceita conexões e responde aos pedidos da lista; procurando jogo, junta os servidores que respondem (até 9, sem repetir, ordenados pelo nome) e tira os que se despedem (defeito: despedida de quem não está na lista aumenta a contagem); depois lê as conexões (o cabeçalho de 4 bytes com o tamanho, depois o pacote) e entrega um pacote completo com o número da conexão; leitura -1 ou -2 fecha a conexão',
          'each turn: hosting, accepts connections and answers the list requests; looking for a game, collects the servers that answer (up to 9, no repeats, sorted by name) and removes those saying goodbye (defect: a goodbye from one not listed raises the count); then reads the connections (the 4-byte length header, then the packet) and hands out a complete packet with its connection number; a read of -1 or -2 closes the connection')])
