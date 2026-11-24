# A camada TCP/IP, parte 2 (tcpip.cpp; leitor tel47, rodada 5, 23/11/2026): escutar, conectar, aceitar, mandar e
# receber nas conexoes, os soquetes de difusao e os datagramas, e a varredura dos adaptadores (winipcfg, ip.exe ou
# ipconfig num arquivo). Usa o que o tel47_1 traz (tcpip_object, os ws_*, fill_ipv4_sockaddr, tcpip_close_*,
# tcpip_ensure_winsock, tcpip_tune_stream_socket): os dois itens vao juntos.
# Convencoes conferidas na desmontagem (todas thiscall): open_listen_sockets 0x8ae370 (porta) ret 4; connect_to_host
# 0x8ae640 (porta, sockaddr) ret 8 -> short (ax); accept_connection 0x8ae840 ret 0 -> short; send_to_connection
# 0x8ae8d0 (short, dados, n) ret 0xc; receive_from_connection 0x8ae9c0 (short, buffer, n) ret 0xc;
# open_broadcast_sockets 0x8aea80 (porta) ret 4; send_datagram 0x8aec90 (dados, n, porta, para, tamanho, indice
# short) ret 0x18; receive_datagram 0x8aed80 (buffer, n, de, &tamanho, &indice word) ret 0x14;
# tcpip_scan_adapter_addresses 0x8aee20 (saida, &contagem short) ret 8, sem usar ecx. GetVersionExA pela IAT
# (0x9670c8); _spawnl 0x9533ca, getenv 0x947480, system 0x9473ea, fopen, fgets 0x94700c, fclose cdecl;
# graphics_init_display 0x5e2620 cdecl de 6.
# Quem chama (refs.py): so a rede (Network::Network, host_or_join_game, receive_packet, send_packet,
# get_local_addresses, o setup de rede 0x81f930..0x81fe06) e as proprias rotinas do tcpip.cpp.
# Prologos: todos terminam em fronteira de instrucao, sem call rel32 nem salto (nada para RELATIVAS).
# cm0102-1080.exe: a varredura tem a largura e a altura das duas chamadas a graphics_init_display trocadas pela
# resolucao (0x8aeed4..0x8aef02, 800x600 -> 1920x1080); a copia as le do exe em execucao (ADAPTER_SCAN_*), entao
# serve aos dois exes sem EXE.
# Comparadores: so a original (soquetes, processo, arquivo).
TELAS = True
ROTINAS = [('tcpip_open_listen_sockets', 0x8ae370, [0x81, 0xec, 0xa8, 0x02, 0x00, 0x00]),
           ('tcpip_connect_to_host', 0x8ae640, [0x83, 0xec, 0x1c, 0x53, 0x56]),
           ('tcpip_accept_connection', 0x8ae840, [0x51, 0x53, 0x55, 0x56, 0x8b, 0xf1]),
           ('tcpip_send_to_connection', 0x8ae8d0, [0x51, 0x8b, 0x01, 0x53, 0x55]),
           ('tcpip_receive_from_connection', 0x8ae9c0, [0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x07]),
           ('tcpip_open_broadcast_sockets', 0x8aea80, [0x83, 0xec, 0x20, 0x55, 0x56]),
           ('tcpip_send_datagram', 0x8aec90, [0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c]),
           ('tcpip_receive_datagram', 0x8aed80, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('tcpip_scan_adapter_addresses', 0x8aee20, [0x81, 0xec, 0x88, 0x03, 0x00, 0x00])]
DOCS = ('a camada TCP/IP, parte 2: escutar, conectar, aceitar, mandar e receber, a difusão e os datagramas, e a varredura dos adaptadores',
        'the TCP/IP layer, part 2: listening, connecting, accepting, sending and receiving, broadcasting and datagrams, and the adapter scan',
        [('tcpip_open_listen_sockets', 'TcpIp::open_listen_sockets', '0x8ae370', 'tcpip.cpp', 'thiscall, ret 4',
          'abre um soquete TCP de escuta, não bloqueante, em cada adaptador local na porta dada; vale quando ao menos um abre; defeito: mais de 20 adaptadores estouram a tabela',
          'opens a non-blocking TCP listen socket on every local adapter at the given port; true when at least one opens; defect: more than 20 adapters overflow the table'),
         ('tcpip_connect_to_host', 'TcpIp::connect_to_host', '0x8ae640', 'tcpip.cpp', 'thiscall, ret 8',
          'conecta à máquina dada na primeira vaga livre e devolve a vaga (ou -1); defeitos: a falha do SO_REUSEADDR deixa o soquete aberto e a do connect fecha o número da vaga em vez do soquete',
          'connects to the given host in the first free slot and returns the slot (or -1); defects: a failed SO_REUSEADDR leaves the socket open and a failed connect closes the slot number instead of the socket'),
         ('tcpip_accept_connection', 'TcpIp::accept_connection', '0x8ae840', 'tcpip.cpp', 'thiscall',
          'aceita uma conexão que espera num soquete de escuta e a põe numa vaga livre; defeito: a última vaga da tabela, mesmo livre, recusa a conexão e fica presa para a próxima',
          'accepts a connection waiting on a listen socket into a free slot; defect: the last slot of the table, even free, refuses the connection and stays held for the next one'),
         ('tcpip_send_to_connection', 'TcpIp::send_to_connection', '0x8ae8d0', 'tcpip.cpp', 'thiscall, ret 0xc',
          'manda dados numa conexão, repetindo enquanto o soquete bloquearia; outro erro fecha a vaga (-2); um envio parcial volta como está',
          'sends data on a connection, retrying while the socket would block; another error closes the slot (-2); a partial send comes back as is'),
         ('tcpip_receive_from_connection', 'TcpIp::receive_from_connection', '0x8ae9c0', 'tcpip.cpp', 'thiscall, ret 0xc',
          'lê os dados que esperam numa conexão; nada esperando dá 0, erro ou desconexão fecham a vaga (-2)',
          'reads the data waiting on a connection; nothing waiting gives 0, an error or a disconnection closes the slot (-2)'),
         ('tcpip_open_broadcast_sockets', 'TcpIp::open_broadcast_sockets', '0x8aea80', 'tcpip.cpp', 'thiscall, ret 4',
          'varre os adaptadores para o próprio objeto e abre um soquete UDP de difusão em cada um na porta dada; vale quando ao menos um se liga; defeito: o bind que falha fecha o soquete duas vezes',
          'scans the adapters into the object and opens a UDP broadcast socket on each at the given port; true when at least one binds; defect: a failed bind closes the socket twice'),
         ('tcpip_send_datagram', 'TcpIp::send_datagram', '0x8aec90', 'tcpip.cpp', 'thiscall, ret 0x18',
          'manda um datagrama a um endereço por um soquete de difusão ou, sem endereço, ao endereço de difusão de cada adaptador (a soma dos resultados)',
          "sends a datagram to one address through one broadcast socket or, with no address, to every adapter's broadcast address (the sum of the results)"),
         ('tcpip_receive_datagram', 'TcpIp::receive_datagram', '0x8aed80', 'tcpip.cpp', 'thiscall, ret 0x14',
          'lê o primeiro datagrama que espera nos soquetes de difusão, com o remetente e o índice do soquete',
          'reads the first datagram waiting on the broadcast sockets, with the sender and the socket index'),
         ('tcpip_scan_adapter_addresses', 'tcpip_scan_adapter_addresses', '0x8aee20', 'tcpip.cpp', 'thiscall, ret 8',
          'roda winipcfg (Windows 9x), ip.exe (NT 4) ou ipconfig (2000 em diante, saindo da tela cheia antes) para ~net.out e lê de lá o endereço e a máscara de cada adaptador (sem o 127.0.0.1), calculando o endereço de difusão; as linhas são achadas contando os dois-pontos; defeitos: a última linha é lida duas vezes, sem limite de 20 adaptadores, e o ipconfig com IPv6 desencontra a contagem',
          'runs winipcfg (Windows 9x), ip.exe (NT 4) or ipconfig (2000 on, leaving full screen first) into ~net.out and reads each adapter\'s address and mask from it (without 127.0.0.1), computing the broadcast address; the lines are found by counting colons; defects: the last line is read twice, there is no 20-adapter bound, and ipconfig with IPv6 throws the count off')])
