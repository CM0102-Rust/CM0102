# A camada TCP/IP, parte 1 (tcpip.cpp; leitor tel47, rodada 5, 23/11/2026): os enderecos IPv4 (ler e escrever o
# "a.b.c.d"), o objeto TcpIp (construtor), o Winsock (start_winsock), fechar soquetes (todos, uma conexao, os de
# difusao), a busca reversa do nome e o teste de loopback. Traz o registro tcpip_object, os envoltorios do Winsock
# (os thunks jmp [IAT] do exe em 0x944c2e..0x944c8e, stdcall) e os auxiliares que o tel47_2 usa.
# Usa: tcpip_fn, network_same, network_copy, network_strlen, NETWORK_PORT, TCPIP_CONNECTIONS (tel36_14/tel40_2),
# tel26_error_box, c_string_bytes.
# Convencoes conferidas na desmontagem (todas thiscall): parse_ipv4_address 0x8ae0d0 (saida, texto) ret 8, sem usar
# ecx; format_ipv4_address 0x8ae140 (sockaddr, saida) ret 8, sem usar ecx; TcpIp::TcpIp 0x8ae180 (short) ret 4 ->
# this; start_winsock 0x8ae260 ret 0; close_all_sockets 0x8ae2a0 ret 0; close_connection 0x8ae340 (short) ret 4;
# close_broadcast_sockets 0x8aec60 ret 0; can_resolve_address 0x8aee00 (sockaddr) ret 4, sem usar ecx;
# test_loopback_connection 0x8af290 ret 0. sscanf 0x946fd8 e sprintf 0x944c9f cdecl.
# Quem chama (refs.py): so a rede (Network::Network 0x77a7d0, host_or_join_game, receive_packet,
# get_local_addresses, os destrutores) e as proprias rotinas do tcpip.cpp; nada roda num jogo de uma maquina so.
# Prologos: todos terminam em fronteira de instrucao, sem call rel32 nem salto (nada para RELATIVAS).
# Iguais no cm0102-1080.exe (os bytes trocados do tcpip.cpp, 0x8aeed4..0x8aef02, sao do tel47_2).
# Comparadores: parse e format lado a lado (puras; no parse so os bytes que o sscanf leu); o construtor e os tres
# fechamentos so a original com o plano conferido depois; start_winsock, can_resolve_address e o loopback so a original.
TELAS = True
ROTINAS = [('tcpip_parse_ipv4_address', 0x8ae0d0, [0x83, 0xec, 0x0c, 0x8d, 0x44, 0x24, 0x08]),
           ('tcpip_format_ipv4_address', 0x8ae140, [0x8b, 0x44, 0x24, 0x04, 0x33, 0xc9]),
           ('tcpip_construct', 0x8ae180, [0x66, 0x8b, 0x44, 0x24, 0x04]),
           ('tcpip_start_winsock', 0x8ae260, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0xb6, 0x04, 0x00, 0x00]),
           ('tcpip_close_all_sockets', 0x8ae2a0, [0x53, 0x56, 0x57, 0x8b, 0xd9, 0x33, 0xff]),
           ('tcpip_close_connection', 0x8ae340, [0x56, 0x57, 0x8b, 0xf9, 0x8b, 0x07]),
           ('tcpip_close_broadcast_sockets', 0x8aec60, [0x56, 0x57, 0x8d, 0xb1, 0x88, 0x02, 0x00, 0x00]),
           ('tcpip_can_resolve_address', 0x8aee00, [0x8b, 0x44, 0x24, 0x04, 0x6a, 0x02]),
           ('tcpip_test_loopback_connection', 0x8af290, [0x83, 0xec, 0x2c, 0x56, 0x57])]
DOCS = ('a camada TCP/IP, parte 1: endereços IPv4, o objeto TcpIp, o Winsock, fechar soquetes e o teste de loopback',
        'the TCP/IP layer, part 1: IPv4 addresses, the TcpIp object, Winsock, closing sockets and the loopback test',
        [('tcpip_parse_ipv4_address', 'parse_ipv4_address', '0x8ae0d0', 'tcpip.cpp', 'thiscall, ret 8',
          'lê um endereço "a.b.c.d" num sockaddr_in (família AF_INET, porta 0); defeito: com menos de quatro números os bytes que faltam ficam com o resto da pilha',
          'reads an "a.b.c.d" address into a sockaddr_in (family AF_INET, port 0); defect: with fewer than four numbers the missing bytes keep stack residue'),
         ('tcpip_format_ipv4_address', 'format_ipv4_address', '0x8ae140', 'tcpip.cpp', 'thiscall, ret 8',
          'escreve os quatro bytes do endereço de um sockaddr_in como "a.b.c.d"',
          "writes a sockaddr_in's four address bytes as \"a.b.c.d\""),
         ('tcpip_construct', 'TcpIp::TcpIp', '0x8ae180', 'tcpip.cpp', 'thiscall, ret 4',
          'o objeto começa com o Winsock parado, os 20 soquetes de escuta e os 20 de difusão fechados e uma tabela de conexões do tamanho dado, toda livre; defeito: sem memória mostra a caixa "tcpip.cpp 124" e escreve na tabela nula',
          'the object starts with Winsock stopped, the 20 listen and 20 broadcast sockets closed and a connection table of the given length, all free; defect: with no memory it shows the "tcpip.cpp 124" box and writes into the null table'),
         ('tcpip_start_winsock', 'TcpIp::start_winsock', '0x8ae260', 'tcpip.cpp', 'thiscall',
          'liga o Winsock 1.1 uma vez e diz se o TCP/IP funciona pelo teste de loopback (feito sempre)',
          'starts Winsock 1.1 once and reports whether TCP/IP works by the loopback test (always run)'),
         ('tcpip_close_all_sockets', 'TcpIp::close_all_sockets', '0x8ae2a0', 'tcpip.cpp', 'thiscall',
          'fecha as conexões, os soquetes de escuta e os de difusão e desliga o Winsock quando este objeto o ligou',
          'closes the connections, the listen and the broadcast sockets and shuts Winsock down when this object started it'),
         ('tcpip_close_connection', 'TcpIp::close_connection', '0x8ae340', 'tcpip.cpp', 'thiscall, ret 4',
          'fecha o soquete de uma vaga de conexão e a deixa livre',
          "closes one connection slot's socket and frees the slot"),
         ('tcpip_close_broadcast_sockets', 'TcpIp::close_broadcast_sockets', '0x8aec60', 'tcpip.cpp', 'thiscall',
          'fecha os 20 soquetes de difusão',
          'closes the 20 broadcast sockets'),
         ('tcpip_can_resolve_address', 'TcpIp::can_resolve_address', '0x8aee00', 'tcpip.cpp', 'thiscall, ret 4',
          'um endereço só vale quando a busca reversa do nome acha o host',
          'an address counts only when a reverse name look-up finds its host'),
         ('tcpip_test_loopback_connection', 'TcpIp::test_loopback_connection', '0x8af290', 'tcpip.cpp', 'thiscall',
          'o TCP/IP funciona quando um soquete escutando em 127.0.0.1:6667 aceita a conexão de um segundo soquete; os dois são fechados',
          'TCP/IP works when a socket listening on 127.0.0.1:6667 takes a connection from a second socket; both are closed')])
